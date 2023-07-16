rust
#![feature(const_generics)]
#![allow(incomplete_features)]

async fn baz<const H: usize>() {
    biz(&Vec::new()).await;
}

const SIZE: usize = 16;

async fn biz(_: &[[u8; SIZE]]) {}
