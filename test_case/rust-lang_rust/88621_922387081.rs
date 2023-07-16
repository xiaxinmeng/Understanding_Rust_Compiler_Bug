rust
// Works with or without repr.
enum Fieldless {
    Foo(),
    Bar{},
    Baz,
}

fn main() {
    assert_eq!(Fieldless::Foo() as u8, 0);
    assert_eq!(Fieldless::Bar{} as u8, 1);
    assert_eq!(Fieldless::Baz as u8, 2);
}
