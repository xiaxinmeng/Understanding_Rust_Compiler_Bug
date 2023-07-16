plain
........................................................................................  5368/14982
........................................................................................  5456/14982
........................................................................................  5544/14982
........................................................................................  5632/14982
.............................................F....................................F.....  5720/14982
........................................................................................  5896/14982
........................................................................................  5984/14982
......................i..........iiii...................................................  6072/14982
...............................i........................................................  6160/14982
---
............................i........................................................ii.  7480/14982
ii........i....i........................................................................  7568/14982
.................i........F.............................................................  7656/14982
........................................................................................  7744/14982
................i....i................F....F............................................  7832/14982
..i.....................................................................................  8008/14982
...........................i............................................................  8096/14982
........................................................................................  8184/14982
........................................................................................  8272/14982
---
........................................................................................ 12320/14982
........................................................................................ 12408/14982
........................................................................................ 12496/14982
........................................................................................ 12584/14982
..................i...F................................................................. 12672/14982
........................................................................................ 12848/14982
........................................................................................ 12936/14982
........................................................................................ 13024/14982
........................................................................................ 13112/14982
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

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:999:25: projection clauses should be implied from elsewhere. obligation: `Obligation(predicate=Binder(ProjectionPredicate(AliasTy { substs: [[async fn body@fake-test-src-base/chalkify/bugs/async.rs:23:29: 25:2]], def_id: DefId(2:12187 ~ core[c6ab]::future::future::Future::Output) }, Term::Ty(u32)), []), depth=0)`
  --> fake-test-src-base/chalkify/bugs/async.rs:23:25
   |
