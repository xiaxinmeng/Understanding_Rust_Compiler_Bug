plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0308]: mismatched types
    --> compiler/rustc_query_system/src/dep_graph/graph.rs:542:68
     |
542  |                     data.current.record_edge(dep_node_index, node, _current_fingerprint);
     |                                  -----------                       ^^^^^^^^^^^^^^^^^^^^ expected `Fingerprint`, found `()`
     |                                  arguments to this method are incorrect
     |
note: method defined here
    --> compiler/rustc_query_system/src/dep_graph/graph.rs:1158:8
    --> compiler/rustc_query_system/src/dep_graph/graph.rs:1158:8
     |
1158 |     fn record_edge(&self, dep_node_index: DepNodeIndex, key: DepNode<K>, fingerprint: Fingerprint) {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
