 rust
struct Node {
    next: Option<Box<Node>>,
}

fn main() {
    let mut first = Node { next: None };
    let second = Node { next: Some(Box::new(first)) };
    first.next = Some(Box::new(second));
}
