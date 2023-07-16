plain
.................................................................................................... 9400/11812
.................................................................................................... 9500/11812
.................................................................................................... 9600/11812
..................................i......i.......................................................... 9700/11812
.................................................................................iiiiiii.iiiiii.i... 9800/11812
.................................................................................................... 10000/11812
.................................................................................................... 10100/11812
.................................................................................................... 10200/11812
.................................................................................................... 10300/11812
.................................................................................................... 10300/11812
.................................................................................................... 10400/11812
.................................................................................................... 10500/11812
...........................................................................................i........ 10600/11812
.................................................................................................... 10700/11812
.................................................................................................... 10800/11812
.................................................................................................... 10900/11812
...........................................................F.F...................................... 11000/11812
................ii.................................................................................. 11200/11812
.................................................................................................... 11300/11812
.................................................................................................... 11400/11812
.................................................................................................... 11500/11812
---
failures:

---- [ui] ui/consts/const-eval/const-eval-query-stack.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:24:28
   |
LL |     let x: &'static i32 = &X;
   |                            ^ referenced constant has errors
error: internal compiler error: ty::ConstKind::Error constructed but no error reported.


thread 'rustc' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1038:36
   0: rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_errors::HandlerInner::emit_diagnostic::{{closure}}
   3: rustc_interface::callbacks::track_diagnostic
   3: rustc_interface::callbacks::track_diagnostic
   4: rustc_errors::HandlerInner::emit_diagnostic
   5: rustc_errors::HandlerInner::emit_diag_at_span
   6: rustc_errors::HandlerInner::span_bug
   7: rustc_session::session::Session::delay_span_bug
   8: rustc_middle::ty::context::TyCtxt::const_error
   9: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
  10: rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with
  11: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
  12: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  13: rustc_infer::infer::InferCtxtBuilder::enter
  14: core::ops::function::FnOnce::call_once
  15: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_mir_const_after_erasing_regions>::compute
  16: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::force_query_with_job
  20: rustc_query_system::query::plumbing::get_query_impl
  21: rustc_query_system::query::plumbing::get_query
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
  23: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  24: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
  25: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand
  26: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  27: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
  28: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  29: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  31: rustc_mir::transform::optimized_mir
  31: rustc_mir::transform::optimized_mir
  32: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  33: rustc_data_structures::stack::ensure_sufficient_stack
  34: rustc_query_system::query::plumbing::force_query_with_job
  35: rustc_query_system::query::plumbing::get_query_impl
  36: rustc_query_system::query::plumbing::get_query
  37: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  38: rustc_mir::monomorphize::collector::collect_neighbours
  39: rustc_data_structures::stack::ensure_sufficient_stack
  40: rustc_mir::monomorphize::collector::collect_items_rec
  41: rustc_mir::monomorphize::collector::collect_crate_mono_items
  42: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  43: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
  44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  45: rustc_data_structures::stack::ensure_sufficient_stack
  46: rustc_query_system::query::plumbing::force_query_with_job
  47: rustc_query_system::query::plumbing::get_query_impl
  48: rustc_query_system::query::plumbing::get_query
  49: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  50: rustc_codegen_ssa::base::codegen_crate
  51: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  52: rustc_session::utils::<impl rustc_session::session::Session>::time
  53: rustc_interface::queries::Queries::ongoing_codegen
  54: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  55: rustc_span::with_source_map
  56: scoped_tls::ScopedKey<T>::set
  57: rustc_span::with_session_globals


thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f11bd53eef0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f11bd5b31fc - core::fmt::write::hf00778d011964c9e
   2:     0x7f11bd532cc6 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f11bd543287 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f11bd542c94 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f11bde184ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f11bd543bc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f11bd5436f7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f11bd53f38c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f11bd543659 - rust_begin_unwind
  10:     0x7f11bd5af801 - core::panicking::panic_fmt::hacef841ef98e28f3
  11:     0x7f11bd5afc03 - core::result::unwrap_failed::heca1ed5f139fb358
  12:     0x7f11bdf8e8de - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  13:     0x7f11c04a05b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  14:     0x7f11c049fe41 - rustc_errors::Handler::emit_diagnostic::h1fdf1708c3ce7ac6
  15:     0x7f11bde18640 - rustc_driver::report_ice::he956548a9b05d3f4
  16:     0x7f11bd543bc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
  17:     0x7f11bd5436f7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
  18:     0x7f11bd53f38c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
  19:     0x7f11bd543659 - rust_begin_unwind
  20:     0x7f11bd54360b - std::panicking::begin_panic_fmt::h6bd78b11ce36bb90
  21:     0x7f11c04a0974 - rustc_errors::HandlerInner::emit_diagnostic::{{closure}}::hfcdcb2cf15991544
  22:     0x7f11bdf8e87f - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  23:     0x7f11c04a05b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  24:     0x7f11c0198751 - rustc_errors::HandlerInner::emit_diag_at_span::h4fdc1222d9bd0c97
  25:     0x7f11c01987b0 - rustc_errors::HandlerInner::span_bug::hdd4d5e678185daf9
  26:     0x7f11c0199a29 - rustc_session::session::Session::delay_span_bug::h7984af9ed13a8d9d
  27:     0x7f11c01e194a - rustc_middle::ty::context::TyCtxt::const_error::h5756d9d5ee6e3d69
  28:     0x7f11bfe8b02c - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h2959e07be3d41a3c
  29:     0x7f11bff189d5 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h79807003f8c0e478
  30:     0x7f11bfe8b27f - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::h2cfb7a76756041c9
  31:     0x7f11be2cfc8a - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::hbf10df506a46cc46
  32:     0x7f11be1ef9e6 - rustc_infer::infer::InferCtxtBuilder::enter::hb1598f4ed2c83b72
  33:     0x7f11be2abac6 - core::ops::function::FnOnce::call_once::h551a79ccc2fe89f4
  34:     0x7f11beb42329 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_mir_const_after_erasing_regions>::compute::h334efdce70d35558
  35:     0x7f11bee753b5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf25673d327eab726
  36:     0x7f11bee7e8c5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h4285382e20632337
  37:     0x7f11bebfcb67 - rustc_data_structures::stack::ensure_sufficient_stack::h61bf5af20280035c
  38:     0x7f11bea71d0b - rustc_query_system::query::plumbing::force_query_with_job::ha04971584a25f023
  39:     0x7f11bea09308 - rustc_query_system::query::plumbing::get_query_impl::hd994d0b27964899f
  40:     0x7f11beb36f05 - rustc_query_system::query::plumbing::get_query::hf0b8c1d1468567c9
  41:     0x7f11beda6242 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::h718614d38b84e611
  42:     0x7f11bf57a215 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::hace648924f4b8e2c
  43:     0x7f11bf57b424 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::h1528c107df3ef154
  44:     0x7f11bf4d2d73 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_operand::h1942d7964d8deec8
  45:     0x7f11bf4f0987 - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place::h0f0b84c5889e025a
  46:     0x7f11bf515f16 - <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement::hbf25c52c06330c66
  47:     0x7f11bf5155dd - <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body::hdb7d9c5d10e43217
  48:     0x7f11bf50bd65 - <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass::h87e12afe46281379
  49:     0x7f11bf1ddf9a - rustc_mir::transform::run_passes::h50bb9b2b5a0c3df2
  50:     0x7f11bf1e30d8 - rustc_mir::transform::optimized_mir::haf05ddcbf499b9c8
  51:     0x7f11bee76054 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf7884ec39e8423ad
  52:     0x7f11bebfeb27 - rustc_data_structures::stack::ensure_sufficient_stack::h6a55b2624d49f855
  53:     0x7f11bea8214d - rustc_query_system::query::plumbing::force_query_with_job::hf01e4c85f8c7babe
  54:     0x7f11be9deeca - rustc_query_system::query::plumbing::get_query_impl::h8524ff451ae8b84c
  55:     0x7f11beb1619c - rustc_query_system::query::plumbing::get_query::h380c9b2fe07ff622
  56:     0x7f11c01f13f8 - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir::h9050a0e90490faec
  57:     0x7f11bf2c20e7 - rustc_mir::monomorphize::collector::collect_neighbours::h585c59102978fa1b
  58:     0x7f11bf172bb6 - rustc_data_structures::stack::ensure_sufficient_stack::he43d5ba26a574747
  59:     0x7f11bf2b8ea2 - rustc_mir::monomorphize::collector::collect_items_rec::h42464e325481faa3
  60:     0x7f11bf2b816b - rustc_mir::monomorphize::collector::collect_crate_mono_items::h6d788e05da68ba3a
  61:     0x7f11bf2c3ffa - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::he9f39258c9a934b3
  62:     0x7f11beb421d9 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute::h1071e7a1028a6e81
  63:     0x7f11bee52bb9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h729eab6ff0884b29
  64:     0x7f11bec00187 - rustc_data_structures::stack::ensure_sufficient_stack::h72aa460f04c6acbf
  65:     0x7f11bea58da1 - rustc_query_system::query::plumbing::force_query_with_job::h069ed8e53c0e9ad2
  66:     0x7f11be9b45f6 - rustc_query_system::query::plumbing::get_query_impl::h2b2e4e2f9e8a8649
  67:     0x7f11beb29992 - rustc_query_system::query::plumbing::get_query::ha67216392fc62948
  68:     0x7f11beda611e - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items::hb5aacc172bab7f79
  69:     0x7f11be0b839c - rustc_codegen_ssa::base::codegen_crate::hcbda64ac2ce292fb
  70:     0x7f11be0b0795 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::hd74d6b703cfd9956
  71:     0x7f11bdf5c2b1 - rustc_session::utils::<impl rustc_session::session::Session>::time::h9077d8a387101efe
  72:     0x7f11bdf7d528 - rustc_interface::queries::Queries::ongoing_codegen::hfe7ea4c77b0f5162
  73:     0x7f11bde310a8 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb6e9ea0d60143445
  74:     0x7f11bde1a280 - rustc_span::with_source_map::heac904d2395b36aa
  75:     0x7f11bde2f74c - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  76:     0x7f11bde1a8f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  77:     0x7f11bde32b82 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  78:     0x7f11bde32e86 - std::panicking::try::hf44e1134f75332f0
  79:     0x7f11bde5550a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  80:     0x7f11bd553d3a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  81:     0x7f11b820a6db - start_thread
  82:     0x7f11bd1d671f - __clone
  83:                0x0 - <unknown>

