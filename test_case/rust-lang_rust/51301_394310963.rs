
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: (MoveData { move_paths: [MovePath { place: _0 }, MovePath { place: _1 }, MovePath { place: _2 }, MovePath { place: _3 }, MovePath { place: _4 }, MovePath { place: _5 }], moves: [mp4@bb0[6], mp5@bb0[6], mp5@bb2[0], mp4@bb2[1], mp3@bb2[2], mp3@bb3[0], mp3@bb4[0], mp0@bb4[1]], loc_map: LocationMap { map: [[[], [], [], [], [], [], [mo0, mo1]], [[]], [[mo2], [mo3], [mo4]], [[mo5]], [[mo6], [mo7]]] }, path_map: [[mo7], [], [], [mo4, mo5, mo6], [mo0, mo3], [mo1, mo2]], rev_lookup: MovePathLookup { locals: [mp0, mp1, mp2, mp3, mp4, mp5], projections: {} }, inits: [mp1@src/lib.rs:40:19: 40:57 (Deep), mp2@src/lib.rs:40:20: 40:36 (Deep), mp3@src/lib.rs:40:22: 40:32 (Deep), mp4@src/lib.rs:40:38: 40:43 (Deep), mp5@src/lib.rs:40:47: 40:57 (Deep), mp0@src/lib.rs:40:38: 40:57 (NonPanicPathOnly)], init_loc_map: LocationMap { map: [[[], [in2], [], [in3], [], [in4], [in5]], [[]], [[], [], []], [[]], [[], []]] }, init_path_map: [[in5], [in0], [in1], [in2], [in3], [in4]] }, [IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/lib.rs:40:22: 40:32, kind: BorrowedContent } }])', libcore/result.rs:945:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:515
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:426
   7: rust_begin_unwind
             at libstd/panicking.rs:337
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:92
   9: core::result::unwrap_failed
  10: <rustc_mir::transform::elaborate_drops::ElaborateDrops as rustc_mir::transform::MirPass>::run_pass
  11: rustc_mir::transform::optimized_mir::{{closure}}
  12: rustc_mir::transform::optimized_mir
  13: rustc::ty::maps::__query_compute::optimized_mir
  14: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::optimized_mir<'tcx>>::compute
  15: rustc::ty::context::tls::with_context
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::context::tls::with_related_context
  18: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  19: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::optimized_mir
  21: rustc_metadata::encoder::<impl rustc_metadata::isolated_encoder::IsolatedEncoder<'a, 'b, 'tcx>>::encode_optimized_mir
  22: rustc_metadata::encoder::<impl rustc_metadata::isolated_encoder::IsolatedEncoder<'a, 'b, 'tcx>>::encode_info_for_closure
  23: rustc::ty::context::tls::with_context
  24: rustc_metadata::encoder::<impl rustc_metadata::index_builder::IndexBuilder<'a, 'b, 'tcx>>::encode_info_for_expr
  25: rustc::hir::intravisit::walk_expr
  26: rustc::hir::intravisit::walk_expr
  27: rustc::hir::intravisit::walk_expr
  28: rustc::hir::intravisit::walk_expr
  29: rustc::hir::intravisit::walk_expr
  30: rustc::hir::intravisit::walk_local
  31: rustc::hir::intravisit::walk_expr
  32: rustc::hir::intravisit::Visitor::visit_nested_body
  33: rustc::hir::Crate::visit_all_item_likes
  34: rustc_metadata::encoder::encode_metadata
  35: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  36: rustc::ty::context::TyCtxt::encode_metadata
  37: rustc_codegen_llvm::base::write_metadata
  38: rustc::util::common::time
  39: rustc_codegen_llvm::base::codegen_crate
  40: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  41: rustc::util::common::time
  42: rustc_driver::driver::phase_4_codegen
  43: rustc_driver::driver::compile_input::{{closure}}
  44: rustc::ty::context::tls::enter_context
  45: <std::thread::local::LocalKey<T>>::with
  46: rustc::ty::context::TyCtxt::create_and_enter
  47: rustc_driver::driver::compile_input
  48: rustc_driver::run_compiler_with_pool
  49: <scoped_tls::ScopedKey<T>>::set
  50: syntax::with_globals
  51: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  52: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  53: rustc_driver::run
  54: rustc_driver::main
  55: std::rt::lang_start::{{closure}}
  56: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  57: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  58: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  59: main
  60: __libc_start_main
  61: <unknown>
query stack during panic:
#0 [optimized_mir] processing `<StateMachine<EventType>>::inner_process_event::{{closure}}`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (29f48ccf3 2018-06-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `tst`.