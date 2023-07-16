
error[E0499]: cannot borrow `my_struct.field` as mutable more than once at a time
  --> ice.rs:15:9
   |
13 |     let value = &mut my_struct.field;
   |                      --------------- first mutable borrow occurs here
14 |     loop {
15 |         my_struct.field.push_str("Hello, world!");
   |         ^^^^^^^^^^^^^^^ second mutable borrow occurs here
...
19 | }
   | - first borrow ends here

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'end region not found for ReScope(Remainder(BlockRemainder { block: ItemLocalId(34), first_statement_index: 1 }))', rust://src/librustc_mir/dataflow/impls/borrows.rs:121:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at rust://src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at rust://src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at rust://src/libstd/sys_common/backtrace.rs:60
             at rust://src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at rust://src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at rust://src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at rust://src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at rust://src/libstd/panicking.rs:522
   7: rustc_mir::dataflow::impls::borrows::Borrows::region_span
             at rust://src/librustc_mir/dataflow/impls/borrows.rs:121
   8: rustc_mir::borrow_check::MirBorrowckCtxt::access_lvalue::{{closure}}
             at rust://src/libcore/ptr.rs:0
   9: rustc_mir::borrow_check::MirBorrowckCtxt::access_lvalue
             at rust://src/librustc_mir/borrow_check.rs:738
             at rust://src/librustc_mir/borrow_check.rs:387
  10: <rustc_mir::borrow_check::MirBorrowckCtxt<'c, 'b, 'a, 'gcx, 'tcx> as rustc_mir::dataflow::DataflowResultsConsumer<'b, 'gcx>>::visit_statement_entry
             at rust://src/librustc_mir/borrow_check.rs:468
             at rust://src/librustc_mir/borrow_check.rs:184
  11: rustc_mir::borrow_check::mir_borrowck::{{closure}}
             at rust://src/librustc_mir/dataflow/mod.rs:309
             at rust://src/librustc_mir/dataflow/mod.rs:299
             at rust://src/librustc_mir/borrow_check.rs:88
  12: rustc_mir::borrow_check::mir_borrowck
             at rust://src/librustc/infer/mod.rs:375
             at rust://src/librustc/ty/context.rs:1477
             at rust://src/libstd/thread/local.rs:379
             at rust://src/libstd/thread/local.rs:293
             at rust://src/librustc/ty/context.rs:1474
             at rust://src/librustc/ty/context.rs:1285
             at rust://src/librustc/infer/mod.rs:375
             at rust://src/librustc_mir/borrow_check.rs:60
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result
             at rust://src/librustc/ty/maps/plumbing.rs:349
  14: rustc::dep_graph::graph::DepGraph::with_task
             at rust://src/librustc/dep_graph/graph.rs:272
  15: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
             at rust://src/librustc/ty/maps/plumbing.rs:395
             at rust://src/librustc_errors/lib.rs:555
             at rust://src/librustc/ty/maps/plumbing.rs:394
             at rust://src/librustc/ty/maps/plumbing.rs:130
             at rust://src/librustc/ty/maps/plumbing.rs:393
  16: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
             at rust://src/librustc/ty/maps/plumbing.rs:279
             at rust://src/librustc/ty/maps/plumbing.rs:423
  17: rustc::ty::maps::TyCtxtAt::mir_borrowck
             at rust://src/librustc/ty/maps/plumbing.rs:462
  18: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
             at rust://src/librustc/ty/maps/plumbing.rs:455
  19: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
             at rust://src/librustc_driver/driver.rs:1099
  20: rustc::ty::context::TyCtxt::create_and_enter
             at rust://src/librustc/util/common.rs:120
             at rust://src/librustc_driver/driver.rs:1097
             at rust://src/librustc/ty/context.rs:1477
             at rust://src/libstd/thread/local.rs:379
             at rust://src/libstd/thread/local.rs:293
             at rust://src/librustc/ty/context.rs:1474
             at rust://src/librustc/ty/context.rs:1461
             at rust://src/libstd/thread/local.rs:379
             at rust://src/libstd/thread/local.rs:293
             at rust://src/librustc/ty/context.rs:1458
             at rust://src/librustc/ty/context.rs:1118
  21: rustc_driver::driver::compile_input
             at rust://src/librustc_driver/driver.rs:1045
             at rust://src/librustc_driver/driver.rs:211
  22: rustc_driver::run_compiler
             at rust://src/librustc_driver/lib.rs:250
