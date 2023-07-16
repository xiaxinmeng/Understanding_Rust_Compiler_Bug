Rust
#![crate_type="rlib"]

pub fn foo(get: fn() -> [u64; 128], sink: fn(u32),
           may_panic: fn([u64; 128]) -> u32,
           something_random_with_a_dtor: Box<u32>) {
    sink(may_panic(get()));
    sink(may_panic(get()));
}
