text
[wmacro/src/lib.rs:15] &tokens = TokenStream [
    Ident {
        ident: "pub",
        span: #0 bytes(0..0),
    },
    Ident {
        ident: "extern",
        span: #0 bytes(0..0),
    },
    Literal { lit: Lit { kind: Str, symbol: C, suffix: None }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } },
    Ident {
        ident: "fn",
        span: #0 bytes(0..0),
    },
    Ident {
        ident: "third",
        span: #0 bytes(0..0),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                ident: "_",
                span: #0 bytes(0..0),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #0 bytes(0..0),
            },
            Ident {
                ident: "Vec",
                span: #0 bytes(0..0),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [
                    Ident {
                        ident: "u32",
                        span: #0 bytes(0..0),
                    },
                ],
                span: #0 bytes(0..0),
            },
        ],
        span: #0 bytes(0..0),
    },
    Punct {
        ch: '-',
        spacing: Joint,
        span: #0 bytes(0..0),
    },
    Punct {
        ch: '>',
        spacing: Alone,
        span: #0 bytes(0..0),
    },
    Ident {
        ident: "u32",
        span: #0 bytes(0..0),
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Literal { lit: Lit { kind: Integer, symbol: 0, suffix: None }, span: Span { lo: BytePos(0), hi: BytePos(0), ctxt: #0 } },
        ],
        span: #0 bytes(0..0),
    },
]