LL | async fn foo(x: u32) -> u32 {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:994:33
stack backtrace:
   0:     0x7f5db5f73434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   0:     0x7f5db5f73434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7f5db5fd9ff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7f5db5f67be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7f5db5f73241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7f5db5f763ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7f5db5f7609c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7f5db6aba14b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7f5db5f76ac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7f5db93657f3 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}
   9:     0x7f5db93579a6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_end_short_backtrace::<std[6389014f4a64c7aa]::panicking::begin_panic<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}, !>
  10:     0x7f5db69d8786 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>
  11:     0x7f5db930d5b1 - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::span_bug::<rustc_span[14791a26829ba58d]::span_encoding::Span, alloc[73332b167c8d46aa]::string::String>
  12:     0x7f5db930d3ec - <rustc_errors[b322d23e6ca248f4]::Handler>::span_bug::<rustc_span[14791a26829ba58d]::span_encoding::Span, alloc[73332b167c8d46aa]::string::String>
  13:     0x7f5db94e5e78 - rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt::<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}
  14:     0x7f5db94e5ecc - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt::<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f5db94d0844 - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_context_opt::<rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f5db69d7297 - rustc_middle[6ca339b62ee52f9a]::util::bug::span_bug_fmt::<rustc_span[14791a26829ba58d]::span_encoding::Span>
  17:     0x7f5db93bfca0 - <rustc_infer[4aad1e9a9bdbf03b]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1cb2c8fd3cdb01b0]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  18:     0x7f5db93cdd2d - <rustc_infer[4aad1e9a9bdbf03b]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1cb2c8fd3cdb01b0]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  19:     0x7f5db93bb71b - <rustc_infer[4aad1e9a9bdbf03b]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1cb2c8fd3cdb01b0]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  20:     0x7f5db6fb4c31 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_return_expr
  21:     0x7f5db71ba407 - rustc_hir_typeck[fb20a30e9a3459f6]::check::check_fn
  22:     0x7f5db70742bf - rustc_hir_typeck[fb20a30e9a3459f6]::typeck
  23:     0x7f5db89a41df - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::typeck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  24:     0x7f5db873329f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::typeck
  25:     0x7f5db7e29d9a - rustc_mir_build[c9bc81230973e476]::thir::cx::thir_body
  26:     0x7f5db89c0a27 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::thir_body, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  27:     0x7f5db871576f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::thir_body
  28:     0x7f5db7ea0b0d - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 16usize]>>>
  29:     0x7f5db7ea4bed - rustc_mir_build[c9bc81230973e476]::thir::pattern::check_match::check_match
  30:     0x7f5db8804171 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::check_match, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  31:     0x7f5db873c257 - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::check_match
  32:     0x7f5db7d9def2 - rustc_mir_build[c9bc81230973e476]::build::mir_build
  33:     0x7f5db7d9db0f - rustc_mir_build[c9bc81230973e476]::build::mir_built
  34:     0x7f5db89ba80e - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_built, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  35:     0x7f5db87178cf - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_built
  36:     0x7f5db753d0cf - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7f5db7545208 - rustc_mir_transform[4f056a5f48fe549d]::check_unsafety::unsafety_check_result
  38:     0x7f5db8909c50 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::unsafety_check_result, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  39:     0x7f5db872b54f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::unsafety_check_result
  40:     0x7f5db75ed57a - rustc_mir_transform[4f056a5f48fe549d]::mir_const
  41:     0x7f5db89bbeca - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_const, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  42:     0x7f5db8717f7f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_const
  43:     0x7f5db75bb01f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  44:     0x7f5db75edaa5 - rustc_mir_transform[4f056a5f48fe549d]::mir_promoted
  45:     0x7f5db8826f69 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_promoted, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  46:     0x7f5db8719a4f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_promoted
  47:     0x7f5db7f38460 - rustc_borrowck[a7734a23bb6b876]::mir_borrowck
  48:     0x7f5db882383c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_borrowck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  49:     0x7f5db87353ff - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_borrowck
  50:     0x7f5db73dc72f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  51:     0x7f5db73f201e - rustc_hir_analysis[82c2ffdb456254]::collect::type_of::find_opaque_ty_constraints_for_rpit
  52:     0x7f5db73f057b - rustc_hir_analysis[82c2ffdb456254]::collect::type_of::type_of
  53:     0x7f5db89a793c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::type_of, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  54:     0x7f5db870d8ce - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::type_of
  55:     0x7f5db72960ff - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_span[14791a26829ba58d]::def_id::DefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  56:     0x7f5db72aace1 - rustc_hir_analysis[82c2ffdb456254]::check::check::check_opaque
  57:     0x7f5db72aeb70 - rustc_hir_analysis[82c2ffdb456254]::check::check::check_item_type
  58:     0x7f5db72b5f9a - rustc_hir_analysis[82c2ffdb456254]::check::check::check_mod_item_types
  59:     0x7f5db88dfcc6 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::check_mod_item_types, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  60:     0x7f5db872f767 - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::check_mod_item_types
  61:     0x7f5db7291dad - <rustc_middle[6ca339b62ee52f9a]::hir::map::Map>::for_each_module::<rustc_hir_analysis[82c2ffdb456254]::check_crate::{closure#6}::{closure#0}>
  62:     0x7f5db7319f85 - <rustc_session[f5437c3c47b698c6]::session::Session>::time::<(), rustc_hir_analysis[82c2ffdb456254]::check_crate::{closure#6}>
  63:     0x7f5db73f73e1 - rustc_hir_analysis[82c2ffdb456254]::check_crate
  64:     0x7f5db6b7ff66 - rustc_interface[6f6522f1e7fd4d79]::passes::analysis
  65:     0x7f5db89a98d1 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::analysis, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  66:     0x7f5db870f41f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::analysis
  67:     0x7f5db6a586b6 - <rustc_middle[6ca339b62ee52f9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  68:     0x7f5db6a6f8be - <rustc_interface[6f6522f1e7fd4d79]::interface::Compiler>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}, core[c6ab9fa5fff67170]::result::Result<core[c6ab9fa5fff67170]::option::Option<rustc_interface[6f6522f1e7fd4d79]::queries::Linker>, rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  69:     0x7f5db6afcd10 - rustc_span[14791a26829ba58d]::set_source_map::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7f5db6af9be9 - <rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install::<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}
  71:     0x7f5db6af290f - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[3ec58862d0a38d5e]::job::JobResult<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>>::call<<rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  72:     0x7f5db6b08aeb - <rayon_core[3ec58862d0a38d5e]::job::StackJob<rayon_core[3ec58862d0a38d5e]::latch::LatchRef<rayon_core[3ec58862d0a38d5e]::latch::LockLatch>, <rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>> as rayon_core[3ec58862d0a38d5e]::job::Job>::execute
  73:     0x7f5db6a52983 - <rayon_core[3ec58862d0a38d5e]::registry::WorkerThread>::wait_until_cold
  74:     0x7f5db9eb02ee - <rayon_core[3ec58862d0a38d5e]::registry::ThreadBuilder>::run
  75:     0x7f5db6a62a37 - <scoped_tls[5f4ee6aff00e9f97]::ScopedKey<rustc_span[14791a26829ba58d]::SessionGlobals>>::set::<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<(), rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  76:     0x7f5db6a5d7f0 - <<crossbeam_utils[9323b80cd3a6c795]::thread::ScopedThreadBuilder>::spawn<<rayon_core[3ec58862d0a38d5e]::ThreadPoolBuilder>::build_scoped<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  77:     0x7f5db6aa55fe - <alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send> as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once
  78:     0x7f5db6af85b6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>
  79:     0x7f5db6af2e76 - std[6389014f4a64c7aa]::panicking::try::<(), core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  80:     0x7f5db6aabbe4 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7f5db5f833fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
  82:     0x7f5db5d1eb43 - <unknown>
  83:     0x7f5db5db0a00 - <unknown>
  84:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=chalk
query stack during panic:
end of query stack
error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0277`.
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
   0:     0x7ff853de3434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7ff853e49ff8 - core::fmt::write::ha3db1235befb69cd
   1:     0x7ff853e49ff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7ff853dd7be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7ff853de3241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7ff853de63ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7ff853de609c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7ff85492a14b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7ff853de6ac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7ff8577a4f63 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}
   9:     0x7ff8577963c6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_end_short_backtrace::<std[6389014f4a64c7aa]::panicking::begin_panic<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}, !>
  10:     0x7ff8548686b6 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>
  11:     0x7ff8576e5267 - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::bug::<alloc[73332b167c8d46aa]::string::String>
  12:     0x7ff8576e4edd - <rustc_errors[b322d23e6ca248f4]::Handler>::bug::<alloc[73332b167c8d46aa]::string::String>
  13:     0x7ff85763d837 - rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt::<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}
  14:     0x7ff85763d37c - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt::<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7ff85763d304 - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_context_opt::<rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7ff85485f882 - rustc_middle[6ca339b62ee52f9a]::util::bug::bug_fmt
  17:     0x7ff855ea1b68 - <rustc_borrowck[a7734a23bb6b876]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7ff8560aaf50 - <rustc_borrowck[a7734a23bb6b876]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7ff8560acd7d - <rustc_borrowck[a7734a23bb6b876]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7ff855f1e1bb - <rustc_borrowck[a7734a23bb6b876]::type_check::TypeChecker>::push_region_constraints
  21:     0x7ff855f1ac88 - <rustc_borrowck[a7734a23bb6b876]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7ff855f07a9c - rustc_borrowck[a7734a23bb6b876]::type_check::type_check
  23:     0x7ff855f5acde - rustc_borrowck[a7734a23bb6b876]::nll::compute_regions
  24:     0x7ff855dbd784 - rustc_borrowck[a7734a23bb6b876]::do_mir_borrowck
  25:     0x7ff855da8ad3 - rustc_borrowck[a7734a23bb6b876]::mir_borrowck
  26:     0x7ff85669383c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_borrowck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  27:     0x7ff8565a53ff - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_borrowck
  28:     0x7ff85542b01f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7ff85545ee13 - rustc_mir_transform[4f056a5f48fe549d]::mir_drops_elaborated_and_const_checked
  30:     0x7ff856801cda - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  31:     0x7ff856588cef - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7ff85542b01f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7ff85545eb12 - rustc_mir_transform[4f056a5f48fe549d]::mir_for_ctfe
  34:     0x7ff85669532c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_for_ctfe, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  35:     0x7ff8565893ae - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_for_ctfe
  36:     0x7ff855701081 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_span[14791a26829ba58d]::def_id::DefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7ff8557093bd - <rustc_const_eval[89e031c4a92f2f33]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[89e031c4a92f2f33]::interpret::machine::Machine>::load_mir
  38:     0x7ff85561768c - <rustc_const_eval[89e031c4a92f2f33]::interpret::eval_context::InterpCx<rustc_const_eval[89e031c4a92f2f33]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7ff855759787 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7ff856784e76 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_allocation_raw, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  41:     0x7ff8565a7c5c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_allocation_raw
  42:     0x7ff85574dc12 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 16usize]>>>
  43:     0x7ff855759467 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_queries::eval_to_allocation_raw_provider
  44:     0x7ff856784e76 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_allocation_raw, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  45:     0x7ff8565a7c5c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_allocation_raw
  46:     0x7ff855669e13 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_to_valtree
  47:     0x7ff855695f70 - <rustc_const_eval[89e031c4a92f2f33]::provide::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<(rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt, rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>)>>::call_once
  48:     0x7ff8566d0595 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_valtree, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  49:     0x7ff8565a8a7c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_valtree
  50:     0x7ff857662f11 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 24usize]>>>
  51:     0x7ff85767f149 - <rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  52:     0x7ff85767e128 - <rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  53:     0x7ff854e05386 - <rustc_middle[6ca339b62ee52f9a]::ty::consts::kind::ConstKind>::eval
  54:     0x7ff854e8c5f6 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  55:     0x7ff854e22a04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  56:     0x7ff854e89c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  57:     0x7ff854e438b5 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_decl
  58:     0x7ff854e43c6e - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_stmt
  59:     0x7ff854e44375 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_block_with_expected
  60:     0x7ff854e8acf6 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  61:     0x7ff854e22a04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  62:     0x7ff854e89c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  63:     0x7ff854e2491d - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_return_expr
  64:     0x7ff85502a407 - rustc_hir_typeck[fb20a30e9a3459f6]::check::check_fn
  65:     0x7ff854ee42bf - rustc_hir_typeck[fb20a30e9a3459f6]::typeck
  66:     0x7ff8568141df - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::typeck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  67:     0x7ff8565a329f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::typeck
  68:     0x7ff854ed8daf - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  69:     0x7ff854ee3c72 - rustc_hir_typeck[fb20a30e9a3459f6]::used_trait_imports
  70:     0x7ff856730610 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::used_trait_imports, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  71:     0x7ff8565a3fff - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::used_trait_imports
  72:     0x7ff855201daf - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  73:     0x7ff85520873b - rustc_hir_analysis[82c2ffdb456254]::check_unused::check_crate
  74:     0x7ff8552673eb - rustc_hir_analysis[82c2ffdb456254]::check_crate
  75:     0x7ff8549eff66 - rustc_interface[6f6522f1e7fd4d79]::passes::analysis
  76:     0x7ff8568198d1 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::analysis, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  77:     0x7ff85657f41f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::analysis
  78:     0x7ff8548c86b6 - <rustc_middle[6ca339b62ee52f9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  79:     0x7ff8548df8be - <rustc_interface[6f6522f1e7fd4d79]::interface::Compiler>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}, core[c6ab9fa5fff67170]::result::Result<core[c6ab9fa5fff67170]::option::Option<rustc_interface[6f6522f1e7fd4d79]::queries::Linker>, rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  80:     0x7ff85496cd10 - rustc_span[14791a26829ba58d]::set_source_map::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  81:     0x7ff854969be9 - <rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install::<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}
  82:     0x7ff85496290f - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[3ec58862d0a38d5e]::job::JobResult<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>>::call<<rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  83:     0x7ff854978aeb - <rayon_core[3ec58862d0a38d5e]::job::StackJob<rayon_core[3ec58862d0a38d5e]::latch::LatchRef<rayon_core[3ec58862d0a38d5e]::latch::LockLatch>, <rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>> as rayon_core[3ec58862d0a38d5e]::job::Job>::execute
  84:     0x7ff8548c2983 - <rayon_core[3ec58862d0a38d5e]::registry::WorkerThread>::wait_until_cold
  85:     0x7ff857d202ee - <rayon_core[3ec58862d0a38d5e]::registry::ThreadBuilder>::run
  86:     0x7ff8548d2a37 - <scoped_tls[5f4ee6aff00e9f97]::ScopedKey<rustc_span[14791a26829ba58d]::SessionGlobals>>::set::<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<(), rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  87:     0x7ff8548cd7f0 - <<crossbeam_utils[9323b80cd3a6c795]::thread::ScopedThreadBuilder>::spawn<<rayon_core[3ec58862d0a38d5e]::ThreadPoolBuilder>::build_scoped<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:     0x7ff8549155fe - <alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send> as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once
  89:     0x7ff8549685b6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>
  90:     0x7ff854962e76 - std[6389014f4a64c7aa]::panicking::try::<(), core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  91:     0x7ff85491bbe4 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  92:     0x7ff853df33fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
  93:     0x7ff853b8eb43 - <unknown>
  94:     0x7ff853c20a00 - <unknown>
  95:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] tests/ui/const-generics/late-bound-vars/in_closure.rs stdout ----
diff of stderr:

1 error: query stack during panic:
- #0 [mir_borrowck] borrow-checking `test::{closure#0}::{constant#1}`
- #1 [mir_drops_elaborated_and_const_checked] elaborating drops for `test::{closure#0}::{constant#1}`
- #2 [mir_for_ctfe] caching mir of `test::{closure#0}::{constant#1}` for CTFE
- #3 [eval_to_allocation_raw] const-evaluating + checking `test::{closure#0}::{constant#1}`
- #4 [eval_to_allocation_raw] const-evaluating + checking `test::{closure#0}::{constant#1}`
Build completed unsuccessfully in 0:13:23
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
   0:     0x7f8b37100434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7f8b37166ff8 - core::fmt::write::ha3db1235befb69cd
   1:     0x7f8b37166ff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7f8b370f4be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7f8b37100241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7f8b371033ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7f8b3710309c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7f8b37c4714b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7f8b37103ac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7f8b3aac1f63 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}
   9:     0x7f8b3aab33c6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_end_short_backtrace::<std[6389014f4a64c7aa]::panicking::begin_panic<rustc_errors[b322d23e6ca248f4]::ExplicitBug>::{closure#0}, !>
  10:     0x7f8b37b856b6 - std[6389014f4a64c7aa]::panicking::begin_panic::<rustc_errors[b322d23e6ca248f4]::ExplicitBug>
  11:     0x7f8b3aa02267 - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::bug::<alloc[73332b167c8d46aa]::string::String>
  12:     0x7f8b3aa01edd - <rustc_errors[b322d23e6ca248f4]::Handler>::bug::<alloc[73332b167c8d46aa]::string::String>
  13:     0x7f8b3a95a837 - rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt::<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}
  14:     0x7f8b3a95a37c - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt::<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f8b3a95a304 - rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_context_opt::<rustc_middle[6ca339b62ee52f9a]::ty::context::tls::with_opt<rustc_middle[6ca339b62ee52f9a]::util::bug::opt_span_bug_fmt<rustc_span[14791a26829ba58d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f8b37b7c882 - rustc_middle[6ca339b62ee52f9a]::util::bug::bug_fmt
  17:     0x7f8b391beb68 - <rustc_borrowck[a7734a23bb6b876]::universal_regions::UniversalRegionIndices>::to_region_vid
  18:     0x7f8b393c7f50 - <rustc_borrowck[a7734a23bb6b876]::type_check::constraint_conversion::ConstraintConversion>::convert
  19:     0x7f8b393c9d7d - <rustc_borrowck[a7734a23bb6b876]::type_check::constraint_conversion::ConstraintConversion>::convert_all
  20:     0x7f8b3923b1bb - <rustc_borrowck[a7734a23bb6b876]::type_check::TypeChecker>::push_region_constraints
  21:     0x7f8b39237c88 - <rustc_borrowck[a7734a23bb6b876]::type_check::TypeChecker>::ascribe_user_type
  22:     0x7f8b39224a9c - rustc_borrowck[a7734a23bb6b876]::type_check::type_check
  23:     0x7f8b39277cde - rustc_borrowck[a7734a23bb6b876]::nll::compute_regions
  24:     0x7f8b390da784 - rustc_borrowck[a7734a23bb6b876]::do_mir_borrowck
  25:     0x7f8b390c5ad3 - rustc_borrowck[a7734a23bb6b876]::mir_borrowck
  26:     0x7f8b399b083c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_borrowck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  27:     0x7f8b398c23ff - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_borrowck
  28:     0x7f8b3874801f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  29:     0x7f8b3877be13 - rustc_mir_transform[4f056a5f48fe549d]::mir_drops_elaborated_and_const_checked
  30:     0x7f8b39b1ecda - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  31:     0x7f8b398a5cef - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_drops_elaborated_and_const_checked
  32:     0x7f8b3874801f - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  33:     0x7f8b3877bb12 - rustc_mir_transform[4f056a5f48fe549d]::mir_for_ctfe
  34:     0x7f8b399b232c - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::mir_for_ctfe, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  35:     0x7f8b398a63ae - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::mir_for_ctfe
  36:     0x7f8b38a1e081 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_span[14791a26829ba58d]::def_id::DefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  37:     0x7f8b38a263bd - <rustc_const_eval[89e031c4a92f2f33]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[89e031c4a92f2f33]::interpret::machine::Machine>::load_mir
  38:     0x7f8b3893468c - <rustc_const_eval[89e031c4a92f2f33]::interpret::eval_context::InterpCx<rustc_const_eval[89e031c4a92f2f33]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  39:     0x7f8b38a76787 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_queries::eval_to_allocation_raw_provider
  40:     0x7f8b39aa1e76 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_allocation_raw, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  41:     0x7f8b398c4c5c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_allocation_raw
  42:     0x7f8b38a6ac12 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 16usize]>>>
  43:     0x7f8b38a76467 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_queries::eval_to_allocation_raw_provider
  44:     0x7f8b39aa1e76 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_allocation_raw, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  45:     0x7f8b398c4c5c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_allocation_raw
  46:     0x7f8b38986e13 - rustc_const_eval[89e031c4a92f2f33]::const_eval::eval_to_valtree
  47:     0x7f8b389b2f70 - <rustc_const_eval[89e031c4a92f2f33]::provide::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<(rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt, rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>)>>::call_once
  48:     0x7f8b399ed595 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::eval_to_valtree, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  49:     0x7f8b398c5a7c - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::eval_to_valtree
  50:     0x7f8b3a97ff11 - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::DefaultCache<rustc_middle[6ca339b62ee52f9a]::ty::ParamEnvAnd<rustc_middle[6ca339b62ee52f9a]::mir::interpret::GlobalId>, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 24usize]>>>
  51:     0x7f8b3a99c149 - <rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  52:     0x7f8b3a99b128 - <rustc_middle[6ca339b62ee52f9a]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  53:     0x7f8b38122386 - <rustc_middle[6ca339b62ee52f9a]::ty::consts::kind::ConstKind>::eval
  54:     0x7f8b381a95f6 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  55:     0x7f8b3813fa04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  56:     0x7f8b381a6c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  57:     0x7f8b381608b5 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_decl
  58:     0x7f8b38160c6e - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_stmt
  59:     0x7f8b38161375 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_block_with_expected
  60:     0x7f8b381a7cf6 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  61:     0x7f8b3813fa04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  62:     0x7f8b381a6c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  63:     0x7f8b3814191d - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_return_expr
  64:     0x7f8b38347407 - rustc_hir_typeck[fb20a30e9a3459f6]::check::check_fn
  65:     0x7f8b3819fdd4 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_closure
  66:     0x7f8b381a8165 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  67:     0x7f8b3813fa04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  68:     0x7f8b381a6c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  69:     0x7f8b381608b5 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_decl
  70:     0x7f8b38160c6e - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_stmt
  71:     0x7f8b38161375 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_block_with_expected
  72:     0x7f8b381a7cf6 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_kind
  73:     0x7f8b3813fa04 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  74:     0x7f8b381a6c02 - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  75:     0x7f8b3814191d - <rustc_hir_typeck[fb20a30e9a3459f6]::fn_ctxt::FnCtxt>::check_return_expr
  76:     0x7f8b38347407 - rustc_hir_typeck[fb20a30e9a3459f6]::check::check_fn
  77:     0x7f8b382012bf - rustc_hir_typeck[fb20a30e9a3459f6]::typeck
  78:     0x7f8b39b311df - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::typeck, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  79:     0x7f8b398c029f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::typeck
  80:     0x7f8b381f5daf - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  81:     0x7f8b38200c72 - rustc_hir_typeck[fb20a30e9a3459f6]::used_trait_imports
  82:     0x7f8b39a4d610 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::used_trait_imports, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  83:     0x7f8b398c0fff - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::used_trait_imports
  84:     0x7f8b3851edaf - rustc_middle[6ca339b62ee52f9a]::ty::query::query_get_at::<rustc_query_system[5fc7d2958e6b2548]::query::caches::VecCache<rustc_span[14791a26829ba58d]::def_id::LocalDefId, rustc_middle[6ca339b62ee52f9a]::query::erase::Erased<[u8; 8usize]>>>
  85:     0x7f8b3852573b - rustc_hir_analysis[82c2ffdb456254]::check_unused::check_crate
  86:     0x7f8b385843eb - rustc_hir_analysis[82c2ffdb456254]::check_crate
  87:     0x7f8b37d0cf66 - rustc_interface[6f6522f1e7fd4d79]::passes::analysis
  88:     0x7f8b39b368d1 - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::analysis, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  89:     0x7f8b3989c41f - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::analysis
  90:     0x7f8b37be56b6 - <rustc_middle[6ca339b62ee52f9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  91:     0x7f8b37bfc8be - <rustc_interface[6f6522f1e7fd4d79]::interface::Compiler>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}, core[c6ab9fa5fff67170]::result::Result<core[c6ab9fa5fff67170]::option::Option<rustc_interface[6f6522f1e7fd4d79]::queries::Linker>, rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  92:     0x7f8b37c89d10 - rustc_span[14791a26829ba58d]::set_source_map::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  93:     0x7f8b37c86be9 - <rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install::<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}
  94:     0x7f8b37c7f90f - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[3ec58862d0a38d5e]::job::JobResult<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>>::call<<rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  95:     0x7f8b37c95aeb - <rayon_core[3ec58862d0a38d5e]::job::StackJob<rayon_core[3ec58862d0a38d5e]::latch::LatchRef<rayon_core[3ec58862d0a38d5e]::latch::LockLatch>, <rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>> as rayon_core[3ec58862d0a38d5e]::job::Job>::execute
  96:     0x7f8b37bdf983 - <rayon_core[3ec58862d0a38d5e]::registry::WorkerThread>::wait_until_cold
  97:     0x7f8b3b03d2ee - <rayon_core[3ec58862d0a38d5e]::registry::ThreadBuilder>::run
  98:     0x7f8b37befa37 - <scoped_tls[5f4ee6aff00e9f97]::ScopedKey<rustc_span[14791a26829ba58d]::SessionGlobals>>::set::<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<(), rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  99:     0x7f8b37bea7f0 - <<crossbeam_utils[9323b80cd3a6c795]::thread::ScopedThreadBuilder>::spawn<<rayon_core[3ec58862d0a38d5e]::ThreadPoolBuilder>::build_scoped<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 100:     0x7f8b37c325fe - <alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send> as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once
 101:     0x7f8b37c855b6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>
 102:     0x7f8b37c7fe76 - std[6389014f4a64c7aa]::panicking::try::<(), core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1}::{closure#0}>>
 103:     0x7f8b37c38be4 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 104:     0x7f8b371103fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
 105:     0x7f8b36eabb43 - <unknown>
 106:     0x7f8b36f3da00 - <unknown>
 107:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu


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

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/limits/issue-15919-64.rs stdout ----
diff of stderr:

4 LL |     let x = [0usize; 0xffff_ffff_ffff_ffff];
6 
6 
+ thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1307:29
+    0:     0x7fb68ba3c434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
+    1:     0x7fb68baa2ff8 - core::fmt::write::ha3db1235befb69cd
+    2:     0x7fb68ba30be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
+    3:     0x7fb68ba3c241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
+    3:     0x7fb68ba3c241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
+    4:     0x7fb68ba3f3ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
+    5:     0x7fb68ba3f09c - std::panicking::default_hook::h03077d813fea8a22
+    6:     0x7fb68c58314b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
+    7:     0x7fb68ba3fac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
+    8:     0x7fb68ba3f809 - std::panicking::begin_panic_handler::{{closure}}::hb0958a30d13a622c
+    9:     0x7fb68ba3c916 - std::sys_common::backtrace::__rust_end_short_backtrace::h8fae90878591516a
+   10:     0x7fb68ba3f537 - rust_begin_unwind
+   11:     0x7fb68b9f40d3 - core::panicking::panic_fmt::h5cb73fab1524c913
+   12:     0x7fb68c87e1ed - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>
+   13:     0x7fb68c7ae3fc - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
+   14:     0x7fb68c84f223 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   15:     0x7fb68ba4c3fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
+   16:     0x7fb68b7e7b43 - <unknown>
+   17:     0x7fb68b879a00 - <unknown>
+   18:                0x0 - <unknown>
+ error: the compiler unexpectedly panicked. this is a bug.
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ 
+ note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
+ query stack during panic:
+ end of query stack
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args limits/issue-15919-64.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/issue-15919-64.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-15919-64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-15919-64/auxiliary"
stdout: none
--- stderr -------------------------------
error: values of the type `[usize; usize::MAX]` are too big for the current architecture
  --> fake-test-src-base/limits/issue-15919-64.rs:9:9
   |
LL |     let x = [0usize; 0xffff_ffff_ffff_ffff]; //~ ERROR too big


thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1307:29
   0:     0x7fb68ba3c434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7fb68baa2ff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7fb68ba30be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7fb68ba3c241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   3:     0x7fb68ba3c241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7fb68ba3f3ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7fb68ba3f09c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7fb68c58314b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7fb68ba3fac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7fb68ba3f809 - std::panicking::begin_panic_handler::{{closure}}::hb0958a30d13a622c
   9:     0x7fb68ba3c916 - std::sys_common::backtrace::__rust_end_short_backtrace::h8fae90878591516a
  10:     0x7fb68ba3f537 - rust_begin_unwind
  11:     0x7fb68b9f40d3 - core::panicking::panic_fmt::h5cb73fab1524c913
  12:     0x7fb68c87e1ed - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>
  13:     0x7fb68c7ae3fc - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
  14:     0x7fb68c84f223 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  15:     0x7fb68ba4c3fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
  16:     0x7fb68b7e7b43 - <unknown>
  17:     0x7fb68b879a00 - <unknown>
  18:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] tests/ui/limits/issue-75158-64.rs stdout ----
