rust
#![feature(nll)]

fn main() {
    S1 {}
        .into_iter()
        .map(|_| ());
}

pub struct S1 {}

pub struct S2<'a>
{
    phantom: &'a (),
}

pub struct S3<'a, I: 'a>
    where I: Iterator,
          I::Item: 'a,
{
    it: &'a I,
}

impl<'a> IntoIterator for &'a S1
{
    type Item = S3<'a, std::iter::Empty<()>>;
    type IntoIter = S2<'a>;

    fn into_iter(self) -> Self::IntoIter {
        S2 { phantom: &() }
    }
}

impl<'a> Iterator for S2<'a>
{
    type Item = S3<'a, std::iter::Empty<()>>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
