
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'no entry found for key', ../src/libcore/option.rs:332

stack backtrace:
   1:        0x10a9d7300 - sys::backtrace::tracing::imp::write::h5eae06ee4288506cVks
   2:        0x10a9df2db - panicking::on_panic::hf2970156b8548126NXw
   3:        0x10a9a15d2 - rt::unwind::begin_unwind_inner::h5c58ec90c1a4644c0sw
   4:        0x10a9a235d - rt::unwind::begin_unwind_fmt::h5a6da7c1a29307d36rw
   5:        0x10a9deee7 - rust_begin_unwind
   6:        0x10aa227a0 - panicking::panic_fmt::h272a520f43a4257f8PE
   7:        0x1079c8f1c - middle::region::RegionMaps::lookup_code_extent::h6bc6326f95c7df2fRzM
   8:        0x10788984a - middle::region::RegionMaps::node_extent::hedfc3700829896647zM
   9:        0x106f6bc02 - check::regionck::visit_expr::hc059d49cb8dfc24fUXc
  10:        0x106f67521 - check::regionck::regionck_expr::h7bd2e031449a87a8Boc
  11:        0x1070041c7 - check::CheckItemTypesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_ty::h592633d83bc845bctrp
  12:        0x107003ebe - visit::walk_path::h6693029354764006395
  13:        0x1070051e6 - visit::walk_generics::h12942862391973452724
  14:        0x107005064 - visit::Visitor::visit_fn::h5767647039744146314
  15:        0x1070038e8 - visit::walk_item::h6743178770303995216
  16:        0x1070c30cc - check_crate::h995cbc2d16b2670dIEE
  17:        0x106cc2e51 - driver::phase_3_run_analysis_passes::closure.20693
  18:        0x106ca499a - middle::ty::ctxt<'tcx>::create_and_enter::h8328830853489448585
  19:        0x106ca0043 - driver::phase_3_run_analysis_passes::h5251239526943436491
  20:        0x106c847a4 - driver::compile_input::h978de80e162cc6e2Tba
  21:        0x106de21b0 - run_compiler::hca19a948e49f9a000bc
  22:        0x106ddfaa9 - boxed::F.FnBox<A>::call_box::h16706733663139354280
  23:        0x106ddf4a2 - rt::unwind::try::try_fn::h6113644049204425945
  24:        0x10a9dee98 - __rust_try
  25:        0x10a9cb670 - rt::unwind::try::inner_try::h445685aa67c6bfc0Tow
  26:        0x106ddf652 - boxed::F.FnBox<A>::call_box::h11971768399365057599
  27:        0x10a9de20d - sys::thread::Thread::new::thread_start::h2921c73b7c72e024BNv
  28:     0x7fff847b9059 - _pthread_body
  29:     0x7fff847b8fd6 - _pthread_start
