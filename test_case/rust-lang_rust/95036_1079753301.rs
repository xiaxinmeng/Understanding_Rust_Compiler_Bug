rust
pub fn test_l3(mut node: Box<[u8], &Global>) {
    node[0] = 7;
    black_box(node);
}
