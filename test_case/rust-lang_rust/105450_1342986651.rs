rust
struct Foo {
    x: i32,
}

impl TryFrom<Foo> for i32 {
    type Error = Infallible;

    fn try_from(value: Foo) -> Result<Self, Self::Error> {
        Ok(value.x)
    }
}

fn main() {
    let foo = Foo { x: 42 };
    let res = Option::try_from(foo); // Result<Option<Foo>, Infallible>
}
