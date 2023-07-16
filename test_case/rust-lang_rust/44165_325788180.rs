rust
#[cfg(foo)]
impl Foo {
    #[cfg(foo)]
    pub fn new() {}
}

#[cfg(foo)]
pub struct Foo {
    #[cfg(foo)]
    pub x: u32,
}

#[cfg(foo)]
pub enum Foo2 {
    #[cfg(foo)]
    Bar {
        #[cfg(foo)]
        baz: u32,
    }
}
