rust
enum MetaItemKind {
    Word,
    List(Vec<NestedMetaItem>),
    NameValue(Lit),
}

enum NestedMetaItemKind {
    MetaItem(MetaItem),
    Literal(Lit)
}
