
pub enum Bar {
    A = 1,
    B = 2,
    C = 3,
}

pub enum Baz {
    D = 4,
    E = 5,
    F = 6,
}

pub enum Foo {
    Bar(Bar),
    Baz(Baz),
}
