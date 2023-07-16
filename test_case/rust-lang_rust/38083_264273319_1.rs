
$ rustc -Zunstable-options --unpretty=ast test.rs                                                                                                                                                                                                         11:39:24
Crate {
    module: Mod {
        inner: Span { lo: BytePos(0), hi: BytePos(59), expn_id: ExpnId(4294967295) },
        items: [
            Item {
                ident: main#0,
                attrs: [],
                id: NodeId(
                    4294967295
                ),
                node: Fn(
                    FnDecl {
                        inputs: [],
                        output: Default(
                            Span { lo: BytePos(10), hi: BytePos(10), expn_id: ExpnId(4294967295) }
                        ),
                        variadic: false
                    },
                    Normal,
                    Spanned {
                        node: NotConst,
                        span: Span { lo: BytePos(0), hi: BytePos(2), expn_id: ExpnId(4294967295) }
                    },
                    Rust,
                    Generics {
                        lifetimes: [],
                        ty_params: [],
                        where_clause: WhereClause {
                            id: NodeId(
                                4294967295
                            ),
                            predicates: []
                        },
                        span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                    },
                    Block {
                        stmts: [
                            stmt(4294967295: println!("Hello, World!");),
                            stmt(4294967295: go(there);)
                        ],
                        id: NodeId(
                            4294967295
                        ),
                        rules: Default,
                        span: Span { lo: BytePos(10), hi: BytePos(59), expn_id: ExpnId(4294967295) }
                    }
                ),
                vis: Inherited,
                span: Span { lo: BytePos(0), hi: BytePos(59), expn_id: ExpnId(4294967295) }
            }
        ]
    },
    attrs: [],
    span: Span { lo: BytePos(0), hi: BytePos(58), expn_id: ExpnId(4294967295) },
    exported_macros: []
}
