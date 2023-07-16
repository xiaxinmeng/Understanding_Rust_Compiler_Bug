rust
pub struct Foo {
    pub x: i32,
}

impl From<i32> for Foo {
    fn from(x: i32) -> Foo {
        Foo {x}
    }
}

pub fn add_1_foo<T>(y: T) -> Foo
where
    Foo: From<T>,
{
    let foo = Foo::from(y);
    let foo_1 = Foo::from(1);
    Foo {
        x: foo.x + foo_1.x
    }
}

pub fn add_1_foo_works(y: i32) -> Foo
{
    let foo = Foo::from(y);
    let foo_1 = Foo::from(1);
    Foo {
        x: foo.x + foo_1.x
    }
}
