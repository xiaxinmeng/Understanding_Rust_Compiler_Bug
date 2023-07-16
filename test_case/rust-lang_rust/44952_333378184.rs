Rust
struct Node<K>(K);
impl<K> Node<K> {
    fn balance<'a, L: Into<&'a Node<K>>>() {}
}

fn main() {}
