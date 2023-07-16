Rust
#![feature(const_trait_impl)]
const fn foo(f: &dyn ~const Fn() -> i32) -> i32 {
    f()
}
