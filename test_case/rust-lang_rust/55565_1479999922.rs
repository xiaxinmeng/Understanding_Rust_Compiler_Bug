rust
TokenStream [
    Ident {
        ident: "struct",
        span: #6 bytes(112..118),
    },
    Ident {
        ident: "_S",
        span: #6 bytes(119..121),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Group {
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "u8",
                        span: #6 bytes(123..125),
                    },
                    Punct {
                        ch: ';',
                        spacing: Alone,
                        span: #6 bytes(125..126),
                    },
                    Ident {
                        ident: "repro",
                        span: #6 bytes(127..132),
                    },
                    Punct {
                        ch: ':',
                        spacing: Joint,
                        span: #6 bytes(132..134),
                    },
                    Punct {
                        ch: ':',
                        spacing: Alone,
                        span: #6 bytes(132..134),
                    },
                    Ident {
                        ident: "zero",
                        span: #6 bytes(134..138),
                    },
                    Punct {
                        ch: '!',
                        spacing: Alone,
                        span: #6 bytes(138..139),
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Literal {
                                kind: Integer,
                                symbol: "2",
                                suffix: None,
                                span: #6 bytes(140..141),
                            },
                            Punct {
                                ch: '*',
                                spacing: Alone,
                                span: #6 bytes(142..143),
                            },
                            Group {
                                delimiter: None,
                                stream: TokenStream [
                                    Literal {
                                        kind: Integer,
                                        symbol: "1",
                                        suffix: None,
                                        span: #0 bytes(164..165),
                                    },
                                    Punct {
                                        ch: '+',
                                        spacing: Alone,
                                        span: #0 bytes(166..167),
                                    },
                                    Literal {
                                        kind: Integer,
                                        symbol: "1",
                                        suffix: None,
                                        span: #0 bytes(168..169),
                                    },
                                ],
                                span: #6 bytes(144..146),
                            },
                        ],
                        span: #6 bytes(139..147),
                    },
                ],
                span: #6 bytes(122..148),
            },
        ],
        span: #6 bytes(121..149),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #6 bytes(149..150),
    },
]
