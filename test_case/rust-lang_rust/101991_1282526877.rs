Rust
fn join_spans(sp1: Span, sp2: Span) -> Span {
    let begin = TokenTree::Ident(Ident::new("A", sp1));
    let end = TokenTree::Ident(Ident::new("B", sp2));
    let group = Group::new(Delimiter::None, [begin, end].into_iter().collect());
    group.span()
}
