plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 12408/14530
........................................................................................ 12496/14530
........................................................................................ 12584/14530
........................................................................................ 12672/14530
.........................................................F....................i........F 12760/14530
........................................................................................ 12936/14530
.........................................................F.............................. 13024/14530
........................................................................................ 13112/14530
........................................................................................ 13112/14530
...................................F........F........................................... 13200/14530
.....................................................................FF................. 13288/14530
........................................................................................ 13464/14530
........................................................................................ 13552/14530
.........................................i.............................................. 13640/14530
........................................................................................ 13728/14530
---

---- [ui] tests/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

37    = help: the trait `Future` is not implemented for `[async fn body@$DIR/async.rs:23:29: 25:2]`
38    = note: [async fn body@$DIR/async.rs:23:29: 25:2] must be a future or must implement `IntoFuture` to be awaited
39 
- error: internal compiler error: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:23:29: 25:2]], def_id: ...) }, Term::Ty(u32)), []), depth=0)`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: internal compiler error: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@$DIR/async.rs:23:29: 25:2]], def_id: ...) }, Term::Ty(u32)), [], []), depth=0)`
42    |
42    |
43 LL | async fn foo(x: u32) -> u32 {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "--edition=2021" "-Z" "trait-solver=chalk"
stdout: none
--- stderr -------------------------------
error[E0277]: `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
  --> fake-test-src-base/chalkify/bugs/async.rs:23:29
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________-
LL | |     x
LL | | }
LL | | }
   | | ^
   | | |
   | |_`[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
   |
   |
   = help: the trait `Future` is not implemented for `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]`
   = note: [async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2] must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:78:1

error[E0277]: the size for values of type `<[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2] as Future>::Output` cannot be known at compilation time
  --> fake-test-src-base/chalkify/bugs/async.rs:23:29
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
LL | | }
LL | | }
   | |_^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `<[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2] as Future>::Output`
note: required by a bound in `identity_future`
  --> /rustc/FAKE_PREFIX/library/core/src/future/mod.rs:78:1

