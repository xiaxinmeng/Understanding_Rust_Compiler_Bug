
warning: variable does not need to be mutable
 --> src/main.rs:5:25
  |
5 |         .map(|(mut old, mut new)| {
  |                         ----^^^
  |                         |
  |                         help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default

warning: variable does not need to be mutable
 --> src/main.rs:5:16
  |
5 |         .map(|(mut old, mut new)| {
  |                ----^^^
  |                |
  |                help: remove this `mut`

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: (MoveData { move_paths: [MovePath { place: _0 }, MovePath { place: _1 }, MovePath { place: _2 }, MovePath { place: _3 }, MovePath { place: _4 }, MovePath { place: _5 }, MovePath { place: _6 }], moves: [mp4@bb0[5], mp3@bb0[7], mp5@bb0[8], mp6@bb0[8], mp6@bb0[9], mp3@bb2[0], mp4@bb3[0], mp5@bb4[0], mp6@bb5[0], mp5@bb5[1], mp5@bb6[0], mp4@bb6[1], mp4@bb7[0], mp3@bb7[1], mp3@bb8[0], mp0@bb8[1]], loc_map: LocationMap { map: [[[], [], [], [], [], [mo0], [], [mo1], [mo2, mo3], [mo4]], [[]], [[mo5]], [[mo6]], [[mo7]], [[mo8], [mo9]], [[mo10], [mo11]], [[mo12], [mo13]], [[mo14], [mo15]]] }, path_map: [[mo15], [], [], [mo1, mo5, mo13, mo14], [mo0, mo6, mo11, mo12], [mo2, mo7, mo9, mo10], [mo3, mo4, mo8]], rev_lookup: MovePathLookup { locals: [mp0, mp1, mp2, mp3, mp4, mp5, mp6], projections: {} }, inits: [mp1@src/main.rs:5:14: 7:10 (Deep), mp2@src/main.rs:5:15: 5:33 (Deep), mp3@src/main.rs:5:16: 5:23 (Deep), mp4@src/main.rs:5:25: 5:32 (Deep), mp5@src/main.rs:6:14: 6:17 (Deep), mp6@src/main.rs:6:19: 6:22 (Deep), mp0@src/main.rs:6:13: 6:23 (Deep)], init_loc_map: LocationMap { map: [[[], [in2], [], [in3], [], [in4], [], [in5], [in6], []], [[]], [[]], [[]], [[]], [[], []], [[], []], [[], []], [[], []]] }, init_path_map: [[in6], [in0], [in1], [in2], [in3], [in4], [in5]] }, [IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/main.rs:5:16: 5:23, kind: BorrowedContent } }, IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/main.rs:5:25: 5:32, kind: BorrowedContent } }])', libcore/result.rs:945:5
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
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:350
   7: rust_begin_unwind
             at libstd/panicking.rs:328
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:71
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
  20: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir
  21: rustc_mir::monomorphize::collector::collect_items_rec
  22: rustc_mir::monomorphize::collector::collect_items_rec
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_mir::monomorphize::collector::collect_crate_mono_items
  32: rustc::util::common::time
  33: rustc_codegen_llvm::base::collect_and_partition_mono_items
  34: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::collect_and_partition_mono_items<'tcx>>::compute
  35: rustc::ty::context::tls::with_context
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::context::tls::with_related_context
  38: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  39: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  40: rustc_codegen_llvm::base::codegen_crate
  41: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  42: rustc::util::common::time
  43: rustc_driver::driver::phase_4_codegen
  44: rustc_driver::driver::compile_input::{{closure}}
  45: rustc::ty::context::tls::enter_context
  46: <std::thread::local::LocalKey<T>>::with
  47: rustc::ty::context::TyCtxt::create_and_enter
  48: rustc_driver::driver::compile_input
  49: rustc_driver::run_compiler_with_pool
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
#0 [optimized_mir] processing `main::{{closure}}`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (5bf68db6e 2018-05-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
