Rust
    struct Node {
        hdr: Header,
        variance_children: L<VarianceNode>,
        impl_: Option</*...*/>,
    }
    struct VarianceNode {
        hdr: Header,
        children: L<Node>,
    }
    