error[E0277]: `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
  --> fake-test-src-base/chalkify/bugs/async.rs:23:25
   |
LL | async fn foo(x: u32) -> u32 {
   |                         ^^^ `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
   |
   = help: the trait `Future` is not implemented for `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]`
   = note: [async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2] must be a future or must implement `IntoFuture` to be awaited

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1163:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]], def_id: DefId(2:11751 ~ core[7535]::future::future::Future::Output) }, Term::Ty(u32)), [], []), depth=0)`
  --> fake-test-src-base/chalkify/bugs/async.rs:23:25
   |
LL | async fn foo(x: u32) -> u32 {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:995:33
stack backtrace:
stack backtrace:
   0:     0x7fbbd4cb3045 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hede8ed481b9c6dba
   1:     0x7fbbd4d1c8f8 - core::fmt::write::hce870cc9a77c830d
   2:     0x7fbbd4ca4fa1 - std::io::Write::write_fmt::h081dd4c6675130e5
   3:     0x7fbbd4cb2e51 - std::sys_common::backtrace::print::h6e4cd777c507beac
   4:     0x7fbbd4cb6054 - std::panicking::default_hook::{{closure}}::heb2740e7b5f0fc01
   5:     0x7fbbd4cb5d3a - std::panicking::default_hook::hc8c5f841440f300d
   6:     0x7fbbd5791d05 - rustc_driver_impl[c924d8e3076fb322]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fbbd4cb6771 - std::panicking::rust_panic_with_hook::h500c7a940c18c94e
   8:     0x7fbbd80e6e53 - std[20a5f02fba5bfa83]::panicking::begin_panic::<rustc_errors[2ecfe13b889bf395]::ExplicitBug>::{closure#0}
   9:     0x7fbbd80d1c86 - std[20a5f02fba5bfa83]::sys_common::backtrace::__rust_end_short_backtrace::<std[20a5f02fba5bfa83]::panicking::begin_panic<rustc_errors[2ecfe13b889bf395]::ExplicitBug>::{closure#0}, !>
  10:     0x7fbbd56fe126 - std[20a5f02fba5bfa83]::panicking::begin_panic::<rustc_errors[2ecfe13b889bf395]::ExplicitBug>
  11:     0x7fbbd8012da6 - std[20a5f02fba5bfa83]::panic::panic_any::<rustc_errors[2ecfe13b889bf395]::ExplicitBug>
  12:     0x7fbbd800f9b4 - <rustc_errors[2ecfe13b889bf395]::HandlerInner>::span_bug::<rustc_span[73716f2ae1efc222]::span_encoding::Span, &alloc[8f77823ecb88b25c]::string::String>
  13:     0x7fbbd800f747 - <rustc_errors[2ecfe13b889bf395]::Handler>::span_bug::<rustc_span[73716f2ae1efc222]::span_encoding::Span, &alloc[8f77823ecb88b25c]::string::String>
  14:     0x7fbbd8099af5 - rustc_middle[fe35cc8f3113a2cf]::util::bug::opt_span_bug_fmt::<rustc_span[73716f2ae1efc222]::span_encoding::Span>::{closure#0}
  15:     0x7fbbd8099d9c - rustc_middle[fe35cc8f3113a2cf]::ty::context::tls::with_opt::<rustc_middle[fe35cc8f3113a2cf]::util::bug::opt_span_bug_fmt<rustc_span[73716f2ae1efc222]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fbbd80874d6 - rustc_middle[fe35cc8f3113a2cf]::ty::context::tls::with_context_opt::<rustc_middle[fe35cc8f3113a2cf]::ty::context::tls::with_opt<rustc_middle[fe35cc8f3113a2cf]::util::bug::opt_span_bug_fmt<rustc_span[73716f2ae1efc222]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fbbd8086ca9 - rustc_middle[fe35cc8f3113a2cf]::util::bug::opt_span_bug_fmt::<rustc_span[73716f2ae1efc222]::span_encoding::Span>
  18:     0x7fbbd56faae7 - rustc_middle[fe35cc8f3113a2cf]::util::bug::span_bug_fmt::<rustc_span[73716f2ae1efc222]::span_encoding::Span>
  19:     0x7fbbd7ffb5c8 - <rustc_infer[d6eb720282f8c395]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[7b5451b806754ec]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  20:     0x7fbbd8006f4f - <rustc_infer[d6eb720282f8c395]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[7b5451b806754ec]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  21:     0x7fbbd7ff54e4 - <rustc_infer[d6eb720282f8c395]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[7b5451b806754ec]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  22:     0x7fbbd5c4e445 - <rustc_hir_typeck[b17c2f1e86c18df8]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7fbbd5e44363 - rustc_hir_typeck[b17c2f1e86c18df8]::check::check_fn
  24:     0x7fbbd5e3f8d8 - rustc_hir_typeck[b17c2f1e86c18df8]::typeck
  25:     0x7fbbd75bdde2 - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::typeck, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  26:     0x7fbbd7261fad - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::typeck
  27:     0x7fbbd843e42d - <rustc_middle[fe35cc8f3113a2cf]::ty::context::TyCtxt>::typeck_opt_const_arg
  28:     0x7fbbd6938d8a - rustc_mir_build[b8467f4bb783233c]::thir::cx::thir_body
  29:     0x7fbbd75d901b - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::thir_body, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  30:     0x7fbbd7237495 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::thir_body
  31:     0x7fbbd685a1d9 - rustc_mir_build[b8467f4bb783233c]::build::mir_built
  32:     0x7fbbd75d3ad6 - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::mir_built, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  33:     0x7fbbd723aa47 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::mir_built
  34:     0x7fbbd61baff4 - rustc_mir_transform[aa8df32cceba6131]::check_unsafety::unsafety_check_result
  35:     0x7fbbd61b70a0 - <rustc_mir_transform[aa8df32cceba6131]::check_unsafety::provide::{closure#0} as core[7535b7c318fc0ba3]::ops::function::FnOnce<(rustc_middle[fe35cc8f3113a2cf]::ty::context::TyCtxt, rustc_span[73716f2ae1efc222]::def_id::LocalDefId)>>::call_once
  36:     0x7fbbd7524d4d - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::unsafety_check_result, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  37:     0x7fbbd725600d - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::unsafety_check_result
  38:     0x7fbbd621070c - rustc_mir_transform[aa8df32cceba6131]::mir_const
  39:     0x7fbbd75d506b - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::mir_const, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  40:     0x7fbbd723b337 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::mir_const
  41:     0x7fbbd62110db - rustc_mir_transform[aa8df32cceba6131]::mir_promoted
  42:     0x7fbbd744f14b - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::mir_promoted, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  43:     0x7fbbd723e905 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::mir_promoted
  44:     0x7fbbd69b5f55 - rustc_borrowck[33594f7b692eb861]::mir_borrowck
  45:     0x7fbbd6981ef0 - <rustc_borrowck[33594f7b692eb861]::provide::{closure#0} as core[7535b7c318fc0ba3]::ops::function::FnOnce<(rustc_middle[fe35cc8f3113a2cf]::ty::context::TyCtxt, rustc_span[73716f2ae1efc222]::def_id::LocalDefId)>>::call_once
  46:     0x7fbbd744c080 - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::mir_borrowck, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  47:     0x7fbbd726540d - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::mir_borrowck
  48:     0x7fbbd6083b10 - rustc_hir_analysis[85018b7ec7a2b8a5]::collect::type_of::find_opaque_ty_constraints_for_rpit
  49:     0x7fbbd6083000 - rustc_hir_analysis[85018b7ec7a2b8a5]::collect::type_of::type_of
  50:     0x7fbbd75c0baa - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::type_of, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  51:     0x7fbbd722cda8 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::type_of
  52:     0x7fbbd5f15ffc - rustc_hir_analysis[85018b7ec7a2b8a5]::check::check::check_opaque
  53:     0x7fbbd5f1a033 - rustc_hir_analysis[85018b7ec7a2b8a5]::check::check::check_item_type
  54:     0x7fbbd5f25e7a - rustc_hir_analysis[85018b7ec7a2b8a5]::check::check::check_mod_item_types
  55:     0x7fbbd75012e2 - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::check_mod_item_types, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  56:     0x7fbbd725c91d - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::check_mod_item_types
  57:     0x7fbbd5fb005a - <rustc_middle[fe35cc8f3113a2cf]::hir::map::Map>::for_each_module::<rustc_hir_analysis[85018b7ec7a2b8a5]::check_crate::{closure#6}::{closure#0}>
  58:     0x7fbbd5efca22 - <rustc_session[773b4bc8b2f0c01d]::session::Session>::time::<(), rustc_hir_analysis[85018b7ec7a2b8a5]::check_crate::{closure#6}>
  59:     0x7fbbd6085fa4 - rustc_hir_analysis[85018b7ec7a2b8a5]::check_crate
  60:     0x7fbbd58a31f0 - rustc_interface[43749454435cb26]::passes::analysis
  61:     0x7fbbd75c2359 - rustc_query_system[10e34677fd4aeb96]::query::plumbing::try_execute_query::<rustc_query_impl[21d08503e1561e42]::queries::analysis, rustc_query_impl[21d08503e1561e42]::plumbing::QueryCtxt>
  62:     0x7fbbd722f0c9 - <rustc_query_impl[21d08503e1561e42]::Queries as rustc_middle[fe35cc8f3113a2cf]::ty::query::QueryEngine>::analysis
  63:     0x7fbbd5794b50 - <rustc_middle[fe35cc8f3113a2cf]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>
  64:     0x7fbbd57de0f8 - <rustc_interface[43749454435cb26]::interface::Compiler>::enter::<rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}::{closure#2}, core[7535b7c318fc0ba3]::result::Result<core[7535b7c318fc0ba3]::option::Option<rustc_interface[43749454435cb26]::queries::Linker>, rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>
  65:     0x7fbbd579300f - rustc_span[73716f2ae1efc222]::with_source_map::<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_interface[43749454435cb26]::interface::run_compiler<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  66:     0x7fbbd57a55fb - <scoped_tls[12439e72793c8af9]::ScopedKey<rustc_span[73716f2ae1efc222]::SessionGlobals>>::set::<rustc_interface[43749454435cb26]::interface::run_compiler<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}>::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>
  67:     0x7fbbd57a1649 - std[20a5f02fba5bfa83]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[43749454435cb26]::util::run_in_thread_pool_with_globals<rustc_interface[43749454435cb26]::interface::run_compiler<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}>::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>
  68:     0x7fbbd57deb46 - std[20a5f02fba5bfa83]::panicking::try::<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, core[7535b7c318fc0ba3]::panic::unwind_safe::AssertUnwindSafe<<std[20a5f02fba5bfa83]::thread::Builder>::spawn_unchecked_<rustc_interface[43749454435cb26]::util::run_in_thread_pool_with_globals<rustc_interface[43749454435cb26]::interface::run_compiler<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}>::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  69:     0x7fbbd57a2725 - <<std[20a5f02fba5bfa83]::thread::Builder>::spawn_unchecked_<rustc_interface[43749454435cb26]::util::run_in_thread_pool_with_globals<rustc_interface[43749454435cb26]::interface::run_compiler<core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>, rustc_driver_impl[c924d8e3076fb322]::run_compiler::{closure#1}>::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7535b7c318fc0ba3]::result::Result<(), rustc_span[73716f2ae1efc222]::ErrorGuaranteed>>::{closure#1} as core[7535b7c318fc0ba3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  70:     0x7fbbd4cc29ee - std::sys::unix::thread::Thread::new::thread_start::h574645423260ea26
  71:     0x7fbbd4a59b43 - <unknown>
  72:     0x7fbbd4aeba00 - <unknown>
  73:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (1b52bfd66 2023-02-27) running on x86_64-unknown-linux-gnu


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
------------------------------------------


---- [ui] tests/ui/symbol-names/basic.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN5basic4main17he9f658e438f1cac0E)
+ error: symbol-name(_ZN5basic4main17he7814342fba8ded5E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic::main::he9f658e438f1cac0)
+ error: demangling(basic::main::he7814342fba8ded5)
9    |
10 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/basic.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary" "-Z" "unstable-options" "-C" "symbol-mangling-version=legacy"
stdout: none
--- stderr -------------------------------
error: symbol-name(_ZN5basic4main17he7814342fba8ded5E)
  --> fake-test-src-base/symbol-names/basic.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic::main::he7814342fba8ded5)
  --> fake-test-src-base/symbol-names/basic.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(basic::main)
  --> fake-test-src-base/symbol-names/basic.rs:8:1
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^

error: def-path(main)
---

---- [ui] tests/ui/symbol-names/issue-60925.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h13209029be24b923E)
+ error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h89636b8bace23702E)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h13209029be24b923)
+ error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h89636b8bace23702)
9    |
10 LL |         #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary" "-Z" "unstable-options" "-C" "symbol-mangling-version=legacy"
stdout: none
--- stderr -------------------------------
error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h89636b8bace23702E)
  --> fake-test-src-base/symbol-names/issue-60925.rs:21:9
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h89636b8bace23702)
  --> fake-test-src-base/symbol-names/issue-60925.rs:21:9
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
  --> fake-test-src-base/symbol-names/issue-60925.rs:21:9
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors
------------------------------------------


---- [ui] tests/ui/traits/cache-reached-depth-ice.rs stdout ----
diff of stderr:

- error: evaluate(Binder(TraitPredicate(<A as std::marker::Send>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<A as std::marker::Send>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
3    |
3    |
4 LL | fn test<X: ?Sized + Send>() {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cache-reached-depth-ice/cache-reached-depth-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/cache-reached-depth-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/cache-reached-depth-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cache-reached-depth-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cache-reached-depth-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error: evaluate(Binder(TraitPredicate(<A as std::marker::Send>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/cache-reached-depth-ice.rs:43:5
   |
LL | fn test<X: ?Sized + Send>() {}
...
...
LL |     test::<A>();

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/traits/issue-83538-tainted-cache-after-cycle.rs stdout ----
diff of stderr:

- error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
3    |
3    |
4 LL |     Vec<First>: Unpin,
7 LL |     forward();
8    |     ^^^^^^^
9 
9 
- error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
12    |
12    |
13 LL |     Third<'a, Ty>: Unpin,
16 LL |     forward();
17    |     ^^^^^^^
18 
18 
- error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
21    |
21    |
22 LL |     Third<'a, Ty>: Unpin,
25 LL |     reverse();
26    |     ^^^^^^^
27 
27 
- error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
30    |
30    |
31 LL |     Vec<First>: Unpin,

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-83538-tainted-cache-after-cycle/issue-83538-tainted-cache-after-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-83538-tainted-cache-after-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/issue-83538-tainted-cache-after-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-83538-tainted-cache-after-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-83538-tainted-cache-after-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/issue-83538-tainted-cache-after-cycle.rs:59:5
   |
LL |     Vec<First>: Unpin,
...
LL |     forward();
   |     ^^^^^^^


error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
  --> fake-test-src-base/traits/issue-83538-tainted-cache-after-cycle.rs:59:5
   |
LL |     Third<'a, Ty>: Unpin,
...
LL |     forward();
   |     ^^^^^^^


error: evaluate(Binder(TraitPredicate(<Third<'_, Ty> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
  --> fake-test-src-base/traits/issue-83538-tainted-cache-after-cycle.rs:63:5
   |
LL |     Third<'a, Ty>: Unpin,
...
LL |     reverse();
   |     ^^^^^^^


error: evaluate(Binder(TraitPredicate(<std::vec::Vec<First> as std::marker::Unpin>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/issue-83538-tainted-cache-after-cycle.rs:63:5
   |
LL |     Vec<First>: Unpin,
...
LL |     reverse();
   |     ^^^^^^^


error: aborting due to 4 previous errors
------------------------------------------


---- [ui] tests/ui/traits/issue-85360-eval-obligation-ice.rs stdout ----
diff of stderr:

- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
3    |
3    |
4 LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());

7 LL | fn test<T: Sized>(_: T) {}
8    |         - predicate
9 
- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
12    |
12    |
13 LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());

16 LL | fn test<T: Sized>(_: T) {}
18 
18 
- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
21    |
21    |
22 LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());

25 LL | fn test<T: Sized>(_: T) {}
26    |         - predicate
27 
- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
30    |
30    |
31 LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice/issue-85360-eval-obligation-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-85360-eval-obligation-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/issue-85360-eval-obligation-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/issue-85360-eval-obligation-ice.rs:9:5
   |
LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}
   |         - predicate

error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/issue-85360-eval-obligation-ice.rs:9:5
   |
LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}


error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
  --> fake-test-src-base/traits/issue-85360-eval-obligation-ice.rs:13:5
   |
LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}
   |         - predicate

error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
  --> fake-test-src-base/traits/issue-85360-eval-obligation-ice.rs:13:5
   |
LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] tests/ui/traits/project-modulo-regions.rs#without_clause stdout ----
diff of stderr:

- error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [])) = Ok(EvaluatedToOk)
+ error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
3    |
3    |
4 LL | fn test(val: MyStruct) where Helper: HelperTrait  {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.without_clause/project-modulo-regions.without_clause.stderr
To only update this specific test, also pass `--test-args traits/project-modulo-regions.rs`


error in revision `without_clause`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/project-modulo-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "without_clause" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.without_clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.without_clause/auxiliary"
stdout: none
--- stderr -------------------------------
error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [], [])) = Ok(EvaluatedToOk)
  --> fake-test-src-base/traits/project-modulo-regions.rs:50:5
   |
LL | fn test(val: MyStruct) where Helper: HelperTrait  {
...
LL |     test(val);
   |     ^^^^


error: aborting due to previous error
------------------------------------------


---- [ui] tests/ui/traits/project-modulo-regions.rs#with_clause stdout ----
diff of stderr:

- error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
3    |
3    |
4 LL | fn test(val: MyStruct) where Helper: HelperTrait  {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.with_clause/project-modulo-regions.with_clause.stderr
To only update this specific test, also pass `--test-args traits/project-modulo-regions.rs`


error in revision `with_clause`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/project-modulo-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "with_clause" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.with_clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/project-modulo-regions.with_clause/auxiliary"
stdout: none
--- stderr -------------------------------
error: evaluate(Binder(TraitPredicate(<Helper as HelperTrait>, polarity:Positive), [], [])) = Ok(EvaluatedToOkModuloRegions)
  --> fake-test-src-base/traits/project-modulo-regions.rs:50:5
   |
LL | fn test(val: MyStruct) where Helper: HelperTrait  {
...
LL |     test(val);
   |     ^^^^

