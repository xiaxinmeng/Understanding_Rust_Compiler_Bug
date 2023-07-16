rust
pub enum TokenKind {
    Sequence(Delimiter, TokenStream),

    // The content of the comment can be found from the span.
    Comment(CommentKind),

    // `text` is the string contents, not including delimiters. It would be nice
    // to avoid an allocation in the common case that the string is in the
    // source code. We might be able to use `&'codemap str` or something.
    // `raw_markers` is for the count of `#`s if the string is a raw string. If
    // the string is not raw, then it will be `None`.
    String { text: Symbol, raw_markers: Option<usize>, kind: StringKind },

    // char literal, span includes the `'` delimiters.
    Char(char),

    // These tokens are treated specially since they are used for macro
    // expansion or delimiting items.
    Exclamation,  // `!`
    Dollar,       // `$`
    // Not actually sure if we need this or if semicolons can be treated like
    // other punctuation.
    Semicolon,    // `;`
    Eof,          // Do we need this?

    // Word is defined by Unicode Standard Annex 31 -
    // [Unicode Identifier and Pattern Syntax](http://unicode.org/reports/tr31/)
    Word(Symbol),
    Punctuation(char),
}

pub enum StringKind {
    Regular,
    Byte,
}
