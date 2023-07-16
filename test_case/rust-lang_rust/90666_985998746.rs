rust
use std::sync::{Arc, Weak};

struct Root {
    left: Leaf,
    right: Leaf,
}

struct Leaf {
    root: Weak<Root>,
}

let tree = Arc::new_cyclic(|me| {
    assert!(root.upgrade().is_none());
    Root {
        left: Leaf { root: me.clone() },
        right: Leaf { root: me.clone() },
    }
});

assert!(tree.left.root.upgrade().is_some());
assert!(Arc::ptr_eq(&tree.right.root.upgrade().unwrap(), &tree));
