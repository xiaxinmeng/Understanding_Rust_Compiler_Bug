plain
---- [ui] src/test/ui/chalkify/println.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/println.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/println" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/println/auxiliary" "-Z" "chalk"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_traits/src/chalk/lowering.rs:194:49: unexpected well formed predicate: ReLateBound(DebruijnIndex(1), BoundRegion { var: 0, kind: BrAnon(0, None) })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1560:9
stack backtrace:
   0:     0x7f580d968bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   1:     0x7f580d9d85e8 - core::fmt::write::h5bbeb8c686edca77
   2:     0x7f580d95aa61 - std::io::Write::write_fmt::hbd4175feb6647397
   3:     0x7f580d968a01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   4:     0x7f580d96bd64 - std::panicking::default_hook::{{closure}}::h2e112b0fa365fcd9
   5:     0x7f580d96ba29 - std::panicking::default_hook::h7e14da905e95e9aa
   6:     0x7f580e3ae3b4 - rustc_driver[3658c77f76a9c99b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f580d96c4b4 - std::panicking::rust_panic_with_hook::h36d1740164bc7729
   8:     0x7f58110238b3 - std[7ecdc0224e2dd782]::panicking::begin_panic::<rustc_errors[4179f80a31162f2b]::ExplicitBug>::{closure#0}
   9:     0x7f5811017a96 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_end_short_backtrace::<std[7ecdc0224e2dd782]::panicking::begin_panic<rustc_errors[4179f80a31162f2b]::ExplicitBug>::{closure#0}, !>
  10:     0x7f580e34b0c6 - std[7ecdc0224e2dd782]::panicking::begin_panic::<rustc_errors[4179f80a31162f2b]::ExplicitBug>
  11:     0x7f58110117d6 - std[7ecdc0224e2dd782]::panic::panic_any::<rustc_errors[4179f80a31162f2b]::ExplicitBug>
  12:     0x7f581101140d - <rustc_errors[4179f80a31162f2b]::HandlerInner>::bug::<&alloc[feb9c2381da4f6ce]::string::String>
  13:     0x7f5811010fd0 - <rustc_errors[4179f80a31162f2b]::Handler>::bug::<&alloc[feb9c2381da4f6ce]::string::String>
  14:     0x7f58111925ee - rustc_middle[bcebdc05307597e1]::ty::context::tls::with_context_opt::<rustc_middle[bcebdc05307597e1]::ty::context::tls::with_opt<rustc_middle[bcebdc05307597e1]::util::bug::opt_span_bug_fmt<rustc_span[e2c12e69ff4a0ed9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f581119b519 - rustc_middle[bcebdc05307597e1]::util::bug::opt_span_bug_fmt::<rustc_span[e2c12e69ff4a0ed9]::span_encoding::Span>
  16:     0x7f580e353898 - rustc_middle[bcebdc05307597e1]::util::bug::bug_fmt
  17:     0x7f580f8ef2ae - <rustc_middle[bcebdc05307597e1]::ty::Predicate as rustc_traits[924ca65e6c30ccf5]::chalk::lowering::LowerInto<chalk_ir[f2d3166ca17d4736]::GoalData<rustc_middle[bcebdc05307597e1]::traits::chalk::RustInterner>>>::lower_into
  18:     0x7f580f9c3f88 - <rustc_middle[bcebdc05307597e1]::traits::chalk::ChalkEnvironmentAndGoal as rustc_traits[924ca65e6c30ccf5]::chalk::lowering::LowerInto<chalk_ir[f2d3166ca17d4736]::InEnvironment<chalk_ir[f2d3166ca17d4736]::Goal<rustc_middle[bcebdc05307597e1]::traits::chalk::RustInterner>>>>::lower_into
  19:     0x7f580f97a66e - rustc_traits[924ca65e6c30ccf5]::chalk::evaluate_goal
  20:     0x7f5810257249 - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::evaluate_goal, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  21:     0x7f580fe53206 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::evaluate_goal
  22:     0x7f5810cdf9c9 - <rustc_trait_selection[ad41850ddcfe1aa]::traits::chalk_fulfill::FulfillmentContext as rustc_infer[de89cc7b045fc256]::traits::engine::TraitEngine>::select_where_possible
  23:     0x7f580f98bce7 - <rustc_infer[de89cc7b045fc256]::infer::InferCtxt>::make_canonicalized_query_response::<()>
  24:     0x7f580f8f0e73 - <rustc_trait_selection[ad41850ddcfe1aa]::traits::engine::ObligationCtxt>::make_canonicalized_query_response::<()>
  25:     0x7f580f9940d2 - <rustc_infer[de89cc7b045fc256]::infer::InferCtxtBuilder as rustc_trait_selection[ad41850ddcfe1aa]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[bcebdc05307597e1]::ty::ParamEnvAnd<rustc_middle[bcebdc05307597e1]::traits::query::type_op::AscribeUserType>, (), rustc_traits[924ca65e6c30ccf5]::type_op::type_op_ascribe_user_type::{closure#0}>
  26:     0x7f580f9c4a66 - rustc_traits[924ca65e6c30ccf5]::type_op::type_op_ascribe_user_type
  27:     0x7f5810294091 - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::type_op_ascribe_user_type, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  28:     0x7f580fe539a8 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::type_op_ascribe_user_type
  29:     0x7f5810c219c2 - <rustc_middle[bcebdc05307597e1]::traits::query::type_op::AscribeUserType as rustc_trait_selection[ad41850ddcfe1aa]::traits::query::type_op::QueryTypeOp>::perform_query
  30:     0x7f580f72a9b8 - <rustc_middle[bcebdc05307597e1]::traits::query::type_op::AscribeUserType as rustc_trait_selection[ad41850ddcfe1aa]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  31:     0x7f580f855016 - <rustc_middle[bcebdc05307597e1]::ty::ParamEnvAnd<rustc_middle[bcebdc05307597e1]::traits::query::type_op::AscribeUserType> as rustc_trait_selection[ad41850ddcfe1aa]::traits::query::type_op::TypeOp>::fully_perform
  32:     0x7f580f744fe7 - <rustc_borrowck[d2b3b70365b97d54]::type_check::TypeChecker>::ascribe_user_type
  33:     0x7f580f73220c - rustc_borrowck[d2b3b70365b97d54]::type_check::type_check
  34:     0x7f580f619f2a - rustc_borrowck[d2b3b70365b97d54]::nll::compute_regions
  35:     0x7f580f7a0997 - rustc_borrowck[d2b3b70365b97d54]::do_mir_borrowck
  36:     0x7f580f791ea5 - rustc_borrowck[d2b3b70365b97d54]::mir_borrowck
  37:     0x7f580f75eabe - <rustc_borrowck[d2b3b70365b97d54]::provide::{closure#0} as core[9063fd8b88d3339a]::ops::function::FnOnce<(rustc_middle[bcebdc05307597e1]::ty::context::TyCtxt, rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId)>>::call_once
  38:     0x7f5810179449 - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId, &rustc_middle[bcebdc05307597e1]::mir::query::BorrowCheckResult>>
  39:     0x7f5810255585 - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::mir_borrowck, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  40:     0x7f580fe0dac0 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::mir_borrowck
  41:     0x7f580e582776 - <core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_interface[a323614b9422972e]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once
  42:     0x7f580e4d4866 - std[7ecdc0224e2dd782]::panic::catch_unwind::<core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_interface[a323614b9422972e]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  43:     0x7f580e4c1e6d - rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in::<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_interface[a323614b9422972e]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  44:     0x7f580e4dfca5 - <rustc_session[cfb6c8c79fe50e23]::session::Session>::time::<(), rustc_interface[a323614b9422972e]::passes::analysis::{closure#2}>
  45:     0x7f580e512dfb - rustc_interface[a323614b9422972e]::passes::analysis
  46:     0x7f58101bcb9f - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>>
  47:     0x7f58102a71fb - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::analysis, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  48:     0x7f580fde173a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::analysis
  49:     0x7f580e40978d - <rustc_interface[a323614b9422972e]::passes::QueryContext>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  50:     0x7f580e41cc6a - <rustc_interface[a323614b9422972e]::interface::Compiler>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}, core[9063fd8b88d3339a]::result::Result<core[9063fd8b88d3339a]::option::Option<rustc_interface[a323614b9422972e]::queries::Linker>, rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  51:     0x7f580e3afafe - rustc_span[e2c12e69ff4a0ed9]::with_source_map::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  52:     0x7f580e40f3bc - <scoped_tls[e0d1874e2825f8de]::ScopedKey<rustc_span[e2c12e69ff4a0ed9]::SessionGlobals>>::set::<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  53:     0x7f580e3cd659 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  54:     0x7f580e410476 - std[7ecdc0224e2dd782]::panicking::try::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7f580e3bef5a - <<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1} as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f580d978bde - std::sys::unix::thread::Thread::new::thread_start::h853e937d366c7f55
  57:     0x7f580d70eb43 - <unknown>
  58:     0x7f580d7a0a00 - <unknown>
  59:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (350ee9de0 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z chalk
query stack during panic:
query stack during panic:
#0 [evaluate_goal] evaluating trait selection obligation `environment: [], goal: 'a well-formed`
#1 [type_op_ascribe_user_type] evaluating `type_op_ascribe_user_type` `AscribeUserType { mir_ty: fn(&'a [&'static str], &'a [core::fmt::ArgumentV1<'a>]) -> core::fmt::Arguments<'a> {core::fmt::Arguments::<'_>::new_v1}, user_ty: TypeOf(DefId(2:10618 ~ core[9063]::fmt::{impl#4}::new_v1), UserSubsts { substs: [ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrAnon(1, None) })], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(2:10616 ~ core[9063]::fmt::{impl#4}), self_ty: core::fmt::Arguments<'_> }) }) }`
#2 [mir_borrowck] borrow-checking `main`
#3 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/empty/empty-struct-braces-pat-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-pat-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(TypeNS)`,
 right: `Some(ValueNS)`', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1014:9
   0:     0x7fdbe5180bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   1:     0x7fdbe51f05e8 - core::fmt::write::h5bbeb8c686edca77
   1:     0x7fdbe51f05e8 - core::fmt::write::h5bbeb8c686edca77
   2:     0x7fdbe5172a61 - std::io::Write::write_fmt::hbd4175feb6647397
   3:     0x7fdbe5180a01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   4:     0x7fdbe5183d64 - std::panicking::default_hook::{{closure}}::h2e112b0fa365fcd9
   5:     0x7fdbe5183a29 - std::panicking::default_hook::h7e14da905e95e9aa
   6:     0x7fdbe5bc63b4 - rustc_driver[3658c77f76a9c99b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fdbe51844b4 - std::panicking::rust_panic_with_hook::h36d1740164bc7729
   8:     0x7fdbe5184217 - std::panicking::begin_panic_handler::{{closure}}::h745616476b564158
   9:     0x7fdbe5181134 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b985c2449af84e8
  10:     0x7fdbe5183ee2 - rust_begin_unwind
  11:     0x7fdbe5134f83 - core::panicking::panic_fmt::hd1e7554aa3b304ae
  12:     0x7fdbe51ecf08 - core::panicking::assert_failed_inner::h25774d7065544fb7
  13:     0x7fdbe59a6b5b - core[9063fd8b88d3339a]::panicking::assert_failed::<core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>, core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>>
  14:     0x7fdbe6140aec - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::instantiate_value_path
  15:     0x7fdbe614b155 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_pat
  16:     0x7fdbe6125fae - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_match
  17:     0x7fdbe613317a - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  18:     0x7fdbe60c7c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:     0x7fdbe61323e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  20:     0x7fdbe60e8b92 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_stmt
  21:     0x7fdbe60e9267 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_block_with_expected
  22:     0x7fdbe613334b - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7fdbe60c7c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7fdbe61323e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7fdbe60c950f - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7fdbe624f1a2 - rustc_hir_typeck[6d2ec8a243be3d62]::check::check_fn
  27:     0x7fdbe6212074 - <rustc_hir_typeck[6d2ec8a243be3d62]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_with_fallback<rustc_hir_typeck[6d2ec8a243be3d62]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>
  28:     0x7fdbe61e3a73 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck
  29:     0x7fdbe7990699 - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>>
  30:     0x7fdbe7abee4d - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  31:     0x7fdbe7622e70 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck
  32:     0x7fdbe61a8696 - <core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once
  33:     0x7fdbe62e03e6 - std[7ecdc0224e2dd782]::panicking::try::<(), core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  34:     0x7fdbe6196e4d - rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in::<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>
  35:     0x7fdbe61e2a17 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies
  36:     0x7fdbe79e035e - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), ()>>
  37:     0x7fdbe7a8673b - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck_item_bodies, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  38:     0x7fdbe762271a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck_item_bodies
  39:     0x7fdbe63467fb - <rustc_session[cfb6c8c79fe50e23]::session::Session>::time::<(), rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate::{closure#7}>
  40:     0x7fdbe656f633 - rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate
  41:     0x7fdbe5d2adc1 - rustc_interface[a323614b9422972e]::passes::analysis
  42:     0x7fdbe79d4b9f - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>>
  43:     0x7fdbe7abf1fb - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::analysis, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  44:     0x7fdbe75f973a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::analysis
  45:     0x7fdbe5c2178d - <rustc_interface[a323614b9422972e]::passes::QueryContext>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  46:     0x7fdbe5c34c6a - <rustc_interface[a323614b9422972e]::interface::Compiler>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}, core[9063fd8b88d3339a]::result::Result<core[9063fd8b88d3339a]::option::Option<rustc_interface[a323614b9422972e]::queries::Linker>, rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  47:     0x7fdbe5bc7afe - rustc_span[e2c12e69ff4a0ed9]::with_source_map::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  48:     0x7fdbe5c273bc - <scoped_tls[e0d1874e2825f8de]::ScopedKey<rustc_span[e2c12e69ff4a0ed9]::SessionGlobals>>::set::<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  49:     0x7fdbe5be5659 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  50:     0x7fdbe5c28476 - std[7ecdc0224e2dd782]::panicking::try::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7fdbe5bd6f5a - <<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1} as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7fdbe5190bde - std::sys::unix::thread::Thread::new::thread_start::h853e937d366c7f55
  53:     0x7fdbe4f26b43 - <unknown>
  54:     0x7fdbe4fb8a00 - <unknown>
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (350ee9de0 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/parser/recover-from-bad-variant.rs stdout ----
---- [ui] src/test/ui/parser/recover-from-bad-variant.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-from-bad-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-bad-variant" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-bad-variant/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `struct` delimiters or `fn` call arguments
   |
   |
LL |     let x = Enum::Foo(a: 3, b: 4);
   |
help: if `Enum::Foo` is a struct, use braces as delimiters
   |
   |
LL |     let x = Enum::Foo { a: 3, b: 4 };
help: if `Enum::Foo` is a function, use the arguments directly
   |
   |
LL -     let x = Enum::Foo(a: 3, b: 4);
LL +     let x = Enum::Foo(3, 4);

thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(TypeNS)`,
 right: `Some(ValueNS)`', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1014:9
   0:     0x7faacf3b0bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   0:     0x7faacf3b0bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   1:     0x7faacf4205e8 - core::fmt::write::h5bbeb8c686edca77
   2:     0x7faacf3a2a61 - std::io::Write::write_fmt::hbd4175feb6647397
   3:     0x7faacf3b0a01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   4:     0x7faacf3b3d64 - std::panicking::default_hook::{{closure}}::h2e112b0fa365fcd9
   5:     0x7faacf3b3a29 - std::panicking::default_hook::h7e14da905e95e9aa
   6:     0x7faacfdf63b4 - rustc_driver[3658c77f76a9c99b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7faacf3b44b4 - std::panicking::rust_panic_with_hook::h36d1740164bc7729
   8:     0x7faacf3b4217 - std::panicking::begin_panic_handler::{{closure}}::h745616476b564158
   9:     0x7faacf3b1134 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b985c2449af84e8
  10:     0x7faacf3b3ee2 - rust_begin_unwind
  11:     0x7faacf364f83 - core::panicking::panic_fmt::hd1e7554aa3b304ae
  12:     0x7faacf41cf08 - core::panicking::assert_failed_inner::h25774d7065544fb7
  13:     0x7faacfbd6b5b - core[9063fd8b88d3339a]::panicking::assert_failed::<core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>, core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>>
  14:     0x7faad0370aec - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::instantiate_value_path
  15:     0x7faad037b155 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_pat
  16:     0x7faad0355fae - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_match
  17:     0x7faad036317a - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  18:     0x7faad02f7c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:     0x7faad03623e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  20:     0x7faad03192a6 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_block_with_expected
  21:     0x7faad036334b - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  22:     0x7faad02f7c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:     0x7faad03623e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  24:     0x7faad02f950f - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_return_expr
  25:     0x7faad047f1a2 - rustc_hir_typeck[6d2ec8a243be3d62]::check::check_fn
  26:     0x7faad0442074 - <rustc_hir_typeck[6d2ec8a243be3d62]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_with_fallback<rustc_hir_typeck[6d2ec8a243be3d62]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>
  27:     0x7faad0413a73 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck
  28:     0x7faad1bc0699 - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>>
  29:     0x7faad1ceee4d - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  30:     0x7faad1852e70 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck
  31:     0x7faad03d8696 - <core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once
  32:     0x7faad05103e6 - std[7ecdc0224e2dd782]::panicking::try::<(), core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7faad03c6e4d - rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in::<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>
  34:     0x7faad0412a17 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies
  35:     0x7faad1c1035e - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), ()>>
  36:     0x7faad1cb673b - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck_item_bodies, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  37:     0x7faad185271a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck_item_bodies
  38:     0x7faad05767fb - <rustc_session[cfb6c8c79fe50e23]::session::Session>::time::<(), rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate::{closure#7}>
  39:     0x7faad079f633 - rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate
  40:     0x7faacff5adc1 - rustc_interface[a323614b9422972e]::passes::analysis
  41:     0x7faad1c04b9f - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>>
  42:     0x7faad1cef1fb - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::analysis, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  43:     0x7faad182973a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::analysis
  44:     0x7faacfe5178d - <rustc_interface[a323614b9422972e]::passes::QueryContext>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  45:     0x7faacfe64c6a - <rustc_interface[a323614b9422972e]::interface::Compiler>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}, core[9063fd8b88d3339a]::result::Result<core[9063fd8b88d3339a]::option::Option<rustc_interface[a323614b9422972e]::queries::Linker>, rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  46:     0x7faacfdf7afe - rustc_span[e2c12e69ff4a0ed9]::with_source_map::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  47:     0x7faacfe573bc - <scoped_tls[e0d1874e2825f8de]::ScopedKey<rustc_span[e2c12e69ff4a0ed9]::SessionGlobals>>::set::<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  48:     0x7faacfe15659 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  49:     0x7faacfe58476 - std[7ecdc0224e2dd782]::panicking::try::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7faacfe06f5a - <<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1} as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7faacf3c0bde - std::sys::unix::thread::Thread::new::thread_start::h853e937d366c7f55
  52:     0x7faacf156b43 - <unknown>
  53:     0x7faacf1e8a00 - <unknown>
  54:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (350ee9de0 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/suggestions/issue-84700.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-84700.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84700" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-84700/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0532]: expected tuple struct or tuple variant, found unit variant `FarmAnimal::Cow`
   |
LL |     Cow,
LL |     Cow,
   |     --- `FarmAnimal::Cow` defined here
...
LL |         FarmAnimal::Cow(_) => "moo".to_string(),
   |         ^^^^^^^^^^^^^^^^^^ help: use this syntax instead: `FarmAnimal::Cow`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(TypeNS)`,
 right: `Some(ValueNS)`', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1014:9
   0:     0x7f241c92dbfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   0:     0x7f241c92dbfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   1:     0x7f241c99d5e8 - core::fmt::write::h5bbeb8c686edca77
   2:     0x7f241c91fa61 - std::io::Write::write_fmt::hbd4175feb6647397
   3:     0x7f241c92da01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   4:     0x7f241c930d64 - std::panicking::default_hook::{{closure}}::h2e112b0fa365fcd9
   5:     0x7f241c930a29 - std::panicking::default_hook::h7e14da905e95e9aa
   6:     0x7f241d3733b4 - rustc_driver[3658c77f76a9c99b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f241c9314b4 - std::panicking::rust_panic_with_hook::h36d1740164bc7729
   8:     0x7f241c931217 - std::panicking::begin_panic_handler::{{closure}}::h745616476b564158
   9:     0x7f241c92e134 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b985c2449af84e8
  10:     0x7f241c930ee2 - rust_begin_unwind
  11:     0x7f241c8e1f83 - core::panicking::panic_fmt::hd1e7554aa3b304ae
  12:     0x7f241c999f08 - core::panicking::assert_failed_inner::h25774d7065544fb7
  13:     0x7f241d153b5b - core[9063fd8b88d3339a]::panicking::assert_failed::<core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>, core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>>
  14:     0x7f241d8edaec - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::instantiate_value_path
  15:     0x7f241d8f8155 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_pat
  16:     0x7f241d8d2fae - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_match
  17:     0x7f241d8e017a - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  18:     0x7f241d874c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:     0x7f241d8df3e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  20:     0x7f241d8957f0 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_decl
  21:     0x7f241d895b2c - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_stmt
  22:     0x7f241d896267 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_block_with_expected
  23:     0x7f241d8e034b - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  24:     0x7f241d874c94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7f241d8df3e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  26:     0x7f241d87650f - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_return_expr
  27:     0x7f241d9fc1a2 - rustc_hir_typeck[6d2ec8a243be3d62]::check::check_fn
  28:     0x7f241d9bf074 - <rustc_hir_typeck[6d2ec8a243be3d62]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_with_fallback<rustc_hir_typeck[6d2ec8a243be3d62]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>
  29:     0x7f241d990a73 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck
  30:     0x7f241f13d699 - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>>
  31:     0x7f241f26be4d - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  32:     0x7f241edcfe70 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck
  33:     0x7f241d955696 - <core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once
  34:     0x7f241da8d3e6 - std[7ecdc0224e2dd782]::panicking::try::<(), core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  35:     0x7f241d943e4d - rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in::<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>
  36:     0x7f241d98fa17 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies
  37:     0x7f241f18d35e - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), ()>>
  38:     0x7f241f23373b - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck_item_bodies, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  39:     0x7f241edcf71a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck_item_bodies
  40:     0x7f241daf37fb - <rustc_session[cfb6c8c79fe50e23]::session::Session>::time::<(), rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate::{closure#7}>
  41:     0x7f241dd1c633 - rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate
  42:     0x7f241d4d7dc1 - rustc_interface[a323614b9422972e]::passes::analysis
  43:     0x7f241f181b9f - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>>
  44:     0x7f241f26c1fb - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::analysis, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  45:     0x7f241eda673a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::analysis
  46:     0x7f241d3ce78d - <rustc_interface[a323614b9422972e]::passes::QueryContext>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  47:     0x7f241d3e1c6a - <rustc_interface[a323614b9422972e]::interface::Compiler>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}, core[9063fd8b88d3339a]::result::Result<core[9063fd8b88d3339a]::option::Option<rustc_interface[a323614b9422972e]::queries::Linker>, rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  48:     0x7f241d374afe - rustc_span[e2c12e69ff4a0ed9]::with_source_map::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  49:     0x7f241d3d43bc - <scoped_tls[e0d1874e2825f8de]::ScopedKey<rustc_span[e2c12e69ff4a0ed9]::SessionGlobals>>::set::<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  50:     0x7f241d392659 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  51:     0x7f241d3d5476 - std[7ecdc0224e2dd782]::panicking::try::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  52:     0x7f241d383f5a - <<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1} as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f241c93dbde - std::sys::unix::thread::Thread::new::thread_start::h853e937d366c7f55
  54:     0x7f241c6d3b43 - <unknown>
  55:     0x7f241c765a00 - <unknown>
  56:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (350ee9de0 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `what_does_the_animal_say`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0532`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0533]: expected value, found struct variant `Alias::Braced`
   |
LL |     Alias::Braced;
   |     ^^^^^^^^^^^^^ not a value


error[E0533]: expected unit struct, unit variant or constant, found struct variant `Alias::Braced`
   |
LL |     let Alias::Braced = panic!();
   |         ^^^^^^^^^^^^^ not a unit struct, unit variant or constant


thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(TypeNS)`,
 right: `Some(ValueNS)`', compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1014:9
   0:     0x7f51bef47bfe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb5fd25bc706aca8b
   1:     0x7f51befb75e8 - core::fmt::write::h5bbeb8c686edca77
   2:     0x7f51bef39a61 - std::io::Write::write_fmt::hbd4175feb6647397
   3:     0x7f51bef47a01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   3:     0x7f51bef47a01 - std::sys_common::backtrace::print::hde07b85e7c675d1f
   4:     0x7f51bef4ad64 - std::panicking::default_hook::{{closure}}::h2e112b0fa365fcd9
   5:     0x7f51bef4aa29 - std::panicking::default_hook::h7e14da905e95e9aa
   6:     0x7f51bf98d3b4 - rustc_driver[3658c77f76a9c99b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f51bef4b4b4 - std::panicking::rust_panic_with_hook::h36d1740164bc7729
   8:     0x7f51bef4b217 - std::panicking::begin_panic_handler::{{closure}}::h745616476b564158
   9:     0x7f51bef48134 - std::sys_common::backtrace::__rust_end_short_backtrace::h2b985c2449af84e8
  10:     0x7f51bef4aee2 - rust_begin_unwind
  11:     0x7f51beefbf83 - core::panicking::panic_fmt::hd1e7554aa3b304ae
  12:     0x7f51befb3f08 - core::panicking::assert_failed_inner::h25774d7065544fb7
  13:     0x7f51bf76db5b - core[9063fd8b88d3339a]::panicking::assert_failed::<core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>, core[9063fd8b88d3339a]::option::Option<rustc_hir[d8b8001f4e53fde3]::def::Namespace>>
  14:     0x7f51bff07aec - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::instantiate_value_path
  15:     0x7f51bff12155 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_pat
  16:     0x7f51bfeaf900 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_decl
  17:     0x7f51bfeafb2c - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_stmt
  18:     0x7f51bfeb0267 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_block_with_expected
  19:     0x7f51bfefa34b - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f51bfe8ec94 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f51bfef93e2 - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f51bfe9050f - <rustc_hir_typeck[6d2ec8a243be3d62]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7f51c00161a2 - rustc_hir_typeck[6d2ec8a243be3d62]::check::check_fn
  24:     0x7f51bffd9074 - <rustc_hir_typeck[6d2ec8a243be3d62]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_with_fallback<rustc_hir_typeck[6d2ec8a243be3d62]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>
  25:     0x7f51bffaaa73 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck
  26:     0x7f51c1757699 - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId, &rustc_middle[bcebdc05307597e1]::ty::context::TypeckResults>>
  27:     0x7f51c1885e4d - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  28:     0x7f51c13e9e70 - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck
  29:     0x7f51bff6f696 - <core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once
  30:     0x7f51c00a73e6 - std[7ecdc0224e2dd782]::panicking::try::<(), core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7f51bff5de4d - rustc_data_structures[2a5226a6d4340f5a]::sync::par_for_each_in::<&[rustc_span[e2c12e69ff4a0ed9]::def_id::LocalDefId], <rustc_middle[bcebdc05307597e1]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies::{closure#0}>::{closure#0}>
  32:     0x7f51bffa9a17 - rustc_hir_typeck[6d2ec8a243be3d62]::typeck_item_bodies
  33:     0x7f51c17a735e - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), ()>>
  34:     0x7f51c184d73b - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::typeck_item_bodies, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  35:     0x7f51c13e971a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::typeck_item_bodies
  36:     0x7f51c010d7fb - <rustc_session[cfb6c8c79fe50e23]::session::Session>::time::<(), rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate::{closure#7}>
  37:     0x7f51c0336633 - rustc_hir_analysis[d7d61c3ead6b85f5]::check_crate
  38:     0x7f51bfaf1dc1 - rustc_interface[a323614b9422972e]::passes::analysis
  39:     0x7f51c179bb9f - rustc_query_system[d692168115ac663a]::query::plumbing::try_execute_query::<rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt, rustc_query_system[d692168115ac663a]::query::caches::DefaultCache<(), core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>>
  40:     0x7f51c18861fb - rustc_query_system[d692168115ac663a]::query::plumbing::get_query::<rustc_query_impl[36c3f8a285d34203]::queries::analysis, rustc_query_impl[36c3f8a285d34203]::plumbing::QueryCtxt>
  41:     0x7f51c13c073a - <rustc_query_impl[36c3f8a285d34203]::Queries as rustc_middle[bcebdc05307597e1]::ty::query::QueryEngine>::analysis
  42:     0x7f51bf9e878d - <rustc_interface[a323614b9422972e]::passes::QueryContext>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  43:     0x7f51bf9fbc6a - <rustc_interface[a323614b9422972e]::interface::Compiler>::enter::<rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}::{closure#2}, core[9063fd8b88d3339a]::result::Result<core[9063fd8b88d3339a]::option::Option<rustc_interface[a323614b9422972e]::queries::Linker>, rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  44:     0x7f51bf98eafe - rustc_span[e2c12e69ff4a0ed9]::with_source_map::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  45:     0x7f51bf9ee3bc - <scoped_tls[e0d1874e2825f8de]::ScopedKey<rustc_span[e2c12e69ff4a0ed9]::SessionGlobals>>::set::<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  46:     0x7f51bf9ac659 - std[7ecdc0224e2dd782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>
  47:     0x7f51bf9ef476 - std[7ecdc0224e2dd782]::panicking::try::<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, core[9063fd8b88d3339a]::panic::unwind_safe::AssertUnwindSafe<<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  48:     0x7f51bf99df5a - <<std[7ecdc0224e2dd782]::thread::Builder>::spawn_unchecked_<rustc_interface[a323614b9422972e]::util::run_in_thread_pool_with_globals<rustc_interface[a323614b9422972e]::interface::run_compiler<core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>, rustc_driver[3658c77f76a9c99b]::run_compiler::{closure#1}>::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9063fd8b88d3339a]::result::Result<(), rustc_errors[4179f80a31162f2b]::ErrorGuaranteed>>::{closure#1} as core[9063fd8b88d3339a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f51bef57bde - std::sys::unix::thread::Thread::new::thread_start::h853e937d366c7f55
  50:     0x7f51becedb43 - <unknown>
  51:     0x7f51bed7fa00 - <unknown>
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (350ee9de0 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0533`.
------------------------------------------
