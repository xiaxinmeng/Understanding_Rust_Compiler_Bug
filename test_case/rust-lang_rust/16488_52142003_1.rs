 rust
#[repr(C)]
struct foo_t {
    x: i32
}

struct Foo {
    data: foo_t
}
impl Drop for Foo {
    fn drop(&mut self) { ... }
}
