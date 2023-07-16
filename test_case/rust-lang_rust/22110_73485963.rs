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

fn baz<F:for<'a> Foo<(&'a Path,)>>(f: F) { }

impl Path {
    fn components<T>(&self, t: fn(&Path))
        where fn(&Path) : for<'a> Foo<(&'a Path,)>,
    {
        baz(t)
    }
}

fn main() {
}
