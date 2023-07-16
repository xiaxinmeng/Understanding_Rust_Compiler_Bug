rust
pub enum TokenTree {
    Token(Span, token::Token),
    Delimited(DelimSpan, DelimToken, TokenStream),
}
