
D:\code\temp\rand\rand_core\src\le.rs:
    1|       |// Copyright 2018 Developers of the Rand project.
    2|       |//
    3|       |// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
    4|       |// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
    5|       |// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
    6|       |// option. This file may not be copied, modified, or distributed
    7|       |// except according to those terms.
    8|       |
    9|       |//! Little-Endian utilities
   10|       |//!
   11|       |//! Little-Endian order has been chosen for internal usage; this makes some
   12|       |//! useful functions available.
   13|       |
   14|       |use core::convert::TryInto;
   15|       |
   16|       |/// Reads unsigned 32 bit integers from `src` into `dst`.
   17|       |#[inline]
   18|      0|pub fn read_u32_into(src: &[u8], dst: &mut [u32]) {
   19|      0|    assert!(src.len() >= 4 * dst.len());
   20|      0|    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(4)) {
   21|      0|        *out = u32::from_le_bytes(chunk.try_into().unwrap());
   22|      0|    }
   23|      0|}
   24|       |
   25|       |/// Reads unsigned 64 bit integers from `src` into `dst`.
   26|       |#[inline]
   27|      0|pub fn read_u64_into(src: &[u8], dst: &mut [u64]) {
   28|      0|    assert!(src.len() >= 8 * dst.len());
   29|      0|    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(8)) {
   30|      0|        *out = u64::from_le_bytes(chunk.try_into().unwrap());
   31|      0|    }
   32|      0|}
