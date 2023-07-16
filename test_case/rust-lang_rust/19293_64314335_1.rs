 rust
extern crate meow;

#[test]
fn test() {
    // This compiles, but shouldn’t; it should have to be meow::MyEnum::Foo(5)
    let _: meow::MyEnum = meow::Foo(5);
}
