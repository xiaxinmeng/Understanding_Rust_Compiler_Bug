
> RUST_BACKTRACE=1 cargo +stable test
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: (MoveData { move_paths: [MovePath { place: _0 }, MovePath { place: _1 }, MovePath { place: _2 }, MovePath { place: _3 }, MovePath { place: _4 }, MovePath { place: _5 }, MovePath { place: _6 }, MovePath { place: _7 }, MovePath { place: _8 }, MovePath { place: _9 }, MovePath { place: _10 }], moves: [mp7@bb0[7], mp7@bb2[0], mp6@bb2[5], mp8@bb2[5], mp3@bb3[0], mp8@bb4[0], mp6@bb4[1], mp6@bb5[0], mp6@bb6[0], mp5@bb7[0], mp10@bb7[0], mp10@bb7[1], mp5@bb7[2], mp4@bb7[3], mp4@bb7[4], mp9@bb7[5], mp3@bb7[6], mp3@bb8[0], mp0@bb8[1]], loc_map: LocationMap { map: [[[], [], [], [], [], [], [], [mo0]], [[]], [[mo1], [], [], [], [], [mo2, mo3]], [[mo4]], [[mo5], [mo6]], [[mo7]], [[mo8], [], []], [[mo9, mo10], [mo11], [mo12], [mo13], [mo14], [mo15], [mo16]], [[mo17], [mo18]]] }, path_map: [[mo18], [], [], [mo4, mo16, mo17], [mo13, mo14], [mo9, mo12], [mo2, mo6, mo7, mo8], [mo0, mo1], [mo3, mo5], [mo15], [mo10, mo11]], rev_lookup: MovePathLookup { locals: [mp0, mp1, mp2, mp3, mp4, mp5, mp6, mp7, mp8, mp9, mp10], projections: {} }, inits: [mp1@src/circuit/drgporep/mod.rs:268:22: 276:18 (Deep), mp2@src/circuit/drgporep/mod.rs:268:23: 268:38 (Deep), mp3@src/circuit/drgporep/mod.rs:268:27: 268:37 (Deep), mp7@src/circuit/drgporep/mod.rs:270:32: 271:34 (Deep), mp6@src/circuit/drgporep/mod.rs:270:32: 272:56 (NonPanicPathOnly), mp9@src/circuit/drgporep/mod.rs:273:37: 273:80 (Deep), mp8@src/circuit/drgporep/mod.rs:273:37: 273:80 (Deep), mp5@src/circuit/drgporep/mod.rs:270:32: 273:81 (NonPanicPathOnly), mp10@src/circuit/drgporep/mod.rs:274:37: 274:59 (NonPanicPathOnly), mp4@src/circuit/drgporep/mod.rs:269:26: 275:22 (Deep), mp0@src/circuit/drgporep/mod.rs:269:21: 275:23 (Deep)], init_loc_map: LocationMap { map: [[[], [in2], [], [], [], [], [in3], [in4]], [[]], [[], [], [], [in5], [in6], [in7]], [[]], [[], []], [[]], [[], [], [in8]], [[in9], [], [], [in10], [], [], []], [[], []]] }, init_path_map: [[in10], [in0], [in1], [in2], [in9], [in7], [in4], [in3], [in6], [in5], [in8]] }, [IllegalMove { cannot_move_out_of: IllegalMoveOrigin { span: src/circuit/drgporep/mod.rs:268:27: 268:37, kind: BorrowedContent } }])', libcore/result.rs:945:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: <rustc_mir::transform::elaborate_drops::ElaborateDrops as rustc_mir::transform::MirPass>::run_pass
  11: rustc_mir::transform::optimized_mir::{{closure}}
  12: rustc_mir::transform::optimized_mir
  13: rustc::dep_graph::graph::DepGraph::with_task_impl
  14: rustc_errors::Handler::track_diagnostics
  15: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  16: rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::force
  17: rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get
  18: rustc::ty::maps::TyCtxtAt::optimized_mir
  19: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir
  20: rustc_mir::monomorphize::collector::collect_items_rec
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
  31: rustc_mir::monomorphize::collector::collect_items_rec
  32: rustc_mir::monomorphize::collector::collect_items_rec
  33: rustc_mir::monomorphize::collector::collect_crate_mono_items
  34: rustc::util::common::time
  35: rustc_trans::base::collect_and_partition_translation_items
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc_errors::Handler::track_diagnostics
  38: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  39: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force
  40: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get
  41: rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items
  42: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items
  43: rustc_trans::base::trans_crate
  44: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  45: rustc::util::common::time
  46: rustc_driver::driver::phase_4_translate_to_llvm
  47: rustc_driver::driver::compile_input::{{closure}}
  48: <std::thread::local::LocalKey<T>>::with
  49: <std::thread::local::LocalKey<T>>::with
  50: rustc::ty::context::TyCtxt::create_and_enter
  51: rustc_driver::driver::compile_input
  52: rustc_driver::run_compiler_impl
  53: syntax::with_globals

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.2 (594fb253c 2018-06-01) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `proofs`.

To learn more, run the command again with --verbose.
