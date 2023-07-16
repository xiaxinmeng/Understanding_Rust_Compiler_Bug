
enum Node<T> {
    SplitNode {
        cut_dim_and_right_child_index: core::num::NonZeroU32,
        cut_value: T
    },
    LeafNode(u32)
}
