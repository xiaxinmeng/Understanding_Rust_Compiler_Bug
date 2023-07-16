plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `Sharded`
  --> compiler/rustc_middle/src/ty/context.rs:30:51
   |
30 | use rustc_data_structures::sharded::{IntoPointer, Sharded, ShardedHashMap};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_data_structures::sharded`
  --> compiler/rustc_middle/src/ty/context.rs:60:5
   |
60 | use rustc_data_structures::sharded;
