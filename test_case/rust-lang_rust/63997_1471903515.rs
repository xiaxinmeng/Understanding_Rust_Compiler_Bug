Rust
#![feature(const_trait_impl)]

const fn foo<T: ~const Fn() -> i32>(f: &T) -> i32 {
    f()
}
