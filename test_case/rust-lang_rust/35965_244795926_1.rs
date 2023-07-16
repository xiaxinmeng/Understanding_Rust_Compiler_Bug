 rust
pub struct Item {
    pub name: Spanned<Name>,
    pub attrs: HirVec<Attribute>,
    pub id: NodeId,
    pub node: Item_,
    pub vis: Visibility,
    pub span: Span,
}
