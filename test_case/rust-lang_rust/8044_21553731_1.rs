 rust
extern mod minimal_int;
use minimal_int::{BTree, leaf};

fn main() {
    BTree { node: leaf(1) };
}
