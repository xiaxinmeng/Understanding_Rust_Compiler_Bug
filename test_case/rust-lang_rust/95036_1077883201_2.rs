rust
pub fn test() {
    let mut node = Box::new_in([5u8], Global);
    node[0] = 7u8;
    black_box(node);
}