diff of stderr:

1 error: values of the type `[u8; usize::MAX]` are too big for the current architecture
2 
+ thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', $COMPILER_DIR/rustc_codegen_ssa/src/back/write.rs:1307:29
+    0:     0x7f68898c9434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
+    1:     0x7f688992fff8 - core::fmt::write::ha3db1235befb69cd
+    2:     0x7f68898bdbe1 - std::io::Write::write_fmt::hf1007ccbc10edf13
+    3:     0x7f68898c9241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
+    3:     0x7f68898c9241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
+    4:     0x7f68898cc3ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
+    5:     0x7f68898cc09c - std::panicking::default_hook::h03077d813fea8a22
+    6:     0x7f688a41014b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
+    7:     0x7f68898ccac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
+    8:     0x7f68898cc809 - std::panicking::begin_panic_handler::{{closure}}::hb0958a30d13a622c
+    9:     0x7f68898c9916 - std::sys_common::backtrace::__rust_end_short_backtrace::h8fae90878591516a
+   10:     0x7f68898cc537 - rust_begin_unwind
+   11:     0x7f68898810d3 - core::panicking::panic_fmt::h5cb73fab1524c913
+   12:     0x7f688a70b1ed - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>
+   13:     0x7f688a63b3fc - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
+   14:     0x7f688a6dc223 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+   15:     0x7f68898d93fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
+   16:     0x7f6889674b43 - <unknown>
+   17:     0x7f6889706a00 - <unknown>
+   18:                0x0 - <unknown>
+ error: the compiler unexpectedly panicked. this is a bug.
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ 
+ note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu
+ 
+ note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args limits/issue-75158-64.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/issue-75158-64.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-75158-64" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-75158-64/auxiliary"
stdout: none
--- stderr -------------------------------
error: values of the type `[u8; usize::MAX]` are too big for the current architecture

