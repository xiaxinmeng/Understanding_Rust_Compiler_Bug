 rust
use std::vec::Vec;
use std::iter::Iterator;
use std::slice::{AsSlice, Split, SliceExt};

struct Path;

trait Foo<A> {
    fn foo(&self, a: A);
}

impl<A,F:Fn(A)> Foo<A> for F {
    fn foo(&self, a: A) { }
}

fn baz<A,F:for<'a> Foo<(&'a A,)>>(f: F) { }

fn components<T,A>(t: fn(&A))
    where fn(&A) : for<'a> Foo<(&'a A,)>,
{
    baz(t)
}

fn main() {
}
