rust
enum TokenTree {
    Leaf(LeafAuxData /* leaf kind, span, ... */),
    Subtree(Vec<TokenTree>, SubTreeAuxData /* delimiter kind, span, ... */),
}
