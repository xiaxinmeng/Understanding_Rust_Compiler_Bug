rust
pub struct Node {
    next: Option<Box<Node>>,
}

fn example(mut list: &mut Node) {
    if let Some(ref mut node) = list.next {
        if true {
            list = node;
        }
    }

    if let Some(ref mut node) = list.next {
        list = node;
    }
}
