
$ cat test.rs
fn main() {}
$ rustc -Zast test.rs
Crate {
    module: Mod {
        inner: Span { lo: BytePos(0), hi: BytePos(12), expn_id: ExpnId(4294967295) },
        items: [
            Item {
                ident: #0,
                attrs: [
                    Attribute {
                        id: AttrId(
                            1
                        ),
                        style: Outer,
                        value: MetaItem {
                            name: prelude_import(85),
                            node: Word,
                            span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(0) }
                        },
                        is_sugared_doc: false,
                        span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(0) }
                    }
                ],
                id: NodeId(
                    2
                ),
                node: Use(
                    Spanned {
                        node: ViewPathGlob(
                            path(std::prelude::v1)
                        ),
                        span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                    }
                ),
                vis: Inherited,
                span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(0) }
            },
            Item {
                ident: std#0,
                attrs: [
                    Attribute {
                        id: AttrId(
                            0
                        ),
                        style: Outer,
                        value: MetaItem {
                            name: macro_use(83),
                            node: Word,
                            span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                        },
                        is_sugared_doc: false,
                        span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                    }
                ],
                id: NodeId(
                    3
                ),
                node: ExternCrate(
                    Some(
                        std(82)
                    )
                ),
                vis: Inherited,
                span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
            },
            Item {
                ident: main#0,
                attrs: [],
                id: NodeId(
                    4
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
                                5
                            ),
                            predicates: []
                        },
                        span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                    },
                    Block {
                        stmts: [],
                        id: NodeId(
                            6
                        ),
                        rules: Default,
                        span: Span { lo: BytePos(10), hi: BytePos(12), expn_id: ExpnId(4294967295) }
                    }
                ),
                vis: Inherited,
                span: Span { lo: BytePos(0), hi: BytePos(12), expn_id: ExpnId(4294967295) }
            }
        ]
    },
    attrs: [],
    span: Span { lo: BytePos(0), hi: BytePos(11), expn_id: ExpnId(4294967295) },
    exported_macros: []
}
