plain
........................................................................................  5368/14985
........................................................................................  5456/14985
........................................................................................  5544/14985
........................................................................................  5632/14985
................................................F..................................F....  5720/14985
........................................................................................  5896/14985
........................................................................................  5984/14985
.........................i..........iiii................................................  6072/14985
..................................i.....................................................  6160/14985
---
---- [ui] tests/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

12    |
13 LL | async fn foo(x: u32) -> u32 {
14    |                         ^^^query stack during panic:
- #0 [typeck] type-checking `foo`
- #1 [thir_body] building THIR for `foo`
- #2 [check_match] match-checking `foo`
- #3 [mir_built] building MIR for `foo`
- #4 [unsafety_check_result] unsafety-checking `foo`
- #5 [mir_const] preparing `foo` for borrow checking
- #6 [mir_promoted] promoting constants in MIR for `foo`
- #7 [mir_borrowck] borrow-checking `foo`
- #8 [type_of] computing type of `foo::{opaque#0}`
- #9 [check_mod_item_types] checking item types in top-level module
- #10 [analysis] running analysis passes on this crate
27 error: aborting due to 2 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "--edition=2021" "-Z" "trait-solver=chalk"
stdout: none
--- stderr -------------------------------
error[E0277]: `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
  --> fake-test-src-base/chalkify/bugs/async.rs:23:25
   |
LL | async fn foo(x: u32) -> u32 {
   |                         ^^^ `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]` is not a future
   |
   = help: the trait `Future` is not implemented for `[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]`
   = note: [async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2] must be a future or must implement `IntoFuture` to be awaited

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:999:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]], def_id: DefId(2:12187 ~ core[7e3c]::future::future::Future::Output) }, Term::Ty(u32)), []), depth=0)`
  --> fake-test-src-base/chalkify/bugs/async.rs:23:25
   |
