Rust
#![feature(const_trait_impl)]

const fn foo(f: &impl ~const Fn() -> i32) -> i32 {
    f()
}
