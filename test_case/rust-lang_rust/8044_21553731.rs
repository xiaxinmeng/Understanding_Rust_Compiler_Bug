 rust
pub struct BTree {
    node: TreeItem,
}

pub enum TreeItem {
    TreeLeaf { value: int },
}

pub fn leaf(value: int) -> TreeItem {
    TreeLeaf { value: value }
}

fn main() {
    BTree { node: leaf(1) };
}
