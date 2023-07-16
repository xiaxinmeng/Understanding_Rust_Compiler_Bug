
inner: ImplItem(
    Impl {
        unsafety: Normal,
        generics: Generics {
            params: [],
            where_predicates: [],
        },
        provided_trait_methods: {},
        trait_: None,
        for_: ResolvedPath {
            path: Path {
                global: false,
                res: Def(
                    Struct,
                    DefId(0:3 ~ foo[8787]::MyStruct[0]),
                ),
                segments: [
                    PathSegment {
                        name: "MyStruct",
                        args: AngleBracketed {
                            args: [],
                            bindings: [],
                        },
                    },
                ],
            },
            param_names: None,
            did: DefId(0:3 ~ foo[8787]::MyStruct[0]),
            is_generic: false,
        },
        items: [
            Item {
                source: Span {
                    filename: Real(
                        "/home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs",
                    ),
                    cnum: crate0,
                    loline: 11,
                    locol: 4,
                    hiline: 13,
                    hicol: 5,
                    original: /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:11:5: 13:6,
                },
                name: Some(
                    "resolves_ok",
                ),
                attrs: Attributes {
                    doc_strings: [
                        SugaredDoc(
                            0,
                            /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:8:5: 10:47,
                            "[`resolves_ok`]\n\n[`resolves_ok`]: MyStruct::resolves_ok",
                        ),
                    ],
                    other_attrs: [],
                    cfg: None,
                    span: Some(
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:8:5: 8:24,
                    ),
                    links: [],
                    inner_docs: false,
                },
                inner: MethodItem(
                    Method {
                        generics: Generics {
                            params: [],
                            where_predicates: [],
                        },
                        decl: FnDecl {
                            inputs: Arguments {
                                values: [],
                            },
                            output: Return(
                                Generic(
                                    "Self",
                                ),
                            ),
                            c_variadic: false,
                            attrs: Attributes {
                                doc_strings: [],
                                other_attrs: [],
                                cfg: None,
                                span: None,
                                links: [],
                                inner_docs: false,
                            },
                        },
                        header: FnHeader {
                            unsafety: Normal,
                            constness: NotConst,
                            asyncness: NotAsync,
                            abi: Rust,
                        },
                        defaultness: Some(
                            Final,
                        ),
                        all_types: [],
                        ret_types: [],
                    },
                ),
                visibility: Public,
                def_id: DefId(0:6 ~ foo[8787]::{{impl}}[0]::resolves_ok[0]),
                stability: None,
                deprecation: None,
            },
            Item {
                source: Span {
                    filename: Real(
                        "/home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs",
                    ),
                    cnum: crate0,
                    loline: 17,
                    locol: 4,
                    hiline: 19,
                    hicol: 5,
                    original: /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:17:5: 19:6,
                },
                name: Some(
                    "resolves_bad",
                ),
                attrs: Attributes {
                    doc_strings: [
                        SugaredDoc(
                            0,
                            /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:14:5: 16:45,
                            "[`resolves_bad`]\n\n[`resolves_bad`]: Self::resolves_bad",
                        ),
                    ],
                    other_attrs: [],
                    cfg: None,
                    span: Some(
                        /home/xliiv/workspace/rust/src/test/rustdoc/intra-link-self.rs:14:5: 14:25,
                    ),
                    links: [],
                    inner_docs: false,
                },
                inner: MethodItem(
                    Method {
                        generics: Generics {
                            params: [],
                            where_predicates: [],
                        },
                        decl: FnDecl {
                            inputs: Arguments {
                                values: [],
                            },
                            output: Return(
                                Generic(
                                    "Self",
                                ),
                            ),
                            c_variadic: false,
                            attrs: Attributes {
                                doc_strings: [],
                                other_attrs: [],
                                cfg: None,
                                span: None,
                                links: [],
                                inner_docs: false,
                            },
                        },
                        header: FnHeader {
                            unsafety: Normal,
                            constness: NotConst,
                            asyncness: NotAsync,
                            abi: Rust,
                        },
                        defaultness: Some(
                            Final,
                        ),
                        all_types: [],
                        ret_types: [],
                    },
                ),
                visibility: Public,
                def_id: DefId(0:7 ~ foo[8787]::{{impl}}[0]::resolves_bad[0]),
                stability: None,
                deprecation: None,
            },
        ],
