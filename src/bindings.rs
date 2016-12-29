/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub const TJ_NUMSAMP: ::std::os::raw::c_uchar = 6;
pub const TJ_NUMPF: ::std::os::raw::c_uchar = 12;
pub const TJ_NUMCS: ::std::os::raw::c_uchar = 5;
pub const TJFLAG_BOTTOMUP: ::std::os::raw::c_uchar = 2;
pub const TJFLAG_FASTUPSAMPLE: ::std::os::raw::c_ushort = 256;
pub const TJFLAG_NOREALLOC: ::std::os::raw::c_ushort = 1024;
pub const TJFLAG_FASTDCT: ::std::os::raw::c_ushort = 2048;
pub const TJFLAG_ACCURATEDCT: ::std::os::raw::c_ushort = 4096;
pub const TJ_NUMXOP: ::std::os::raw::c_uchar = 8;
pub const TJXOPT_PERFECT: ::std::os::raw::c_uchar = 1;
pub const TJXOPT_TRIM: ::std::os::raw::c_uchar = 2;
pub const TJXOPT_CROP: ::std::os::raw::c_uchar = 4;
pub const TJXOPT_GRAY: ::std::os::raw::c_uchar = 8;
pub const TJXOPT_NOOUTPUT: ::std::os::raw::c_uchar = 16;
pub const TJFLAG_FORCEMMX: ::std::os::raw::c_uchar = 8;
pub const TJFLAG_FORCESSE: ::std::os::raw::c_uchar = 16;
pub const TJFLAG_FORCESSE2: ::std::os::raw::c_uchar = 32;
pub const TJFLAG_FORCESSE3: ::std::os::raw::c_uchar = 128;
pub const NUMSUBOPT: ::std::os::raw::c_uchar = 6;
pub const TJ_BGR: ::std::os::raw::c_uchar = 1;
pub const TJ_BOTTOMUP: ::std::os::raw::c_uchar = 2;
pub const TJ_FORCEMMX: ::std::os::raw::c_uchar = 8;
pub const TJ_FORCESSE: ::std::os::raw::c_uchar = 16;
pub const TJ_FORCESSE2: ::std::os::raw::c_uchar = 32;
pub const TJ_ALPHAFIRST: ::std::os::raw::c_uchar = 64;
pub const TJ_FORCESSE3: ::std::os::raw::c_uchar = 128;
pub const TJ_FASTUPSAMPLE: ::std::os::raw::c_ushort = 256;
pub const TJ_YUV: ::std::os::raw::c_ushort = 512;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TJSAMP {
    TJSAMP_444 = 0,
    TJSAMP_422 = 1,
    TJSAMP_420 = 2,
    TJSAMP_GRAY = 3,
    TJSAMP_440 = 4,
    TJSAMP_411 = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TJPF {
    TJPF_RGB = 0,
    TJPF_BGR = 1,
    TJPF_RGBX = 2,
    TJPF_BGRX = 3,
    TJPF_XBGR = 4,
    TJPF_XRGB = 5,
    TJPF_GRAY = 6,
    TJPF_RGBA = 7,
    TJPF_BGRA = 8,
    TJPF_ABGR = 9,
    TJPF_ARGB = 10,
    TJPF_CMYK = 11,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TJCS {
    TJCS_RGB = 0,
    TJCS_YCbCr = 1,
    TJCS_GRAY = 2,
    TJCS_CMYK = 3,
    TJCS_YCCK = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TJXOP {
    TJXOP_NONE = 0,
    TJXOP_HFLIP = 1,
    TJXOP_VFLIP = 2,
    TJXOP_TRANSPOSE = 3,
    TJXOP_TRANSVERSE = 4,
    TJXOP_ROT90 = 5,
    TJXOP_ROT180 = 6,
    TJXOP_ROT270 = 7,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct tjscalingfactor {
    pub num: ::std::os::raw::c_int,
    pub denom: ::std::os::raw::c_int,
}
impl ::std::default::Default for tjscalingfactor {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct tjregion {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
}
impl ::std::default::Default for tjregion {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct tjtransform {
    pub r: tjregion,
    pub op: ::std::os::raw::c_int,
    pub options: ::std::os::raw::c_int,
    pub data: *mut ::std::os::raw::c_void,
    pub customFilter: ::std::option::Option<unsafe extern "C" fn(coeffs:
                                                                     *mut ::std::os::raw::c_short,
                                                                 arrayRegion:
                                                                     tjregion,
                                                                 planeRegion:
                                                                     tjregion,
                                                                 componentIndex:
                                                                     ::std::os::raw::c_int,
                                                                 transformIndex:
                                                                     ::std::os::raw::c_int,
                                                                 transform:
                                                                     *mut tjtransform)
                                                -> ::std::os::raw::c_int>,
}
impl ::std::default::Default for tjtransform {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type tjhandle = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn tjInitCompress() -> tjhandle;
    pub fn tjCompress2(handle: tjhandle,
                       srcBuf: *const ::std::os::raw::c_uchar,
                       width: ::std::os::raw::c_int,
                       pitch: ::std::os::raw::c_int,
                       height: ::std::os::raw::c_int,
                       pixelFormat: ::std::os::raw::c_int,
                       jpegBuf: *mut *mut ::std::os::raw::c_uchar,
                       jpegSize: *mut ::std::os::raw::c_ulong,
                       jpegSubsamp: ::std::os::raw::c_int,
                       jpegQual: ::std::os::raw::c_int,
                       flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tjCompressFromYUV(handle: tjhandle,
                             srcBuf: *const ::std::os::raw::c_uchar,
                             width: ::std::os::raw::c_int,
                             pad: ::std::os::raw::c_int,
                             height: ::std::os::raw::c_int,
                             subsamp: ::std::os::raw::c_int,
                             jpegBuf: *mut *mut ::std::os::raw::c_uchar,
                             jpegSize: *mut ::std::os::raw::c_ulong,
                             jpegQual: ::std::os::raw::c_int,
                             flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjCompressFromYUVPlanes(handle: tjhandle,
                                   srcPlanes:
                                       *mut *const ::std::os::raw::c_uchar,
                                   width: ::std::os::raw::c_int,
                                   strides: *const ::std::os::raw::c_int,
                                   height: ::std::os::raw::c_int,
                                   subsamp: ::std::os::raw::c_int,
                                   jpegBuf: *mut *mut ::std::os::raw::c_uchar,
                                   jpegSize: *mut ::std::os::raw::c_ulong,
                                   jpegQual: ::std::os::raw::c_int,
                                   flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjBufSize(width: ::std::os::raw::c_int,
                     height: ::std::os::raw::c_int,
                     jpegSubsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn tjBufSizeYUV2(width: ::std::os::raw::c_int,
                         pad: ::std::os::raw::c_int,
                         height: ::std::os::raw::c_int,
                         subsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn tjPlaneSizeYUV(componentID: ::std::os::raw::c_int,
                          width: ::std::os::raw::c_int,
                          stride: ::std::os::raw::c_int,
                          height: ::std::os::raw::c_int,
                          subsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn tjPlaneWidth(componentID: ::std::os::raw::c_int,
                        width: ::std::os::raw::c_int,
                        subsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjPlaneHeight(componentID: ::std::os::raw::c_int,
                         height: ::std::os::raw::c_int,
                         subsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjEncodeYUV3(handle: tjhandle,
                        srcBuf: *const ::std::os::raw::c_uchar,
                        width: ::std::os::raw::c_int,
                        pitch: ::std::os::raw::c_int,
                        height: ::std::os::raw::c_int,
                        pixelFormat: ::std::os::raw::c_int,
                        dstBuf: *mut ::std::os::raw::c_uchar,
                        pad: ::std::os::raw::c_int,
                        subsamp: ::std::os::raw::c_int,
                        flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjEncodeYUVPlanes(handle: tjhandle,
                             srcBuf: *const ::std::os::raw::c_uchar,
                             width: ::std::os::raw::c_int,
                             pitch: ::std::os::raw::c_int,
                             height: ::std::os::raw::c_int,
                             pixelFormat: ::std::os::raw::c_int,
                             dstPlanes: *mut *mut ::std::os::raw::c_uchar,
                             strides: *mut ::std::os::raw::c_int,
                             subsamp: ::std::os::raw::c_int,
                             flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjInitDecompress() -> tjhandle;
    pub fn tjDecompressHeader3(handle: tjhandle,
                               jpegBuf: *const ::std::os::raw::c_uchar,
                               jpegSize: ::std::os::raw::c_ulong,
                               width: *mut ::std::os::raw::c_int,
                               height: *mut ::std::os::raw::c_int,
                               jpegSubsamp: *mut ::std::os::raw::c_int,
                               jpegColorspace: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjGetScalingFactors(numscalingfactors: *mut ::std::os::raw::c_int)
     -> *mut tjscalingfactor;
    pub fn tjDecompress2(handle: tjhandle,
                         jpegBuf: *const ::std::os::raw::c_uchar,
                         jpegSize: ::std::os::raw::c_ulong,
                         dstBuf: *mut ::std::os::raw::c_uchar,
                         width: ::std::os::raw::c_int,
                         pitch: ::std::os::raw::c_int,
                         height: ::std::os::raw::c_int,
                         pixelFormat: ::std::os::raw::c_int,
                         flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompressToYUV2(handle: tjhandle,
                              jpegBuf: *const ::std::os::raw::c_uchar,
                              jpegSize: ::std::os::raw::c_ulong,
                              dstBuf: *mut ::std::os::raw::c_uchar,
                              width: ::std::os::raw::c_int,
                              pad: ::std::os::raw::c_int,
                              height: ::std::os::raw::c_int,
                              flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompressToYUVPlanes(handle: tjhandle,
                                   jpegBuf: *const ::std::os::raw::c_uchar,
                                   jpegSize: ::std::os::raw::c_ulong,
                                   dstPlanes:
                                       *mut *mut ::std::os::raw::c_uchar,
                                   width: ::std::os::raw::c_int,
                                   strides: *mut ::std::os::raw::c_int,
                                   height: ::std::os::raw::c_int,
                                   flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecodeYUV(handle: tjhandle,
                       srcBuf: *const ::std::os::raw::c_uchar,
                       pad: ::std::os::raw::c_int,
                       subsamp: ::std::os::raw::c_int,
                       dstBuf: *mut ::std::os::raw::c_uchar,
                       width: ::std::os::raw::c_int,
                       pitch: ::std::os::raw::c_int,
                       height: ::std::os::raw::c_int,
                       pixelFormat: ::std::os::raw::c_int,
                       flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tjDecodeYUVPlanes(handle: tjhandle,
                             srcPlanes: *mut *const ::std::os::raw::c_uchar,
                             strides: *const ::std::os::raw::c_int,
                             subsamp: ::std::os::raw::c_int,
                             dstBuf: *mut ::std::os::raw::c_uchar,
                             width: ::std::os::raw::c_int,
                             pitch: ::std::os::raw::c_int,
                             height: ::std::os::raw::c_int,
                             pixelFormat: ::std::os::raw::c_int,
                             flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjInitTransform() -> tjhandle;
    pub fn tjTransform(handle: tjhandle,
                       jpegBuf: *const ::std::os::raw::c_uchar,
                       jpegSize: ::std::os::raw::c_ulong,
                       n: ::std::os::raw::c_int,
                       dstBufs: *mut *mut ::std::os::raw::c_uchar,
                       dstSizes: *mut ::std::os::raw::c_ulong,
                       transforms: *mut tjtransform,
                       flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tjDestroy(handle: tjhandle) -> ::std::os::raw::c_int;
    pub fn tjAlloc(bytes: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_uchar;
    pub fn tjFree(buffer: *mut ::std::os::raw::c_uchar);
    pub fn tjGetErrorStr() -> *mut ::std::os::raw::c_char;
    pub fn TJBUFSIZE(width: ::std::os::raw::c_int,
                     height: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn TJBUFSIZEYUV(width: ::std::os::raw::c_int,
                        height: ::std::os::raw::c_int,
                        jpegSubsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn tjBufSizeYUV(width: ::std::os::raw::c_int,
                        height: ::std::os::raw::c_int,
                        subsamp: ::std::os::raw::c_int)
     -> ::std::os::raw::c_ulong;
    pub fn tjCompress(handle: tjhandle, srcBuf: *mut ::std::os::raw::c_uchar,
                      width: ::std::os::raw::c_int,
                      pitch: ::std::os::raw::c_int,
                      height: ::std::os::raw::c_int,
                      pixelSize: ::std::os::raw::c_int,
                      dstBuf: *mut ::std::os::raw::c_uchar,
                      compressedSize: *mut ::std::os::raw::c_ulong,
                      jpegSubsamp: ::std::os::raw::c_int,
                      jpegQual: ::std::os::raw::c_int,
                      flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tjEncodeYUV(handle: tjhandle, srcBuf: *mut ::std::os::raw::c_uchar,
                       width: ::std::os::raw::c_int,
                       pitch: ::std::os::raw::c_int,
                       height: ::std::os::raw::c_int,
                       pixelSize: ::std::os::raw::c_int,
                       dstBuf: *mut ::std::os::raw::c_uchar,
                       subsamp: ::std::os::raw::c_int,
                       flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn tjEncodeYUV2(handle: tjhandle,
                        srcBuf: *mut ::std::os::raw::c_uchar,
                        width: ::std::os::raw::c_int,
                        pitch: ::std::os::raw::c_int,
                        height: ::std::os::raw::c_int,
                        pixelFormat: ::std::os::raw::c_int,
                        dstBuf: *mut ::std::os::raw::c_uchar,
                        subsamp: ::std::os::raw::c_int,
                        flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompressHeader(handle: tjhandle,
                              jpegBuf: *mut ::std::os::raw::c_uchar,
                              jpegSize: ::std::os::raw::c_ulong,
                              width: *mut ::std::os::raw::c_int,
                              height: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompressHeader2(handle: tjhandle,
                               jpegBuf: *mut ::std::os::raw::c_uchar,
                               jpegSize: ::std::os::raw::c_ulong,
                               width: *mut ::std::os::raw::c_int,
                               height: *mut ::std::os::raw::c_int,
                               jpegSubsamp: *mut ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompress(handle: tjhandle,
                        jpegBuf: *mut ::std::os::raw::c_uchar,
                        jpegSize: ::std::os::raw::c_ulong,
                        dstBuf: *mut ::std::os::raw::c_uchar,
                        width: ::std::os::raw::c_int,
                        pitch: ::std::os::raw::c_int,
                        height: ::std::os::raw::c_int,
                        pixelSize: ::std::os::raw::c_int,
                        flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn tjDecompressToYUV(handle: tjhandle,
                             jpegBuf: *mut ::std::os::raw::c_uchar,
                             jpegSize: ::std::os::raw::c_ulong,
                             dstBuf: *mut ::std::os::raw::c_uchar,
                             flags: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}