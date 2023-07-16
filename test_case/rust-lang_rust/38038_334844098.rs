rust
/// this is a simplified version of std::iter::Chain
pub struct Chain2<I> { a: I, b: Option<I> }

fn chain2<I>(a: I, b: I) -> Chain2<I> where I: Iterator {
    Chain2 { a: a, b: Some(b) }
}

impl<I> Iterator for Chain2<I> where I: Iterator {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.a.next() {
            return Some(item);
        }
        if let Some(b) = self.b.take() {
            self.a = b;
        }
        self.a.next()
    }
}

fn main() {
    let v0 = vec![0, 1];
    let v1 = vec![0, 1, 2];

    let mut v3 = vec![];
    for &x in chain2(v0.iter(), v1.iter()) {
        v3.push(x);
    }

    assert_eq!(v3, [0, 1, 0, 1, 2]);
}
