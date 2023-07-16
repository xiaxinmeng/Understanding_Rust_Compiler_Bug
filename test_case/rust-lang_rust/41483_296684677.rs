rust
struct Foo(u32);

extern {
    fn as_struct(a: Foo);
    fn as_u32(a: u32);
}
