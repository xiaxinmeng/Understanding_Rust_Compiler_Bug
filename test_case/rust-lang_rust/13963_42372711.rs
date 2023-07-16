 rust
pub trait FromVec<T> {
    fn from_vec(v: Vec<T>) -> Self
}

impl<T> FromVec<T> for ~[T] {
    fn from_vec(v: Vec<T>) -> ~[T] {
        // ..
    }
}
