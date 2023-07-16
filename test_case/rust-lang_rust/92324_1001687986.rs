rust
#![feature(generic_associated_types)]

use std::marker::PhantomData;

trait Machine<'a> {
    type Datum<'b>
    where
        Self: 'b;
}

#[derive(Default)]
struct LuaMachine<'a> {
    _phantom: PhantomData<fn(&'a ()) -> &'a ()>,
}

struct LuaDatum<'a, 'b> {
    _machine: &'b LuaMachine<'a>,
}

impl<'a> Machine<'a> for LuaMachine<'a> {
    type Datum<'b>
    where
        Self: 'b,
    = LuaDatum<'a, 'b>;
}

type M<'a> = LuaMachine<'a>;
// This won't work:
type D<'a, 'b> = <M<'a> as Machine>::Datum<'b>;

fn main() {
    let _m: M = Default::default();
    // But this works:
    let _d: <M as Machine>::Datum<'_> = LuaDatum { _machine: &_m };
}
