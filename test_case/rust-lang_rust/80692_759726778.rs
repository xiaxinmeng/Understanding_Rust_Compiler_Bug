plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `DefIdForest` doesn't implement `std::fmt::Debug`
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:519:36
     |
250  |  /    macro_rules! define_queries {
251  |  |        (<$tcx:tt>
252  |  |         $($(#[$attr:meta])*
253  |  |            [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...     |
262  | /|            define_queries_struct! {
263  | ||                tcx: $tcx,
264  | ||                input: ($(([$($modifiers)*] [$($attr)*] [$name]))*)
265  | ||            }
     | ||____________- in this macro invocation (#3)
500  |  |        }
501  |  |    }
501  |  |    }
     |  |____- in this expansion of `define_queries!` (#2)
...
506  | /     macro_rules! define_queries_struct {
507  | |         (tcx: $tcx:tt,
508  | |          input: ($(([$($modifiers:tt)*] [$($attr:tt)*] [$name:ident]))*)) => {
509  | |             pub struct Queries<$tcx> {
...    |
519  |                   $($(#[$attr])*  $name: QueryState<
     |  ________________________________________^
520  |                       crate::dep_graph::DepKind,
521  |                       <TyCtxt<$tcx> as QueryContext>::Query,
522  |                       <queries::$name<$tcx> as QueryAccessors<TyCtxt<'tcx>>>::Cache,
523  | |                 >,)*
     | |_________________^ `DefIdForest` cannot be formatted using `{:?}`
556  | |         };
557  | |     }
557  | |     }
     | |_____- in this expansion of `define_queries_struct!` (#3)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |       rustc_query_append! { [define_queries!][<'tcx>] }
     |       ------------------------------------------------- in this macro invocation (#1)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |     / rustc_queries! {
38   |     |     Other {
39   |     |         query trigger_delay_span_bug(key: DefId) -> () {
40   |     |             desc { "trigger a delay span bug" }
1690 |     |     }
1691 |     | }
     |     | -
     |     | |
     |     | |
     |     |_in this expansion of `rustc_query_append!` (#1)
     |       in this macro invocation (#2)
     |
     = help: the trait `std::fmt::Debug` is not implemented for `DefIdForest`
     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
     = note: required because of the requirements on the impl of `QueryCache` for `rustc_query_system::query::caches::DefaultCache<ty::ParamEnvAnd<'_, &TyS<'_>>, DefIdForest>`

error[E0277]: `DefIdForest` doesn't implement `std::fmt::Debug`
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:349:13
     |
250  |  /  macro_rules! define_queries {
251  |  |      (<$tcx:tt>
252  |  |       $($(#[$attr:meta])*
253  |  |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...     |
349  | /|              type Stored = <
350  | ||                  query_storage!([$($modifiers)*][$($K)*, $V])
351  | ||                  as QueryStorage
352  | ||              >::Stored;
     | ||_______________________^ `DefIdForest` cannot be formatted using `{:?}`
500  |  |      }
501  |  |  }
501  |  |  }
     |  |__- in this expansion of `define_queries!` (#2)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |   / rustc_queries! {
38   |   |     Other {
39   |   |         query trigger_delay_span_bug(key: DefId) -> () {
40   |   |             desc { "trigger a delay span bug" }
1690 |   |     }
1691 |   | }
     |   | -
     |   | |
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
     |
     = help: the trait `std::fmt::Debug` is not implemented for `DefIdForest`
     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
     = note: required because of the requirements on the impl of `rustc_query_system::query::QueryStorage` for `rustc_query_system::query::caches::DefaultCache<ty::ParamEnvAnd<'tcx, &'tcx TyS<'tcx>>, DefIdForest>`

error[E0277]: `DefIdForest` doesn't implement `std::fmt::Debug`
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:364:54
     |
250  | /  macro_rules! define_queries {
251  | |      (<$tcx:tt>
252  | |       $($(#[$attr:meta])*
253  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
364  | |              fn query_state<'a>(tcx: TyCtxt<$tcx>) -> &'a QueryState<crate::dep_graph::DepKind, <TyCtxt<$tcx> as QueryContext>::Query, Self::Cache...
     | |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `DefIdForest` cannot be formatted using `{:?}`
500  | |      }
501  | |  }
501  | |  }
     | |__- in this expansion of `define_queries!` (#2)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |    rustc_query_append! { [define_queries!][<'tcx>] }
     |    ------------------------------------------------- in this macro invocation (#1)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |  / rustc_queries! {
38   |  |     Other {
39   |  |         query trigger_delay_span_bug(key: DefId) -> () {
40   |  |             desc { "trigger a delay span bug" }
1690 |  |     }
1691 |  | }
     |  | -
     |  | |
     |  | |
     |  |_in this expansion of `rustc_query_append!` (#1)
     |    in this macro invocation (#2)
     |
     = help: the trait `std::fmt::Debug` is not implemented for `DefIdForest`
     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
     = note: required because of the requirements on the impl of `QueryCache` for `rustc_query_system::query::caches::DefaultCache<ty::ParamEnvAnd<'_, &TyS<'_>>, DefIdForest>`

error[E0277]: `DefIdForest` doesn't implement `std::fmt::Debug`
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:445:13
     |
250  |  /  macro_rules! define_queries {
251  |  |      (<$tcx:tt>
252  |  |       $($(#[$attr:meta])*
253  |  |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...     |
445  | /|              pub fn $name(self, key: query_helper_param_ty!($($K)*))
446  | ||                  -> <queries::$name<$tcx> as QueryConfig>::Stored
447  | ||              {
448  | ||                  self.at(DUMMY_SP).$name(key.into_query_param())
449  | ||              })*
     | ||______________^ `DefIdForest` cannot be formatted using `{:?}`
500  |  |      }
501  |  |  }
501  |  |  }
     |  |__- in this expansion of `define_queries!` (#2)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |   / rustc_queries! {
38   |   |     Other {
39   |   |         query trigger_delay_span_bug(key: DefId) -> () {
40   |   |             desc { "trigger a delay span bug" }
1690 |   |     }
1691 |   | }
     |   | -
     |   | |
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
     |
     = help: the trait `std::fmt::Debug` is not implemented for `DefIdForest`
     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
     = note: required because of the requirements on the impl of `rustc_query_system::query::QueryStorage` for `rustc_query_system::query::caches::DefaultCache<ty::ParamEnvAnd<'_, &TyS<'_>>, DefIdForest>`

error[E0277]: `DefIdForest` doesn't implement `std::fmt::Debug`
    --> compiler/rustc_middle/src/ty/query/plumbing.rs:484:13
     |
250  |  /  macro_rules! define_queries {
251  |  |      (<$tcx:tt>
252  |  |       $($(#[$attr:meta])*
253  |  |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...     |
484  | /|              pub fn $name(self, key: query_helper_param_ty!($($K)*))
485  | ||                  -> <queries::$name<$tcx> as QueryConfig>::Stored
486  | ||              {
487  | ||                  get_query::<queries::$name<'_>, _>(self.tcx, self.span, key.into_query_param())
488  | ||              })*
     | ||______________^ `DefIdForest` cannot be formatted using `{:?}`
500  |  |      }
501  |  |  }
501  |  |  }
     |  |__- in this expansion of `define_queries!` (#2)
     | 
    ::: compiler/rustc_middle/src/query/mod.rs:37:1
     |
37   |   / rustc_queries! {
38   |   |     Other {
39   |   |         query trigger_delay_span_bug(key: DefId) -> () {
40   |   |             desc { "trigger a delay span bug" }
1690 |   |     }
1691 |   | }
     |   | -
     |   | |
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     | 
    ::: compiler/rustc_middle/src/ty/query/mod.rs:104:1
     |
104  |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
     |
     = help: the trait `std::fmt::Debug` is not implemented for `DefIdForest`
     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
     = note: required because of the requirements on the impl of `rustc_query_system::query::QueryStorage` for `rustc_query_system::query::caches::DefaultCache<ty::ParamEnvAnd<'_, &TyS<'_>>, DefIdForest>`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:08:36
