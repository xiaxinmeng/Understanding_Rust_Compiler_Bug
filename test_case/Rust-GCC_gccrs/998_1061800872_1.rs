rust
Crate {
    attrs: [],
    items: [
        Item {
            attrs: [],
            id: NodeId(4294967040),
            span: test.rs:12:1: 14:2 (#0),
            vis: Visibility {
                kind: Inherited,
                span: test.rs:12:1: 12:1 (#0),
                tokens: None,
            },
            ident: define_vars#0,
            kind: MacroDef(
                MacroDef {
                    body: Delimited(
                        DelimSpan {
                            open: test.rs:12:26: 12:27 (#0),
                            close: test.rs:14:1: 14:2 (#0),
                        },
                        Brace,
                        TokenStream(
                            [
                                (
                                    Delimited(
                                        DelimSpan {
                                            open: test.rs:13:5: 13:6 (#0),
                                            close: test.rs:13:18: 13:19 (#0),
                                        },
                                        Paren,
                                        TokenStream(
                                            [
                                                (
                                                    Token(
                                                        Token {
                                                            kind: Dollar,
                                                            span: test.rs:13:6: 13:7 (#0),
                                                        },
                                                    ),
                                                    Alone,
                                                ),
                                                (
                                                    Delimited(
                                                        DelimSpan {
                                                            open: test.rs:13:7: 13:8 (#0),
                                                            close: test.rs:13:16: 13:17 (#0),
                                                        },
                                                        Paren,
                                                        TokenStream(
                                                            [
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Dollar,
                                                                            span: test.rs:13:8: 13:9 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Ident(
                                                                                "v",
                                                                                false,
                                                                            ),
                                                                            span: test.rs:13:9: 13:10 (#0),
                                                                        },
                                                                    ),
                                                                    Joint,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Colon,
                                                                            span: test.rs:13:10: 13:11 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Ident(
                                                                                "ident",
                                                                                false,
                                                                            ),
                                                                            span: test.rs:13:11: 13:16 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                            ],
                                                        ),
                                                    ),
                                                    Alone,
                                                ),
                                                (
                                                    Token(
                                                        Token {
                                                            kind: BinOp(
                                                                Star,
                                                            ),
                                                            span: test.rs:13:17: 13:18 (#0),
                                                        },
                                                    ),
                                                    Alone,
                                                ),
                                            ],
                                        ),
                                    ),
                                    Alone,
                                ),
                                (
                                    Token(
                                        Token {
                                            kind: FatArrow,
                                            span: test.rs:13:20: 13:22 (#0),
                                        },
                                    ),
                                    Alone,
                                ),
                                (
                                    Delimited(
                                        DelimSpan {
                                            open: test.rs:13:23: 13:24 (#0),
                                            close: test.rs:13:42: 13:43 (#0),
                                        },
                                        Brace,
                                        TokenStream(
                                            [
                                                (
                                                    Token(
                                                        Token {
                                                            kind: Dollar,
                                                            span: test.rs:13:25: 13:26 (#0),
                                                        },
                                                    ),
                                                    Alone,
                                                ),
                                                (
                                                    Delimited(
                                                        DelimSpan {
                                                            open: test.rs:13:26: 13:27 (#0),
                                                            close: test.rs:13:39: 13:40 (#0),
                                                        },
                                                        Paren,
                                                        TokenStream(
                                                            [
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Ident(
                                                                                "let",
                                                                                false,
                                                                            ),
                                                                            span: test.rs:13:27: 13:30 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Dollar,
                                                                            span: test.rs:13:31: 13:32 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Ident(
                                                                                "v",
                                                                                false,
                                                                            ),
                                                                            span: test.rs:13:32: 13:33 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Eq,
                                                                            span: test.rs:13:34: 13:35 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Literal(
                                                                                Lit {
                                                                                    kind: Integer,
                                                                                    symbol: "15",
                                                                                    suffix: None,
                                                                                },
                                                                            ),
                                                                            span: test.rs:13:36: 13:38 (#0),
                                                                        },
                                                                    ),
                                                                    Joint,
                                                                ),
                                                                (
                                                                    Token(
                                                                        Token {
                                                                            kind: Semi,
                                                                            span: test.rs:13:38: 13:39 (#0),
                                                                        },
                                                                    ),
                                                                    Alone,
                                                                ),
                                                            ],
                                                        ),
                                                    ),
                                                    Alone,
                                                ),
                                                (
                                                    Token(
                                                        Token {
                                                            kind: BinOp(
                                                                Star,
                                                            ),
                                                            span: test.rs:13:40: 13:41 (#0),
                                                        },
                                                    ),
                                                    Alone,
                                                ),
                                            ],
                                        ),
                                    ),
                                    Alone,
                                ),
                            ],
                        ),
                    ),
                    macro_rules: true,
                },
            ),
            tokens: None,
        },
        Item {
            attrs: [],
            id: NodeId(4294967040),
            span: test.rs:17:1: 19:2 (#0),
            vis: Visibility {
                kind: Inherited,
                span: test.rs:17:1: 17:1 (#0),
                tokens: None,
            },
            ident: main#0,
            kind: Fn(
                Fn {
                    defaultness: Final,
                    generics: Generics {
                        params: [],
                        where_clause: WhereClause {
                            has_where_token: false,
                            predicates: [],
                            span: test.rs:17:10: 17:10 (#0),
                        },
                        span: test.rs:17:8: 17:8 (#0),
                    },
                    sig: FnSig {
                        header: FnHeader {
                            unsafety: No,
                            asyncness: No,
                            constness: No,
                            ext: None,
                        },
                        decl: FnDecl {
                            inputs: [],
                            output: Default(
                                test.rs:17:11: 17:11 (#0),
                            ),
                        },
                        span: test.rs:17:1: 17:10 (#0),
                    },
                    body: Some(
                        Block {
                            stmts: [
                                Stmt {
                                    id: NodeId(4294967040),
                                    kind: MacCall(
                                        MacCallStmt {
                                            mac: MacCall {
                                                path: Path {
                                                    span: test.rs:18:5: 18:16 (#0),
                                                    segments: [
                                                        PathSegment {
                                                            ident: define_vars#0,
                                                            id: NodeId(4294967040),
                                                            args: None,
                                                        },
                                                    ],
                                                    tokens: None,
                                                },
                                                args: Delimited(
                                                    DelimSpan {
                                                        open: test.rs:18:17: 18:18 (#0),
                                                        close: test.rs:18:25: 18:26 (#0),
                                                    },
                                                    Parenthesis,
                                                    TokenStream(
                                                        [
                                                            (
                                                                Token(
                                                                    Token {
                                                                        kind: Ident(
                                                                            "f",
                                                                            false,
                                                                        ),
                                                                        span: test.rs:18:18: 18:19 (#0),
                                                                    },
                                                                ),
                                                                Alone,
                                                            ),
                                                            (
                                                                Token(
                                                                    Token {
                                                                        kind: Ident(
                                                                            "g",
                                                                            false,
                                                                        ),
                                                                        span: test.rs:18:20: 18:21 (#0),
                                                                    },
                                                                ),
                                                                Alone,
                                                            ),
                                                            (
                                                                Token(
                                                                    Token {
                                                                        kind: Ident(
                                                                            "h",
                                                                            false,
                                                                        ),
                                                                        span: test.rs:18:22: 18:23 (#0),
                                                                    },
                                                                ),
                                                                Alone,
                                                            ),
                                                            (
                                                                Token(
                                                                    Token {
                                                                        kind: Ident(
                                                                            "i",
                                                                            false,
                                                                        ),
                                                                        span: test.rs:18:24: 18:25 (#0),
                                                                    },
                                                                ),
                                                                Alone,
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                                prior_type_ascription: None,
                                            },
                                            style: Semicolon,
                                            attrs: ThinVec(
                                                None,
                                            ),
                                            tokens: None,
                                        },
                                    ),
                                    span: test.rs:18:5: 18:27 (#0),
                                },
                            ],
                            id: NodeId(4294967040),
                            rules: Default,
                            span: test.rs:17:11: 19:2 (#0),
                            tokens: None,
                            could_be_bare_literal: false,
                        },
                    ),
                },
            ),
            tokens: None,
        },
    ],
    span: test.rs:12:1: 19:2 (#0),
    id: NodeId(4294967040),
    is_placeholder: false,
}
