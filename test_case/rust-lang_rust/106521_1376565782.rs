plain

---- [ui] src/test/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

9 error[E0277]: `[async fn body@$DIR/async.rs:14:29: 16:2]` is not a future
10 LL | async fn foo(x: u32) -> u32 {
11 
- error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1105:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:14:29: 16:2]], def_id: ...), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
+ error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1114:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:14:29: 16:2]], def_id: ...), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
13 LL | async fn foo(x: u32) -> u32 {
15 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "-Z" "trait-solver=chalk" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0277]: `[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________-
LL | |     x
LL | |     x
LL | | }
   | | ^
   | | |
   | |_`[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   |
   = help: the trait `Future` is not implemented for `[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]`
   = note: [async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2] must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:119:1

error[E0277]: the size for values of type `<[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2] as Future>::Output` cannot be known at compilation time
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
LL | |     x
LL | | }
   | |_^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `<[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2] as Future>::Output`
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:119:1

error[E0277]: `[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   |
LL | async fn foo(x: u32) -> u32 {
   |                         ^^^ `[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   = help: the trait `Future` is not implemented for `[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]`
   = note: [async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2] must be a future or must implement `IntoFuture` to be awaited

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1114:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@/checkout/src/test/ui/chalkify/bugs/async.rs:14:29: 16:2]], def_id: DefId(2:11627 ~ core[19b3]::future::future::Future::Output), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
   |
   |
