rust
pub struct Flatten<I: Iterator>
where I::Item: IntoIterator {
    iter: I,
    frontiter: Option<<I::Item as IntoIterator>::IntoIter>,
    backiter: Option<<I::Item as IntoIterator>::IntoIter>,
}
