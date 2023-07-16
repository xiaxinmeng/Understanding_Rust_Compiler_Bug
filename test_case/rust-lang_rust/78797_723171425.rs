
[src/librustdoc/passes/collect_intra_doc_links.rs:1546] item = Item {
    source: Span {
        filename: Real(
            Named(
                "/home/peter/Dev/rust-78797-ws/inner/src/lib.rs",
            ),
        ),
        cnum: crate18,
        loline: 4,
        locol: 0,
        hiline: 4,
        hicol: 10,
        original: /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:4:1: 4:11 (#0),
    },
    name: Some(
        "f",
    ),
    attrs: Attributes {
        doc_strings: [
            DocFragment {
                line: 0,
                span: /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:3:1: 3:24 (#0),
                parent_module: None,
                doc: "Links to [crate::g]",
                kind: SugaredDoc,
            },
        ],
        other_attrs: [],
        cfg: None,
        span: Some(
            /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:3:1: 3:24 (#0),
        ),
        links: [],
        inner_docs: false,
    },
    inner: FunctionItem(
        Function {
            decl: FnDecl {
                inputs: Arguments {
                    values: [],
                },
                output: Return(
                    Tuple(
                        [],
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
            generics: Generics {
                params: [],
                where_predicates: [],
            },
            header: FnHeader {
                unsafety: Normal,
                constness: NotConst,
                asyncness: NotAsync,
                abi: Rust,
            },
            all_types: [],
            ret_types: [],
        },
    ),
    visibility: Public,
    def_id: DefId(18:3 ~ inner[ea92]::f),
    stability: None,
    deprecation: None,
}
