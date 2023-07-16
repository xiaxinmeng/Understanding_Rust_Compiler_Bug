rust
fn iter<'a>(data: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
    data.iter().map(|x| x).map(|x| *x)
}
