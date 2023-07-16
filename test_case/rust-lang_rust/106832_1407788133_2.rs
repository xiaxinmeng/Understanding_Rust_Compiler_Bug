rust
fn test_fun<'b, T, U>(get_to: U, other: &'b T) -> T
where
    U: Get,
    T: Get<T<'b> = T>,
    T: Get<T<'b> = U>,
{
    get_to.get();
    other.get()
}
