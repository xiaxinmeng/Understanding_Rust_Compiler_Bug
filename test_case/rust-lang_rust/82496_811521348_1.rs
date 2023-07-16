
TRACE rustdoc::passes::collect_intra_doc_links considering link 'core::cmp::Ord#lexicographical-comparison'
DEBUG rustdoc::passes::collect_intra_doc_links resolving core::cmp::Ord as a macro in the module DefId(5:5981 ~ alloc[5d23]::vec)
2:rustcDEBUG rustc_resolve resolve_path(path=[Segment { ident: core#0, id: Some(NodeId(57541)), has_generic_args: false }, Segment { ident: cmp#0, id: Some(NodeId(57542)), ha
s_generic_args: false }, Segment { ident: Ord#0, id: Some(NodeId(57543)), has_generic_args: false }], opt_ns=Some(MacroNS), record_used=false, path_span=library/proc_macro/src/lib.rs:1:1: 1:1 (#0), crate_lint=No)
2:rustcDEBUG rustc_resolve resolve_path ident 0 core#0 Some(NodeId(57541))
2:rustcDEBUG rustc_resolve resolve_path ident 1 cmp#0 Some(NodeId(57542))
error[E0773]: attempted to define built-in macro more than once
