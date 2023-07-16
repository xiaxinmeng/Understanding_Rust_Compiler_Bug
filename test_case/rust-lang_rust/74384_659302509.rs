rust
#[derive(Copy, Clone)]
pub enum Foo {
    A,
    B(u8),
}

pub fn foo() -> [[[Foo; 50]; 50]; 50] {
    [[[Foo::A; 50]; 50]; 50]
}
