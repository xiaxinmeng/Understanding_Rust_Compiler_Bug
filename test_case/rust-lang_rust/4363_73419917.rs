 rust
fn main() {
    let _ = FingerTree::Deep(Node { count: 0,
        front: Digit { count: 0, content: [None, None, None, None] },
        inner: Box::new(FingerTree::Single((1, 2))),
        back: Digit { count: 0, content: [None, None, None, None] }}
    );
}
