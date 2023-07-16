rust
struct AttrItem {
    path: Path,
    kind: AttrItemKind,
}

enum AttrItemKind {
    Delimited(TokenStream, DelimToken),
    Word,
    KeyValue(???),
}
