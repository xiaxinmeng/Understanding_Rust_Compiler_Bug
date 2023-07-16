rust
enum TokenKind {
    Group(Delimiter, TokenStream),
    Term(Term),
    Punctuation(char, Spacing),
    Literal(Literal),
}
