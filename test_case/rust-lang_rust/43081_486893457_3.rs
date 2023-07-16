
TokenStream [
    Ident {
        ident: "enum",
        span: #0 bytes(0..0)
    },
    Ident {
        ident: "Dummy",
        span: #0 bytes(0..0)
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                ch: '#',
                spacing: Alone,
                span: #0 bytes(0..0)
            },
            Group {
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "foo",
                        span: #0 bytes(0..0)
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "bar",
                                span: #0 bytes(0..0)
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [
                                    Ident {
                                        ident: "value",
                                        span: #0 bytes(0..0)
                                    }
                                ],
                                span: #0 bytes(0..0)
                            }
                        ],
                        span: #0 bytes(0..0)
                    }
                ],
                span: #0 bytes(0..0)
            },
            Ident {
                ident: "Variant",
                span: #0 bytes(0..0)
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(0..0)
            }
        ],
        span: #0 bytes(0..0)
    }
]
