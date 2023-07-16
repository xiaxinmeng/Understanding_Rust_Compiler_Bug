rust
trait Get {
    type T<'a> where Self: 'a;
    fn get(&self) -> Self::T<'_>;
}

// Note that we don't actually *need* to generate a `T` in the function body
fn test_fun<'b, T>(get_to: T) where T: Get<T<'b> = T> + 'b {
    get_to.get(); // So this could be called with something besides &'b get_to
}
