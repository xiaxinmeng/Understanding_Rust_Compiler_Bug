rust
fn iter_len<I: IntoIterator>(iter: I)
where
    I::IntoIter: ExactSizeIterator,
{
    println!("size {}", iter.into_iter().len());
}
