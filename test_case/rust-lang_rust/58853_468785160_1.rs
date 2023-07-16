
File {
    shebang: None,
    attrs: [],
    items: [
        ForeignMod(
            ItemForeignMod {
                attrs: [],
                abi: Abi {
                    extern_token: Extern,
                    name: Some(
                        LitStr {
                            token: Literal { lit: Str_(C), suffix: None, span: Span { lo: BytePos(101), hi: BytePos(104), ctxt: #0 } }
                        }
                    )
                },
                brace_token: Brace,
                items: [
                    Fn(
                        ForeignItemFn {
                            attrs: [],
                            vis: Public(
                                VisPublic {
                                    pub_token: Pub
                                }
                            ),
                            ident: Ident {
                                ident: "foo3",
                                span: #0 bytes(122..126)
                            },
                            decl: FnDecl {
                                fn_token: Fn,
                                generics: Generics {
                                    lt_token: None,
                                    params: [],
                                    gt_token: None,
                                    where_clause: None
                                },
                                paren_token: Paren,
                                inputs: [
                                    Captured(
                                        ArgCaptured {
                                            pat: Ident(
                                                PatIdent {
                                                    by_ref: None,
                                                    mutability: None,
                                                    ident: Ident {
                                                        ident: "x",
                                                        span: #0 bytes(127..128)
                                                    },
                                                    subpat: None
                                                }
                                            ),
                                            colon_token: Colon,
                                            ty: Path(
                                                TypePath {
                                                    qself: None,
                                                    path: Path {
                                                        leading_colon: None,
                                                        segments: [
                                                            PathSegment {
                                                                ident: Ident {
                                                                    ident: "i32",
                                                                    span: #0 bytes(130..133)
                                                                },
                                                                arguments: None
                                                            }
                                                        ]
                                                    }
                                                }
                                            )
                                        }
                                    ),
                                    Comma
                                ],
                                variadic: Some(
                                    Dot3
                                ),
                                output: Default
                            },
                            semi_token: Semi
                        }
                    )
                ]
            }
        )
    ]
}
