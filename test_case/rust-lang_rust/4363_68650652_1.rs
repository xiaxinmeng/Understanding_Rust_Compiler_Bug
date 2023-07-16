
#![allow(dead_code, non_snake_case)]

enum FingerTree<A> {
    Empty,
    Single(A),
    Deep(Node<A>)
}

struct Node<A> {
    count: int,
    front: Digit<A>,
    inner: Box<FingerTree<(A,A)>>,
    back: Digit<A>
}

struct Digit<A> {
    count: int,
    content: [Option<A>; 4]
}

fn FingerTree<A>() -> FingerTree<A> { FingerTree::Empty }

fn main() {}
