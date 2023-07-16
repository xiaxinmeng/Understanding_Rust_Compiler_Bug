rust
use std::cmp;

#[derive(Eq, PartialEq)]
struct Foo<'a, T: 'a>(&'a mut [T]);

impl<'a, T: PartialOrd + 'a> PartialOrd for Foo<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
