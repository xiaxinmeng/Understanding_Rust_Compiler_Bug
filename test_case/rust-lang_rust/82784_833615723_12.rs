text
[/Users/taiki/.cargo/git/checkouts/tokio-7eef92cc7b2e8568/13bdbe4/tokio-macros/src/lib.rs:183] &out = TokenStream [
    Ident {
        ident: "fn",
        span: #3 bytes(103..105),
    },
    Ident {
        ident: "f",
        span: #3 bytes(106..107),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: #3 bytes(107..109),
    },
    Punct {
        ch: '-',
        spacing: Joint,
        span: #3 bytes(110..112),
    },
    Punct {
        ch: '>',
        spacing: Alone,
        span: #3 bytes(110..112),
    },
    Ident {
        ident: "i32",
        span: #3 bytes(113..116),
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "tokio",
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "runtime",
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "Builder",
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Joint,
                span: #5 bytes(70..84),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "new_multi_thread",
                span: #5 bytes(70..84),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [],
                span: #5 bytes(70..84),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "enable_all",
                span: #5 bytes(70..84),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [],
                span: #5 bytes(70..84),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "build",
                span: #5 bytes(70..84),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [],
                span: #5 bytes(70..84),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "unwrap",
                span: #5 bytes(70..84),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [],
                span: #5 bytes(70..84),
            },
            Punct {
                ch: '.',
                spacing: Alone,
                span: #5 bytes(70..84),
            },
            Ident {
                ident: "block_on",
                span: #5 bytes(70..84),
            },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [
                    Ident {
                        ident: "async",
                        span: #5 bytes(70..84),
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Group {
                                delimiter: None,
                                stream: TokenStream [
                                    Punct {
                                        ch: '|',
                                        spacing: Alone,
                                        span: #0 bytes(182..183),
                                    },
                                    Ident {
                                        ident: "_",
                                        span: #0 bytes(183..184),
                                    },
                                    Punct {
                                        ch: '|',
                                        spacing: Alone,
                                        span: #0 bytes(184..185),
                                    },
                                    Literal {
                                        kind: Integer,
                                        symbol: "5",
                                        suffix: None,
                                        span: #0 bytes(186..187),
                                    },
                                ],
                                span: #3 bytes(135..137),
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Group {
                                        delimiter: Parenthesis,
                                        stream: TokenStream [],
                                        span: #3 bytes(138..140),
                                    },
                                ],
                                span: #3 bytes(137..141),
                            },
                        ],
                        span: #3 bytes(117..155),
                    },
                ],
                span: #5 bytes(70..84),
            },
        ],
        span: #5 bytes(70..84),
    },
]
