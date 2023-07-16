rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct Foo<const P: bool>;
struct Bar<const N: usize, T = Foo<{ N < 7 }>>;
type X = Bar<3>;

fn main() {}
