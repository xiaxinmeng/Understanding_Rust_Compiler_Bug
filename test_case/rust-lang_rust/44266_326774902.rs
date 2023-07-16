rust
#[macro_use] extern crate enum_primitive_derive;
extern crate num_traits;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Thing {
    Foo = 0,
    Bar = 1,
}

match Thing::from_u8(0) {
    Some(Thing::Foo) => ...,
    Some(Thing::Bar) => ....,
    None => panic!("out of range"),
}
