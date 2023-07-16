rust
struct Node {
    left: Option<Box<Node>>,
}

fn leftmost_child(n: &mut Node) -> &mut Node {
    let mut leftest = n;
    while let Some(ref mut nl) = leftest.left {
        leftest = nl;
    }
    leftest
}

fn main() {}
