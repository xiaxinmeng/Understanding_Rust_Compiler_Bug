rust
#![feature(generic_associated_types)]

trait Foo {
    type Bar<'a>;
    fn baz<'a>(_: Self::Bar) ;
}

impl Foo for () {
    type Bar<'a> = &'a ();
    fn baz<'a>(_: Self::Bar) {}
}

fn main () {}
