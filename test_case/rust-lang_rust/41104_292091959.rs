rust
struct Node;

struct RBST {
    root: Option<Node>,
}

impl RBST {
    fn merge(right: Option<Node>) -> Option<Node> {
        match (&right,) {
            (&Some(ref mut r),) => Some(*r),
            _ => loop {}
        }
    }
}

fn main() {}
