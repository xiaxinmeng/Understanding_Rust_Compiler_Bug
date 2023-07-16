rust
extern crate cookie;

use std::collections::HashSet;

#[derive(Debug)]
struct A<'a>(&'a str, bool);

fn main() {
    let mut a = HashSet::new();
    a.insert(A("name", false));
    a.insert(A("second", false));

    let mut b = HashSet::new();
    b.replace(A("new", false));
    b.replace(A("another", false));
    b.replace(A("yac", false));

    b.replace(A("name", true));
    b.remove(&A("another", false));

    println!("{:?}", b.union(&a).collect::<Vec<_>>());
    println!("{:?}", b.iter().chain(a.difference(&b)).collect::<Vec<_>>());
}

impl<'a> std::hash::Hash for A<'a> {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

impl<'a> PartialEq for A<'a> {
    fn eq(&self, other: &A<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> Eq for A<'a> {
}
