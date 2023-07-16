rust
#![feature(associated_type_defaults, specialization)]

trait Eq<T> {}
impl<T> Eq<T> for T {}

trait Foo { // no error here
    type Xyz : Eq<Self::Uvw> = u32;
    type Uvw = u32;
}

impl Foo for () { //~ ERROR here
    type Xyz = ();
}

fn main() {
}
