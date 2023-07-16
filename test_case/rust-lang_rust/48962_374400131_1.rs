rust
#![feature(nll)]

struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut node = Node {
        elem: 5,
        next: None,
    };

    let mut src = &mut node;
    {src};
    src.next = None;
}