LL | async fn foo(x: u32) -> u32 {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:978:33
stack backtrace:
   0:     0x7f653c7f1215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0aa259b0ea23397c
   0:     0x7f653c7f1215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0aa259b0ea23397c
   1:     0x7f653c861468 - core::fmt::write::h31ebfc695bc6402a
   2:     0x7f653c7e3231 - std::io::Write::write_fmt::h03f90797bde603ba
   3:     0x7f653c7f1021 - std::sys_common::backtrace::print::hab6c7fbaa8208eab
   4:     0x7f653c7f4404 - std::panicking::default_hook::{{closure}}::h293de1f8b8d10d6b
   5:     0x7f653c7f40ca - std::panicking::default_hook::he298b06575165d5a
   6:     0x7f653d240562 - rustc_driver[b1dc7b15259d67fb]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f653c7f4b74 - std::panicking::rust_panic_with_hook::hea8064f999a599bc
   8:     0x7f653faed463 - std[dbd2f9e86fae0f8e]::panicking::begin_panic::<rustc_errors[9771bd9b06579b2f]::ExplicitBug>::{closure#0}
   9:     0x7f653fae6b46 - std[dbd2f9e86fae0f8e]::sys_common::backtrace::__rust_end_short_backtrace::<std[dbd2f9e86fae0f8e]::panicking::begin_panic<rustc_errors[9771bd9b06579b2f]::ExplicitBug>::{closure#0}, !>
  10:     0x7f653d1ad456 - std[dbd2f9e86fae0f8e]::panicking::begin_panic::<rustc_errors[9771bd9b06579b2f]::ExplicitBug>
  11:     0x7f653fbf2cb6 - std[dbd2f9e86fae0f8e]::panic::panic_any::<rustc_errors[9771bd9b06579b2f]::ExplicitBug>
  12:     0x7f653fbefe64 - <rustc_errors[9771bd9b06579b2f]::HandlerInner>::span_bug::<rustc_span[6325d40afc7ce70e]::span_encoding::Span, &alloc[bda6cf56b9ad1504]::string::String>
  13:     0x7f653fbefca7 - <rustc_errors[9771bd9b06579b2f]::Handler>::span_bug::<rustc_span[6325d40afc7ce70e]::span_encoding::Span, &alloc[bda6cf56b9ad1504]::string::String>
  14:     0x7f653fb5d915 - rustc_middle[8ef92b8cf6359f9e]::util::bug::opt_span_bug_fmt::<rustc_span[6325d40afc7ce70e]::span_encoding::Span>::{closure#0}
  15:     0x7f653fb5d98c - rustc_middle[8ef92b8cf6359f9e]::ty::context::tls::with_opt::<rustc_middle[8ef92b8cf6359f9e]::util::bug::opt_span_bug_fmt<rustc_span[6325d40afc7ce70e]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f653fb589d6 - rustc_middle[8ef92b8cf6359f9e]::ty::context::tls::with_context_opt::<rustc_middle[8ef92b8cf6359f9e]::ty::context::tls::with_opt<rustc_middle[8ef92b8cf6359f9e]::util::bug::opt_span_bug_fmt<rustc_span[6325d40afc7ce70e]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f653fb58949 - rustc_middle[8ef92b8cf6359f9e]::util::bug::opt_span_bug_fmt::<rustc_span[6325d40afc7ce70e]::span_encoding::Span>
  18:     0x7f653d1b6a67 - rustc_middle[8ef92b8cf6359f9e]::util::bug::span_bug_fmt::<rustc_span[6325d40afc7ce70e]::span_encoding::Span>
  19:     0x7f653fcc4b20 - <rustc_infer[5dd0d26bf9741ac1]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[dab0cd5b1c082a44]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  20:     0x7f653fcd0a12 - <rustc_infer[5dd0d26bf9741ac1]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[dab0cd5b1c082a44]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  21:     0x7f653fcbcd01 - <rustc_infer[5dd0d26bf9741ac1]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[dab0cd5b1c082a44]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  22:     0x7f653d75ae75 - <rustc_hir_typeck[21fbffefba2733d0]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7f653d938f6e - rustc_hir_typeck[21fbffefba2733d0]::check::check_fn
  24:     0x7f653d8aba33 - rustc_hir_typeck[21fbffefba2733d0]::typeck
  25:     0x7f653f072ef3 - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::typeck, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  26:     0x7f653f126bd0 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::typeck, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  27:     0x7f653ed0f320 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::typeck
  28:     0x7f6540073a40 - <rustc_middle[8ef92b8cf6359f9e]::ty::context::TyCtxt>::typeck_opt_const_arg
  29:     0x7f653e455b8d - rustc_mir_build[55d46713e5421d94]::thir::cx::thir_body
  30:     0x7f653f12b8bf - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::thir_body, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  31:     0x7f653eced1e6 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::thir_body
  32:     0x7f653e39961c - rustc_mir_build[55d46713e5421d94]::build::mir_built
  33:     0x7f653f128d61 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::mir_built, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  34:     0x7f653ecef764 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::mir_built
  35:     0x7f653de9a258 - rustc_mir_transform[ac0c13978cd1c55d]::check_unsafety::unsafety_check_result
  36:     0x7f653de95ed0 - <rustc_mir_transform[ac0c13978cd1c55d]::check_unsafety::provide::{closure#0} as core[19b35d6a50b7c7d5]::ops::function::FnOnce<(rustc_middle[8ef92b8cf6359f9e]::ty::context::TyCtxt, rustc_span[6325d40afc7ce70e]::def_id::LocalDefId)>>::call_once
  37:     0x7f653f0462ee - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::unsafety_check_result, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  38:     0x7f653f101210 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::unsafety_check_result, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  39:     0x7f653ed048c0 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::unsafety_check_result
  40:     0x7f653de3093e - rustc_mir_transform[ac0c13978cd1c55d]::mir_const
  41:     0x7f653f1299e1 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::mir_const, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  42:     0x7f653ecefed4 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::mir_const
  43:     0x7f653de317f1 - rustc_mir_transform[ac0c13978cd1c55d]::mir_promoted
  44:     0x7f653f0e0550 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::mir_promoted, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  45:     0x7f653ecf2ba4 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::mir_promoted
  46:     0x7f653e692a98 - rustc_borrowck[90bd0d40a71e7fbf]::mir_borrowck
  47:     0x7f653e65e980 - <rustc_borrowck[90bd0d40a71e7fbf]::provide::{closure#0} as core[19b35d6a50b7c7d5]::ops::function::FnOnce<(rustc_middle[8ef92b8cf6359f9e]::ty::context::TyCtxt, rustc_span[6325d40afc7ce70e]::def_id::LocalDefId)>>::call_once
  48:     0x7f653eff0bae - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::mir_borrowck, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  49:     0x7f653f0dff00 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::mir_borrowck, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  50:     0x7f653ed11f90 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::mir_borrowck
  51:     0x7f653da05e6f - rustc_hir_analysis[65981d8d5706d0f0]::collect::type_of::find_opaque_ty_constraints_for_rpit
  52:     0x7f653da0545f - rustc_hir_analysis[65981d8d5706d0f0]::collect::type_of::type_of
  53:     0x7f653f074919 - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::type_of, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  54:     0x7f653f126da3 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::type_of, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  55:     0x7f653ece5365 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::type_of
  56:     0x7f653da4ad9f - rustc_hir_analysis[65981d8d5706d0f0]::check::check::check_opaque
  57:     0x7f653da4ed24 - rustc_hir_analysis[65981d8d5706d0f0]::check::check::check_item_type
  58:     0x7f653da5e67a - rustc_hir_analysis[65981d8d5706d0f0]::check::check::check_mod_item_types
  59:     0x7f653f039519 - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::check_mod_item_types, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  60:     0x7f653f0fb910 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::check_mod_item_types, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  61:     0x7f653ed0a930 - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::check_mod_item_types
  62:     0x7f653d9e8eaa - <rustc_middle[8ef92b8cf6359f9e]::hir::map::Map>::for_each_module::<rustc_hir_analysis[65981d8d5706d0f0]::check_crate::{closure#6}::{closure#0}>
  63:     0x7f653da882e2 - <rustc_session[99c9f7f7478e70a1]::session::Session>::time::<(), rustc_hir_analysis[65981d8d5706d0f0]::check_crate::{closure#6}>
  64:     0x7f653da32421 - rustc_hir_analysis[65981d8d5706d0f0]::check_crate
  65:     0x7f653d390dd1 - rustc_interface[7a9729cdad3629f8]::passes::analysis
  66:     0x7f653f0751ee - rustc_query_system[cbb620bab4105475]::query::plumbing::try_execute_query::<rustc_query_impl[396d33e888b0894a]::queries::analysis, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt>
  67:     0x7f653f126e61 - rustc_query_system[cbb620bab4105475]::query::plumbing::get_query::<rustc_query_impl[396d33e888b0894a]::queries::analysis, rustc_query_impl[396d33e888b0894a]::plumbing::QueryCtxt, rustc_middle[8ef92b8cf6359f9e]::dep_graph::dep_node::DepKind>
  68:     0x7f653ece624a - <rustc_query_impl[396d33e888b0894a]::Queries as rustc_middle[8ef92b8cf6359f9e]::ty::query::QueryEngine>::analysis
  69:     0x7f653d299a4c - <rustc_interface[7a9729cdad3629f8]::passes::QueryContext>::enter::<rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>
  70:     0x7f653d2b37ca - <rustc_interface[7a9729cdad3629f8]::interface::Compiler>::enter::<rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}::{closure#2}, core[19b35d6a50b7c7d5]::result::Result<core[19b35d6a50b7c7d5]::option::Option<rustc_interface[7a9729cdad3629f8]::queries::Linker>, rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>
  71:     0x7f653d241d2c - rustc_span[6325d40afc7ce70e]::with_source_map::<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_interface[7a9729cdad3629f8]::interface::run_compiler<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  72:     0x7f653d2a4d7a - <scoped_tls[9cb6d87b50b819c]::ScopedKey<rustc_span[6325d40afc7ce70e]::SessionGlobals>>::set::<rustc_interface[7a9729cdad3629f8]::interface::run_compiler<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}>::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>
  73:     0x7f653d29cfe9 - std[dbd2f9e86fae0f8e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a9729cdad3629f8]::util::run_in_thread_pool_with_globals<rustc_interface[7a9729cdad3629f8]::interface::run_compiler<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}>::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>
  74:     0x7f653d29ce56 - std[dbd2f9e86fae0f8e]::panic::catch_unwind::<core[19b35d6a50b7c7d5]::panic::unwind_safe::AssertUnwindSafe<<std[dbd2f9e86fae0f8e]::thread::Builder>::spawn_unchecked_<rustc_interface[7a9729cdad3629f8]::util::run_in_thread_pool_with_globals<rustc_interface[7a9729cdad3629f8]::interface::run_compiler<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}>::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>
  75:     0x7f653d250c95 - <<std[dbd2f9e86fae0f8e]::thread::Builder>::spawn_unchecked_<rustc_interface[7a9729cdad3629f8]::util::run_in_thread_pool_with_globals<rustc_interface[7a9729cdad3629f8]::interface::run_compiler<core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>, rustc_driver[b1dc7b15259d67fb]::run_compiler::{closure#1}>::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[19b35d6a50b7c7d5]::result::Result<(), rustc_errors[9771bd9b06579b2f]::ErrorGuaranteed>>::{closure#1} as core[19b35d6a50b7c7d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  76:     0x7f653c80197e - std::sys::unix::thread::Thread::new::thread_start::h870fb1b21011af6e
  77:     0x7f653c596b43 - <unknown>
  78:     0x7f653c628a00 - <unknown>
  79:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (9f04a640c 2023-01-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=chalk
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [thir_body] building THIR for `foo`
#2 [mir_built] building MIR for `foo`
#3 [unsafety_check_result] unsafety-checking `foo`
#4 [mir_const] preparing `foo` for borrow checking
#5 [mir_promoted] processing MIR for `foo`
#6 [mir_borrowck] borrow-checking `foo`
#7 [type_of] computing type of `foo::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