thread '<unnamed>' panicked at 'Could not send Message::CodegenItem to main thread', /checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1307:29
   0:     0x7f68898c9434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7f688992fff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7f68898bdbe1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7f68898c9241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   3:     0x7f68898c9241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7f68898cc3ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7f68898cc09c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7f688a41014b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7f68898ccac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7f68898cc809 - std::panicking::begin_panic_handler::{{closure}}::hb0958a30d13a622c
   9:     0x7f68898c9916 - std::sys_common::backtrace::__rust_end_short_backtrace::h8fae90878591516a
  10:     0x7f68898cc537 - rust_begin_unwind
  11:     0x7f68898810d3 - core::panicking::panic_fmt::h5cb73fab1524c913
  12:     0x7f688a70b1ed - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>
  13:     0x7f688a63b3fc - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
  14:     0x7f688a6dc223 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend as rustc_codegen_ssa[919a0e846e623162]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[919a0e846e623162]::back::write::start_executing_work<rustc_codegen_llvm[350bc159166263fc]::LlvmCodegenBackend>::{closure#4}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<rustc_codegen_ssa[919a0e846e623162]::back::write::CompiledModules, ()>>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  15:     0x7f68898d93fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
  16:     0x7f6889674b43 - <unknown>
  17:     0x7f6889706a00 - <unknown>
  18:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
---

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

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

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
   0:     0x7f5e73f43434 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha06b7a1afa022464
   1:     0x7f5e73fa9ff8 - core::fmt::write::ha3db1235befb69cd
   2:     0x7f5e73f37be1 - std::io::Write::write_fmt::hf1007ccbc10edf13
   3:     0x7f5e73f43241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   3:     0x7f5e73f43241 - std::sys_common::backtrace::print::h6c82ade57cf29d7a
   4:     0x7f5e73f463ba - std::panicking::default_hook::{{closure}}::hed7a5ed97cb53ad3
   5:     0x7f5e73f4609c - std::panicking::default_hook::h03077d813fea8a22
   6:     0x7f5e74a8a14b - rustc_driver_impl[dff241efb9ede0b1]::install_ice_hook::{closure#0}
   7:     0x7f5e73f46ac7 - std::panicking::rust_panic_with_hook::h8341644cfba93233
   8:     0x7f5e73f46809 - std::panicking::begin_panic_handler::{{closure}}::hb0958a30d13a622c
   9:     0x7f5e73f43916 - std::sys_common::backtrace::__rust_end_short_backtrace::h8fae90878591516a
  10:     0x7f5e73f46537 - rust_begin_unwind
  11:     0x7f5e73efb0d3 - core::panicking::panic_fmt::h5cb73fab1524c913
  12:     0x7f5e77b5dac7 - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7f5e77b5ceec - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::emit_diagnostic::{closure#2}
  14:     0x7f5e74be89d2 - rustc_interface[6f6522f1e7fd4d79]::callbacks::track_diagnostic
  15:     0x7f5e77b5c6a7 - <rustc_errors[b322d23e6ca248f4]::HandlerInner>::emit_diagnostic
  16:     0x7f5e77b5b6fa - <rustc_errors[b322d23e6ca248f4]::Handler>::emit_diagnostic
  17:     0x7f5e77b65daa - <rustc_span[14791a26829ba58d]::ErrorGuaranteed as rustc_errors[b322d23e6ca248f4]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  18:     0x7f5e75ba5865 - <rustc_resolve[4cb17b11dfdbf1d8]::Resolver>::report_errors
  19:     0x7f5e75b7ebce - <rustc_session[f5437c3c47b698c6]::session::Session>::time::<(), <rustc_resolve[4cb17b11dfdbf1d8]::Resolver>::resolve_crate::{closure#0}>
  20:     0x7f5e75bd10e9 - <rustc_resolve[4cb17b11dfdbf1d8]::Resolver>::resolve_crate
  21:     0x7f5e74b4c5f0 - rustc_interface[6f6522f1e7fd4d79]::passes::resolver_for_lowering
  22:     0x7f5e768d6fcd - rustc_query_system[5fc7d2958e6b2548]::query::plumbing::try_execute_query::<rustc_query_impl[ba5bb9c054e7fb9d]::queries::resolver_for_lowering, rustc_query_impl[ba5bb9c054e7fb9d]::plumbing::QueryCtxt>
  23:     0x7f5e766d901b - rustc_query_impl[ba5bb9c054e7fb9d]::get_query::resolver_for_lowering
  24:     0x7f5e74a2835a - <rustc_middle[6ca339b62ee52f9a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[71c5a3ba435a3fee]::steal::Steal<(rustc_middle[6ca339b62ee52f9a]::ty::ResolverAstLowering, alloc[73332b167c8d46aa]::sync::Arc<rustc_ast[b239b640cd654213]::ast::Crate>)>>
  25:     0x7f5e74a3f7b4 - <rustc_interface[6f6522f1e7fd4d79]::interface::Compiler>::enter::<rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}::{closure#2}, core[c6ab9fa5fff67170]::result::Result<core[c6ab9fa5fff67170]::option::Option<rustc_interface[6f6522f1e7fd4d79]::queries::Linker>, rustc_span[14791a26829ba58d]::ErrorGuaranteed>>
  26:     0x7f5e74accd10 - rustc_span[14791a26829ba58d]::set_source_map::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7f5e74ac9be9 - <rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install::<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}
  28:     0x7f5e74ac290f - std[6389014f4a64c7aa]::panicking::try::<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<rayon_core[3ec58862d0a38d5e]::job::JobResult<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>>::call<<rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}>::{closure#0}>>
  29:     0x7f5e74ad8aeb - <rayon_core[3ec58862d0a38d5e]::job::StackJob<rayon_core[3ec58862d0a38d5e]::latch::LatchRef<rayon_core[3ec58862d0a38d5e]::latch::LockLatch>, <rayon_core[3ec58862d0a38d5e]::registry::Registry>::in_worker_cold<<rayon_core[3ec58862d0a38d5e]::thread_pool::ThreadPool>::install<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>> as rayon_core[3ec58862d0a38d5e]::job::Job>::execute
  30:     0x7f5e74a22983 - <rayon_core[3ec58862d0a38d5e]::registry::WorkerThread>::wait_until_cold
  31:     0x7f5e77e802ee - <rayon_core[3ec58862d0a38d5e]::registry::ThreadBuilder>::run
  32:     0x7f5e74a32a37 - <scoped_tls[5f4ee6aff00e9f97]::ScopedKey<rustc_span[14791a26829ba58d]::SessionGlobals>>::set::<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<(), rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#0}>::{closure#0}, ()>::{closure#2}::{closure#0}::{closure#0}::{closure#0}, ()>
  33:     0x7f5e74a2d7f0 - <<crossbeam_utils[9323b80cd3a6c795]::thread::ScopedThreadBuilder>::spawn<<rayon_core[3ec58862d0a38d5e]::ThreadPoolBuilder>::build_scoped<rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#0}, rustc_interface[6f6522f1e7fd4d79]::util::run_in_thread_pool_with_globals<rustc_interface[6f6522f1e7fd4d79]::interface::run_compiler<core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>, rustc_driver_impl[dff241efb9ede0b1]::run_compiler::{closure#1}>::{closure#0}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#2}::{closure#0}::{closure#1}, core[c6ab9fa5fff67170]::result::Result<(), rustc_span[14791a26829ba58d]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}, ()>::{closure#0} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f5e74a755fe - <alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send> as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once
  35:     0x7f5e74ac85b6 - std[6389014f4a64c7aa]::sys_common::backtrace::__rust_begin_short_backtrace::<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>
  36:     0x7f5e74ac2e76 - std[6389014f4a64c7aa]::panicking::try::<(), core[c6ab9fa5fff67170]::panic::unwind_safe::AssertUnwindSafe<<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1}::{closure#0}>>
  37:     0x7f5e74a7bbe4 - <<std[6389014f4a64c7aa]::thread::Builder>::spawn_unchecked_<alloc[73332b167c8d46aa]::boxed::Box<dyn core[c6ab9fa5fff67170]::ops::function::FnOnce<(), Output = ()> + core[c6ab9fa5fff67170]::marker::Send>, ()>::{closure#1} as core[c6ab9fa5fff67170]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f5e73f533fe - std::sys::unix::thread::Thread::new::thread_start::hfd702694e591ab9d
  39:     0x7f5e73ceeb43 - <unknown>
  40:     0x7f5e73d80a00 - <unknown>
  41:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
end of query stack
------------------------------------------



---- [ui] tests/ui/stats/hir-stats.rs stdout ----
diff of stderr:

115 ast-stats-2 ----------------------------------------------------------------
116 ast-stats-2 Total                  7_088
117 ast-stats-2
- hir-stats HIR STATS
- hir-stats Name                Accumulated Size         Count     Item Size
- hir-stats ----------------------------------------------------------------
- hir-stats ForeignItemRef            24 ( 0.3%)             1            24
- hir-stats Lifetime                  24 ( 0.3%)             1            24
- hir-stats Mod                       32 ( 0.4%)             1            32
- hir-stats ExprField                 40 ( 0.4%)             1            40
- hir-stats TraitItemRef              56 ( 0.6%)             2            28
- hir-stats Local                     64 ( 0.7%)             1            64
- hir-stats Param                     64 ( 0.7%)             2            32
- hir-stats InlineAsm                 72 ( 0.8%)             1            72
- hir-stats ImplItemRef               72 ( 0.8%)             2            36
- hir-stats Body                      96 ( 1.1%)             3            32
- hir-stats FieldDef                  96 ( 1.1%)             2            48
- hir-stats Arm                       96 ( 1.1%)             2            48
- hir-stats Stmt                      96 ( 1.1%)             3            32
- hir-stats - Local                     32 ( 0.4%)             1
- hir-stats - Semi                      32 ( 0.4%)             1
- hir-stats - Expr                      32 ( 0.4%)             1
- hir-stats FnDecl                   120 ( 1.3%)             3            40
- hir-stats Attribute                128 ( 1.4%)             4            32
- hir-stats GenericArg               128 ( 1.4%)             4            32
- hir-stats - Type                      32 ( 0.4%)             1
- hir-stats - Lifetime                  96 ( 1.1%)             3
- hir-stats GenericArgs              144 ( 1.6%)             3            48
- hir-stats Variant                  176 ( 1.9%)             2            88
- hir-stats GenericBound             192 ( 2.1%)             4            48
- hir-stats - Trait                    192 ( 2.1%)             4
- hir-stats WherePredicate           192 ( 2.1%)             3            64
- hir-stats - BoundPredicate           192 ( 2.1%)             3
- hir-stats Block                    288 ( 3.2%)             6            48
- hir-stats Pat                      360 ( 4.0%)             5            72
- hir-stats - Wild                      72 ( 0.8%)             1
- hir-stats - Struct                    72 ( 0.8%)             1
- hir-stats - Binding                  216 ( 2.4%)             3
- hir-stats GenericParam             400 ( 4.4%)             5            80
- hir-stats Generics                 560 ( 6.2%)            10            56
- hir-stats Ty                       720 ( 8.0%)            15            48
- hir-stats - Ptr                       48 ( 0.5%)             1
- hir-stats - Ref                       48 ( 0.5%)             1
- hir-stats - Path                     624 ( 6.9%)            13
- hir-stats Expr                     768 ( 8.5%)            12            64
- hir-stats - Path                      64 ( 0.7%)             1
- hir-stats - Struct                    64 ( 0.7%)             1
- hir-stats - Match                     64 ( 0.7%)             1
- hir-stats - InlineAsm                 64 ( 0.7%)             1
- hir-stats - Lit                      128 ( 1.4%)             2
- hir-stats - Block                    384 ( 4.2%)             6
- hir-stats Item                     880 ( 9.7%)            11            80
- hir-stats - Trait                     80 ( 0.9%)             1
- hir-stats - Enum                      80 ( 0.9%)             1
- hir-stats - ExternCrate               80 ( 0.9%)             1
- hir-stats - ForeignMod                80 ( 0.9%)             1
- hir-stats - Impl                      80 ( 0.9%)             1
- hir-stats - Fn                       160 ( 1.8%)             2
- hir-stats - Use                      320 ( 3.5%)             4
- hir-stats Path                   1_240 (13.7%)            31            40
- hir-stats PathSegment            1_920 (21.2%)            40            48
- hir-stats ----------------------------------------------------------------
- hir-stats
179 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/hir-stats.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stats/hir-stats.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/stats/hir-stats.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/auxiliary" "-Zhir-stats"
stdout: none
--- stderr -------------------------------
ast-stats-1 PRE EXPANSION AST STATS
ast-stats-1 Name                Accumulated Size         Count     Item Size
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 GenericArgs               40 ( 0.6%)             1            40
ast-stats-1 - AngleBracketed            40 ( 0.6%)             1
ast-stats-1 Crate                     40 ( 0.6%)             1            40
ast-stats-1 ExprField                 48 ( 0.7%)             1            48
ast-stats-1 WherePredicate            56 ( 0.9%)             1            56
ast-stats-1 - BoundPredicate            56 ( 0.9%)             1
ast-stats-1 Attribute                 64 ( 1.0%)             2            32
ast-stats-1 - Normal                    32 ( 0.5%)             1
ast-stats-1 - DocComment                32 ( 0.5%)             1
ast-stats-1 Local                     72 ( 1.1%)             1            72
ast-stats-1 Arm                       96 ( 1.5%)             2            48
ast-stats-1 ForeignItem               96 ( 1.5%)             1            96
ast-stats-1 - Fn                        96 ( 1.5%)             1
ast-stats-1 FnDecl                   120 ( 1.8%)             5            24
ast-stats-1 FieldDef                 160 ( 2.5%)             2            80
ast-stats-1 Stmt                     160 ( 2.5%)             5            32
ast-stats-1 - Local                     32 ( 0.5%)             1
ast-stats-1 - MacCall                   32 ( 0.5%)             1
ast-stats-1 - Expr                      96 ( 1.5%)             3
ast-stats-1 Param                    160 ( 2.5%)             4            40
ast-stats-1 Block                    192 ( 3.0%)             6            32
ast-stats-1 Variant                  208 ( 3.2%)             2           104
ast-stats-1 GenericBound             224 ( 3.5%)             4            56
ast-stats-1 - Trait                    224 ( 3.5%)             4
ast-stats-1 AssocItem                352 ( 5.4%)             4            88
ast-stats-1 - Type                     176 ( 2.7%)             2
ast-stats-1 - Fn                       176 ( 2.7%)             2
ast-stats-1 GenericParam             480 ( 7.4%)             5            96
ast-stats-1 Pat                      504 ( 7.8%)             7            72
ast-stats-1 - Struct                    72 ( 1.1%)             1
ast-stats-1 - Wild                      72 ( 1.1%)             1
ast-stats-1 - Ident                    360 ( 5.5%)             5
ast-stats-1 Expr                     576 ( 8.9%)             8            72
ast-stats-1 - Path                      72 ( 1.1%)             1
ast-stats-1 - Match                     72 ( 1.1%)             1
ast-stats-1 - Struct                    72 ( 1.1%)             1
ast-stats-1 - Lit                      144 ( 2.2%)             2
ast-stats-1 - Block                    216 ( 3.3%)             3
ast-stats-1 PathSegment              720 (11.1%)            30            24
ast-stats-1 Ty                       896 (13.8%)            14            64
ast-stats-1 - Ptr                       64 ( 1.0%)             1
ast-stats-1 - Ref                       64 ( 1.0%)             1
ast-stats-1 - ImplicitSelf             128 ( 2.0%)             2
ast-stats-1 - Path                     640 ( 9.9%)            10
ast-stats-1 Item                   1_224 (18.9%)             9           136
ast-stats-1 - Trait                    136 ( 2.1%)             1
ast-stats-1 - Enum                     136 ( 2.1%)             1
ast-stats-1 - ForeignMod               136 ( 2.1%)             1
ast-stats-1 - Impl                     136 ( 2.1%)             1
ast-stats-1 - Fn                       272 ( 4.2%)             2
ast-stats-1 - Use                      408 ( 6.3%)             3
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 Total                  6_488
ast-stats-1
ast-stats-2 POST EXPANSION AST STATS
ast-stats-2 Name                Accumulated Size         Count     Item Size
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 GenericArgs               40 ( 0.6%)             1            40
ast-stats-2 - AngleBracketed            40 ( 0.6%)             1
ast-stats-2 Crate                     40 ( 0.6%)             1            40
ast-stats-2 ExprField                 48 ( 0.7%)             1            48
ast-stats-2 WherePredicate            56 ( 0.8%)             1            56
ast-stats-2 - BoundPredicate            56 ( 0.8%)             1
ast-stats-2 Local                     72 ( 1.0%)             1            72
ast-stats-2 Arm                       96 ( 1.4%)             2            48
ast-stats-2 ForeignItem               96 ( 1.4%)             1            96
ast-stats-2 - Fn                        96 ( 1.4%)             1
ast-stats-2 InlineAsm                120 ( 1.7%)             1           120
ast-stats-2 FnDecl                   120 ( 1.7%)             5            24
ast-stats-2 Attribute                128 ( 1.8%)             4            32
ast-stats-2 - DocComment                32 ( 0.5%)             1
ast-stats-2 - Normal                    96 ( 1.4%)             3
ast-stats-2 FieldDef                 160 ( 2.3%)             2            80
ast-stats-2 Stmt                     160 ( 2.3%)             5            32
ast-stats-2 - Local                     32 ( 0.5%)             1
ast-stats-2 - Semi                      32 ( 0.5%)             1
ast-stats-2 - Expr                      96 ( 1.4%)             3
ast-stats-2 Param                    160 ( 2.3%)             4            40
ast-stats-2 Block                    192 ( 2.7%)             6            32
ast-stats-2 Variant                  208 ( 2.9%)             2           104
ast-stats-2 GenericBound             224 ( 3.2%)             4            56
ast-stats-2 - Trait                    224 ( 3.2%)             4
ast-stats-2 AssocItem                352 ( 5.0%)             4            88
ast-stats-2 - Type                     176 ( 2.5%)             2
ast-stats-2 - Fn                       176 ( 2.5%)             2
ast-stats-2 GenericParam             480 ( 6.8%)             5            96
ast-stats-2 Pat                      504 ( 7.1%)             7            72
ast-stats-2 - Struct                    72 ( 1.0%)             1
ast-stats-2 - Wild                      72 ( 1.0%)             1
ast-stats-2 - Ident                    360 ( 5.1%)             5
ast-stats-2 Expr                     648 ( 9.1%)             9            72
ast-stats-2 - Path                      72 ( 1.0%)             1
ast-stats-2 - Match                     72 ( 1.0%)             1
ast-stats-2 - Struct                    72 ( 1.0%)             1
ast-stats-2 - InlineAsm                 72 ( 1.0%)             1
ast-stats-2 - Lit                      144 ( 2.0%)             2
ast-stats-2 - Block                    216 ( 3.0%)             3
ast-stats-2 PathSegment              792 (11.2%)            33            24
ast-stats-2 Ty                       896 (12.6%)            14            64
ast-stats-2 - Ptr                       64 ( 0.9%)             1
ast-stats-2 - Ref                       64 ( 0.9%)             1
ast-stats-2 - ImplicitSelf             128 ( 1.8%)             2
ast-stats-2 - Path                     640 ( 9.0%)            10
ast-stats-2 Item                   1_496 (21.1%)            11           136
ast-stats-2 - Trait                    136 ( 1.9%)             1
ast-stats-2 - Enum                     136 ( 1.9%)             1
ast-stats-2 - ExternCrate              136 ( 1.9%)             1
ast-stats-2 - ForeignMod               136 ( 1.9%)             1
ast-stats-2 - Impl                     136 ( 1.9%)             1
ast-stats-2 - Fn                       272 ( 3.8%)             2
ast-stats-2 - Use                      544 ( 7.7%)             4
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 Total                  7_088
------------------------------------------


---- [ui] tests/ui/treat-err-as-bug/delay_span_bug.rs stdout ----
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

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

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

note: rustc 1.71.0-nightly (0a080d478 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
end of query stack
------------------------------------------

