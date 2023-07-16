rust
crate fn tokenize_into_token_trees(
    sess: &'a ParseSess,
    source_file: Lrc<rustc_span::SourceFile>,
    override_span: Option<Span>,
) -> (PResult<'a, TokenStream>, Vec<UnmatchedBrace>)
