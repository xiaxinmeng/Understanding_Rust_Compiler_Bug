rust
fn do_something<T, U>(sl: &[T], other: &U)
where
    T: PartialEq<U>,
{
    search(sl, other);
            // ^^^^^ expected type parameter `U`, found `&U`
            // but `U` is not `Copy`.

    // ...
}
