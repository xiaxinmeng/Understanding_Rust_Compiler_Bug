plain
    Checking ui_test v0.10.0
error[E0603]: struct `ExternProviders` is private
  --> src/tools/miri/src/bin/miri.rs:32:17
   |
32 |     ty::{query::ExternProviders, TyCtxt},
   |                 ^^^^^^^^^^^^^^^ private struct
note: the struct `ExternProviders` is defined here
  --> /checkout/compiler/rustc_middle/src/ty/query.rs:7:21
   |
   |
7  |     DynamicQueries, ExternProviders, Providers, QueryArenas, QueryCaches, QueryEngine, QueryStates,

error[E0609]: no field `rlib` on type `&mut _`
  --> src/tools/miri/src/bin/miri.rs:54:50
   |
