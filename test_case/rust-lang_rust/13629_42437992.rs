 rust
pub trait T1 {}
pub struct S;
impl T1 for S {}

pub trait Runtime {
    fn spawn_sibling(&self) -> Box<T1:Send>;
}

pub struct Foo;
impl Runtime for Foo {
    fn spawn_sibling(&self) -> Box<T1> {
        box S as Box<T1>
    }
}

fn main() {}
