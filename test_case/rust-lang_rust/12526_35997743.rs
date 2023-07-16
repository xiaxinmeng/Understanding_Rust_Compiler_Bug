 rust
extern {
    fn foo(x: int); // whoops, meant to be c::int, subtly incorrect on x86-64
}
