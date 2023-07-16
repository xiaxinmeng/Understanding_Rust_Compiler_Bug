`rust
#![allow(dead_code)]

use std::arch::x86_64::*;

pub struct AVX2VectorBuilder(());

impl AVX2VectorBuilder {
    pub fn new() -> Option<AVX2VectorBuilder> {
        if is_x86_feature_detected!("avx2") {
            Some(AVX2VectorBuilder(()))
        } else {
            None
        }
    }
}


#[repr(transparent)]
pub struct A {
	a: i32
}

