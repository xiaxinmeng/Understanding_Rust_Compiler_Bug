rust
struct Foo<const X: u32, const Y: u32>;
impl<const X: u32, const Y: u32> Foo<X, Y> {
    const OUTPUT: u32 = X + Y;
}
