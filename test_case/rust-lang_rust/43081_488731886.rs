rust
struct PreexpTokenStream {
    preexp_tts: Vec<PreexpTokenTree>,
}

struct PreexpTokenTree {
    TokenStream(TokenStream),
    PreexpGroup(PreexpGroup),
    AttributeInvocation(Attribute, AttributeInvocationKind),
}

struct PreexpGroup {
    delimiter: Delimiter,
    preexpr_ts: PreexpTokenStream,
}

enum AttributeInvocationKind {
    // #[attr] TOKENS
    Outer(PreexpTokenStream),
    // HEADER_TOKENS { #![attr] TOKENS }
    Inner(PreexpTokenStream, PreexpGroup),
}

struct Attribute {
    path: PreexpTokenStream,
    tts: TokenStream,
}
