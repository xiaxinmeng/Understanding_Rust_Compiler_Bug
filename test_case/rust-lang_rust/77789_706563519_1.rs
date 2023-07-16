
TokenStream [
    Ident {
        ident: "std",
        span: #6 bytes(6504400..6504403),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Group {
                delimiter: None,
                stream: TokenStream [
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "Some",
                                span: #6 bytes(6504404..6504406),
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Ident {
                                        ident: "a",
                                        span: #6 bytes(6504404..6504406),
                                    },
                                ],
                                span: #6 bytes(6504404..6504406),
                            },
                        ],
                        span: #6 bytes(6504404..6504406),
                    },
                ],
                span: #6 bytes(6504404..6504406),
            },
        ],
        span: #6 bytes(6504403..6504407),
    },
    Punct {
        ch: '=',
        spacing: Alone,
        span: #6 bytes(6504408..6504409),
    },
    Ident {
        ident: "Some",
        span: #0 bytes(124..128),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Literal {
                kind: Integer,
                symbol: "42",
                suffix: None,
                span: #0 bytes(129..131),
            },
        ],
        span: #0 bytes(128..132),
    },
]
