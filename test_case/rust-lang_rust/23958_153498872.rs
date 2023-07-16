 rust
pub struct Foo(Vec<i32>);

impl Foo {
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Foo {
    type IntoIter = <&'a Vec<i32> as IntoIterator>::IntoIter;
    type Item = <&'a Vec<i32> as IntoIterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}
