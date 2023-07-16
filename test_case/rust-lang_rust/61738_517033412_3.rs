rust
#![feature(associated_type_bounds)]

pub struct Flatten<I>
where
    I: Iterator<Item: IntoIterator>,
{
    inner: <<I as Iterator>::Item as IntoIterator>::IntoIter,
}
