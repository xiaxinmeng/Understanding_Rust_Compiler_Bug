plain
................................................................................i................... 1700/12040
.................................................................................................... 1800/12040
.................................................................................................... 1900/12040
........................i........................................................................... 2000/12040
.....................................................................F..F........................... 2100/12040
.................................................................................................... 2300/12040
.................................................................................................... 2400/12040
.................................................................................................... 2500/12040
.................................................................................................... 2600/12040
---
failures:

---- [ui] ui/const-generics/issues/issue-85848.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-85848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-85848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-85848/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/fold.rs:908:21: Not enough bound vars: BoundRegion { var: 0, kind: BrAnon(0) } not found in []
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1006:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4c67e86bc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:5 ~ issue_85848[317d]::_Contains::does_contain) not normalized for codegen: [(), &C]', compiler/rustc_middle/src/ty/instance.rs:285:9
stack backtrace:
   0:     0x7f3cf694b140 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0ad22358f179bff1
   1:     0x7f3cf69bfb10 - core::fmt::write::hd3665f81bd96b7f1
   2:     0x7f3cf693b706 - std::io::Write::write_fmt::h9db6f24a0ab4c728
   3:     0x7f3cf694f657 - std::panicking::default_hook::{{closure}}::h374101f43fb27a68
   4:     0x7f3cf694f058 - std::panicking::default_hook::h3ee7b71bf81c784e
   5:     0x7f3cf7231951 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h0e05d17111133f4d
   6:     0x7f3cf694ffa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
   7:     0x7f3cf694fac7 - std::panicking::begin_panic_handler::{{closure}}::hb20fd4d0247775d2
   8:     0x7f3cf694b5dc - std::sys_common::backtrace::__rust_end_short_backtrace::h87567b7080055fa1
   9:     0x7f3cf694fa29 - rust_begin_unwind
  10:     0x7f3cf694f9db - std::panicking::begin_panic_fmt::he630a2624d374851
  11:     0x7f3cf9747e0d - rustc_middle::ty::instance::Instance::new::hf140b85a1b03406e
  12:     0x7f3cf80ac497 - std::thread::local::LocalKey<T>::with::he6fa8f98b7f77465
  13:     0x7f3cf817a7c8 - rustc_query_impl::make_query::resolve_instance::h3decea6b8fda78a1
  14:     0x7f3cf7f31a36 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h3aa385160e846018
  15:     0x7f3cf80fc198 - rustc_query_impl::Queries::try_collect_active_jobs::h4f0120df8e9b31a0
  16:     0x7f3cf834a3d1 - rustc_query_system::query::job::print_query_stack::heefa614c57a60652
  17:     0x7f3cf7359c04 - rustc_interface::interface::try_print_query_stack::h7b1b7e7441f5bc06
  18:     0x7f3cf7232219 - rustc_driver::report_ice::hc4e2f9e1f5c74864
  19:     0x7f3cf694ffa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
  20:     0x7f3cf990d7cd - std::panicking::begin_panic::{{closure}}::h0badbd3b35fa00df
  21:     0x7f3cf990d6b6 - std::sys_common::backtrace::__rust_end_short_backtrace::hc6057bab15158849
  22:     0x7f3cf990d76f - std::panicking::begin_panic::h12ece5e5c8025a50
  23:     0x7f3cf98d30cd - std::panic::panic_any::h836ea5603b780e93
  24:     0x7f3cf98d87e5 - rustc_errors::HandlerInner::bug::had42cea75d0c996f
  25:     0x7f3cf98d6e00 - rustc_errors::Handler::bug::h8018da1aae71d501
  26:     0x7f3cf97ce491 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h931236766824b14a
  27:     0x7f3cf97c91fb - rustc_middle::ty::context::tls::with_opt::{{closure}}::h26e47027a402041c
  28:     0x7f3cf97c91a8 - rustc_middle::ty::context::tls::with_opt::h4a88ca868f72ae0f
  29:     0x7f3cf97ce3b3 - rustc_middle::util::bug::opt_span_bug_fmt::hdcbc0c09dd2caa9f
  30:     0x7f3cf97ce325 - rustc_middle::util::bug::bug_fmt::ha7aaf94b6afd2c6f
  31:     0x7f3cf9772b26 - <rustc_middle::ty::fold::ValidateBoundVars as rustc_middle::ty::fold::TypeVisitor>::visit_region::h2327308febc1c8dc
  32:     0x7f3cf97726d3 - <rustc_middle::ty::fold::ValidateBoundVars as rustc_middle::ty::fold::TypeVisitor>::visit_ty::h98da01cc9208253f
  33:     0x7f3cf7ba15c3 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h55cbb7d301e40d98
  34:     0x7f3cf7b94736 - rustc_middle::ty::sty::Binder<T>::bind_with_vars::hf59ae7a7cd14ca95
  35:     0x7f3cf7bbf1f0 - rustc_ty_utils::instance::inner_resolve_instance::hfa28d19df5250c12
  36:     0x7f3cf7bbdf91 - rustc_ty_utils::instance::resolve_instance::hb78e3ce4150d005d
  37:     0x7f3cf8267ccb - rustc_data_structures::stack::ensure_sufficient_stack::ha98863af9fe9ab74
  38:     0x7f3cf7e26798 - rustc_query_system::query::plumbing::get_query_impl::h8d6c64997d4b52cc
  39:     0x7f3cf7f7456e - rustc_query_system::query::plumbing::get_query::h33435a334a6b8978
  40:     0x7f3cf80fdd1e - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::hf53761e9b9012768
  41:     0x7f3cf9758856 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::h43fe9a0e5a3aef54
  42:     0x7f3cf962e7f0 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  43:     0x7f3cf92cec66 - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h3bcee74775a9f11c
  44:     0x7f3cf935a375 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h7de4d1a6d99b4a3e
  45:     0x7f3cf92cef4f - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::hae4a6f288718ba42
  46:     0x7f3cf76e075a - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::h91c02b6decb508e8
  47:     0x7f3cf76008ae - rustc_infer::infer::InferCtxtBuilder::enter::he4b6da4036ac00e4
  48:     0x7f3cf76bd216 - core::ops::function::FnOnce::call_once::h4891a58ba368007d
  49:     0x7f3cf82725b4 - rustc_data_structures::stack::ensure_sufficient_stack::hf2a7db355c3d755b
  50:     0x7f3cf7e5c542 - rustc_query_system::query::plumbing::get_query_impl::hddedcfd0e61cac53
  51:     0x7f3cf7f836fb - rustc_query_system::query::plumbing::get_query::h8bfd2792aa3e2049
  52:     0x7f3cf80fd882 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::h9cf88d9ef91b7479
  53:     0x7f3cf89df285 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::h08fd0149b78737bd
  54:     0x7f3cf89e1cc4 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::h0f7118e13c971f85
  55:     0x7f3cf891f0b2 - rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame::h5832c3efc38bf489
  56:     0x7f3cf8958920 - rustc_mir::interpret::terminator::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_fn_call::hadf307748ebc53de
  57:     0x7f3cf8949d36 - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run::h5773fb01b075c557
  58:     0x7f3cf87f5a34 - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h48102b5a7395a679
  59:     0x7f3cf82585be - rustc_data_structures::stack::ensure_sufficient_stack::h43f3422914525ee2
  60:     0x7f3cf7dfa395 - rustc_query_system::query::plumbing::get_query_impl::h3d12c438fc200d8b
  61:     0x7f3cf7f6fd05 - rustc_query_system::query::plumbing::get_query::h1e74956c164c48cd
  62:     0x7f3cf80fcc66 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h12cde6a08ed9c948
  63:     0x7f3cf87f3d38 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::hdfa777f137b9d389
  64:     0x7f3cf8269fbe - rustc_data_structures::stack::ensure_sufficient_stack::hb9cfe13d8b78049e
  65:     0x7f3cf7e1ae75 - rustc_query_system::query::plumbing::get_query_impl::h7cf2da0b2a502a0c
  66:     0x7f3cf7f72bd9 - rustc_query_system::query::plumbing::get_query::h2c0460e912bc09e7
  67:     0x7f3cf80fccc6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::hb235697e82c45e22
  68:     0x7f3cf9604310 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::hf2e984f0715606c4
  69:     0x7f3cf962ea9e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  70:     0x7f3cf9492cd6 - rustc_infer::infer::InferCtxt::const_eval_resolve::hef426c709502dc8d
  71:     0x7f3cf92ce4d4 - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::{{closure}}::hfcf63ff6a82d9417
  72:     0x7f3cf92cddd3 - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::h7c5125f11ecb1269
  73:     0x7f3cf92e78ce - rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations::h56bdb4d78e1d7383
  74:     0x7f3cf92cc10b - <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible::h0c6cbd0a5b74d8a6
  75:     0x7f3cf78c570d - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::h74b4fbf943a0086f
  76:     0x7f3cf7896d99 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call::hb680f2d378000bd4
  77:     0x7f3cf7894f20 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call::h278f71d2842a108f
  78:     0x7f3cf78abfd3 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h5fd42fc4b945daad
  79:     0x7f3cf78aad35 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h9510b4cc16a97189
  80:     0x7f3cf78c8e59 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt::he4e62017abc14575
  81:     0x7f3cf78c92b7 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::h31e52474a6ba4b5a
  82:     0x7f3cf78ab87b - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h5fd42fc4b945daad
  83:     0x7f3cf78aad35 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h9510b4cc16a97189
  84:     0x7f3cf78b3337 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr::h8329365760e9c8a6
  85:     0x7f3cf7b18108 - rustc_typeck::check::check::check_fn::he8c42ffd9ed05322
  86:     0x7f3cf7a5a422 - rustc_infer::infer::InferCtxtBuilder::enter::hd609c1b37dea5644
  87:     0x7f3cf79cddbe - rustc_typeck::check::inherited::InheritedBuilder::enter::h4698eb07b2aa484b
  88:     0x7f3cf7b31e4a - rustc_typeck::check::typeck::h52a8d7134859a862
  89:     0x7f3cf7de1ebb - rustc_query_system::query::plumbing::get_query_impl::h18d8d321f7b42a0d
  90:     0x7f3cf7f92cbd - rustc_query_system::query::plumbing::get_query::hdc29f995fc614cbe
  91:     0x7f3cf79a6ead - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners::hfb02a13b77b5d0fa
  92:     0x7f3cf7b3322d - rustc_typeck::check::typeck_item_bodies::h41d2746bdfc86dc1
  93:     0x7f3cf7dfb40b - rustc_query_system::query::plumbing::get_query_impl::h3e715a6d790dc86c
  94:     0x7f3cf7f75229 - rustc_query_system::query::plumbing::get_query::h361f9efb13c476b9
  95:     0x7f3cf79c6532 - rustc_session::utils::<impl rustc_session::session::Session>::time::h51d23cf686c8ad21
  96:     0x7f3cf799c36e - rustc_typeck::check_crate::h25b205119df717ea
  97:     0x7f3cf737024a - rustc_interface::passes::analysis::h1f64f7393f4640e3
  98:     0x7f3cf7e11288 - rustc_query_system::query::plumbing::get_query_impl::h6c7236c03a8f126a
  99:     0x7f3cf7f71289 - rustc_query_system::query::plumbing::get_query::h23e6348178021f5d
 100:     0x7f3cf72a791e - rustc_interface::passes::QueryContext::enter::h38eb884f84b0b71d
 101:     0x7f3cf7285518 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h98fbd93a2bba01ab
 102:     0x7f3cf724b98a - rustc_span::with_source_map::hbc89c96ac0eaf237
 103:     0x7f3cf728654c - rustc_interface::interface::create_compiler_and_run::hd03524749164bca9
 104:     0x7f3cf724c172 - rustc_span::with_session_globals::h9e6c3f9ea9d5bcf3
 105:     0x7f3cf7286f90 - std::sys_common::backtrace::__rust_begin_short_backtrace::haa4b888a76f4c4b9
 106:     0x7f3cf72a8446 - std::panicking::try::h5ce532b765bd974a
 107:     0x7f3cf72342ca - core::ops::function::FnOnce::call_once{{vtable.shim}}::he2bb69d9ca63f50f
 108:     0x7f3cf695d16a - std::sys::unix::thread::Thread::new::thread_start::h37ca38aa26d012f6
 109:     0x7f3cf16176db - start_thread
 110:     0x7f3cf65e371f - __clone
 111:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4c67e86bc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.


