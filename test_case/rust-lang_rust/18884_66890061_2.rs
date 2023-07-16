
RUST_BACKTRACE=1 rustc crash.rs                                                                                                          ⏎ ✱ ◼
crash.rs:1:32: 1:38 error: the type of this value must be known in this context
crash.rs:1 pub static PAF: *mut *mut i8 = 0 as _;
                                          ^~~~~~
error: internal compiler error: asked to compute contents of error type
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:188

stack backtrace:
   1:        0x10e05f5c0 - rt::backtrace::imp::write::h276c1d3da30d25d7bVx
   2:        0x10e0627d0 - failure::on_fail::h26104c4e5109e22dPhy
   3:        0x10e2b2905 - unwind::begin_unwind_inner::heffc9437c9480c1cFBc
   4:        0x10d7f4177 - unwind::begin_unwind::h13488892666725982622
   5:        0x10d7f4b2f - diagnostic::Handler::bug::h0a5c7d07e2e3a42b1IF
   6:        0x10b5d8b38 - session::Session::bug::h4abbf53dc914be36a0z
   7:        0x10b82b290 - middle::ty::type_contents::tc_ty::hdcb1cf32b32a10c2Bdg
   8:        0x10b69180c - middle::ty::type_contents::h2dfd3c492aaf3247Wcg
   9:        0x10b68e80e - middle::ty::type_is_sized::h256af3bc7fd76fac3ih
  10:        0x10ac058a7 - check::check_cast::h614540bb48331fa5Jpl
  11:        0x10ac3de35 - check::check_expr_with_unifier::h52c2f0998d5f7521EAn
  12:        0x10ac775f2 - check::check_const_with_ty::hb5bd039b892f6144gAp
  13:        0x10abed424 - check::check_item::hbb25f5231784013eaqk
  14:        0x10af85990 - check_crate::closure.43637
  15:        0x10af82234 - util::common::time::h4403700308301094385
  16:        0x10af8175a - check_crate::hb4737f877b7bdbdeDUy
  17:        0x10a9da809 - driver::phase_3_run_analysis_passes::h9956ae2c81e7288fCta
  18:        0x10a9c9777 - driver::compile_input::h07eb2d0e88b46326pba
  19:        0x10aa618ed - run_compiler::h657657a0aa19c050EYb
  20:        0x10aa6034e - run::closure.21518
  21:        0x10aa717ae - task::TaskBuilder::try_future::closure.22972
  22:        0x10e038833 - task::TaskBuilder::spawn_internal::closure.30639
  23:        0x10e2b050d - task::Task::spawn::closure.5568
  24:        0x10e317eac - rust_try_inner
  25:        0x10e317e96 - rust_try
  26:        0x10e2b05e7 - unwind::try::h9964c705e60747d7Wqc
  27:        0x10e2b03bc - task::Task::run::h976e36ba3d824167fIb
  28:        0x10e2b00bf - task::Task::spawn::closure.5544
  29:        0x10e2b19c7 - thread::thread_start::h436a5ebf6a0549c5wZb
  30:     0x7fff8f5ce2fc - _pthread_body
  31:     0x7fff8f5ce279 - _pthread_body
