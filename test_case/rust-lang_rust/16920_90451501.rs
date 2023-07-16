 rust
#[macro_use] extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
enum FooBar {
    Foo = 17,
    Bar = 42,
}
}

fn main() {
    assert_eq!(FooBar::from_i32(17), Some(FooBar::Foo));
    assert_eq!(FooBar::from_i32(42), Some(FooBar::Bar));
    assert_eq!(FooBar::from_i32(91), None);
}
