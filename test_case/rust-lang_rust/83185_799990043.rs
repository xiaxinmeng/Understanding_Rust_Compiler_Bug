plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error[E0599]: no method named `dep_node_of` found for reference `&DepGraph<K>` in the current scope
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:454:52
    |
454 | ...                   let src = self.dep_node_of(dep_node_index);
    |                                      ^^^^^^^^^^^ method not found in `&DepGraph<K>`

error[E0599]: no method named `test` found for reference `&EdgeFilter` in the current scope
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:455:55
    |
455 | ...                   if forbidden_edge.test(&src, &target) {
    |                                         ^^^^ method not found in `&EdgeFilter`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_query_system`
