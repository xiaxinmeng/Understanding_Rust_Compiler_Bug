rust
#![feature(type_alias_impl_trait)]
use core::iter::empty;
type Test = impl Iterator<Item = ()>;
fn test() -> Test {
    empty()
}
