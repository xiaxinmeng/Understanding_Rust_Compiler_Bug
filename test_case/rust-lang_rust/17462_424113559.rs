rust
struct Node {
    left: Option<Box<Node>>,
}

fn leftmost_child(n: &mut Node) -> &mut Node {
    let mut leftest = n;
    loop {
        match leftest.left {
            Some(ref mut nl) => leftest = &mut **nl,
            None => break,
        }
    }
    leftest
}

fn main() {}
