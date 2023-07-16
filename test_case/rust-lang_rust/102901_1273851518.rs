rs
pub enum Foo {Bar(bool), Baz, Quux, Quux2}

pub fn baz(foo: Foo) -> bool {
    matches!(foo, Foo::Baz)
}

pub enum E {A, Other(char), B}

pub fn x(x: E) -> char {
    match x {
        Other(c) => c,
        _ => 'x',
    }
}


