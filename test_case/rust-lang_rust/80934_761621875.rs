rust
struct Quux;

trait Bar {
    type Baz;

    fn rah(&self) -> Self::Baz;
}

impl Bar for Quux {
    type Baz = String;

    fn rah(&self) -> Self::Baz {
        "rah".to_owned()
    }
}

#[test]
fn foo<T: Bar<Baz = String>>() -> <T as Bar>::Baz {
    let q = Quux{};
    <Quux as Bar>::rah(&q)
}
