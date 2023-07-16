
macro_rules! bit(
    ($i:expr) => { self.0 & (1 << $i) != 0 };
);

impl Foo {
    fn is_bar(self) -> bool { bit!(1) }
}
