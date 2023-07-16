
thread 'rustc' panicked at 'non-FnLike node found: NodeVariant(Spanned { node: Variant_ { name: Bar, attrs: [], data: Tuple([StructField { span: src/main.rs:2:9: 2:13, name: 0, vis: Inherited, id: NodeId(6), ty: type(i32), attrs: [] }], NodeId(8)), disr_expr: None }, span: src/main.rs:2:5: 2:13 })', librustc/traits/error_reporting.rs:873:18
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
             at libstd/panicking.rs:537
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:521                                                                                                                                                                                                      
   7: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_selection_error                                                                                                                                 
   8: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_errors                                                                                                                              
   9: rustc_typeck::check::FnCtxt::select_obligations_where_possible                                                                                                                                                                         
  10: rustc_typeck::check::FnCtxt::check_argument_types                                                                                                                                                                                      
  11: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call                                                                                                                                  
  12: rustc_typeck::check::FnCtxt::check_expr_kind                                                                                                                                                                                           
  13: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs                                                                                                                                                                     
  14: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}                                                                                                                                                                    
  15: rustc_typeck::check::FnCtxt::check_block_with_expected                                                                                                                                                                                 
  16: rustc_typeck::check::FnCtxt::check_expr_kind                                                                                                                                                                                           
  17: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs                                                                                                                                                                     
  18: rustc_typeck::check::FnCtxt::check_return_expr                                                                                                                                                                                         
  19: rustc_typeck::check::check_fn                                                                                                                                                                                                          
  20: rustc_typeck::check::typeck_tables_of::{{closure}}
  21: rustc_typeck::check::typeck_tables_of
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  25: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  26: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  27: rustc_typeck::check::typeck_item_bodies
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
  30: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  31: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  32: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  33: rustc_typeck::check_crate
  34: rustc::ty::context::TyCtxt::create_and_enter
  35: rustc_driver::driver::compile_input
  36: rustc_driver::run_compiler

error: internal compiler error: unexpected panic
