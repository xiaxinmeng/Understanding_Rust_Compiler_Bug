
Node {
    len: u8,
    // Actually just Uniques that we manually allocate and manage
    keys: Box<ManuallyDrop<[K; n]>>,
    vals: Box<ManuallyDrop<[V; n]>>,
    edges: Option<Box<ManuallyDrop<[Node; n + 1]>>>,
}
