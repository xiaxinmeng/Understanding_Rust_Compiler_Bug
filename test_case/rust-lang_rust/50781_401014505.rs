rust
pub trait X<T> {
    fn foo(&self) where Self: Other<T>, T: Other<Self>
}
