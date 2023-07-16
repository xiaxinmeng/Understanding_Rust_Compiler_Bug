
Node {
  len: u8,
  parent_idx: u8,
  is_leaf: bool,
  parent: *mut Node,
  keys: ManuallyDrop<[K; n]>
  vals: ManuallyDrop<[V; n]>
  edges: ManuallyDrop<[Box<Node>; n + 1]> 
}

Map {
  root: Option<Box<Node>>
}
