 rust
#![crate_type = "lib"]
#![feature(associated_types)]

trait Iterator {
    type Item;

    fn ok() -> Self::Item;
}

trait DoubleEndedIterator: Iterator {
    fn doesnt_work() -> Self::Item;
    fn works() -> <Self as Iterator>::Item;
}
