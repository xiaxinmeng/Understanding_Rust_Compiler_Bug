
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-dev (9b544108c 2017-08-16) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: !infcx.is_in_snapshot()', /src/src/librustc/traits/fulfill.rs:178:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /src/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /src/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /src/src/libstd/sys_common/backtrace.rs:60
             at /src/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /src/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /src/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /src/src/libstd/panicking.rs:572
   6: rustc::traits::fulfill::FulfillmentContext::register_predicate_obligation
             at /src/src/librustc/traits/fulfill.rs:178
   7: rustc_typeck::check::Inherited::register_predicate
             at /src/src/librustc_typeck/check/mod.rs:629
   8: rustc_typeck::check::FnCtxt::check_expr_kind
             at /src/src/librustc_typeck/check/mod.rs:637
             at /src/src/librustc_typeck/check/mod.rs:642
             at /src/src/librustc_typeck/check/coercion.rs:810
             at /src/src/libcore/result.rs:458
             at /src/src/librustc_typeck/check/coercion.rs:808
             at /src/src/librustc/infer/mod.rs:805
             at /src/src/librustc_typeck/check/coercion.rs:807
             at /src/src/librustc_typeck/check/coercion.rs:1119
             at /src/src/librustc_typeck/check/coercion.rs:1040
             at /src/src/librustc_typeck/check/mod.rs:3870
   9: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at /src/src/librustc_typeck/check/mod.rs:3438
  10: rustc_typeck::check::FnCtxt::check_expr_struct_fields
             at /src/src/librustc_typeck/check/mod.rs:2718
             at /src/src/librustc_typeck/check/mod.rs:2712
             at /src/src/librustc_typeck/check/mod.rs:2705
             at /src/src/librustc_typeck/check/mod.rs:3257
  11: rustc_typeck::check::FnCtxt::check_expr_kind
             at /src/src/librustc_typeck/check/mod.rs:3386
             at /src/src/librustc_typeck/check/mod.rs:3947
  12: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at /src/src/librustc_typeck/check/mod.rs:3438
  13: rustc_typeck::check::FnCtxt::check_decl_local
             at /src/src/librustc_typeck/check/mod.rs:2718
             at /src/src/librustc_typeck/check/mod.rs:2712
             at /src/src/librustc_typeck/check/mod.rs:2705
             at /src/src/librustc_typeck/check/mod.rs:4100
             at /src/src/librustc_typeck/check/mod.rs:4109
  14: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
             at /src/src/librustc_typeck/check/mod.rs:4148
             at /src/src/librustc_typeck/check/mod.rs:4222
  15: rustc_typeck::check::FnCtxt::check_block_with_expected
             at /src/src/librustc_typeck/check/mod.rs:4889
             at /src/src/librustc_typeck/check/mod.rs:4220
  16: rustc_typeck::check::FnCtxt::check_expr_kind
             at /src/src/librustc_typeck/check/mod.rs:3815
  17: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at /src/src/librustc_typeck/check/mod.rs:3438
  18: rustc_typeck::check::FnCtxt::check_return_expr
             at /src/src/librustc_typeck/check/mod.rs:2718
             at /src/src/librustc_typeck/check/mod.rs:2712
             at /src/src/librustc_typeck/check/mod.rs:2852
  19: rustc_typeck::check::check_fn
             at /src/src/librustc_typeck/check/mod.rs:1034
  20: rustc_typeck::check::typeck_tables_of::{{closure}}
             at /src/src/librustc_typeck/check/mod.rs:868
  21: rustc_typeck::check::typeck_tables_of
             at /src/src/librustc_typeck/check/mod.rs:595
             at /src/src/librustc/infer/mod.rs:375
             at /src/src/librustc/ty/context.rs:1262
             at /src/src/libstd/thread/local.rs:365
             at /src/src/libstd/thread/local.rs:279
             at /src/src/librustc/ty/context.rs:1259
             at /src/src/librustc/ty/context.rs:1073
             at /src/src/librustc/infer/mod.rs:375
             at /src/src/librustc_typeck/check/mod.rs:595
             at /src/src/librustc_typeck/check/mod.rs:852
  22: rustc::dep_graph::graph::DepGraph::with_task
             at /src/src/librustc/ty/maps.rs:609
             at /src/src/librustc/dep_graph/graph.rs:125
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
             at /src/src/librustc/ty/maps.rs:612
             at /src/src/librustc/ty/maps.rs:257
             at /src/src/librustc/ty/maps.rs:596
             at /src/src/librustc/ty/maps.rs:629
  24: rustc::ty::maps::TyCtxtAt::typeck_tables_of
             at /src/src/librustc/ty/maps.rs:675
  25: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
             at /src/src/librustc/ty/maps.rs:668
  26: rustc_typeck::check::typeck_item_bodies
             at /src/src/librustc_typeck/check/mod.rs:726
             at /src/src/librustc/session/mod.rs:270
             at /src/src/librustc_typeck/check/mod.rs:724
  27: rustc::dep_graph::graph::DepGraph::with_task
             at /src/src/librustc/ty/maps.rs:609
             at /src/src/librustc/dep_graph/graph.rs:125
  28: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
             at /src/src/librustc/ty/maps.rs:612
             at /src/src/librustc/ty/maps.rs:257
             at /src/src/librustc/ty/maps.rs:596
             at /src/src/librustc/ty/maps.rs:629
  29: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
             at /src/src/librustc/ty/maps.rs:675
  30: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
             at /src/src/librustc/ty/maps.rs:668
  31: rustc_typeck::check_crate
             at /src/src/librustc_typeck/check/mod.rs:717
             at /src/src/librustc_typeck/lib.rs:327
             at /src/src/librustc/util/common.rs:48
             at /src/src/librustc_typeck/lib.rs:327
  32: rustc::ty::context::TyCtxt::create_and_enter
             at /src/src/librustc_driver/driver.rs:1036
             at /src/src/librustc/ty/context.rs:1262
             at /src/src/libstd/thread/local.rs:365
             at /src/src/libstd/thread/local.rs:279
             at /src/src/librustc/ty/context.rs:1259
             at /src/src/librustc/ty/context.rs:1246
             at /src/src/libstd/thread/local.rs:365
             at /src/src/libstd/thread/local.rs:279
             at /src/src/librustc/ty/context.rs:1243
             at /src/src/librustc/ty/context.rs:1024
  33: rustc_driver::driver::compile_input
             at /src/src/librustc_driver/driver.rs:1005
             at /src/src/librustc_driver/driver.rs:196
  34: rustc_driver::run_compiler
