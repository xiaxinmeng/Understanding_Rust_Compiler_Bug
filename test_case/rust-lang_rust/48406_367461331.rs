rust
use std::cmp::Ordering;

struct Foo;

impl PartialEq<Foo> for str {
    fn eq(&self, _: &Foo) -> bool {
        true
    }
}

impl PartialOrd<Foo> for str {
    fn partial_cmp(&self, _: &Foo) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

pub fn cmp(p: String, n: &str) -> bool {
    n < &p
}
