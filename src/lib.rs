pub mod bindings;

use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[image {}x{}]", self.width, self.height)
    }
}

pub fn decompress(compressed_image: &[u8]) -> Image {
    let decompressor = unsafe {
        bindings::tjInitDecompress()
    };

    let mut subsamp = 0;
    let mut width = 0;
    let mut height = 0;
    let mut colorspace = 0;
    
    unsafe {
        bindings::tjDecompressHeader3(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            &mut width,
            &mut height,
            &mut subsamp,
            &mut colorspace
        );
    }

    let buf_len: usize = width as usize * height as usize;
    let mut buffer: Vec<Pixel> = Vec::with_capacity(buf_len);
    unsafe { buffer.set_len(buf_len); }

    unsafe {
        bindings::tjDecompress2(
            decompressor,
            compressed_image.as_ptr(),
            compressed_image.len() as ::std::os::raw::c_ulong,
            buffer.as_mut_ptr() as *mut u8,
            width,
            0, // pitch
            height,
            bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
            bindings::TJFLAG_FASTDCT as ::std::os::raw::c_int);
    }
    unsafe {
        bindings::tjDestroy(decompressor);
    }

    Image {
        width: width as u32,
        height: height as u32,
        pixels: buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    use std::slice;
    use std::mem;

    #[test]
    fn test_decompress() {
        let mut jpeg = Vec::new();
        File::open("test/in.jpg").unwrap().read_to_end(&mut jpeg).unwrap();
        let image = decompress(&jpeg);
        println!("{:?}", image);
        assert_eq!(image.width, 75);
        assert_eq!(image.height, 50);

        let mut expected_uncompressed = Vec::new();
        File::open("test/out.dat").unwrap().read_to_end(&mut expected_uncompressed).unwrap();
        let expected_pixels = unsafe {
            slice::from_raw_parts(
                expected_uncompressed.as_ptr() as *const Pixel,
                expected_uncompressed.len() / mem::size_of::<Pixel>())
        };
        assert_eq!(image.pixels[..], expected_pixels[..]);
    }
}
