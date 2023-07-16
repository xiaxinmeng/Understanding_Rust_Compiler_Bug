
---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----

error: compiler panicked
status: exit code: 101
command: "/Users/alex/Software/rust-devel/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/alex/Software/rust-devel/src/test/compile-fail/static-array-across-crate.rs" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/Users/alex/Software/rust-devel/build/x86_64-apple-darwin/test/compile-fail/static-array-across-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/alex/Software/rust-devel/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/alex/Software/rust-devel/build/x86_64-apple-darwin/test/compile-fail/static-array-across-crate/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::allocate_value
  11: rustc_mir::interpret::const_eval::eval_body_using_ecx
  12: rustc_mir::interpret::const_eval::eval_body_and_ecx
  13: rustc_mir::interpret::const_eval::const_eval_provider
  14: rustc::ty::maps::__query_compute::const_eval
  15: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::const_eval<'tcx>>::compute
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::context::tls::with_related_context
  18: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  19: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  20: rustc::ty::maps::TyCtxtAt::const_eval
  21: <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::get
  22: <rustc_mir::interpret::memory::Memory<'a, 'mir, 'tcx, M>>::check_align
  23: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::try_read_value
  24: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::write_value
  25: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_rvalue_into_place
  26: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::step
  27: rustc_mir::interpret::const_eval::eval_body_using_ecx
  28: rustc_mir::interpret::const_eval::eval_body_and_ecx
  29: rustc_mir::interpret::const_eval::const_eval_provider
  30: rustc::ty::maps::__query_compute::const_eval
  31: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::const_eval<'tcx>>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
  37: rustc_mir::monomorphize::collector::collect_items_rec
  38: rustc_mir::monomorphize::collector::collect_crate_mono_items
  39: rustc::util::common::time
  40: rustc_codegen_llvm::base::collect_and_partition_mono_items
  41: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::collect_and_partition_mono_items<'tcx>>::compute
  42: rustc::dep_graph::graph::DepGraph::with_task_impl
  43: rustc::ty::context::tls::with_related_context
  44: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  45: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  46: rustc_codegen_llvm::base::codegen_crate
  47: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  48: rustc::util::common::time
  49: rustc_driver::driver::phase_4_codegen
  50: rustc_driver::driver::compile_input::{{closure}}
  51: rustc::ty::context::tls::enter_context
  52: <std::thread::local::LocalKey<T>>::with
  53: rustc::ty::context::TyCtxt::create_and_enter
  54: rustc_driver::driver::compile_input
  55: rustc_driver::run_compiler_with_pool
  56: syntax::with_globals
  57: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  58: __rust_maybe_catch_panic
  59: std::panicking::try
  60: rustc_driver::run
  61: rustc_driver::main
  62: std::rt::lang_start::{{closure}}
  63: std::panicking::try::do_call
  64: __rust_maybe_catch_panic
  65: std::panic::catch_unwind
  66: std::rt::lang_start_internal
  67: main
query stack during panic:
{"message":"#0 [const_eval] const-evaluating `array::ARRAY`","code":null,"level":"","spans":[{"file_name":"/Users/alex/Software/rust-devel/src/test/compile-fail/static-array-across-crate.rs","byte_start":801,"byte_end":812,"line_start":23,"line_end":23,"column_start":16,"column_end":27,"is_primary":true,"text":[{"text":"static Z: u8 = (&ARRAY)[0];","highlight_start":16,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"#0 [const_eval] const-evaluating `array::ARRAY`\n  --> /Users/alex/Software/rust-devel/src/test/compile-fail/static-array-across-crate.rs:23:16\n   |\nLL | static Z: u8 = (&ARRAY)[0];\n   |                ^^^^^^^^^^^\n"}
{"message":"#1 [const_eval] const-evaluating `Z`","code":null,"level":"","spans":[{"file_name":"/Users/alex/Software/rust-devel/src/test/compile-fail/static-array-across-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2015 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"#1 [const_eval] const-evaluating `Z`\n"}
{"message":"#2 [collect_and_partition_mono_items] collect_and_partition_mono_items","code":null,"level":"","spans":[{"file_name":"/Users/alex/Software/rust-devel/src/test/compile-fail/static-array-across-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2015 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"#2 [collect_and_partition_mono_items] collect_and_partition_mono_items\n"}
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
