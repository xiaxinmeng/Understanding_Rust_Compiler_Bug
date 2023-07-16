rust
#![recursion_limit="10"]

trait Trait {}

impl<'a> Trait for &'a () {}

impl<'a, T> Trait for &'a Option<T>
    where &'a T: Trait
{}

fn testing<T>(_: T)
    where for<'a> &'a T: Trait
{}


fn main() {
    testing(());
}
