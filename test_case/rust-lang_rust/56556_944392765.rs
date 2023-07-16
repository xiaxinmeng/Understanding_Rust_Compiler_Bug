rust
fn test<T>()
where
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::IntoIter: Clone,
{
}

fn main() {
    test::<Vec<u8>>();
}
