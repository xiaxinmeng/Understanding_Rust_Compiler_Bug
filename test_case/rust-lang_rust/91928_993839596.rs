plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0277]: expected a `std::ops::FnOnce<(&&on_disk_cache::OnDiskCache<'_>,)>` closure, found `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:776:27: 776:64]`
     |
12   |  / rustc_queries! {
13   |  |     query trigger_delay_span_bug(key: DefId) -> () {
14   |  |         desc { "trigger a delay span bug" }
14   |  |         desc { "trigger a delay span bug" }
15   |  |     }
...     |
776  |  |                 .and_then(|c| c.try_load_query_result(*tcx, id));
     |  |                  ^^^^^^^^ expected an `FnOnce<(&&on_disk_cache::OnDiskCache<'_>,)>` closure, found `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:776:27: 776:64]`
1911 |  |     }
1912 |  | }
     |  | -
     |  | |
---
     |    ------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_query_impl/src/plumbing.rs:244:1
     |
244  | /  macro_rules! define_queries {
245  | |      (<$tcx:tt>
246  | |       $($(#[$attr:meta])*
247  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
318  | |              rustc_query_description! { $name<$tcx> }
...    |
459  | |      }
460  | |  }
460  | |  }
     | |__- in this expansion of `define_queries!` (#2)
     |
     = help: the trait `std::ops::FnOnce<(&&on_disk_cache::OnDiskCache<'_>,)>` is not implemented for `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:776:27: 776:64]`
note: required by a bound in `Option::<T>::and_then`
     |
     |
1138 |         F: ~const FnOnce(T) -> Option<U>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::and_then`

error[E0277]: expected a `std::ops::FnOnce<(TypeckResults<'_>,)>` closure, found `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:778:32: 778:56]`
     |
12   |  / rustc_queries! {
13   |  |     query trigger_delay_span_bug(key: DefId) -> () {
14   |  |         desc { "trigger a delay span bug" }
14   |  |         desc { "trigger a delay span bug" }
15   |  |     }
...     |
778  |  |             typeck_results.map(|x| &*tcx.arena.alloc(x))
     |  |                            ^^^ expected an `FnOnce<(TypeckResults<'_>,)>` closure, found `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:778:32: 778:56]`
1911 |  |     }
1912 |  | }
     |  | -
     |  | |
---
     |    ------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_query_impl/src/plumbing.rs:244:1
     |
244  | /  macro_rules! define_queries {
245  | |      (<$tcx:tt>
246  | |       $($(#[$attr:meta])*
247  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
318  | |              rustc_query_description! { $name<$tcx> }
...    |
459  | |      }
460  | |  }
460  | |  }
     | |__- in this expansion of `define_queries!` (#2)
     |
     = help: the trait `std::ops::FnOnce<(TypeckResults<'_>,)>` is not implemented for `[closure@/checkout/compiler/rustc_middle/src/query/mod.rs:778:32: 778:56]`
note: required by a bound in `Option::<T>::map`
     |
     |
869  |         F: ~const FnOnce(T) -> U,
     |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_query_impl` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
