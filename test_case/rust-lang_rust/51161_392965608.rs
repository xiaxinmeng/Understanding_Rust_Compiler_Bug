
warning: variable does not need to be mutable
 --> src/main.rs:7:9
  |
7 |     let mut changes: BTreeMap<PackageId, (Vec<PackageId>, Vec<PackageId>)> = BTreeMap::new();
  |         ----^^^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default

thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: (MoveData { move_paths: [MovePath { place: _0 }, MovePath { place: _1 }, MovePath { place: _2 }, MovePath { place: _3 }, MovePath { place: _4 }, MovePath { place: _5 }, MovePath { place: _6 }, MovePath { place: _7 }, MovePath { place: _8 }, MovePath { place: _9 }, MovePath { place: _10 }, MovePath { place: _11 }, MovePath { place: _12 }, MovePath { place: _13 }, MovePath { place: _14 }], moves: [mp8@bb0[8], mp8@bb1[1], mp6@bb1[2], mp3@bb3[0], mp4@bb4[0], mp6@bb5[0], mp7@bb5[1], mp12@bb5[6], mp12@bb6[1], mp10@bb6[2], mp10@bb7[0], mp11@bb7[1], mp4@bb7[3], mp3@bb7[5], mp13@bb7[6], mp14@bb7[6], mp14@bb7[7], mp13@bb8[0], mp14@bb9[0], mp13@bb9[1], mp13@bb10[0], mp4@bb10[1], mp4@bb11[0], mp3@bb11[1], mp3@bb12[0], mp0@bb12[1]], loc_map: LocationMap { map: [[[], [], [], [], [], [], [], [], [mo0]], [[], [mo1], [mo2]], [[]], [[mo3]], [[mo4]], [[mo5], [mo6], [], [], [], [], [mo7]], [[], [mo8], [mo9]], [[mo10], [mo11], [], [mo12], [], [mo13], [mo14, mo15], [mo16]], [[mo17]], [[mo18], [mo19]], [[mo20], [mo21]], [[mo22], [mo23]], [[mo24], [mo25]]] }, path_map: [[mo25], [], [], [mo3, mo13, mo23, mo24], [mo4, mo12, mo21, mo22], [], [mo2, mo5], [mo6], [mo0, mo1], [], [mo9, mo10], [mo11], [mo7, mo8], [mo14, mo17, mo19, mo20], [mo15, mo16, mo18]], rev_lookup: MovePathLookup { locals: [mp0, mp1, mp2, mp3, mp4, mp5, mp6, mp7, mp8, mp9, mp10, mp11, mp12, mp13, mp14], projections: {} }, inits: [mp1@src/main.rs:12:14: 16:10 (Deep), mp2@src/main.rs:12:15: 12:33 (Deep), mp3@src/main.rs:12:16: 12:23 (Deep), mp4@src/main.rs:12:25: 12:32 (Deep), mp8@src/main.rs:13:13: 13:16 (Deep), mp7@src/main.rs:13:13: 13:16 (NonPanicPathOnly), mp6@src/main.rs:13:13: 13:16 (Deep), mp5@src/main.rs:13:13: 13:23 (NonPanicPathOnly), mp12@src/main.rs:14:13: 14:16 (Deep), mp11@src/main.rs:14:13: 14:16 (NonPanicPathOnly), mp10@src/main.rs:14:13: 14:16 (Deep), mp9@src/main.rs:14:13: 14:23 (NonPanicPathOnly), mp13@src/main.rs:15:14: 15:17 (Deep), mp14@src/main.rs:15:19: 15:22 (Deep), mp0@src/main.rs:15:13: 15:23 (Deep)], init_loc_map: LocationMap { map: [[[], [in2], [], [in3], [], [], [], [in4], [in5]], [[in6], [], [in7]], [[]], [[]], [[]], [[], [], [], [], [], [in8], [in9]], [[in10], [], [in11]], [[], [], [], [in12], [], [in13], [in14], []], [[]], [[], []], [[], []], [[], []], [[], []]] }, init_path_map: [[in14], [in0], [in1], [in2], [in3], [in7], [in6], [in5], [in4], [in11], [in10], [in9], [in8], [in12], [in13]] }, [IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/main.rs:12:16: 12:23, kind: BorrowedContent } }, IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/main.rs:12:25: 12:32, kind: BorrowedContent } }])', libcore/result.rs:945:5
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
  29: rustc_mir::monomorphize::collector::collect_crate_mono_items
  30: rustc::util::common::time
  31: rustc_codegen_llvm::base::collect_and_partition_mono_items
  32: rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::collect_and_partition_mono_items<'tcx>>::compute
  33: rustc::ty::context::tls::with_context
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::context::tls::with_related_context
  36: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  37: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  38: rustc_codegen_llvm::base::codegen_crate
  39: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  40: rustc::util::common::time
  41: rustc_driver::driver::phase_4_codegen
  42: rustc_driver::driver::compile_input::{{closure}}
  43: rustc::ty::context::tls::enter_context
  44: <std::thread::local::LocalKey<T>>::with
  45: rustc::ty::context::TyCtxt::create_and_enter
  46: rustc_driver::driver::compile_input
  47: rustc_driver::run_compiler_with_pool
  48: syntax::with_globals
  49: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  50: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  51: rustc_driver::run
  52: rustc_driver::main
  53: std::rt::lang_start::{{closure}}
  54: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  55: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  56: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:374
             at libstd/rt.rs:58
  57: main
  58: __libc_start_main
  59: <unknown>
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