LL | async fn foo(x: u32) -> u32 {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:994:33
stack backtrace:
   0:     0x7fed4f8b3434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   0:     0x7fed4f8b3434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   1:     0x7fed4f919ff8 - core::fmt::write::h37fb89e6376d8e9a
   2:     0x7fed4f8a7be1 - std::io::Write::write_fmt::h7e8bf5292b2d120d
   3:     0x7fed4f8b3241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   4:     0x7fed4f8b63ba - std::panicking::default_hook::{{closure}}::hf51828d0de03f6e9
   5:     0x7fed4f8b609c - std::panicking::default_hook::h4a54269926513068
   6:     0x7fed503d884b - rustc_driver_impl[78a5c304c991613a]::install_ice_hook::{closure#0}
   7:     0x7fed4f8b6ac7 - std::panicking::rust_panic_with_hook::hd53c629fa78974d0
   8:     0x7fed52c136b3 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}
   9:     0x7fed52c07d36 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_end_short_backtrace::<std[c0206ad03f953d64]::panicking::begin_panic<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}, !>
  10:     0x7fed50303b26 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>
  11:     0x7fed52bbc311 - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::span_bug::<rustc_span[61b81ff29812fbd7]::span_encoding::Span, alloc[485ffa6f97f4b983]::string::String>
  12:     0x7fed52bbc14c - <rustc_errors[4d4ec3f221a18e05]::Handler>::span_bug::<rustc_span[61b81ff29812fbd7]::span_encoding::Span, alloc[485ffa6f97f4b983]::string::String>
  13:     0x7fed52d91358 - rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt::<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}
  14:     0x7fed52d9149c - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt::<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7fed52d7bda4 - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_context_opt::<rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7fed50302637 - rustc_middle[77e44844f7ae79af]::util::bug::span_bug_fmt::<rustc_span[61b81ff29812fbd7]::span_encoding::Span>
  17:     0x7fed52c6cdbf - <rustc_infer[20d0dc800496c2c]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[53b8b524635adf18]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  18:     0x7fed52c7afbd - <rustc_infer[20d0dc800496c2c]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[53b8b524635adf18]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  19:     0x7fed52c689cb - <rustc_infer[20d0dc800496c2c]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[53b8b524635adf18]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  20:     0x7fed508bffc1 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_return_expr
  21:     0x7fed50ac4b27 - rustc_hir_typeck[c862ac0bf0a18fe3]::check::check_fn
  22:     0x7fed5097f63f - rustc_hir_typeck[c862ac0bf0a18fe3]::typeck
  23:     0x7fed5226143c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::typeck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  24:     0x7fed52012e3f - rustc_query_impl[faadc628ac2b63f4]::get_query::typeck
  25:     0x7fed51723c3a - rustc_mir_build[6ea5c8f6da792d5a]::thir::cx::thir_body
  26:     0x7fed5227c843 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::thir_body, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  27:     0x7fed51ff530f - rustc_query_impl[faadc628ac2b63f4]::get_query::thir_body
  28:     0x7fed5179a12d - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 16usize]>>>
  29:     0x7fed5179e0bd - rustc_mir_build[6ea5c8f6da792d5a]::thir::pattern::check_match::check_match
  30:     0x7fed520d9eb1 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::check_match, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  31:     0x7fed5201bdf7 - rustc_query_impl[faadc628ac2b63f4]::get_query::check_match
  32:     0x7fed516975f7 - rustc_mir_build[6ea5c8f6da792d5a]::build::mir_build
  33:     0x7fed5169730f - rustc_mir_build[6ea5c8f6da792d5a]::build::mir_built
  34:     0x7fed52276b56 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_built, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  35:     0x7fed51ff746f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_built
  36:     0x7fed50e3f09f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7fed50e4a508 - rustc_mir_transform[bb8ab050da654804]::check_unsafety::unsafety_check_result
  38:     0x7fed521d026c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::unsafety_check_result, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  39:     0x7fed5200b0ef - rustc_query_impl[faadc628ac2b63f4]::get_query::unsafety_check_result
  40:     0x7fed50eef7ca - rustc_mir_transform[bb8ab050da654804]::mir_const
  41:     0x7fed522781e0 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_const, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  42:     0x7fed51ff7b1f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_const
  43:     0x7fed50ebd35f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  44:     0x7fed50eefcb3 - rustc_mir_transform[bb8ab050da654804]::mir_promoted
  45:     0x7fed520fa483 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_promoted, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  46:     0x7fed51ff95ef - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_promoted
  47:     0x7fed51830c90 - rustc_borrowck[8c9e69bb8858d9d]::mir_borrowck
  48:     0x7fed520f6fd7 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_borrowck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  49:     0x7fed52014f9f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_borrowck
  50:     0x7fed50ce1d2f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  51:     0x7fed50cf765e - rustc_hir_analysis[1fcf3da9e367a61e]::collect::type_of::find_opaque_ty_constraints_for_rpit
  52:     0x7fed50cf5bbb - rustc_hir_analysis[1fcf3da9e367a61e]::collect::type_of::type_of
  53:     0x7fed52264866 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::type_of, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  54:     0x7fed51fed46e - rustc_query_impl[faadc628ac2b63f4]::get_query::type_of
  55:     0x7fed50bae3f8 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_span[61b81ff29812fbd7]::def_id::DefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  56:     0x7fed50bb2b13 - rustc_hir_analysis[1fcf3da9e367a61e]::check::check::check_opaque
  57:     0x7fed50bb6951 - rustc_hir_analysis[1fcf3da9e367a61e]::check::check::check_item_type
  58:     0x7fed50bbdc3a - rustc_hir_analysis[1fcf3da9e367a61e]::check::check::check_mod_item_types
  59:     0x7fed521a8be7 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::check_mod_item_types, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  60:     0x7fed5200f307 - rustc_query_impl[faadc628ac2b63f4]::get_query::check_mod_item_types
  61:     0x7fed50b9a2dd - <rustc_middle[77e44844f7ae79af]::hir::map::Map>::for_each_module::<rustc_hir_analysis[1fcf3da9e367a61e]::check_crate::{closure#6}::{closure#0}>
  62:     0x7fed50c21b85 - <rustc_session[ec5f2f7e87692236]::session::Session>::time::<(), rustc_hir_analysis[1fcf3da9e367a61e]::check_crate::{closure#6}>
  63:     0x7fed50cfca21 - rustc_hir_analysis[1fcf3da9e367a61e]::check_crate
  64:     0x7fed5049252b - rustc_interface[abff33c8a295cac5]::passes::analysis
  65:     0x7fed52266681 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::analysis, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  66:     0x7fed51feefbf - rustc_query_impl[faadc628ac2b63f4]::get_query::analysis
  67:     0x7fed50383996 - <rustc_middle[77e44844f7ae79af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  68:     0x7fed5039aace - <rustc_interface[abff33c8a295cac5]::interface::Compiler>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}, core[7e3cf406427b0b2b]::result::Result<core[7e3cf406427b0b2b]::option::Option<rustc_interface[abff33c8a295cac5]::queries::Linker>, rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  69:     0x7fed5041b7b0 - rustc_span[61b81ff29812fbd7]::set_source_map::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7fed50418569 - <rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install::<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}
  71:     0x7fed5041654f - std[c0206ad03f953d64]::panicking::try::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[4934f6694d2502a5]::job::JobResult<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>>::call<<rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  72:     0x7fed5042761b - <rayon_core[4934f6694d2502a5]::job::StackJob<rayon_core[4934f6694d2502a5]::latch::LatchRef<rayon_core[4934f6694d2502a5]::latch::LockLatch>, <rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>> as rayon_core[4934f6694d2502a5]::job::Job>::execute
  73:     0x7fed5037dc93 - <rayon_core[4934f6694d2502a5]::registry::WorkerThread>::wait_until_cold
  74:     0x7fed5374e83e - <rayon_core[4934f6694d2502a5]::registry::ThreadBuilder>::run
  75:     0x7fed5038dc47 - <scoped_tls[68831d74a74a1e7f]::ScopedKey<rustc_span[61b81ff29812fbd7]::SessionGlobals>>::set::<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<(), rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  76:     0x7fed503889a0 - <<crossbeam_utils[228beb6942858fde]::thread::ScopedThreadBuilder>::spawn<<rayon_core[4934f6694d2502a5]::ThreadPoolBuilder>::build_scoped<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  77:     0x7fed503c3e3e - <alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send> as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once
  78:     0x7fed50416f96 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>
  79:     0x7fed50416926 - std[c0206ad03f953d64]::panicking::try::<(), core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  80:     0x7fed503ca2e4 - <<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7fed4f8c33fe - std::sys::unix::thread::Thread::new::thread_start::he41b1d6393e1209a
  82:     0x7fed4f65eb43 - <unknown>
  83:     0x7fed4f6f0a00 - <unknown>
  84:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=chalk
query stack during panic:
end of query stack
error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui/const-generics/late-bound-vars/in_closure.rs stdout ----
diff of stderr:

1 error: query stack during panic:
- #0 [mir_borrowck] borrow-checking `test::{closure#0}::{constant#1}`
- #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{closure#0}::{constant#1}`
- #2 [mir_for_ctfe] caching mir of `test::{closure#0}::{constant#1}` for CTFE
- #3 [eval_to_allocation_raw] const-evaluating + checking `test::{closure#0}::{constant#1}`
- #4 [eval_to_allocation_raw] const-evaluating + checking `test::{closure#0}::{constant#1}`
- #5 [eval_to_valtree] evaluating type-level constant
- #6 [typeck] type-checking `test`
- #7 [used_trait_imports] finding used_trait_imports `test`
- #8 [analysis] running analysis passes on this crate
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/in_closure/in_closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/late-bound-vars/in_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/late-bound-vars/in_closure.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/in_closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/in_closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:882:36: cannot convert `ReFree(DefId(0:5 ~ in_closure[2ba9]::test), BrNamed(DefId(0:6 ~ in_closure[2ba9]::test::'a), 'a))` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f4186c98434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   1:     0x7f4186cfeff8 - core::fmt::write::h37fb89e6376d8e9a
   1:     0x7f4186cfeff8 - core::fmt::write::h37fb89e6376d8e9a
   2:     0x7f4186c8cbe1 - std::io::Write::write_fmt::h7e8bf5292b2d120d
   3:     0x7f4186c98241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   4:     0x7f4186c9b3ba - std::panicking::default_hook::{{closure}}::hf51828d0de03f6e9
   5:     0x7f4186c9b09c - std::panicking::default_hook::h4a54269926513068
   6:     0x7f41877bd84b - rustc_driver_impl[78a5c304c991613a]::install_ice_hook::{closure#0}
   7:     0x7f4186c9bac7 - std::panicking::rust_panic_with_hook::hd53c629fa78974d0
   8:     0x7f418a52b353 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}
   9:     0x7f418a526b46 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_end_short_backtrace::<std[c0206ad03f953d64]::panicking::begin_panic<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}, !>
  10:     0x7f41877085e6 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>
  11:     0x7f418a4d9a47 - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::bug::<alloc[485ffa6f97f4b983]::string::String>
  12:     0x7f418a4d96bd - <rustc_errors[4d4ec3f221a18e05]::Handler>::bug::<alloc[485ffa6f97f4b983]::string::String>
  13:     0x7f418a6c5937 - rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt::<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}
  14:     0x7f418a6c542c - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt::<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f418a6c53b4 - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_context_opt::<rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f4187710f82 - rustc_middle[77e44844f7ae79af]::util::bug::bug_fmt
  17:     0x7f4188d0ef18 - <rustc_borrowck[8c9e69bb8858d9d]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7f4188f17120 - <rustc_borrowck[8c9e69bb8858d9d]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7f4188f18f4d - <rustc_borrowck[8c9e69bb8858d9d]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7f4188d8ae9b - <rustc_borrowck[8c9e69bb8858d9d]::type_check::TypeChecker>::push_region_constraints
  21:     0x7f4188d87968 - <rustc_borrowck[8c9e69bb8858d9d]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7f4188d749ac - rustc_borrowck[8c9e69bb8858d9d]::type_check::type_check
  23:     0x7f4188dc7a8e - rustc_borrowck[8c9e69bb8858d9d]::nll::compute_regions
  24:     0x7f4188c2afb4 - rustc_borrowck[8c9e69bb8858d9d]::do_mir_borrowck
  25:     0x7f4188c16303 - rustc_borrowck[8c9e69bb8858d9d]::mir_borrowck
  26:     0x7f41894dbfd7 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_borrowck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  27:     0x7f41893f9f9f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_borrowck
  28:     0x7f41882a235f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7f41882d6017 - rustc_mir_transform[bb8ab050da654804]::mir_drops_elaborated_and_const_checked
  30:     0x7f4189634e70 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  31:     0x7f41893dd88f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7f41882a235f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7f41882d5d12 - rustc_mir_transform[bb8ab050da654804]::mir_for_ctfe
  34:     0x7f41894dd9f6 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_for_ctfe, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  35:     0x7f41893ddf4e - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_for_ctfe
  36:     0x7f4188573cd8 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_span[61b81ff29812fbd7]::def_id::DefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7f418857b6ad - <rustc_const_eval[f6e41cab0a5d80c6]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[f6e41cab0a5d80c6]::interpret::machine::Machine>::load_mir
  38:     0x7f418848bc4c - <rustc_const_eval[f6e41cab0a5d80c6]::interpret::eval_context::InterpCx<rustc_const_eval[f6e41cab0a5d80c6]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7f41885ca917 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7f41895bfd99 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_allocation_raw, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  41:     0x7f41893fc7fc - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_allocation_raw
  42:     0x7f41885beb11 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 16usize]>>>
  43:     0x7f41885ca5f7 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_queries::eval_to_allocation_raw_provider
  44:     0x7f41895bfd99 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_allocation_raw, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  45:     0x7f41893fc7fc - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_allocation_raw
  46:     0x7f41884dde20 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_to_valtree
  47:     0x7f4188509410 - <rustc_const_eval[f6e41cab0a5d80c6]::provide::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<(rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt, rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>)>>::call_once
  48:     0x7f4189514dd3 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_valtree, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  49:     0x7f41893fd61c - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_valtree
  50:     0x7f418a457dd9 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 24usize]>>>
  51:     0x7f418a473979 - <rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  52:     0x7f418a472958 - <rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  53:     0x7f4187c85566 - <rustc_middle[77e44844f7ae79af]::ty::consts::kind::ConstKind>::eval
  54:     0x7f4187d0cbfc - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  55:     0x7f4187ca2d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  56:     0x7f4187d0a1e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  57:     0x7f4187cc3c95 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_decl
  58:     0x7f4187cc404e - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_stmt
  59:     0x7f4187cc4755 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_block_with_expected
  60:     0x7f4187d0b2d6 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  61:     0x7f4187ca2d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  62:     0x7f4187d0a1e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  63:     0x7f4187ca4cad - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_return_expr
  64:     0x7f4187ea9b27 - rustc_hir_typeck[c862ac0bf0a18fe3]::check::check_fn
  65:     0x7f4187d033b4 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_closure
  66:     0x7f4187d0b75f - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  67:     0x7f4187ca2d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  68:     0x7f4187d0a1e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  69:     0x7f4187cc3c95 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_decl
  70:     0x7f4187cc404e - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_stmt
  71:     0x7f4187cc4755 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_block_with_expected
  72:     0x7f4187d0b2d6 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  73:     0x7f4187ca2d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  74:     0x7f4187d0a1e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  75:     0x7f4187ca4cad - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_return_expr
  76:     0x7f4187ea9b27 - rustc_hir_typeck[c862ac0bf0a18fe3]::check::check_fn
  77:     0x7f4187d6463f - rustc_hir_typeck[c862ac0bf0a18fe3]::typeck
  78:     0x7f418964643c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::typeck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  79:     0x7f41893f7e3f - rustc_query_impl[faadc628ac2b63f4]::get_query::typeck
  80:     0x7f4187d592bf - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  81:     0x7f4187d63ff2 - rustc_hir_typeck[c862ac0bf0a18fe3]::used_trait_imports
  82:     0x7f418957028c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::used_trait_imports, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  83:     0x7f41893f8b9f - rustc_query_impl[faadc628ac2b63f4]::get_query::used_trait_imports
  84:     0x7f418807cc0f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  85:     0x7f418808359b - rustc_hir_analysis[1fcf3da9e367a61e]::check_unused::check_crate
  86:     0x7f41880e1a2b - rustc_hir_analysis[1fcf3da9e367a61e]::check_crate
  87:     0x7f418787752b - rustc_interface[abff33c8a295cac5]::passes::analysis
  88:     0x7f418964b681 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::analysis, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  89:     0x7f41893d3fbf - rustc_query_impl[faadc628ac2b63f4]::get_query::analysis
  90:     0x7f4187768996 - <rustc_middle[77e44844f7ae79af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  91:     0x7f418777face - <rustc_interface[abff33c8a295cac5]::interface::Compiler>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}, core[7e3cf406427b0b2b]::result::Result<core[7e3cf406427b0b2b]::option::Option<rustc_interface[abff33c8a295cac5]::queries::Linker>, rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  92:     0x7f41878007b0 - rustc_span[61b81ff29812fbd7]::set_source_map::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  93:     0x7f41877fd569 - <rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install::<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}
  94:     0x7f41877fb54f - std[c0206ad03f953d64]::panicking::try::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[4934f6694d2502a5]::job::JobResult<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>>::call<<rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  95:     0x7f418780c61b - <rayon_core[4934f6694d2502a5]::job::StackJob<rayon_core[4934f6694d2502a5]::latch::LatchRef<rayon_core[4934f6694d2502a5]::latch::LockLatch>, <rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>> as rayon_core[4934f6694d2502a5]::job::Job>::execute
  96:     0x7f4187762c93 - <rayon_core[4934f6694d2502a5]::registry::WorkerThread>::wait_until_cold
  97:     0x7f418ab3383e - <rayon_core[4934f6694d2502a5]::registry::ThreadBuilder>::run
  98:     0x7f4187772c47 - <scoped_tls[68831d74a74a1e7f]::ScopedKey<rustc_span[61b81ff29812fbd7]::SessionGlobals>>::set::<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<(), rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  99:     0x7f418776d9a0 - <<crossbeam_utils[228beb6942858fde]::thread::ScopedThreadBuilder>::spawn<<rayon_core[4934f6694d2502a5]::ThreadPoolBuilder>::build_scoped<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 100:     0x7f41877a8e3e - <alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send> as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once
 101:     0x7f41877fbf96 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>
 102:     0x7f41877fb926 - std[c0206ad03f953d64]::panicking::try::<(), core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1}::{closure#0}>>
 103:     0x7f41877af2e4 - <<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 104:     0x7f4186ca83fe - std::sys::unix::thread::Thread::new::thread_start::he41b1d6393e1209a
 105:     0x7f4186a43b43 - <unknown>
 106:     0x7f4186ad5a00 - <unknown>
 107:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] tests/ui/const-generics/late-bound-vars/simple.rs stdout ----
