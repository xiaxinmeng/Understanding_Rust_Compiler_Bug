
[src/librustdoc/passes/collect_intra_doc_links.rs:1605] &path_str = "$crate::g"
[src/librustdoc/passes/collect_intra_doc_links.rs:1605] &disambiguator = None
[src/librustdoc/passes/collect_intra_doc_links.rs:1605] &dox = "Links to [crate::g]"
[src/librustdoc/passes/collect_intra_doc_links.rs:1605] &link_range = Some(
    10..18,
)
[src/librustdoc/passes/collect_intra_doc_links.rs:1605] &kinds = [
    NotResolved {
        module_id: DefId(18:0 ~ inner[ea92]),
        partial_res: None,
        unresolved: "$crate::g",
    },
    NotResolved {
        module_id: DefId(18:0 ~ inner[ea92]),
        partial_res: Some(
            Def(
                Mod,
                DefId(0:0 ~ outer[8787]),
            ),
        ),
        unresolved: "g",
    },
    NotResolved {
        module_id: DefId(18:0 ~ inner[ea92]),
        partial_res: None,
        unresolved: "$crate::g",
    },
]
[src/librustdoc/passes/collect_intra_doc_links.rs:1548] cx.as_local_hir_id(item.def_id) = None
