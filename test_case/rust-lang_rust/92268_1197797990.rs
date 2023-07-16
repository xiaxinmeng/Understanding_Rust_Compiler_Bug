plain
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
[RUSTC-TIMING] rustc_symbol_mangling test:false 7.047
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
error[E0432]: unresolved import `std::sync::atomic::AtomicU64`
 --> compiler/rustc_transmute/src/layout/nfa.rs:4:25
4 | use std::sync::atomic::{AtomicU64, Ordering};
  |                         ^^^^^^^^^
  |                         |
  |                         no `AtomicU64` in `sync::atomic`
