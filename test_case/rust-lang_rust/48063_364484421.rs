
error: internal compiler error: librustc/ty/layout.rs:2184: TyLayout::field_type(TyLayout { ty: i16, details: LayoutDetails { variants: Single { index: 0 }, fields: Union(0), abi: Scalar(Scalar { value: Int(I16, true), valid_range: 0..=65535 }), align: Align { abi: 1, pref: 1 }, size: Size { raw: 2 } } }): not applicable

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:535:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-apple-darwin

[08:30:53] मanishearth@ligo ~/mozilla/sand O_o
$ RUST_BACKTRACE=1 rustc +x1 test.rs
error: internal compiler error: librustc/ty/layout.rs:2184: TyLayout::field_type(TyLayout { ty: i16, details: LayoutDetails { variants: Single { index: 0 }, fields: Union(0), abi: Scalar(Scalar { value: Int(I16, true), valid_range: 0..=65535 }), align: Align { abi: 1, pref: 1 }, size: Size { raw: 2 } } }): not applicable

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:535:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: rustc::session::opt_span_bug_fmt
  10: rustc::session::bug_fmt
  11: rustc::ty::layout::TyLayout::field
  12: rustc_trans::mir::place::PlaceRef::project_field
  13: rustc_trans::mir::place::<impl rustc_trans::mir::FunctionCx<'a, 'tcx>>::trans_place
  14: rustc_trans::mir::rvalue::<impl rustc_trans::mir::FunctionCx<'a, 'tcx>>::trans_rvalue_operand
  15: rustc_trans::mir::trans_mir
  16: rustc_trans::base::trans_instance
  17: rustc_trans::trans_item::MonoItemExt::define
  18: rustc_trans::base::compile_codegen_unit
  19: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute_result
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc_errors::Handler::track_diagnostics
  22: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::force
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::try_get
  25: rustc::ty::maps::TyCtxtAt::compile_codegen_unit
  26: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit
  27: rustc_trans::base::trans_crate
  28: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  29: rustc::util::common::time
  30: rustc_driver::driver::phase_4_translate_to_llvm
  31: rustc_driver::driver::compile_input::{{closure}}
  32: <std::thread::local::LocalKey<T>>::with
