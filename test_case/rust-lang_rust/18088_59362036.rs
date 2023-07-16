 Rust
fn main(){}

pub trait Indexable<T>: Index<uint, T>  {
    fn index2(&self, i: uint) -> &T {
        &(*self)[i]
    }
}