diff of stderr:

1 error: query stack during panic:
- #0 [mir_borrowck] borrow-checking `test::{constant#1}`
- #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{constant#1}`
- #2 [mir_for_ctfe] caching mir of `test::{constant#1}` for CTFE
- #3 [eval_to_allocation_raw] const-evaluating + checking `test::{constant#1}`
- #4 [eval_to_allocation_raw] const-evaluating + checking `test::{constant#1}`
- #5 [eval_to_valtree] evaluating type-level constant
- #6 [typeck] type-checking `test`
- #7 [used_trait_imports] finding used_trait_imports `test`
- #8 [analysis] running analysis passes on this crate
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/simple/simple.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/late-bound-vars/simple.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/late-bound-vars/simple.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/simple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/late-bound-vars/simple/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:882:36: cannot convert `ReFree(DefId(0:5 ~ simple[84ba]::test), BrNamed(DefId(0:6 ~ simple[84ba]::test::'a), 'a))` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f916c68e434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   1:     0x7f916c6f4ff8 - core::fmt::write::h37fb89e6376d8e9a
   1:     0x7f916c6f4ff8 - core::fmt::write::h37fb89e6376d8e9a
   2:     0x7f916c682be1 - std::io::Write::write_fmt::h7e8bf5292b2d120d
   3:     0x7f916c68e241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   4:     0x7f916c6913ba - std::panicking::default_hook::{{closure}}::hf51828d0de03f6e9
   5:     0x7f916c69109c - std::panicking::default_hook::h4a54269926513068
   6:     0x7f916d1b384b - rustc_driver_impl[78a5c304c991613a]::install_ice_hook::{closure#0}
   7:     0x7f916c691ac7 - std::panicking::rust_panic_with_hook::hd53c629fa78974d0
   8:     0x7f916ff21353 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}
   9:     0x7f916ff1cb46 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_end_short_backtrace::<std[c0206ad03f953d64]::panicking::begin_panic<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>::{closure#0}, !>
  10:     0x7f916d0fe5e6 - std[c0206ad03f953d64]::panicking::begin_panic::<rustc_errors[4d4ec3f221a18e05]::ExplicitBug>
  11:     0x7f916fecfa47 - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::bug::<alloc[485ffa6f97f4b983]::string::String>
  12:     0x7f916fecf6bd - <rustc_errors[4d4ec3f221a18e05]::Handler>::bug::<alloc[485ffa6f97f4b983]::string::String>
  13:     0x7f91700bb937 - rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt::<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}
  14:     0x7f91700bb42c - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt::<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f91700bb3b4 - rustc_middle[77e44844f7ae79af]::ty::context::tls::with_context_opt::<rustc_middle[77e44844f7ae79af]::ty::context::tls::with_opt<rustc_middle[77e44844f7ae79af]::util::bug::opt_span_bug_fmt<rustc_span[61b81ff29812fbd7]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f916d106f82 - rustc_middle[77e44844f7ae79af]::util::bug::bug_fmt
  17:     0x7f916e704f18 - <rustc_borrowck[8c9e69bb8858d9d]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7f916e90d120 - <rustc_borrowck[8c9e69bb8858d9d]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7f916e90ef4d - <rustc_borrowck[8c9e69bb8858d9d]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7f916e780e9b - <rustc_borrowck[8c9e69bb8858d9d]::type_check::TypeChecker>::push_region_constraints
  21:     0x7f916e77d968 - <rustc_borrowck[8c9e69bb8858d9d]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7f916e76a9ac - rustc_borrowck[8c9e69bb8858d9d]::type_check::type_check
  23:     0x7f916e7bda8e - rustc_borrowck[8c9e69bb8858d9d]::nll::compute_regions
  24:     0x7f916e620fb4 - rustc_borrowck[8c9e69bb8858d9d]::do_mir_borrowck
  25:     0x7f916e60c303 - rustc_borrowck[8c9e69bb8858d9d]::mir_borrowck
  26:     0x7f916eed1fd7 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_borrowck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  27:     0x7f916edeff9f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_borrowck
  28:     0x7f916dc9835f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7f916dccc017 - rustc_mir_transform[bb8ab050da654804]::mir_drops_elaborated_and_const_checked
  30:     0x7f916f02ae70 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  31:     0x7f916edd388f - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7f916dc9835f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7f916dccbd12 - rustc_mir_transform[bb8ab050da654804]::mir_for_ctfe
  34:     0x7f916eed39f6 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::mir_for_ctfe, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  35:     0x7f916edd3f4e - rustc_query_impl[faadc628ac2b63f4]::get_query::mir_for_ctfe
  36:     0x7f916df69cd8 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_span[61b81ff29812fbd7]::def_id::DefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7f916df716ad - <rustc_const_eval[f6e41cab0a5d80c6]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[f6e41cab0a5d80c6]::interpret::machine::Machine>::load_mir
  38:     0x7f916de81c4c - <rustc_const_eval[f6e41cab0a5d80c6]::interpret::eval_context::InterpCx<rustc_const_eval[f6e41cab0a5d80c6]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7f916dfc0917 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7f916efb5d99 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_allocation_raw, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  41:     0x7f916edf27fc - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_allocation_raw
  42:     0x7f916dfb4b11 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 16usize]>>>
  43:     0x7f916dfc05f7 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_queries::eval_to_allocation_raw_provider
  44:     0x7f916efb5d99 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_allocation_raw, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  45:     0x7f916edf27fc - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_allocation_raw
  46:     0x7f916ded3e20 - rustc_const_eval[f6e41cab0a5d80c6]::const_eval::eval_to_valtree
  47:     0x7f916deff410 - <rustc_const_eval[f6e41cab0a5d80c6]::provide::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<(rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt, rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>)>>::call_once
  48:     0x7f916ef0add3 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::eval_to_valtree, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  49:     0x7f916edf361c - rustc_query_impl[faadc628ac2b63f4]::get_query::eval_to_valtree
  50:     0x7f916fe4ddd9 - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::DefaultCache<rustc_middle[77e44844f7ae79af]::ty::ParamEnvAnd<rustc_middle[77e44844f7ae79af]::mir::interpret::GlobalId>, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 24usize]>>>
  51:     0x7f916fe69979 - <rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  52:     0x7f916fe68958 - <rustc_middle[77e44844f7ae79af]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  53:     0x7f916d67b566 - <rustc_middle[77e44844f7ae79af]::ty::consts::kind::ConstKind>::eval
  54:     0x7f916d702bfc - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  55:     0x7f916d698d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
