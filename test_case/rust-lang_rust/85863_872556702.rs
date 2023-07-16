rust
#![feature(rustc_attrs)]
#![feature(min_specialization)]

use core::marker::PhantomData;

struct MyEmptyIterator<T>(PhantomData<T>, &'static str);

#[rustc_specialization_trait]
trait SpecTrait {}

impl SpecTrait for MyEmptyIterator<fn(&'static ())> {}
