pub mod snappy_sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use snappy_sys::*;

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    // SAFETY: `snappy_validate_compressed_buffer()` reads from the memory
    // pointed to by the first argument. The function never reads more than the
    // number of bytes given by the second argument and does not use the pointer
    // after the function returns. The pointee is guaranteed to be immutable for
    // the duration of the call by the existence of a shared reference.
    unsafe {
        snappy_validate_compressed_buffer(
            src.as_ptr().cast::<core::ffi::c_char>(),
            src.len(),
        ) == 0
    }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len();
        let psrc = src.as_ptr().cast::<core::ffi::c_char>();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst: Vec<u8> = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr().cast::<i8>();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}

pub fn compress2(src: &[u8]) -> Vec<u8> {
    let srclen = src.len();
    let psrc = src.as_ptr().cast::<core::ffi::c_char>();

    // SAFETY: By C API contract, this function does not read or write any
    // memory used by Rust and has no side effects.
    let mut dstlen = unsafe { snappy_max_compressed_length(srclen) };

    let mut dst: Vec<u8> = Vec::with_capacity(dstlen as usize);
    let pdst = dst.as_mut_ptr().cast::<i8>();

    // SAFETY: This function reads `srclen` bytes pointed to by `psrc`. Because
    // of the existenc of a shared reference to `src`, these are valid for read.
    // The function writes up to `dstlen` bytes pointed to by `pdst` and also
    // writes `dstlen`. `dstlen` is valid for write due to existenc of a mutable
    // reference to the allocation. The allocation pointed to by `pdst` is valid
    // for write for `dstlen` bytes as we own the allcoation.
    unsafe { snappy_compress(psrc, srclen, pdst, &mut dstlen) };

    // SAFETY: By C API contract dstlen is not greater than `dst` capacity. The
    // elements up to `dstlen` are initialized by the call to `snappy_compress`
    // above.
    unsafe { dst.set_len(dstlen as usize) };
    dst
}