Build completed unsuccessfully in 0:12:50
  56:     0x7f916d7001e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  57:     0x7f916d6b9c95 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_decl
  58:     0x7f916d6ba04e - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_stmt
  59:     0x7f916d6ba755 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_block_with_expected
  60:     0x7f916d7012d6 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_kind
  61:     0x7f916d698d94 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  62:     0x7f916d7001e2 - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  63:     0x7f916d69acad - <rustc_hir_typeck[c862ac0bf0a18fe3]::fn_ctxt::FnCtxt>::check_return_expr
  64:     0x7f916d89fb27 - rustc_hir_typeck[c862ac0bf0a18fe3]::check::check_fn
  65:     0x7f916d75a63f - rustc_hir_typeck[c862ac0bf0a18fe3]::typeck
  66:     0x7f916f03c43c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::typeck, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  67:     0x7f916edede3f - rustc_query_impl[faadc628ac2b63f4]::get_query::typeck
  68:     0x7f916d74f2bf - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  69:     0x7f916d759ff2 - rustc_hir_typeck[c862ac0bf0a18fe3]::used_trait_imports
  70:     0x7f916ef6628c - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::used_trait_imports, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  71:     0x7f916edeeb9f - rustc_query_impl[faadc628ac2b63f4]::get_query::used_trait_imports
  72:     0x7f916da72c0f - rustc_middle[77e44844f7ae79af]::ty::query::query_get_at::<rustc_query_system[5251dfffeb905b9e]::query::caches::VecCache<rustc_span[61b81ff29812fbd7]::def_id::LocalDefId, rustc_middle[77e44844f7ae79af]::query::erase::Erased<[u8; 8usize]>>>
  73:     0x7f916da7959b - rustc_hir_analysis[1fcf3da9e367a61e]::check_unused::check_crate
  74:     0x7f916dad7a2b - rustc_hir_analysis[1fcf3da9e367a61e]::check_crate
  75:     0x7f916d26d52b - rustc_interface[abff33c8a295cac5]::passes::analysis
  76:     0x7f916f041681 - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::analysis, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  77:     0x7f916edc9fbf - rustc_query_impl[faadc628ac2b63f4]::get_query::analysis
  78:     0x7f916d15e996 - <rustc_middle[77e44844f7ae79af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  79:     0x7f916d175ace - <rustc_interface[abff33c8a295cac5]::interface::Compiler>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}, core[7e3cf406427b0b2b]::result::Result<core[7e3cf406427b0b2b]::option::Option<rustc_interface[abff33c8a295cac5]::queries::Linker>, rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  80:     0x7f916d1f67b0 - rustc_span[61b81ff29812fbd7]::set_source_map::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  81:     0x7f916d1f3569 - <rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install::<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}
  82:     0x7f916d1f154f - std[c0206ad03f953d64]::panicking::try::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[4934f6694d2502a5]::job::JobResult<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>>::call<<rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  83:     0x7f916d20261b - <rayon_core[4934f6694d2502a5]::job::StackJob<rayon_core[4934f6694d2502a5]::latch::LatchRef<rayon_core[4934f6694d2502a5]::latch::LockLatch>, <rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>> as rayon_core[4934f6694d2502a5]::job::Job>::execute
  84:     0x7f916d158c93 - <rayon_core[4934f6694d2502a5]::registry::WorkerThread>::wait_until_cold
  85:     0x7f917052983e - <rayon_core[4934f6694d2502a5]::registry::ThreadBuilder>::run
  86:     0x7f916d168c47 - <scoped_tls[68831d74a74a1e7f]::ScopedKey<rustc_span[61b81ff29812fbd7]::SessionGlobals>>::set::<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<(), rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  87:     0x7f916d1639a0 - <<crossbeam_utils[228beb6942858fde]::thread::ScopedThreadBuilder>::spawn<<rayon_core[4934f6694d2502a5]::ThreadPoolBuilder>::build_scoped<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:     0x7f916d19ee3e - <alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send> as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once
  89:     0x7f916d1f1f96 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>
  90:     0x7f916d1f1926 - std[c0206ad03f953d64]::panicking::try::<(), core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  91:     0x7f916d1a52e4 - <<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  92:     0x7f916c69e3fe - std::sys::unix::thread::Thread::new::thread_start::he41b1d6393e1209a
  93:     0x7f916c439b43 - <unknown>
  94:     0x7f916c4cba00 - <unknown>
  95:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] tests/ui/consts/const-eval/const-eval-query-stack.rs stdout ----
