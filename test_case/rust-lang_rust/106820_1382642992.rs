plain

---- [ui] checkout/tests/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

9 error[E0277]: `[async fn body@$DIR/async.rs:14:29: 16:2]` is not a future
10 LL | async fn foo(x: u32) -> u32 {
11 
- error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1114:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:14:29: 16:2]], def_id: ...), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
+ error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1116:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:14:29: 16:2]], def_id: ...), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
13 LL | async fn foo(x: u32) -> u32 {
15 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "-Z" "trait-solver=chalk" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0277]: `[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________-
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | |     x
LL | | }
   | | ^
   | | |
   | |_`[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   |
   = help: the trait `Future` is not implemented for `[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]`
   = note: [async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2] must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:119:1

error[E0277]: the size for values of type `<[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2] as Future>::Output` cannot be known at compilation time
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
LL | |     x
LL | | }
   | |_^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `<[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2] as Future>::Output`
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:119:1

error[E0277]: `[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   |
LL | async fn foo(x: u32) -> u32 {
   |                         ^^^ `[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]` is not a future
   |
   = help: the trait `Future` is not implemented for `[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]`
   = note: [async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2] must be a future or must implement `IntoFuture` to be awaited

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1116:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@/checkout/tests/ui/chalkify/bugs/async.rs:14:29: 16:2]], def_id: DefId(2:11623 ~ core[b6da]::future::future::Future::Output), _use_mk_alias_ty_instead: () }, Term::Ty(u32)), []), depth=0)`
   |
   |
