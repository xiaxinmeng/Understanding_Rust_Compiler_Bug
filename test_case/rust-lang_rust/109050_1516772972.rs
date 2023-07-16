plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0609]: no field `seen_dep_nodes` on type `CurrentDepGraph<K>`
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:653:79
    |
653 |         if cfg!(debug_assertions) && let Some(seen_dep_nodes) = &self.current.seen_dep_nodes {
    |
    |
    = note: available fields are: `encoder`, `prev_index_to_index`, `anon_node_to_index`, `anon_id_seed`, `total_read_count` ... and 2 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_query_system` due to previous error
