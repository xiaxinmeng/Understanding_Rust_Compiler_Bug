plain
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0432]: unresolved import `super::debug`
 --> compiler/rustc_query_system/src/dep_graph/graph.rs:8:13
  |
8 | use {super::debug::EdgeFilter, std::env};
  |             ^^^^^ could not find `debug` in `super`
error: unused import: `std::env`
 --> compiler/rustc_query_system/src/dep_graph/graph.rs:8:32
  |
  |
8 | use {super::debug::EdgeFilter, std::env};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc_query_system` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