diff of stderr:

5    |                ^^^^^ attempt to divide `1_i32` by zero
7 query stack during panic:
7 query stack during panic:
- #0 [eval_to_allocation_raw] const-evaluating + checking `X`
- #1 [eval_to_const_value_raw] simplifying constant for the type system `X`
- #2 [eval_to_const_value_raw] simplifying constant for the type system `X`
- #3 [lint_mod] linting top-level module
- #4 [analysis] running analysis passes on this crate
14 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/const-eval-query-stack.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-query-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="1" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary" "-Ztreat-err-as-bug=1"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/const-eval-query-stack.rs:16:16
   |
   |
LL | const X: i32 = 1 / 0; //~ERROR constant
   |                ^^^^^ attempt to divide `1_i32` by zero

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   4: rustc_interface::callbacks::track_diagnostic
   5: <rustc_errors::HandlerInner>::emit_diagnostic
   6: <rustc_errors::Handler>::emit_diagnostic
   6: <rustc_errors::Handler>::emit_diagnostic
   7: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   8: <rustc_const_eval::const_eval::error::ConstEvalErr>::report
   9: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
  10: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
  11: rustc_query_impl::get_query::eval_to_allocation_raw
  12: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, rustc_middle::query::erase::Erased<[u8; 16]>>>
  13: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  15: rustc_query_impl::get_query::eval_to_const_value_raw
  16: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  17: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  18: rustc_query_impl::get_query::eval_to_const_value_raw
  19: <rustc_middle::ty::query::TyCtxtEnsure>::const_eval_poly
  20: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc_lint::passes::LateLintPass>::check_item
  21: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
  22: rustc_hir::intravisit::walk_mod::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass>>
