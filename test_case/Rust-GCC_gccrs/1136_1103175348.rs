
pat: Pat {
    id: NodeId(25),
    kind: Paren(
        Pat {
            id: NodeId(26),
            kind: Or(
                [
                    Pat {
                        id: NodeId(27),
                        kind: TupleStruct(
                            None,
                            Path {
                                span: main.rs:8:9: 8:29 (#0),
                                segments: [
                                    PathSegment {
                                        ident: NewTypeOrStruct#0,
                                        id: NodeId(28),
                                        args: None,
                                    },
                                    PathSegment {
                                        ident: One#0,
                                        id: NodeId(29),
                                        args: None,
                                    },
                                ],
                                tokens: None,
                            },
                            [
                                Pat {
                                    id: NodeId(30),
                                    kind: Ident(
                                        ByValue(
                                            Not,
                                        ),
                                        foo#0,
                                        None,
                                    ),
                                    span: main.rs:8:30: 8:33 (#0),
                                    tokens: None,
                                },
                            ],
                        ),
                        span: main.rs:8:9: 8:34 (#0),
                        tokens: None,
                    },
                    Pat {
                        id: NodeId(31),
                        kind: Struct(
                            None,
                            Path {
                                span: main.rs:8:37: 8:57 (#0),
                                segments: [
                                    PathSegment {
                                        ident: NewTypeOrStruct#0,
                                        id: NodeId(32),
                                        args: None,
                                    },
                                    PathSegment {
                                        ident: Two#0,
                                        id: NodeId(33),
                                        args: None,
                                    },
                                ],
                                tokens: None,
                            },
                            [
                                PatField {
                                    ident: other#0,
                                    pat: Pat {
                                        id: NodeId(35),
                                        kind: Wild,
                                        span: main.rs:8:65: 8:66 (#0),
                                        tokens: None,
                                    },
                                    is_shorthand: false,
                                    attrs: ThinVec(
                                        None,
                                    ),
                                    id: NodeId(34),
                                    span: main.rs:8:58: 8:66 (#0),
                                    is_placeholder: false,
                                },
                                PatField {
                                    ident: value#0,
                                    pat: Pat {
                                        id: NodeId(37),
                                        kind: Ident(
                                            ByValue(
                                                Not,
                                            ),
                                            foo#0,
                                            None,
                                        ),
                                        span: main.rs:8:75: 8:78 (#0),
                                        tokens: None,
                                    },
                                    is_shorthand: false,
                                    attrs: ThinVec(
                                        None,
                                    ),
                                    id: NodeId(36),
                                    span: main.rs:8:68: 8:78 (#0),
                                    is_placeholder: false,
                                },
                            ],
                            false,
                        ),
                        span: main.rs:8:37: 8:79 (#0),
                        tokens: None,
                    },
                ],
            ),
            span: main.rs:8:9: 8:79 (#0),
            tokens: None,
        },
    ),
    span: main.rs:8:8: 8:80 (#0),
    tokens: None,
},