------------------------------------------


---- [ui] ui/const-generics/issues/issue-83765.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/fold.rs:908:21: Not enough bound vars: BoundRegion { var: 0, kind: BrAnon(0) } not found in []
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1006:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4c67e86bc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:4 ~ issue_83765[317d]::TensorDimension::DIM) not normalized for codegen: [BMap<'_, R, T, F, DIM>]', compiler/rustc_middle/src/ty/instance.rs:285:9
stack backtrace:
   0:     0x7fd3f8c65140 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0ad22358f179bff1
   1:     0x7fd3f8cd9b10 - core::fmt::write::hd3665f81bd96b7f1
   2:     0x7fd3f8c55706 - std::io::Write::write_fmt::h9db6f24a0ab4c728
   3:     0x7fd3f8c69657 - std::panicking::default_hook::{{closure}}::h374101f43fb27a68
   4:     0x7fd3f8c69058 - std::panicking::default_hook::h3ee7b71bf81c784e
   5:     0x7fd3f954b951 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h0e05d17111133f4d
   6:     0x7fd3f8c69fa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
   7:     0x7fd3f8c69ac7 - std::panicking::begin_panic_handler::{{closure}}::hb20fd4d0247775d2
   8:     0x7fd3f8c655dc - std::sys_common::backtrace::__rust_end_short_backtrace::h87567b7080055fa1
   9:     0x7fd3f8c69a29 - rust_begin_unwind
  10:     0x7fd3f8c699db - std::panicking::begin_panic_fmt::he630a2624d374851
  11:     0x7fd3fba61e0d - rustc_middle::ty::instance::Instance::new::hf140b85a1b03406e
  12:     0x7fd3fa3c6497 - std::thread::local::LocalKey<T>::with::he6fa8f98b7f77465
  13:     0x7fd3fa4947c8 - rustc_query_impl::make_query::resolve_instance::h3decea6b8fda78a1
  14:     0x7fd3fa24ba36 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h3aa385160e846018
  15:     0x7fd3fa416198 - rustc_query_impl::Queries::try_collect_active_jobs::h4f0120df8e9b31a0
  16:     0x7fd3fa6643d1 - rustc_query_system::query::job::print_query_stack::heefa614c57a60652
  17:     0x7fd3f9673c04 - rustc_interface::interface::try_print_query_stack::h7b1b7e7441f5bc06
  18:     0x7fd3f954c219 - rustc_driver::report_ice::hc4e2f9e1f5c74864
  19:     0x7fd3f8c69fa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
  20:     0x7fd3fbc277cd - std::panicking::begin_panic::{{closure}}::h0badbd3b35fa00df
  21:     0x7fd3fbc276b6 - std::sys_common::backtrace::__rust_end_short_backtrace::hc6057bab15158849
  22:     0x7fd3fbc2776f - std::panicking::begin_panic::h12ece5e5c8025a50
  23:     0x7fd3fbbed0cd - std::panic::panic_any::h836ea5603b780e93
  24:     0x7fd3fbbf27e5 - rustc_errors::HandlerInner::bug::had42cea75d0c996f
  25:     0x7fd3fbbf0e00 - rustc_errors::Handler::bug::h8018da1aae71d501
  26:     0x7fd3fbae8491 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h931236766824b14a
  27:     0x7fd3fbae31fb - rustc_middle::ty::context::tls::with_opt::{{closure}}::h26e47027a402041c
  28:     0x7fd3fbae31a8 - rustc_middle::ty::context::tls::with_opt::h4a88ca868f72ae0f
  29:     0x7fd3fbae83b3 - rustc_middle::util::bug::opt_span_bug_fmt::hdcbc0c09dd2caa9f
  30:     0x7fd3fbae8325 - rustc_middle::util::bug::bug_fmt::ha7aaf94b6afd2c6f
  31:     0x7fd3fba8cb26 - <rustc_middle::ty::fold::ValidateBoundVars as rustc_middle::ty::fold::TypeVisitor>::visit_region::h2327308febc1c8dc
  32:     0x7fd3fb97d053 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h89dc4f865b3fde70
  33:     0x7fd3fb8cf069 - rustc_middle::ty::fold::TypeFoldable::visit_with::h87bb487075fa387e
  34:     0x7fd3f9ebb5c3 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h55cbb7d301e40d98
  35:     0x7fd3f9eae736 - rustc_middle::ty::sty::Binder<T>::bind_with_vars::hf59ae7a7cd14ca95
  36:     0x7fd3f9ed91f0 - rustc_ty_utils::instance::inner_resolve_instance::hfa28d19df5250c12
  37:     0x7fd3f9ed7f91 - rustc_ty_utils::instance::resolve_instance::hb78e3ce4150d005d
  38:     0x7fd3fa581ccb - rustc_data_structures::stack::ensure_sufficient_stack::ha98863af9fe9ab74
  39:     0x7fd3fa140798 - rustc_query_system::query::plumbing::get_query_impl::h8d6c64997d4b52cc
  40:     0x7fd3fa28e56e - rustc_query_system::query::plumbing::get_query::h33435a334a6b8978
  41:     0x7fd3fa417d1e - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::hf53761e9b9012768
  42:     0x7fd3fba72856 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::h43fe9a0e5a3aef54
  43:     0x7fd3fb9487f0 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  44:     0x7fd3fb5e8c66 - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h3bcee74775a9f11c
  45:     0x7fd3fb674375 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h7de4d1a6d99b4a3e
  46:     0x7fd3fb5e8f4f - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::hae4a6f288718ba42
  47:     0x7fd3f99fa75a - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::h91c02b6decb508e8
  48:     0x7fd3f991a8ae - rustc_infer::infer::InferCtxtBuilder::enter::he4b6da4036ac00e4
  49:     0x7fd3f99d7216 - core::ops::function::FnOnce::call_once::h4891a58ba368007d
  50:     0x7fd3fa58c5b4 - rustc_data_structures::stack::ensure_sufficient_stack::hf2a7db355c3d755b
  51:     0x7fd3fa176542 - rustc_query_system::query::plumbing::get_query_impl::hddedcfd0e61cac53
  52:     0x7fd3fa29d6fb - rustc_query_system::query::plumbing::get_query::h8bfd2792aa3e2049
  53:     0x7fd3fa417882 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::h9cf88d9ef91b7479
  54:     0x7fd3facf9285 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::h08fd0149b78737bd
  55:     0x7fd3facfbcc4 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::h0f7118e13c971f85
  56:     0x7fd3fac390b2 - rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame::h5832c3efc38bf489
  57:     0x7fd3fab0fa1b - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h48102b5a7395a679
  58:     0x7fd3fa5725be - rustc_data_structures::stack::ensure_sufficient_stack::h43f3422914525ee2
  59:     0x7fd3fa114395 - rustc_query_system::query::plumbing::get_query_impl::h3d12c438fc200d8b
  60:     0x7fd3fa289d05 - rustc_query_system::query::plumbing::get_query::h1e74956c164c48cd
  61:     0x7fd3fa416c66 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h12cde6a08ed9c948
  62:     0x7fd3fab0dd38 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::hdfa777f137b9d389
  63:     0x7fd3fa583fbe - rustc_data_structures::stack::ensure_sufficient_stack::hb9cfe13d8b78049e
  64:     0x7fd3fa134e75 - rustc_query_system::query::plumbing::get_query_impl::h7cf2da0b2a502a0c
  65:     0x7fd3fa28cbd9 - rustc_query_system::query::plumbing::get_query::h2c0460e912bc09e7
  66:     0x7fd3fa416cc6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::hb235697e82c45e22
  67:     0x7fd3fb91e310 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::hf2e984f0715606c4
  68:     0x7fd3fb948a9e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  69:     0x7fd3fb7accd6 - rustc_infer::infer::InferCtxt::const_eval_resolve::hef426c709502dc8d
  70:     0x7fd3fb5e84d4 - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::{{closure}}::hfcf63ff6a82d9417
  71:     0x7fd3fb5e7dbd - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::h7c5125f11ecb1269
  72:     0x7fd3fb6018ce - rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations::h56bdb4d78e1d7383
  73:     0x7fd3fb5e610b - <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible::h0c6cbd0a5b74d8a6
  74:     0x7fd3fb5e5f6b - <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_all_or_error::hec442c568e132cd6
  75:     0x7fd3f9d6b3b0 - rustc_infer::infer::InferCtxtBuilder::enter::h6f0d1bdeb5dd572a
  76:     0x7fd3f9e123aa - rustc_typeck::check::compare_method::compare_impl_method::h640222abd7efcfbe
  77:     0x7fd3f9e3c1ca - rustc_typeck::check::check::check_item_type::h373c339685187154
  78:     0x7fd3f9c5abc0 - rustc_middle::hir::map::Map::visit_item_likes_in_module::hd5943c0308f52de7
  79:     0x7fd3f9e45cfd - rustc_typeck::check::check::check_mod_item_types::h1c94a91792c2977c
  80:     0x7fd3fa1105ca - rustc_query_system::query::plumbing::get_query_impl::h360577c95543c6b6
  81:     0x7fd3fa2899cd - rustc_query_system::query::plumbing::get_query::h1de72db72bdf721e
  82:     0x7fd3f9ce0a6c - rustc_session::utils::<impl rustc_session::session::Session>::time::hc2b7cb4b948d206f
  83:     0x7fd3f9cb634c - rustc_typeck::check_crate::h25b205119df717ea
  84:     0x7fd3f968a24a - rustc_interface::passes::analysis::h1f64f7393f4640e3
  85:     0x7fd3fa12b288 - rustc_query_system::query::plumbing::get_query_impl::h6c7236c03a8f126a
  86:     0x7fd3fa28b289 - rustc_query_system::query::plumbing::get_query::h23e6348178021f5d
  87:     0x7fd3f95c191e - rustc_interface::passes::QueryContext::enter::h38eb884f84b0b71d
  88:     0x7fd3f959f518 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h98fbd93a2bba01ab
  89:     0x7fd3f956598a - rustc_span::with_source_map::hbc89c96ac0eaf237
  90:     0x7fd3f95a054c - rustc_interface::interface::create_compiler_and_run::hd03524749164bca9
  91:     0x7fd3f9566172 - rustc_span::with_session_globals::h9e6c3f9ea9d5bcf3
  92:     0x7fd3f95a0f90 - std::sys_common::backtrace::__rust_begin_short_backtrace::haa4b888a76f4c4b9
  93:     0x7fd3f95c2446 - std::panicking::try::h5ce532b765bd974a
  94:     0x7fd3f954e2ca - core::ops::function::FnOnce::call_once{{vtable.shim}}::he2bb69d9ca63f50f
  95:     0x7fd3f8c7716a - std::sys::unix::thread::Thread::new::thread_start::h37ca38aa26d012f6
  96:     0x7fd3f39316db - start_thread
  97:     0x7fd3f88fd71f - __clone
  98:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (4c67e86bc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 11941 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 108.80s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:12
