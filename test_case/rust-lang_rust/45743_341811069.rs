rust
struct Wrapper<T>(T);

impl<'a, T> IntoIterator for &'a Wrapper<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;           // NOTE diff
    type IntoIter = <&'a T as IntoIterator>::IntoIter;   // NOTE diff
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn main() { }
