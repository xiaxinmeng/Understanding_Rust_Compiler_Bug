
TokenStream [
    Ident {
        ident: "std",
        span: #9 bytes(6811810..6811813),
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
                            Group {
                                delimiter: None,
                                stream: TokenStream [
                                    Ident {
                                        ident: "Some",
                                        span: #0 bytes(114..118),
                                    },
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [
                                            Ident {
                                                ident: "a",
                                                span: #0 bytes(119..120),
                                            },
                                        ],
                                        span: #0 bytes(118..121),
                                    },
                                ],
                                span: #8 bytes(6811487..6811489),
                            },
                        ],
                        span: #8 bytes(6811486..6811490),
                    },
                ],
                span: #9 bytes(6811814..6811816),
            },
        ],
        span: #9 bytes(6811813..6811817),
    },
    Punct {
        ch: '=',
        spacing: Alone,
        span: #9 bytes(6811818..6811819),
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
