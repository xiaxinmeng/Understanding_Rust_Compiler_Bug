rust
use std::fmt::Debug;

trait Trait<'a> {
    type Assoc;
}

impl<'a> Trait<'a> for () {
    type Assoc = ();
}

fn bad<T>(_: T)
where
    for<'a> T: Trait<'a>,
    for<'a> <T as Trait<'a>>::Assoc: Debug,
{
}

fn main() {
    bad(());
}
