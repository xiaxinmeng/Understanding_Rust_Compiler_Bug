rust
#[rustc_layout_scalar_range_end(0xFFFF_FF00)]
struct Bar(u32);
enum Foo {
    A,
    B,
    Other(Bar),
}
