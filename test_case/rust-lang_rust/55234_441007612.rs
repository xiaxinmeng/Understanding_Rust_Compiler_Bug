rust
#![feature(impl_trait_in_bindings)] // rustc 1.32.0-nightly (0b9f19dff 2018-11-21) running on x86_64-unknown-linux-gnu

fn main () {
    let _: impl Fn(i32) -> i32 = |x| x;
}
