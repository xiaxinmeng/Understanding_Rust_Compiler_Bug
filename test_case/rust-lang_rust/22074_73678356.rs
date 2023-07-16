 rust
fn foo<T:Iterator>(t: T) -> u32
    where T::Item = u32
{
    t.next().unwrap()
}
