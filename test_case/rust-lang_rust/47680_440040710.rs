rust
pub struct Node {
    next: Option<Box<Node>>,
}

fn example(mut list: &mut Node) {
    if let Some(node) = &mut list.next {
        list = node;
    }

    if let Some(node) = &mut list.next {
        list = node;
    }
}
