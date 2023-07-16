Rust
const fn foo(f: ~const fn() -> i32) -> i32 {
    f()
}
