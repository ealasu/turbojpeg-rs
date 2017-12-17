#![feature(fs_read_write)]

extern crate image;

pub mod bindings;

use image::{Image, ImageDimensions, OwnedImage, Rgb};

pub type Pixel = Rgb<u8>;

pub fn decompress(compressed_image: &[u8]) -> OwnedImage<Pixel> {
    let decompressor = unsafe {
        bindings::tjInitDecompress()
    };

    let mut subsamp = 0;
    let mut width = 0;
    let mut height = 0;
    let mut colorspace = 0;
    
    unsafe {
        let res = bindings::tjDecompressHeader3(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            &mut width,
            &mut height,
            &mut subsamp,
            &mut colorspace
        );
        assert_eq!(res, 0);
    }

    let buf_len = width as usize * height as usize;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(buf_len);
    unsafe {
        pixels.set_len(buf_len);
    }

    unsafe {
        let res = bindings::tjDecompress2(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            pixels.as_mut_ptr() as *mut u8,
            width,
            0, // pitch
            height,
            bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
            bindings::TJFLAG_FASTDCT as ::std::os::raw::c_int);
        assert_eq!(res, 0);
        let res = bindings::tjDestroy(decompressor);
        assert_eq!(res, 0);
    }

    OwnedImage {
        dimensions: ImageDimensions {
            width: width as usize,
            pitch: width as usize,
            height: height as usize,
        },
        pixels: pixels
    }
}

pub fn compress<I>(image: &I) -> Vec<u8> where I: Image<Pixel=Pixel> {
    let subsamp = bindings::TJSAMP::TJSAMP_444 as ::std::os::raw::c_int;
    let mut compressed_image_len: ::std::os::raw::c_ulong = unsafe {
        bindings::tjBufSize(
            image.dimensions().width as i32,
            image.dimensions().height as i32,
            subsamp)
    };
    let mut buf: Vec<u8> = Vec::with_capacity(compressed_image_len as usize);
    let mut compressed_image: *mut u8 = buf.as_mut_ptr();
    unsafe {
        let jpeg_compressor = bindings::tjInitCompress();
        let res = bindings::tjCompress2(
            jpeg_compressor,
            image.as_bytes().as_ptr(),
            image.dimensions().width as i32,
            image.pitch_bytes() as i32,
            image.dimensions().height as i32,
            bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
            &mut compressed_image,
            &mut compressed_image_len,
            subsamp,
            100, // quality
            (bindings::TJFLAG_FASTDCT | bindings::TJFLAG_NOREALLOC) as ::std::os::raw::c_int);
        assert_eq!(res, 0);
        let res = bindings::tjDestroy(jpeg_compressor);
        assert_eq!(res, 0);
        buf.set_len(compressed_image_len as usize);
    }
    buf.shrink_to_fit();
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::mem;
    use std::slice;

    #[test]
    fn test_decompress() {
        let jpeg = fs::read("test/in.jpg").unwrap();
        let image = decompress(&jpeg);
        assert_eq!(image.dimensions.width, 75);
        assert_eq!(image.dimensions.height, 50);
        let expected = fs::read("test/out.dat").unwrap();
        assert_eq!(image.as_bytes(), &expected[..]);
    }

    #[test]
    fn test_compress() {
        let uncompressed = fs::read("test/out.dat").unwrap();
        let uncompressed: Vec<Pixel> = unsafe {
            slice::from_raw_parts(
                uncompressed.as_ptr() as *const Pixel,
                uncompressed.len() / mem::size_of::<Pixel>())
        }.to_vec();
        let image = OwnedImage {
            dimensions: ImageDimensions {
                width: 75,
                pitch: 75,
                height: 50,
            },
            pixels: uncompressed,
        };
        let jpeg = compress(&image);
        assert_eq!(jpeg.len(), 2512);
        let expected = fs::read("test/out.jpg").unwrap();
        assert_eq!(jpeg, expected);
    }
}
