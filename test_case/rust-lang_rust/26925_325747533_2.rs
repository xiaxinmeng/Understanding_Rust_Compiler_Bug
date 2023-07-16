rust
impl Clone for Foo<T>
where
    T: Iterator,
    T::Item: Clone, // and another line like this for all field types
{ }
