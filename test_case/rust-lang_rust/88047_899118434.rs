rust
#![feature(const_fn_trait_bound)]
#![feature(const_trait_bound_opt_out)]

const fn foo<T: ?const Copy, const N: usize>(a: [T; N]) -> [T; N] {
    a
}
fn main() {
    const SPAM: [(u32, u32); 1] = foo([(1, 1)]);
}
