rust
#![feature(type_alias_impl_trait)]
use core::iter::Map;

trait Thing {
    type Iter: Iterator<Item = u32>;
    fn do_thing(val: u32) -> u32;
}

type DoThing<T: Thing> = impl FnOnce(u32) -> u32;

struct ThingIter<T: Thing> {
    inner: Map<T::Iter, DoThing<T>>,
}
