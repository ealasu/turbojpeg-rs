extern crate ndarray;
extern crate failure;
#[macro_use] extern crate failure_derive;

pub mod bindings;

use std::mem;
use std::slice;
use std::fmt;
use ndarray::Array2;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Fail)]
pub struct LibError {
    pub code: i32
}

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "turbojpeg error {}", self.code)
    }
}

macro_rules! try_unsafe {
    ($body:expr) => {
        let res = unsafe {
            $body
        };
        if res != 0 {
            return Err(LibError { code: res });
        }
    }
}

pub fn decompress(compressed_image: &[u8]) -> Result<Array2<Pixel>, LibError> {
    let decompressor = unsafe {
        bindings::tjInitDecompress()
    };

    let mut subsamp = 0;
    let mut width = 0;
    let mut height = 0;
    let mut colorspace = 0;
    
    try_unsafe!(bindings::tjDecompressHeader3(
        decompressor,
        compressed_image.as_ptr(),
        compressed_image.len() as ::std::os::raw::c_ulong,
        &mut width,
        &mut height,
        &mut subsamp,
        &mut colorspace
    ));

    let buf_len = width as usize * height as usize;
    let mut pixels: Vec<Pixel> = Vec::with_capacity(buf_len);
    unsafe {
        pixels.set_len(buf_len);
    }

    try_unsafe!(bindings::tjDecompress2(
        decompressor,
        compressed_image.as_ptr(),
        compressed_image.len() as ::std::os::raw::c_ulong,
        pixels.as_mut_ptr() as *mut u8,
        width,
        0, // pitch
        height,
        bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
        bindings::TJFLAG_FASTDCT as ::std::os::raw::c_int));
    try_unsafe!(bindings::tjDestroy(decompressor));

    Ok(Array2::from_shape_vec((height as usize, width as usize), pixels).unwrap())
}


fn slice_as_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * mem::size_of::<T>())
    }
}

pub fn compress(image: &Array2<Pixel>) -> Result<Vec<u8>, LibError> {
    let subsamp = bindings::TJSAMP::TJSAMP_444 as ::std::os::raw::c_int;
    let height = image.shape()[0];
    let width = image.shape()[1];
    let pitch_bytes = width * mem::size_of::<Pixel>();
    let mut compressed_image_len: ::std::os::raw::c_ulong = unsafe {
        bindings::tjBufSize(
            width as i32,
            height as i32,
            subsamp)
    };
    let mut buf: Vec<u8> = Vec::with_capacity(compressed_image_len as usize);
    let mut compressed_image: *mut u8 = buf.as_mut_ptr();
    let jpeg_compressor = unsafe { bindings::tjInitCompress() };
    try_unsafe!(bindings::tjCompress2(
        jpeg_compressor,
        slice_as_bytes(image.view().into_slice().unwrap()).as_ptr(),
        width as i32,
        pitch_bytes as i32,
        height as i32,
        bindings::TJPF::TJPF_RGB as ::std::os::raw::c_int,
        &mut compressed_image,
        &mut compressed_image_len,
        subsamp,
        100, // quality
        (bindings::TJFLAG_FASTDCT | bindings::TJFLAG_NOREALLOC) as ::std::os::raw::c_int));
    try_unsafe!(bindings::tjDestroy(jpeg_compressor));
    unsafe {
        buf.set_len(compressed_image_len as usize);
    }
    buf.shrink_to_fit();
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::slice;
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_decompress() {
        let jpeg = read("test/in.jpg").unwrap();
        let image = decompress(&jpeg).unwrap();
        assert_eq!(image.shape()[1], 75);
        assert_eq!(image.shape()[0], 50);
        assert_eq!(image[[10, 1]], Pixel { r: 189, g: 134, b: 95});
        assert_eq!(image[[1, 20]], Pixel { r: 203, g: 149, b: 103});
        let expected = read("test/out.dat").unwrap();
        assert_eq!(slice_as_bytes(image.view().into_slice().unwrap()), &expected[..]);
    }

    #[test]
    fn test_compress() {
        let uncompressed = read("test/out.dat").unwrap();
        let uncompressed: Vec<Pixel> = unsafe {
            slice::from_raw_parts(
                uncompressed.as_ptr() as *const Pixel,
                uncompressed.len() / mem::size_of::<Pixel>())
        }.to_vec();
        let image = Array2::from_shape_vec((50, 75), uncompressed).unwrap();
        let jpeg = compress(&image).unwrap();
        assert_eq!(jpeg.len(), 2512);
        let expected = read("test/out.jpg").unwrap();
        assert_eq!(jpeg, expected);
    }

    fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        File::open(path)?.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}
