rust
enum EarlyIntraDocLink {
  Resolved(clean::ItemLink),
  AssocItem { base_ty: Res, item_name: Symbol, ns: Namespace, /* possibly more fields are necessary */ },
  Variant { parent_def: Res, variant: Symbol }, // this might be able to be resolved early since the resolver implements DefIdTree, see https://github.com/rust-lang/rust/blob/d1065e6cefa41fe6c55c9819552cdd61529096fc/src/librustdoc/passes/collect_intra_doc_links.rs#L2121 for the current logic
  // This can't be emitted early because it's a HIR lint, it needs a TyCtxt available.
  Error { kind: ResolutionFailure, diag: DiagnosticInfo, path_str: String, disambiguator: Disambiguator },
}
