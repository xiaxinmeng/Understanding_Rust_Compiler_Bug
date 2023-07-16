
struct BaseNode {
   is_leaf: bool,
   keys: ManuallyDrop<[K; n]>
   vals: ManuallyDrop<[V; n]>
   // and other common stuff like parent pointers
}

// Inherit from BaseNode
struct InternalNode: BaseNode {
   edges: ManuallyDrop<[Unique<BaseNode>; n+1]>
}
