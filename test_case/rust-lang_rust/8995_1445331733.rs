
struct Foo {
    bar: Self::Bar,
}
impl Foo {
    pub type Bar = usize;
}
