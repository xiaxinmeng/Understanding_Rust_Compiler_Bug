 rust
struct SnapMap<K, V, Ptr> {
    priv root: Option<Ptr>,
    priv length: uint
}

struct TreeNode<K, V, Ptr> {
    key: K,
    value: V,
    left: Option<Ptr>,
    right: Option<Ptr>,
    level: uint
}

impl<K: TotalOrd, V, Ptr: CowPtr<TreeNode<K, V, Ptr>>> 
     MutableMap<K, V> for SnapMap<K, V, Ptr> {
    // ...
}
