 rust
#[derive(PartialEq, Debug)]
struct Foo;

fn main() {
    let x = Some(Foo);
    assert_eq!(x.unwrap(), Foo);
    assert_eq!(x.is_some(), true);
}
