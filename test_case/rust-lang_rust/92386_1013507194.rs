rust
#![deny(unused_lifetimes)]

pub trait Foo<'a> { //~ ERROR lifetime parameter `'a` never used
    fn foo(&self);
}

pub struct Bar {
    pub foo: u32,
}
