rust
#[derive(Copy, Clone)]
pub struct Foo {
    pub foo: usize,
}

pub fn foo(src: Foo) {
    let mut q = src;
    q.foo = 12;
}
