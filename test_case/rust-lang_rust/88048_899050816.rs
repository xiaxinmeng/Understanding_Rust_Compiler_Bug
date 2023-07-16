rust
pub trait Kind where for<'a> &'a Self::Values: IntoIterator<Item=&'a Self> {
    type Values: IntoIterator<Item=Self>;
}
pub trait DataSource<T: Kind> where for<'a> &'a T::Values: IntoIterator<Item=&'a T> {
    fn get_values(&self) -> &T::Values;
}