thread panicked while processing panic. aborting.
------------------------------------------


---- [ui] ui/treat-err-as-bug/err.rs stdout ----
---- [ui] ui/treat-err-as-bug/err.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/treat-err-as-bug/err.rs:11:21
   |
LL | pub static C: u32 = 0 - 1;
   |                     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1036:27


thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f1044546ef0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f10445bb1fc - core::fmt::write::hf00778d011964c9e
   2:     0x7f104453acc6 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f104454b287 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f104454ac94 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f1044e204ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f104454bbc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f104454b6f7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f104454738c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f104454b659 - rust_begin_unwind
  10:     0x7f10445b7801 - core::panicking::panic_fmt::hacef841ef98e28f3
  11:     0x7f10445b7c03 - core::result::unwrap_failed::heca1ed5f139fb358
  12:     0x7f1044f968de - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  13:     0x7f10474a85b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  14:     0x7f10474a7e41 - rustc_errors::Handler::emit_diagnostic::h1fdf1708c3ce7ac6
  15:     0x7f1044e20640 - rustc_driver::report_ice::he956548a9b05d3f4
  16:     0x7f104454bbc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
  17:     0x7f10474d8464 - std::panicking::begin_panic::{{closure}}::h2d50bfdc71b22e40
  18:     0x7f10474d82fc - std::sys_common::backtrace::__rust_end_short_backtrace::h037f8c7a07f27f2e
  19:     0x7f10474d841c - std::panicking::begin_panic::h1edb7edaf4f6e057
  20:     0x7f10474a898e - rustc_errors::HandlerInner::emit_diagnostic::{{closure}}::hfcdcb2cf15991544
  21:     0x7f1044f9687f - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  22:     0x7f10474a85b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  23:     0x7f10474a7e41 - rustc_errors::Handler::emit_diagnostic::h1fdf1708c3ce7ac6
  24:     0x7f10474a2174 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h4c4af60f57627e1c
  25:     0x7f1046556c67 - rustc_mir::const_eval::error::ConstEvalErr::struct_generic::{{closure}}::h15c0067544316c3f
  26:     0x7f1046556343 - rustc_mir::const_eval::error::ConstEvalErr::report_as_error::h24134641f67d4e04
  27:     0x7f10461b05da - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h80aa764300dd98ad
  28:     0x7f1045b49330 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_allocation_raw>::compute::h3d7951be88332d56
  29:     0x7f1045e4ab29 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h3f5f6f8652f8c6dd
  30:     0x7f1045e8735c - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h6f72bd92bf5cd889
  31:     0x7f1045bf8c6b - rustc_data_structures::stack::ensure_sufficient_stack::h1826028c277dd381
  32:     0x7f1045a62495 - rustc_query_system::query::plumbing::force_query_with_job::h0b358173ac1d248c
  33:     0x7f1045a17dc7 - rustc_query_system::query::plumbing::get_query_impl::he35ddb3462f38b05
  34:     0x7f1045b34a9d - rustc_query_system::query::plumbing::get_query::hba7470b748acd5d1
  35:     0x7f1045dad686 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h4eb978391f9536f6
  36:     0x7f10461af00d - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h80aa764300dd98ad
  37:     0x7f1045b49330 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::eval_to_allocation_raw>::compute::h3d7951be88332d56
  38:     0x7f1045e4ab29 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h3f5f6f8652f8c6dd
  39:     0x7f1045e8735c - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h6f72bd92bf5cd889
  40:     0x7f1045bf8c6b - rustc_data_structures::stack::ensure_sufficient_stack::h1826028c277dd381
  41:     0x7f1045a62495 - rustc_query_system::query::plumbing::force_query_with_job::h0b358173ac1d248c
  42:     0x7f1045a17dc7 - rustc_query_system::query::plumbing::get_query_impl::he35ddb3462f38b05
  43:     0x7f1045b34a9d - rustc_query_system::query::plumbing::get_query::hba7470b748acd5d1
  44:     0x7f1045dad686 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h4eb978391f9536f6
  45:     0x7f10471d22b6 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::eval_static_initializer::h0904a8e257484dc9
  46:     0x7f1046d55dad - <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item::h4bb2e8a31fefeb18
  47:     0x7f1046d43204 - <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item::h40c520cab20151cf
  48:     0x7f1044f5a952 - rustc_hir::intravisit::Visitor::visit_nested_item::h87f22a1f731b5b0f
  49:     0x7f1044ffd7cc - rustc_hir::intravisit::walk_mod::hb0c6755fd88db270
  50:     0x7f1044f59a1c - <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod::hf6a5e85bb6f57be7
  51:     0x7f1044ffa1d5 - rustc_hir::intravisit::walk_crate::hca5476eae1f47be1
  52:     0x7f1044f39323 - rustc_lint::late::late_lint_crate::hede14a651a074f7f
  53:     0x7f1044f646cb - rustc_session::utils::<impl rustc_session::session::Session>::time::hade54cf6eba1f2eb
  54:     0x7f1044f74ec2 - rustc_data_structures::sync::join::h1cd62bb3f5028c85
  55:     0x7f1044f65ca0 - rustc_session::utils::<impl rustc_session::session::Session>::time::hd235714af3f5713f
  56:     0x7f1044f8c896 - std::panic::catch_unwind::h4497f28f452f0f61
  57:     0x7f1044f6456e - rustc_session::utils::<impl rustc_session::session::Session>::time::ha3c16cf8f4152e2e
  58:     0x7f1044f7b3cb - rustc_interface::passes::analysis::ha4a57662de7ec48b
  59:     0x7f1045e43f0c - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h2706a7e0f5827ec3
  60:     0x7f1045e822c9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h9074bb834d6f29ba
  61:     0x7f1045bfd5c6 - rustc_data_structures::stack::ensure_sufficient_stack::h311fb65fcc40ec38
  62:     0x7f1045a6c3e7 - rustc_query_system::query::plumbing::force_query_with_job::h4caa62d0c1565224
  63:     0x7f10459cfec7 - rustc_query_system::query::plumbing::get_query_impl::h55e459bddc0b1fd8
  64:     0x7f1045b15843 - rustc_query_system::query::plumbing::get_query::h0a5c7aa54d0363c0
  65:     0x7f1044e5b27c - rustc_interface::passes::QueryContext::enter::he84f343e2f56c291
  66:     0x7f1044e39047 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb6e9ea0d60143445
  67:     0x7f1044e22280 - rustc_span::with_source_map::heac904d2395b36aa
  68:     0x7f1044e3774c - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  69:     0x7f1044e228f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  70:     0x7f1044e3ab82 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  71:     0x7f1044e3ae86 - std::panicking::try::hf44e1134f75332f0
  72:     0x7f1044e5d50a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  73:     0x7f104455bd3a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  74:     0x7f103f2126db - start_thread
  75:     0x7f10441de71f - __clone
  76:                0x0 - <unknown>

