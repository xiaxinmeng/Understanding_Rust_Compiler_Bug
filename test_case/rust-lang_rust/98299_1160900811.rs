rust
struct S<const K: usize> {
    xs: Vec<[u8; K]>,
}

impl<const K: usize> S<K> {
    fn f(&self, a: [u8; K], cs: i32) -> S<K> {
        g(a).map(|ws| S{xs: ws.to_vec()})
    }
}

fn main() {}
