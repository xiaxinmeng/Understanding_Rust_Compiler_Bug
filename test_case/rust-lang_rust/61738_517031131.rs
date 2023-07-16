rust
pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    inner: I::Item::IntoIter,
}
