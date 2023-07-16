`
   Checking unicode-normalization v0.1.7 (/tmp/unicode-normalization)
  time: 0.083; rss: 67MB    parsing
  time: 0.000; rss: 67MB    attributes injection
  time: 0.000; rss: 68MB    garbage collect incremental cache directory
  time: 0.000; rss: 68MB    recursion limit
  time: 0.000; rss: 68MB    crate injection
  time: 0.000; rss: 68MB    plugin loading
  time: 0.000; rss: 68MB    plugin registration
  time: 0.000; rss: 68MB    background load prev dep-graph
  time: 0.006; rss: 68MB    pre ast expansion lint checks
    time: 0.032; rss: 86MB  expand crate
    time: 0.000; rss: 86MB  check unused macros
  time: 0.032; rss: 86MB    expansion
  time: 0.000; rss: 86MB    maybe building test harness
  time: 0.001; rss: 86MB    maybe creating a macro crate
  time: 0.004; rss: 88MB    creating allocators
  time: 0.003; rss: 88MB    AST validation
  time: 0.010; rss: 91MB    name resolution
  time: 0.001; rss: 91MB    complete gated feature checking
  time: 0.000; rss: 91MB    blocked while dep-graph loading finishes
  time: 0.016; rss: 102MB   lowering ast -> hir
  time: 0.009; rss: 102MB   early lint checks
  time: 0.020; rss: 110MB   indexing hir
  time: 0.000; rss: 96MB    load query result cache
  time: 0.000; rss: 96MB    looking for entry point
  time: 0.000; rss: 96MB    looking for plugin registrar
  time: 0.001; rss: 96MB    loop checking
  time: 0.002; rss: 102MB   attribute checking
  time: 0.005; rss: 102MB   stability checking
  time: 0.018; rss: 121MB   type collecting
  time: 0.000; rss: 121MB   outlives testing
  time: 0.000; rss: 121MB   impl wf inference
  time: 0.042; rss: 129MB   coherence checking
  time: 0.000; rss: 129MB   variance testing
  time: 0.035; rss: 136MB   wf checking
  time: 0.007; rss: 138MB   item-types checking
  time: 12.114; rss: 153MB  item-bodies checking
  time: 0.035; rss: 154MB   rvalue promotion
  time: 0.016; rss: 154MB   privacy checking
  time: 0.002; rss: 154MB   intrinsic checking
  time: 9.746; rss: 158MB   match checking
  time: 0.003; rss: 158MB   liveness checking
  time: 0.364; rss: 181MB   borrow checking
  time: 0.000; rss: 181MB   MIR borrow checking
  time: 0.000; rss: 181MB   dumping chalk-like clauses
  time: 0.000; rss: 181MB   MIR effect checking
  time: 0.004; rss: 181MB   death checking
  time: 0.001; rss: 181MB   unused lib feature checking
  time: 0.012; rss: 181MB   lint checking
  time: 0.000; rss: 181MB   resolving dependency formats
    time: 0.004; rss: 182MB write metadata
    time: 0.000; rss: 182MB assert dep graph
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::type_of
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::generics_of
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::predicates_of
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::used_trait_imports
            time: 0.004; rss: 183MB encode_query_results for rustc::ty::queries::typeck_tables_of
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::codegen_fulfill_obligation
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::optimized_mir
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::unsafety_check_result
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::borrowck
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::mir_borrowck
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::mir_const_qualif
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::def_symbol_name
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::const_is_rvalue_promotable_to_static
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::symbol_name
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::check_match
            time: 0.000; rss: 183MB encode_query_results for rustc::ty::queries::codegen_fn_attrs
            time: 0.000; rss: 184MB encode_query_results for rustc::ty::queries::specialization_graph_of
          time: 0.005; rss: 184MB   encode query results
        time: 0.007; rss: 184MB serialize query result cache
      time: 0.007; rss: 184MB   persist query result cache
          time: 0.001; rss: 185MB   getting serialized graph
          time: 0.001; rss: 185MB   encoding serialized graph
        time: 0.002; rss: 185MB encode dep-graph
      time: 0.003; rss: 185MB   persist dep-graph
    time: 0.010; rss: 185MB serialize dep graph
  time: 0.015; rss: 185MB   codegen
  time: 0.000; rss: 118MB   serialize work products
  time: 0.000; rss: 118MB   linking
    Finished dev [unoptimized + debuginfo] target(s) in 22.94s
