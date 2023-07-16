
Node {
    len: u8,
    keys: [K; n],
    vals: [V; n],
    edges: Option<Box<ManuallyDrop<[Node; n + 1]>>>,
}
