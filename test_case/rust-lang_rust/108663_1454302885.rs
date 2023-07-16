rust
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

use std::fmt::Debug;

type Foo<'a, T: 'a> where &'a T: Debug = (impl Debug + 'a, T);
fn foo<'a, U: Debug + 'a>(v: U) -> Foo<'a, U> where &'a U: Debug {
    (Vec::<&'a U>::new(), v)
}
