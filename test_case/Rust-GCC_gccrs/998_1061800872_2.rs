rust
Crate {
    attrs: [],
    items: [
        Item {
            attrs: [
                Attribute {
                    kind: Normal(
                        AttrItem {
                            path: Path {
                                span: no-location (#1),
                                segments: [
                                    PathSegment {
                                        ident: prelude_import#1,
                                        id: NodeId(2),
                                        args: None,
                                    },
                                ],
                                tokens: None,
                            },
                            args: Empty,
                            tokens: None,
                        },
                        None,
                    ),
                    id: AttrId(1),
                    style: Outer,
                    span: no-location (#1),
                },
            ],
            id: NodeId(1),
            span: no-location (#1),
            vis: Visibility {
                kind: Inherited,
                span: no-location (#1),
                tokens: None,
            },
            ident: #0,
            kind: Use(
                UseTree {
                    prefix: Path {
                        span: no-location (#1),
                        segments: [
                            PathSegment {
                                ident: {{root}}#1,
                                id: NodeId(3),
                                args: None,
                            },
                            PathSegment {
                                ident: std#1,
                                id: NodeId(4),
                                args: None,
                            },
                            PathSegment {
                                ident: prelude#1,
                                id: NodeId(5),
                                args: None,
                            },
                            PathSegment {
                                ident: rust_2015#1,
                                id: NodeId(6),
                                args: None,
                            },
                        ],
                        tokens: None,
                    },
                    kind: Glob,
                    span: no-location (#1),
                },
            ),
            tokens: None,
        },
        Item {
            attrs: [
                Attribute {
                    kind: Normal(
                        AttrItem {
                            path: Path {
                                span: no-location (#1),
                                segments: [
                                    PathSegment {
                                        ident: macro_use#1,
                                        id: NodeId(8),
                                        args: None,
                                    },
                                ],
                                tokens: None,
                            },
                            args: Empty,
                            tokens: None,
                        },
                        None,
                    ),
                    id: AttrId(0),
                    style: Outer,
                    span: no-location (#1),
                },
            ],
            id: NodeId(7),
            span: no-location (#1),
            vis: Visibility {
                kind: Inherited,
                span: no-location (#1),
                tokens: None,
            },
            ident: std#2,
            kind: ExternCrate(
                None,
            ),
            tokens: None,
        },
        Item {
            attrs: [],
            id: NodeId(9),
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
            id: NodeId(10),
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
                                    id: NodeId(12),
                                    kind: Local(
                                        Local {
                                            id: NodeId(13),
                                            pat: Pat {
                                                id: NodeId(14),
                                                kind: Ident(
                                                    ByValue(
                                                        Not,
                                                    ),
                                                    f#0,
                                                    None,
                                                ),
                                                span: test.rs:13:31: 13:33 (#4),
                                                tokens: None,
                                            },
                                            ty: None,
                                            kind: Init(
                                                Expr {
                                                    id: NodeId(15),
                                                    kind: Lit(
                                                        Lit {
                                                            token: Lit {
                                                                kind: Integer,
                                                                symbol: "15",
                                                                suffix: None,
                                                            },
                                                            kind: Int(
                                                                15,
                                                                Unsuffixed,
                                                            ),
                                                            span: test.rs:13:36: 13:38 (#4),
                                                        },
                                                    ),
                                                    span: test.rs:13:36: 13:38 (#4),
                                                    attrs: ThinVec(
                                                        None,
                                                    ),
                                                    tokens: None,
                                                },
                                            ),
                                            span: test.rs:13:27: 13:39 (#4),
                                            attrs: ThinVec(
                                                None,
                                            ),
                                            tokens: None,
                                        },
                                    ),
                                    span: test.rs:13:27: 13:39 (#4),
                                },
                                Stmt {
                                    id: NodeId(16),
                                    kind: Local(
                                        Local {
                                            id: NodeId(17),
                                            pat: Pat {
                                                id: NodeId(18),
                                                kind: Ident(
                                                    ByValue(
                                                        Not,
                                                    ),
                                                    g#0,
                                                    None,
                                                ),
                                                span: test.rs:13:31: 13:33 (#4),
                                                tokens: None,
                                            },
                                            ty: None,
                                            kind: Init(
                                                Expr {
                                                    id: NodeId(19),
                                                    kind: Lit(
                                                        Lit {
                                                            token: Lit {
                                                                kind: Integer,
                                                                symbol: "15",
                                                                suffix: None,
                                                            },
                                                            kind: Int(
                                                                15,
                                                                Unsuffixed,
                                                            ),
                                                            span: test.rs:13:36: 13:38 (#4),
                                                        },
                                                    ),
                                                    span: test.rs:13:36: 13:38 (#4),
                                                    attrs: ThinVec(
                                                        None,
                                                    ),
                                                    tokens: None,
                                                },
                                            ),
                                            span: test.rs:13:27: 13:39 (#4),
                                            attrs: ThinVec(
                                                None,
                                            ),
                                            tokens: None,
                                        },
                                    ),
                                    span: test.rs:13:27: 13:39 (#4),
                                },
                                Stmt {
                                    id: NodeId(20),
                                    kind: Local(
                                        Local {
                                            id: NodeId(21),
                                            pat: Pat {
                                                id: NodeId(22),
                                                kind: Ident(
                                                    ByValue(
                                                        Not,
                                                    ),
                                                    h#0,
                                                    None,
                                                ),
                                                span: test.rs:13:31: 13:33 (#4),
                                                tokens: None,
                                            },
                                            ty: None,
                                            kind: Init(
                                                Expr {
                                                    id: NodeId(23),
                                                    kind: Lit(
                                                        Lit {
                                                            token: Lit {
                                                                kind: Integer,
                                                                symbol: "15",
                                                                suffix: None,
                                                            },
                                                            kind: Int(
                                                                15,
                                                                Unsuffixed,
                                                            ),
                                                            span: test.rs:13:36: 13:38 (#4),
                                                        },
                                                    ),
                                                    span: test.rs:13:36: 13:38 (#4),
                                                    attrs: ThinVec(
                                                        None,
                                                    ),
                                                    tokens: None,
                                                },
                                            ),
                                            span: test.rs:13:27: 13:39 (#4),
                                            attrs: ThinVec(
                                                None,
                                            ),
                                            tokens: None,
                                        },
                                    ),
                                    span: test.rs:13:27: 13:39 (#4),
                                },
                                Stmt {
                                    id: NodeId(24),
                                    kind: Local(
                                        Local {
                                            id: NodeId(25),
                                            pat: Pat {
                                                id: NodeId(26),
                                                kind: Ident(
                                                    ByValue(
                                                        Not,
                                                    ),
                                                    i#0,
                                                    None,
                                                ),
                                                span: test.rs:13:31: 13:33 (#4),
                                                tokens: None,
                                            },
                                            ty: None,
                                            kind: Init(
                                                Expr {
                                                    id: NodeId(27),
                                                    kind: Lit(
                                                        Lit {
                                                            token: Lit {
                                                                kind: Integer,
                                                                symbol: "15",
                                                                suffix: None,
                                                            },
                                                            kind: Int(
                                                                15,
                                                                Unsuffixed,
                                                            ),
                                                            span: test.rs:13:36: 13:38 (#4),
                                                        },
                                                    ),
                                                    span: test.rs:13:36: 13:38 (#4),
                                                    attrs: ThinVec(
                                                        None,
                                                    ),
                                                    tokens: None,
                                                },
                                            ),
                                            span: test.rs:13:27: 13:39 (#4),
                                            attrs: ThinVec(
                                                None,
                                            ),
                                            tokens: None,
                                        },
                                    ),
                                    span: test.rs:13:27: 13:39 (#4),
                                },
                            ],
                            id: NodeId(11),
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
    id: NodeId(0),
    is_placeholder: false,
}
