plain
   Compiling chalk-engine v0.55.0
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: unused imports: `AtomicUsize`, `Ordering`
  --> compiler/rustc_query_system/src/query/plumbing.rs:30:25
30 | use std::sync::atomic::{AtomicUsize, Ordering};
   |                         ^^^^^^^^^^^  ^^^^^^^^
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
