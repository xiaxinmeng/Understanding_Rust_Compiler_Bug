
#![feature(specialization)]

trait Foo {}

struct A;

impl From<&A> for A {
    fn from(_: &A) -> A { A }
}

default impl<K, V> Foo for (K, V)
where
    K: Into<A>,
    V: Into<A>,
{}

default impl Foo for (&A, &A) {}