LL | async fn foo(x: u32) -> u32 {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:987:33
stack backtrace:
   0:     0x7f6b7a02a3e5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4f33e0273f834730
   0:     0x7f6b7a02a3e5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4f33e0273f834730
   1:     0x7f6b7a09a6c8 - core::fmt::write::h9c4fff8cf0703dcc
   2:     0x7f6b7a01c401 - std::io::Write::write_fmt::hd7310d5847249ce9
   3:     0x7f6b7a02a1f1 - std::sys_common::backtrace::print::ha648a3cba3d5fbcd
   4:     0x7f6b7a02d5d4 - std::panicking::default_hook::{{closure}}::h57f8de94420c18f3
   5:     0x7f6b7a02d29a - std::panicking::default_hook::h2550064f30c1967c
   6:     0x7f6b7aa8aaa2 - rustc_driver[57e3e93b9b73eab1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6b7a02dd44 - std::panicking::rust_panic_with_hook::h2a5480d823c4d0ef
   8:     0x7f6b7d372563 - std[676bf90e1ca9bda0]::panicking::begin_panic::<rustc_errors[2e072c53333d4556]::ExplicitBug>::{closure#0}
   9:     0x7f6b7d36c856 - std[676bf90e1ca9bda0]::sys_common::backtrace::__rust_end_short_backtrace::<std[676bf90e1ca9bda0]::panicking::begin_panic<rustc_errors[2e072c53333d4556]::ExplicitBug>::{closure#0}, !>
  10:     0x7f6b7a9f69f6 - std[676bf90e1ca9bda0]::panicking::begin_panic::<rustc_errors[2e072c53333d4556]::ExplicitBug>
  11:     0x7f6b7d518076 - std[676bf90e1ca9bda0]::panic::panic_any::<rustc_errors[2e072c53333d4556]::ExplicitBug>
  12:     0x7f6b7d515154 - <rustc_errors[2e072c53333d4556]::HandlerInner>::span_bug::<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span, &alloc[2dbb8e9faee056fe]::string::String>
  13:     0x7f6b7d514f97 - <rustc_errors[2e072c53333d4556]::Handler>::span_bug::<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span, &alloc[2dbb8e9faee056fe]::string::String>
  14:     0x7f6b7d407005 - rustc_middle[52f99216f5ab3c82]::util::bug::opt_span_bug_fmt::<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span>::{closure#0}
  15:     0x7f6b7d40707c - rustc_middle[52f99216f5ab3c82]::ty::context::tls::with_opt::<rustc_middle[52f99216f5ab3c82]::util::bug::opt_span_bug_fmt<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f6b7d405776 - rustc_middle[52f99216f5ab3c82]::ty::context::tls::with_context_opt::<rustc_middle[52f99216f5ab3c82]::ty::context::tls::with_opt<rustc_middle[52f99216f5ab3c82]::util::bug::opt_span_bug_fmt<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f6b7d4056e9 - rustc_middle[52f99216f5ab3c82]::util::bug::opt_span_bug_fmt::<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span>
  18:     0x7f6b7aa01137 - rustc_middle[52f99216f5ab3c82]::util::bug::span_bug_fmt::<rustc_span[10ee80ad55ab2e0a]::span_encoding::Span>
  19:     0x7f6b7d501890 - <rustc_infer[421c930190c0dfc6]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0a26d506aebfa3]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  20:     0x7f6b7d50d782 - <rustc_infer[421c930190c0dfc6]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0a26d506aebfa3]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  21:     0x7f6b7d4f9a5d - <rustc_infer[421c930190c0dfc6]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0a26d506aebfa3]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  22:     0x7f6b7afb49b1 - <rustc_hir_typeck[931887ced7543193]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7f6b7b0d0c88 - rustc_hir_typeck[931887ced7543193]::check::check_fn
  24:     0x7f6b7b108703 - rustc_hir_typeck[931887ced7543193]::typeck
  25:     0x7f6b7c87fba3 - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::typeck, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  26:     0x7f6b7c935550 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::typeck, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  27:     0x7f6b7c54c5b0 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::typeck
  28:     0x7f6b7d8d9330 - <rustc_middle[52f99216f5ab3c82]::ty::context::TyCtxt>::typeck_opt_const_arg
  29:     0x7f6b7bcb8bbd - rustc_mir_build[596434707d341c7c]::thir::cx::thir_body
  30:     0x7f6b7c93a23f - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::thir_body, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  31:     0x7f6b7c52a476 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::thir_body
  32:     0x7f6b7bbfb25c - rustc_mir_build[596434707d341c7c]::build::mir_built
  33:     0x7f6b7c9376e1 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::mir_built, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  34:     0x7f6b7c52c9f4 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::mir_built
  35:     0x7f6b7b6f42b8 - rustc_mir_transform[382c3166da551ac2]::check_unsafety::unsafety_check_result
  36:     0x7f6b7b6efe60 - <rustc_mir_transform[382c3166da551ac2]::check_unsafety::provide::{closure#0} as core[b6dac795c22a36a2]::ops::function::FnOnce<(rustc_middle[52f99216f5ab3c82]::ty::context::TyCtxt, rustc_span[10ee80ad55ab2e0a]::def_id::LocalDefId)>>::call_once
  37:     0x7f6b7c852f9e - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::unsafety_check_result, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  38:     0x7f6b7c90fb90 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::unsafety_check_result, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  39:     0x7f6b7c541b50 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::unsafety_check_result
  40:     0x7f6b7b68c02e - rustc_mir_transform[382c3166da551ac2]::mir_const
  41:     0x7f6b7c938361 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::mir_const, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  42:     0x7f6b7c52d164 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::mir_const
  43:     0x7f6b7b68cee1 - rustc_mir_transform[382c3166da551ac2]::mir_promoted
  44:     0x7f6b7c8eeeb0 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::mir_promoted, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  45:     0x7f6b7c52fe34 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::mir_promoted
  46:     0x7f6b7bef9ec8 - rustc_borrowck[181cd22ce5996902]::mir_borrowck
  47:     0x7f6b7bec6010 - <rustc_borrowck[181cd22ce5996902]::provide::{closure#0} as core[b6dac795c22a36a2]::ops::function::FnOnce<(rustc_middle[52f99216f5ab3c82]::ty::context::TyCtxt, rustc_span[10ee80ad55ab2e0a]::def_id::LocalDefId)>>::call_once
  48:     0x7f6b7c7fd45e - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::mir_borrowck, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  49:     0x7f6b7c8ee860 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::mir_borrowck, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  50:     0x7f6b7c54f220 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::mir_borrowck
  51:     0x7f6b7b265955 - rustc_hir_analysis[1de264382893472]::collect::type_of::find_opaque_ty_constraints_for_rpit
  52:     0x7f6b7b264f73 - rustc_hir_analysis[1de264382893472]::collect::type_of::type_of
  53:     0x7f6b7c8815c9 - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::type_of, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  54:     0x7f6b7c935723 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::type_of, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  55:     0x7f6b7c5225f5 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::type_of
  56:     0x7f6b7b29f17f - rustc_hir_analysis[1de264382893472]::check::check::check_opaque
  57:     0x7f6b7b2a2f64 - rustc_hir_analysis[1de264382893472]::check::check::check_item_type
  58:     0x7f6b7b2b23aa - rustc_hir_analysis[1de264382893472]::check::check::check_mod_item_types
  59:     0x7f6b7c8461c9 - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::check_mod_item_types, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  60:     0x7f6b7c90a290 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::check_mod_item_types, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  61:     0x7f6b7c547bc0 - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::check_mod_item_types
  62:     0x7f6b7b23fffa - <rustc_session[93167a5651b1320c]::session::Session>::time::<(), rustc_hir_analysis[1de264382893472]::check_crate::{closure#6}>
  63:     0x7f6b7b28e291 - rustc_hir_analysis[1de264382893472]::check_crate
  64:     0x7f6b7abfc073 - rustc_interface[6a4848b47e4c8189]::passes::analysis
  65:     0x7f6b7c881e9e - rustc_query_system[b7517327e1e448c6]::query::plumbing::try_execute_query::<rustc_query_impl[d220e29d9ef08b51]::queries::analysis, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt>
  66:     0x7f6b7c9357e1 - rustc_query_system[b7517327e1e448c6]::query::plumbing::get_query::<rustc_query_impl[d220e29d9ef08b51]::queries::analysis, rustc_query_impl[d220e29d9ef08b51]::plumbing::QueryCtxt, rustc_middle[52f99216f5ab3c82]::dep_graph::dep_node::DepKind>
  67:     0x7f6b7c5234da - <rustc_query_impl[d220e29d9ef08b51]::Queries as rustc_middle[52f99216f5ab3c82]::ty::query::QueryEngine>::analysis
  68:     0x7f6b7aae498c - <rustc_interface[6a4848b47e4c8189]::passes::QueryContext>::enter::<rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>
  69:     0x7f6b7aa8c6ed - <rustc_interface[6a4848b47e4c8189]::queries::QueryResult<rustc_interface[6a4848b47e4c8189]::passes::QueryContext>>::enter::<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}::{closure#2}::{closure#2}>
  70:     0x7f6b7aaff50d - <rustc_interface[6a4848b47e4c8189]::interface::Compiler>::enter::<rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}::{closure#2}, core[b6dac795c22a36a2]::result::Result<core[b6dac795c22a36a2]::option::Option<rustc_interface[6a4848b47e4c8189]::queries::Linker>, rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>
  71:     0x7f6b7aa77e4c - rustc_span[10ee80ad55ab2e0a]::with_source_map::<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_interface[6a4848b47e4c8189]::interface::run_compiler<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  72:     0x7f6b7aaf1f1a - <scoped_tls[cc7f32fd8856c43a]::ScopedKey<rustc_span[10ee80ad55ab2e0a]::SessionGlobals>>::set::<rustc_interface[6a4848b47e4c8189]::interface::run_compiler<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}>::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>
  73:     0x7f6b7aaeb9c9 - std[676bf90e1ca9bda0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6a4848b47e4c8189]::util::run_in_thread_pool_with_globals<rustc_interface[6a4848b47e4c8189]::interface::run_compiler<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}>::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>
  74:     0x7f6b7aae56e6 - std[676bf90e1ca9bda0]::panic::catch_unwind::<core[b6dac795c22a36a2]::panic::unwind_safe::AssertUnwindSafe<<std[676bf90e1ca9bda0]::thread::Builder>::spawn_unchecked_<rustc_interface[6a4848b47e4c8189]::util::run_in_thread_pool_with_globals<rustc_interface[6a4848b47e4c8189]::interface::run_compiler<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}>::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>
  75:     0x7f6b7aa9b0a5 - <<std[676bf90e1ca9bda0]::thread::Builder>::spawn_unchecked_<rustc_interface[6a4848b47e4c8189]::util::run_in_thread_pool_with_globals<rustc_interface[6a4848b47e4c8189]::interface::run_compiler<core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>, rustc_driver[57e3e93b9b73eab1]::run_compiler::{closure#1}>::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b6dac795c22a36a2]::result::Result<(), rustc_errors[2e072c53333d4556]::ErrorGuaranteed>>::{closure#1} as core[b6dac795c22a36a2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  76:     0x7f6b7a03ab5e - std::sys::unix::thread::Thread::new::thread_start::h906ef9c09e5b26ef
  77:     0x7f6b79dcfb43 - <unknown>
  78:     0x7f6b79e61a00 - <unknown>
  79:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (2787ff143 2023-01-13) running on x86_64-unknown-linux-gnu

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
