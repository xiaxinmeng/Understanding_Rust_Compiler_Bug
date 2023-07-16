rust
impl<T, U> Iterator for MyFoo<T>
    where T: Iterator<Item = U>
{
    type Item = U;
    ...
}
