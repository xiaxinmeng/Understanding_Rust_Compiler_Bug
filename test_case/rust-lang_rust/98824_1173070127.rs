rust
#![feature(stdsimd)]

use std::arch::x86_64::__m512d;
use std::collections::VecDeque;


#[no_mangle]
pub fn swap_2_arrays_usize(a: &mut [usize;4], b: &mut [usize;4]){
    std::mem::swap(a, b)
}

#[no_mangle]
pub fn swap_2_vec_deques(a: &mut VecDeque<u32>, b: &mut VecDeque<u32>){
    std::mem::swap(a, b)
}

#[no_mangle]
pub fn swap_avx512_std(x: &mut __m512d, y: &mut __m512d) {
    std::mem::swap(x, y);
}

#[no_mangle]
pub fn swap_2_vecs(a: &mut Vec<u32>, b: &mut Vec<u32>){
    std::mem::swap(a, b);
}

#[no_mangle]
pub fn swap_2_arrays_2_u32(a: &mut [u32;2], b: &mut [u32;2]){
    std::mem::swap(a, b);
}

#[no_mangle]
pub fn swap_2_arrays_3_u32(a: &mut [u32;3], b: &mut [u32;3]){
    std::mem::swap(a, b);
}

#[no_mangle]
pub fn swap_2_arrays_4_32(a: &mut [u32;4], b: &mut [u32;4]){
    std::mem::swap(a, b);
}
