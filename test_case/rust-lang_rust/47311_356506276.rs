rust
TokenTree {
    span: Span::call_site(),
    kind: TokenNode::Group(Delimiter::Parenthesis, vec![
        TokenTree {
            span: Span::def_site(),
            kind: TokenNode::Group(Delimiter::Parenthesis, TokenStream::empty()),
        },
    ].into_iter().collect()),
},