thread panicked while processing panic. aborting.
------------------------------------------


---- [ui] ui/treat-err-as-bug/delay_span_bug.rs stdout ----
---- [ui] ui/treat-err-as-bug/delay_span_bug.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1036:27


thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7f905171aef0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f905178f1fc - core::fmt::write::hf00778d011964c9e
   2:     0x7f905170ecc6 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f905171f287 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f905171ec94 - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f9051ff44ed - rustc_driver::report_ice::he956548a9b05d3f4
   6:     0x7f905171fbc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   7:     0x7f905171f6f7 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   8:     0x7f905171b38c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   9:     0x7f905171f659 - rust_begin_unwind
  10:     0x7f905178b801 - core::panicking::panic_fmt::hacef841ef98e28f3
  11:     0x7f905178bc03 - core::result::unwrap_failed::heca1ed5f139fb358
  12:     0x7f905216a8de - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  13:     0x7f905467c5b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  14:     0x7f905467be41 - rustc_errors::Handler::emit_diagnostic::h1fdf1708c3ce7ac6
  15:     0x7f9051ff4640 - rustc_driver::report_ice::he956548a9b05d3f4
  16:     0x7f905171fbc2 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
  17:     0x7f90546ac464 - std::panicking::begin_panic::{{closure}}::h2d50bfdc71b22e40
  18:     0x7f90546ac2fc - std::sys_common::backtrace::__rust_end_short_backtrace::h037f8c7a07f27f2e
  19:     0x7f90546ac41c - std::panicking::begin_panic::h1edb7edaf4f6e057
  20:     0x7f905467c98e - rustc_errors::HandlerInner::emit_diagnostic::{{closure}}::hfcdcb2cf15991544
  21:     0x7f905216a87f - rustc_interface::callbacks::track_diagnostic::h3042dab8dcd87804
  22:     0x7f905467c5b5 - rustc_errors::HandlerInner::emit_diagnostic::h72a79c726e8dbdd4
  23:     0x7f9054374751 - rustc_errors::HandlerInner::emit_diag_at_span::h4fdc1222d9bd0c97
  24:     0x7f90543747b0 - rustc_errors::HandlerInner::span_bug::hdd4d5e678185daf9
  25:     0x7f9054375a29 - rustc_session::session::Session::delay_span_bug::h7984af9ed13a8d9d
  26:     0x7f905434da13 - rustc_middle::util::bug::trigger_delay_span_bug::h53cee4c080cf01ff
  27:     0x7f905303bd30 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hab2aaead76c48857
  28:     0x7f9052de3edf - rustc_data_structures::stack::ensure_sufficient_stack::h9a2581eeab1e6f02
  29:     0x7f9052c36c3d - rustc_query_system::query::plumbing::force_query_with_job::h0d4ba1ee51e4c8d4
  30:     0x7f9052bc0af9 - rustc_query_system::query::plumbing::get_query_impl::h8f73846870238e71
  31:     0x7f9052d068e9 - rustc_query_system::query::plumbing::get_query::hafe32d3862aa2384
  32:     0x7f9052158b7e - rustc_interface::queries::Queries::ongoing_codegen::hfe7ea4c77b0f5162
  33:     0x7f905200d0a8 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb6e9ea0d60143445
  34:     0x7f9051ff6280 - rustc_span::with_source_map::heac904d2395b36aa
  35:     0x7f905200b74c - scoped_tls::ScopedKey<T>::set::hb157d02c6e1d9ed1
  36:     0x7f9051ff68f7 - rustc_span::with_session_globals::h90562a4355eeae7e
  37:     0x7f905200eb82 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf4e52f73a463d956
  38:     0x7f905200ee86 - std::panicking::try::hf44e1134f75332f0
  39:     0x7f905203150a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6b0a9da6be2956a8
  40:     0x7f905172fd3a - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  41:     0x7f904c3e66db - start_thread
  42:     0x7f90513b271f - __clone
  43:                0x0 - <unknown>

thread panicked while processing panic. aborting.
------------------------------------------



---
test result: FAILED. 11712 passed; 3 failed; 97 ignored; 0 measured; 0 filtered out; finished in 126.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:08
