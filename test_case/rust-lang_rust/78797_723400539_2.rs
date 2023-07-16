
[src/librustdoc/passes/collect_intra_doc_links.rs:366] "fn resolve" = "fn resolve"
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &path_str = "$crate"
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &span = /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:5:1: 5:11 (#0)
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &ns = TypeNS
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &current_item = Some(
    "f",
)
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &module_id = DefId(18:0 ~ inner[ea92])
[src/librustdoc/passes/collect_intra_doc_links.rs:366] &extra_fragment = None
[compiler/rustc_resolve/src/lib.rs:3202] module = Some(Def(Mod, DefId(18:0 ~ inner[ea92])))
[compiler/rustc_resolve/src/lib.rs:3203] parent_scope = ParentScope {
    module: Some(Def(Mod, DefId(18:0 ~ inner[ea92]))),
    expansion: ExpnId(
        0,
    ),
    macro_rules: Empty,
    derives: [],
}
[compiler/rustc_resolve/src/lib.rs:3214] self.resolve_path(&Segment::from_path(path), Some(ns), parent_scope, false,
                  path.span, CrateLint::No) = Module(
    Module(
        Some(Def(Mod, DefId(0:0 ~ outer[8787]))),
    ),
)
[compiler/rustc_resolve/src/lib.rs:3204] path = Path {
    span: /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:5:1: 5:11 (#0),
    segments: [
        PathSegment {
            ident: $crate#0,
            id: NodeId(18),
            args: None,
        },
    ],
    tokens: None,
}
[compiler/rustc_resolve/src/lib.rs:3204] res = Def(
    Mod,
    DefId(0:0 ~ outer[8787]),
)
[src/librustdoc/passes/collect_intra_doc_links.rs:342] resolver.resolve_str_path_error(span, &path_str, ns, module_id) = Ok(
    (
        Path {
            span: /home/peter/Dev/rust-78797-ws/inner/src/lib.rs:5:1: 5:11 (#0),
            segments: [
                PathSegment {
                    ident: $crate#0,
                    id: NodeId(18),
                    args: None,
                },
            ],
            tokens: None,
        },
        Def(
            Mod,
            DefId(0:0 ~ outer[8787]),
        ),
    ),
)
[src/librustdoc/passes/collect_intra_doc_links.rs:368] self.resolve_path(path_str, span, ns, module_id) = Some(
    Def(
        Mod,
        DefId(0:0 ~ outer[8787]),
    ),
)
