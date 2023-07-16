rust
struct Node<T> {
    nxt: Option<Box<Node<(T, T)>>>,
    data: T
}

impl <T> Node<T> {
    pub fn new(x: T) -> Self {
        Self {
            nxt: None,
            data: x
        }
    }
}
