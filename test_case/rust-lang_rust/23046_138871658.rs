
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore/option.rs:362

stack backtrace:
   1:        0x1075d57b5 - sys::backtrace::write::hf5ea20500b66cd24uns
   2:        0x1075de013 - panicking::on_panic::hbe02cb0d925cad49iGw
   3:        0x107599dd2 - rt::unwind::begin_unwind_inner::h12ba0ba9dffdecc2uow
   4:        0x10759ab29 - rt::unwind::begin_unwind_fmt::hadf0dbf11d345ebfAnw
   5:        0x1075ddb9c - rust_begin_unwind
   6:        0x107631d95 - panicking::panic_fmt::h987a4890059dc6e0H8B
   7:        0x10762a7e1 - panicking::panic::hfd3e1c225039d9cae7B
   8:        0x104510932 - check::regionck::type_must_outlive::h5135304db321371czqe
   9:        0x10453e144 - check::regionck::type_of_node_must_outlive::h82fdc64a63ca862cOQd
  10:        0x10453c6fb - check::regionck::visit_expr::ha7c5f37838a301d2P8c
  11:        0x104539c0b - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::ha3a67b8e2a6eacbbALc
  12:        0x1045390b7 - check::regionck::regionck_fn::ha71fc7731cec857eWDc
  13:        0x1045b1272 - check::check_bare_fn::h88c035244660e365WMn
  14:        0x1045bcce7 - check::check_method_body::h069f1c1260fb3f85Qpo
  15:        0x1045aec47 - check::check_item_body::h9873e3da412bca20ydo
  16:        0x1045af212 - visit::walk_item::h2899112994373683709
  17:        0x1045b0bed - check::check_item_types::h63240bfbe991be87tKn
  18:        0x10466ecd9 - check_crate::h117ec0c1269afe619fD
  19:        0x103eb5d16 - driver::phase_3_run_analysis_passes::closure.15766
  20:        0x103eb4204 - middle::ty::with_ctxt::h14728011725879770170
  21:        0x103eaf00a - driver::phase_3_run_analysis_passes::h16713467199444562124
  22:        0x103e92107 - driver::compile_input::hb6d2be5b0fa2247fTba
  23:        0x103f6e13f - run_compiler::h21d74b88eec3fe3bx7b
  24:        0x103f6b9f3 - boxed::F.FnBox<A>::call_box::h1689969825914258414
  25:        0x103f6b1b7 - rt::unwind::try::try_fn::h11273853850686318048
  26:        0x107668cc8 - rust_try_inner
  27:        0x107668cb5 - rust_try
  28:        0x1075c7c95 - rt::unwind::try::inner_try::h480e3107f6a4b5b9nkw
  29:        0x103f6b3e8 - boxed::F.FnBox<A>::call_box::h888215220722514405
  30:        0x1075dca9d - sys::thread::Thread::new::thread_start::hdb3d925f69c5da4aHIv
  31:     0x7fff96b7d059 - _pthread_body
  32:     0x7fff96b7cfd6 - _pthread_start
