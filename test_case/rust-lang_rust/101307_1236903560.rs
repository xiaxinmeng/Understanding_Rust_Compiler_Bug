plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `try_load_from_disk` in module `crate::plumbing`
    --> compiler/rustc_query_impl/src/plumbing.rs:266:31
     |
261  | /     macro_rules! should_ever_cache_on_disk {
262  | |         ([]) => {{
264  | |         }};
264  | |         }};
265  | |         ([(cache) $($rest:tt)*]) => {{
266  | |             Some(crate::plumbing::try_load_from_disk::<Self::Value>)
     | |                                   ^^^^^^^^^^^^^^^^^^ not found in `crate::plumbing`
...    |
269  | |             should_ever_cache_on_disk!([$($modifiers)*])
     | |             |
     | |             in this macro invocation (#4)
     | |             in this macro invocation (#5)
270  | |         };
270  | |         };
271  | |     }
     | |     -
     | |     |
     | |     in this expansion of `should_ever_cache_on_disk!` (#3)
     | |_____in this expansion of `should_ever_cache_on_disk!` (#4)
     |       in this expansion of `should_ever_cache_on_disk!` (#5)
380  | /     macro_rules! define_queries {
381  | |         (
381  | |         (
382  | |          $($(#[$attr:meta])*
383  | |             [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
408  | |                     = should_ever_cache_on_disk!([$($modifiers)*]);
...    |
517  | |         }
518  | |     }
     | |_____- in this expansion of `define_queries!` (#2)
     | |_____- in this expansion of `define_queries!` (#2)
     |
    ::: compiler/rustc_query_impl/src/lib.rs:58:1
     |
58   |       rustc_query_append! { define_queries! }
     |
    ::: /checkout/compiler/rustc_middle/src/query/mod.rs:18:1
     |
18   |     / rustc_queries! {
---
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)

error[E0425]: cannot find value `try_load_from_disk` in module `crate::plumbing`
    --> compiler/rustc_query_impl/src/plumbing.rs:266:31
     |
261  | /   macro_rules! should_ever_cache_on_disk {
262  | |       ([]) => {{
264  | |       }};
264  | |       }};
265  | |       ([(cache) $($rest:tt)*]) => {{
266  | |           Some(crate::plumbing::try_load_from_disk::<Self::Value>)
     | |                                 ^^^^^^^^^^^^^^^^^^ not found in `crate::plumbing`
270  | |       };
271  | |   }
271  | |   }
     | |___- in this expansion of `should_ever_cache_on_disk!` (#3)
...
380  | /   macro_rules! define_queries {
381  | |       (
382  | |        $($(#[$attr:meta])*
383  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
408  | |                   = should_ever_cache_on_disk!([$($modifiers)*]);
...    |
517  | |       }
518  | |   }
518  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:58:1
     |
     |
58   |     rustc_query_append! { define_queries! }
     |
    ::: /checkout/compiler/rustc_middle/src/query/mod.rs:18:1
     |
18   |   / rustc_queries! {
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)

error: unused import: `crate::on_disk_cache::CacheDecoder`
 --> compiler/rustc_query_impl/src/plumbing.rs:6:5
6 | use crate::on_disk_cache::CacheDecoder;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_serialize::Decodable`
  --> compiler/rustc_query_impl/src/plumbing.rs:22:5
   |
22 | use rustc_serialize::Decodable;
