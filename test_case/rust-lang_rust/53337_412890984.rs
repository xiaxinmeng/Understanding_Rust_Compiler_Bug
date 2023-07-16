diff
#![allow(dead_code, unused_variables)]
#![feature(nll)]
+ #![feature(in_band_lifetimes)]

fn foo(n: usize,
       ignore: &mut [u32; 9],
       data: &'a mut [u32; 9]
) -> &'a [u32] {
    &data[.. n]
}
fn main() {}