---
To only update this specific test, also pass `--test-args debuginfo/debuginfo-type-name-layout-ice-94961-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1" "-A" "unused" "-Crpath" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1/auxiliary" "-C" "debuginfo=2"
stdout: none
--- stderr -------------------------------
error: values of the type `[u8; usize::MAX]` are too big for the current architecture

thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1307:29
stack backtrace:
   0:     0x7fc53ebb8434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   1:     0x7fc53ec1eff8 - core::fmt::write::h37fb89e6376d8e9a
   2:     0x7fc53ebacbe1 - std::io::Write::write_fmt::h7e8bf5292b2d120d
   3:     0x7fc53ebb8241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   4:     0x7fc53ebbb3ba - std::panicking::default_hook::{{closure}}::hf51828d0de03f6e9
   5:     0x7fc53ebbb09c - std::panicking::default_hook::h4a54269926513068
   6:     0x7fc53f6dd84b - rustc_driver_impl[78a5c304c991613a]::install_ice_hook::{closure#0}
   7:     0x7fc53ebbbac7 - std::panicking::rust_panic_with_hook::hd53c629fa78974d0
   8:     0x7fc53ebbb809 - std::panicking::begin_panic_handler::{{closure}}::hc4aa488426b2a52b
   9:     0x7fc53ebb8916 - std::sys_common::backtrace::__rust_end_short_backtrace::ha052df49916415a3
  10:     0x7fc53ebbb537 - rust_begin_unwind
  11:     0x7fc53eb700d3 - core::panicking::panic_fmt::h2fa1e73fb5d7f4e0
  12:     0x7fc53f9ba5ad - std[c0206ad03f953d64]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend as rustc_codegen_ssa[5e01ed75112d19a]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[5e01ed75112d19a]::back::write::start_executing_work<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend>::{closure#4}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>
  13:     0x7fc53f8efbdc - std[c0206ad03f953d64]::panicking::try::<core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>, core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend as rustc_codegen_ssa[5e01ed75112d19a]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[5e01ed75112d19a]::back::write::start_executing_work<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend>::{closure#4}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
  14:     0x7fc53f98b743 - <<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend as rustc_codegen_ssa[5e01ed75112d19a]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[5e01ed75112d19a]::back::write::start_executing_work<rustc_codegen_llvm[8f6e52b4932c0d1]::LlvmCodegenBackend>::{closure#4}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<rustc_codegen_ssa[5e01ed75112d19a]::back::write::CompiledModules, ()>>::{closure#1} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  15:     0x7fc53ebc83fe - std::sys::unix::thread::Thread::new::thread_start::he41b1d6393e1209a
  16:     0x7fc53e963b43 - <unknown>
  17:     0x7fc53e9f5a00 - <unknown>
  18:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=2
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
---

7 error: the compiler unexpectedly panicked. this is a bug.
8 
9 query stack during panic:
- #0 [type_of] computing type of `TransactionFuture::{opaque#0}`
- #1 [check_mod_item_types] checking item types in top-level module
13 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800/issue-86800.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-86800.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/issues/issue-86800.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800/auxiliary" "--edition=2021" "-Z" "treat-err-as-bug=1"
stdout: none
error: unconstrained opaque type
  --> fake-test-src-base/impl-trait/issues/issue-86800.rs:31:34
   |
   |
LL | type TransactionFuture<'__, O> = impl '__ + Future<Output = TransactionResult<O>>;
   |
   |
   = note: `TransactionFuture` must be used in combination with a concrete type within the same module

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/impl-trait/normalize-tait-in-const.rs stdout ----
diff of stderr:

1 error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:LL:CC: Failed to normalize <for<'a, 'b> fn(&'a Alias<'b>) {foo} as std::ops::FnOnce<(&&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
3 query stack during panic:
3 query stack during panic:
- #0 [eval_to_allocation_raw] const-evaluating + checking `BAR`
- #1 [eval_to_const_value_raw] simplifying constant for the type system `BAR`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/normalize-tait-in-const/normalize-tait-in-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/normalize-tait-in-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/normalize-tait-in-const.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/normalize-tait-in-const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/normalize-tait-in-const/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:195:90: Failed to normalize <for<'a, 'b> fn(&'a Alias<'b>) {foo} as std::ops::FnOnce<(&&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
---

1 error: the compiler unexpectedly panicked. this is a bug.
2 
3 query stack during panic:
- #0 [layout_of] computing layout of `Foo`
- #1 [eval_to_allocation_raw] const-evaluating + checking `FOO`
7 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/valid_range_oob/valid_range_oob.stderr
To only update this specific test, also pass `--test-args layout/valid_range_oob.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/layout/valid_range_oob.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/valid_range_oob" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/valid_range_oob/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '257 > 255', /checkout/compiler/rustc_abi/src/layout.rs:232:25

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/mir/validate/storage-live.rs stdout ----
diff of stderr:

8 error: the compiler unexpectedly panicked. this is a bug.
9 
10 query stack during panic:
- #0 [mir_const] preparing `multiple_storage` for borrow checking
- #1 [mir_promoted] promoting constants in MIR for `multiple_storage`
14 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live/storage-live.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir/validate/storage-live.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mir/validate/storage-live.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/validate/storage-live/auxiliary" "-Zvalidate-mir" "-Ztreat-err-as-bug"
stdout: none
--- stderr -------------------------------
error: internal compiler error: broken MIR in Item(DefId(0:8 ~ storage_live[e681]::multiple_storage)) (before pass CheckPackedRef) at bb0[1]:
                                StorageLive(_1) which already has storage here
  --> fake-test-src-base/mir/validate/storage-live.rs:22:13
LL |             StorageLive(a);
   |             ^^^^^^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z validate-mir -Z treat-err-as-bug
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/panics/default-backtrace-ice.rs stdout ----
diff of stderr:

18 
19 
20 query stack during panic:
- #0 [resolver_for_lowering] getting the resolver for lowering
23 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice/default-backtrace-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panics/default-backtrace-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/panics/default-backtrace-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice/auxiliary" "-Z" "treat-err-as-bug=1"
stdout: none
error[E0425]: cannot find value `missing_ident` in this scope
  --> fake-test-src-base/panics/default-backtrace-ice.rs:21:13
   |
   |
LL | fn main() { missing_ident; }


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30
   0:     0x7fa56f68b434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h086fe7893d3eff27
   1:     0x7fa56f6f1ff8 - core::fmt::write::h37fb89e6376d8e9a
   2:     0x7fa56f67fbe1 - std::io::Write::write_fmt::h7e8bf5292b2d120d
   3:     0x7fa56f68b241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   3:     0x7fa56f68b241 - std::sys_common::backtrace::print::h7156f0782b0268c0
   4:     0x7fa56f68e3ba - std::panicking::default_hook::{{closure}}::hf51828d0de03f6e9
   5:     0x7fa56f68e09c - std::panicking::default_hook::h4a54269926513068
   6:     0x7fa5701b084b - rustc_driver_impl[78a5c304c991613a]::install_ice_hook::{closure#0}
   7:     0x7fa56f68eac7 - std::panicking::rust_panic_with_hook::hd53c629fa78974d0
   8:     0x7fa56f68e809 - std::panicking::begin_panic_handler::{{closure}}::hc4aa488426b2a52b
   9:     0x7fa56f68b916 - std::sys_common::backtrace::__rust_end_short_backtrace::ha052df49916415a3
  10:     0x7fa56f68e537 - rust_begin_unwind
  11:     0x7fa56f6430d3 - core::panicking::panic_fmt::h2fa1e73fb5d7f4e0
  12:     0x7fa573204107 - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7fa57320352c - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::emit_diagnostic::{closure#2}
  14:     0x7fa57030c001 - rustc_interface[abff33c8a295cac5]::callbacks::track_diagnostic
  15:     0x7fa573202ce7 - <rustc_errors[4d4ec3f221a18e05]::HandlerInner>::emit_diagnostic
  16:     0x7fa573201d3a - <rustc_errors[4d4ec3f221a18e05]::Handler>::emit_diagnostic
  17:     0x7fa57320c3ea - <rustc_span[61b81ff29812fbd7]::ErrorGuaranteed as rustc_errors[4d4ec3f221a18e05]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7fa5712a79b5 - <rustc_resolve[6a26609a5d8be131]::Resolver>::report_errors
  19:     0x7fa57128272e - <rustc_session[ec5f2f7e87692236]::session::Session>::time::<(), <rustc_resolve[6a26609a5d8be131]::Resolver>::resolve_crate::{closure#0}>
  20:     0x7fa5712d3139 - <rustc_resolve[6a26609a5d8be131]::Resolver>::resolve_crate
  21:     0x7fa570266c50 - rustc_interface[abff33c8a295cac5]::passes::resolver_for_lowering
  22:     0x7fa571fa587d - rustc_query_system[5251dfffeb905b9e]::query::plumbing::try_execute_query::<rustc_query_impl[faadc628ac2b63f4]::queries::resolver_for_lowering, rustc_query_impl[faadc628ac2b63f4]::plumbing::QueryCtxt>
  23:     0x7fa571dc0bbb - rustc_query_impl[faadc628ac2b63f4]::get_query::resolver_for_lowering
  24:     0x7fa57015b63a - <rustc_middle[77e44844f7ae79af]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[b23b2265246da500]::steal::Steal<(rustc_middle[77e44844f7ae79af]::ty::ResolverAstLowering, alloc[485ffa6f97f4b983]::sync::Arc<rustc_ast[24ee3e15fa5ff3e4]::ast::Crate>)>>
  25:     0x7fa5701729c4 - <rustc_interface[abff33c8a295cac5]::interface::Compiler>::enter::<rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}::{closure#2}, core[7e3cf406427b0b2b]::result::Result<core[7e3cf406427b0b2b]::option::Option<rustc_interface[abff33c8a295cac5]::queries::Linker>, rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>
  26:     0x7fa5701f37b0 - rustc_span[61b81ff29812fbd7]::set_source_map::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7fa5701f0569 - <rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install::<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}
  28:     0x7fa5701ee54f - std[c0206ad03f953d64]::panicking::try::<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[4934f6694d2502a5]::job::JobResult<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>>::call<<rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  29:     0x7fa5701ff61b - <rayon_core[4934f6694d2502a5]::job::StackJob<rayon_core[4934f6694d2502a5]::latch::LatchRef<rayon_core[4934f6694d2502a5]::latch::LockLatch>, <rayon_core[4934f6694d2502a5]::registry::Registry>::in_worker_cold<<rayon_core[4934f6694d2502a5]::thread_pool::ThreadPool>::install<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>> as rayon_core[4934f6694d2502a5]::job::Job>::execute
  30:     0x7fa570155c93 - <rayon_core[4934f6694d2502a5]::registry::WorkerThread>::wait_until_cold
  31:     0x7fa57352683e - <rayon_core[4934f6694d2502a5]::registry::ThreadBuilder>::run
  32:     0x7fa570165c47 - <scoped_tls[68831d74a74a1e7f]::ScopedKey<rustc_span[61b81ff29812fbd7]::SessionGlobals>>::set::<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<(), rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  33:     0x7fa5701609a0 - <<crossbeam_utils[228beb6942858fde]::thread::ScopedThreadBuilder>::spawn<<rayon_core[4934f6694d2502a5]::ThreadPoolBuilder>::build_scoped<rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[abff33c8a295cac5]::util::run_in_thread_pool_with_globals<rustc_interface[abff33c8a295cac5]::interface::run_compiler<core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>, rustc_driver_impl[78a5c304c991613a]::run_compiler::{closure#1}>::{closure#0}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[7e3cf406427b0b2b]::result::Result<(), rustc_span[61b81ff29812fbd7]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fa57019be3e - <alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send> as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once
  35:     0x7fa5701eef96 - std[c0206ad03f953d64]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>
  36:     0x7fa5701ee926 - std[c0206ad03f953d64]::panicking::try::<(), core[7e3cf406427b0b2b]::panic::unwind_safe::AssertUnwindSafe<<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  37:     0x7fa5701a22e4 - <<std[c0206ad03f953d64]::thread::Builder>::spawn_unchecked_<alloc[485ffa6f97f4b983]::boxed::Box<dyn core[7e3cf406427b0b2b]::ops::function::FnOnce<(), Output = ()> + core[7e3cf406427b0b2b]::marker::Send>, ()>::{closure#1} as core[7e3cf406427b0b2b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fa56f69b3fe - std::sys::unix::thread::Thread::new::thread_start::he41b1d6393e1209a
  39:     0x7fa56f436b43 - <unknown>
  40:     0x7fa56f4c8a00 - <unknown>
  41:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/treat-err-as-bug/delay_span_bug.rs stdout ----
diff of stderr:

7 error: the compiler unexpectedly panicked. this is a bug.
8 
9 query stack during panic:
- #0 [trigger_delay_span_bug] triggering a delay span bug
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/delay_span_bug.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args treat-err-as-bug/delay_span_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary" "-Ztreat-err-as-bug"
stdout: none
--- stderr -------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
  --> fake-test-src-base/treat-err-as-bug/delay_span_bug.rs:12:1
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/treat-err-as-bug/err.rs stdout ----
diff of stderr:

7 error: the compiler unexpectedly panicked. this is a bug.
8 
9 query stack during panic:
- #0 [eval_to_allocation_raw] const-evaluating + checking `C`
- #1 [eval_to_allocation_raw] const-evaluating + checking `C`
13 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args treat-err-as-bug/err.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary" "-Ztreat-err-as-bug"
stdout: none
error[E0080]: could not evaluate static initializer
  --> fake-test-src-base/treat-err-as-bug/err.rs:11:21
   |
   |
LL | pub static C: u32 = 0 - 1;


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (96534c85d 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
end of query stack
------------------------------------------

