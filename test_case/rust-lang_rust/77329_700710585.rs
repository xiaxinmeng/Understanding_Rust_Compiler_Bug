rust
#![feature(min_const_generics)]

trait Foo {}
struct Bar<const N: usize>;

fn bug() -> impl Foo<Bar<{|_: ()| {}}>> {}
