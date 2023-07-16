rust
error: internal compiler error: /home/simon/projects/rust1/src/librustc/hir/def.rs:139: attempted .def_id() on invalid def: Err

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /home/simon/projects/rust1/src/librustc_errors/lib.rs:423
stack backtrace:
   1:     0x7fc35e73fdf3 - std::sys::imp::backtrace::tracing::imp::write::hdfa3e34b856f69a3
   2:     0x7fc35e74e5dd - std::panicking::default_hook::{{closure}}::h20f8c1082e0ce86e
   3:     0x7fc35e74e17b - std::panicking::default_hook::hd31552ac08c2a241
   4:     0x7fc35e74ea68 - std::panicking::rust_panic_with_hook::h73969d230a072c1a
   5:     0x7fc357eda667 - std::panicking::begin_panic::hb89a2261f4776ce3
   6:     0x7fc357eef99f - rustc_errors::Handler::bug::h13b3b29154320abd
   7:     0x7fc35bab053a - rustc::session::opt_span_bug_fmt::{{closure}}::hfea18dd68f59976d
   8:     0x7fc35baafff7 - rustc::session::opt_span_bug_fmt::h7d370a0810dbdbbd
   9:     0x7fc35baafce2 - rustc::session::bug_fmt::hcde5f50c54004a4f
  10:     0x7fc35b984552 - rustc::hir::def::Def::def_id::h728c22aafbe84efc
  11:     0x7fc35c6ddb3a - rustc_privacy::ObsoleteVisiblePrivateTypesVisitor::path_is_private_type::h24756a49a68d1da9
  12:     0x7fc35c6d53db - rustc::hir::intravisit::walk_ty::h64b13c0c45b7a09a
  13:     0x7fc35c6d49ba - rustc::hir::intravisit::Visitor::visit_fn::ha8ad665c42b1d643
  14:     0x7fc35c6de7cf - <rustc_privacy::ObsoleteVisiblePrivateTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::hae9f32c7942ccca1
  15:     0x7fc35c6deab5 - <rustc_privacy::ObsoleteVisiblePrivateTypesVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::hae9f32c7942ccca1
  16:     0x7fc35c6e1b7a - rustc_privacy::check_crate::h5ba75e3c4ebe0690
  17:     0x7fc35eaef747 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h388b9236cf305089
  18:     0x7fc35ead66d6 - rustc_driver::driver::phase_3_run_analysis_passes::h080c1a734c4f2909
  19:     0x7fc35eac5425 - rustc_driver::driver::compile_input::hab55539b85c4ceed
  20:     0x7fc35eb0ea51 - rustc_driver::run_compiler::h128b89b79f649304
  21:     0x7fc35ea236fb - std::panicking::try::do_call::h15bca94dda232804
  22:     0x7fc35e757b06 - __rust_maybe_catch_panic
  23:     0x7fc35ea44d01 - <F as alloc::boxed::FnBox<A>>::call_box::h8832766c4bda585b
  24:     0x7fc35e74d470 - std::sys::imp::thread::Thread::new::thread_start::h7d03d727f96da002
  25:     0x7fc357684453 - start_thread
  26:     0x7fc35e4147de - __GI___clone
  27:                0x0 - <unknown>