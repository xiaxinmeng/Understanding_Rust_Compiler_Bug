
$ RUSTDOC_LOG=rustdoc::passes::collect=debug,rustc_resolve=debug xpy test --stage 1 linkchecker library/alloc
: Some(NodeId(101625)), has_generic_args: false }], opt_ns=Some(MacroNS), record_used=true, path_span=library/alloc/src/lib.rs:1:1: 1:1 (#0), crate_lint=No)
2020-08-29T22:46:53.052491Z DEBUG rustc_resolve: resolve_path ident 0 crate#0 Some(NodeId(101624))
2020-08-29T22:46:53.052494Z DEBUG rustc_resolve: resolve_crate_root(crate#0)
2020-08-29T22:46:53.052496Z DEBUG rustc_resolve: resolve_crate_root: not DollarCrate
2020-08-29T22:46:53.052499Z DEBUG rustc_resolve: resolve_crate_root(crate#0): found no mark (ident.span = library/alloc/src/lib.rs:1:1: 1:1 (#0))
2020-08-29T22:46:53.052502Z DEBUG rustc_resolve: resolve_path ident 1 ptr#0 Some(NodeId(101625))
2020-08-29T22:46:53.053654Z DEBUG rustc_resolve: resolve_path(path=[Segment { ident: crate#0, id: Some(NodeId(101626)), has_generic_args: false }, Segment { ident: ptr#0, id
: Some(NodeId(101627)), has_generic_args: false }], opt_ns=Some(TypeNS), record_used=true, path_span=library/alloc/src/lib.rs:1:1: 1:1 (#0), crate_lint=No)
2020-08-29T22:46:53.053666Z DEBUG rustc_resolve: resolve_path ident 0 crate#0 Some(NodeId(101626))
2020-08-29T22:46:53.053669Z DEBUG rustc_resolve: resolve_crate_root(crate#0)
2020-08-29T22:46:53.053671Z DEBUG rustc_resolve: resolve_crate_root: not DollarCrate
2020-08-29T22:46:53.053673Z DEBUG rustc_resolve: resolve_crate_root(crate#0): found no mark (ident.span = library/alloc/src/lib.rs:1:1: 1:1 (#0))
2020-08-29T22:46:53.053677Z DEBUG rustc_resolve: resolve_path ident 1 ptr#0 Some(NodeId(101627))
2020-08-29T22:46:53.054499Z DEBUG rustdoc::passes::collect_intra_doc_links: crate::ptr resolved to Err(()) in namespace TypeNS with parent DefId(1:6606 ~ core[c85e]::slice[0
])
2020-08-29T22:46:53.055390Z  INFO rustdoc::passes::collect_intra_doc_links: ignoring warning from parent crate: unresolved link to `crate::ptr`
