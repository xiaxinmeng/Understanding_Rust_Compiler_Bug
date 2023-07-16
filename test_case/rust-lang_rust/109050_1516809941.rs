plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: unused variable: `dep_node`
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:650:9
    |
650 |         dep_node: &DepNode<K>,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_dep_node`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `msg`
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:651:9
    |
651 |         msg: impl FnOnce() -> S,
