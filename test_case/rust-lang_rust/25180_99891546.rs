
thread 'rustc' panicked at 'assertion failed: self.mode == Mode::Var', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/middle/check_const.rs:256

stack backtrace:
   1:     0x7f53daf5a099 - sys::backtrace::write::h5f9c68bafc81b9cffhs
   2:     0x7f53daf61fe9 - panicking::on_panic::h373abe6a65309d56eXw
   3:     0x7f53daf22202 - rt::unwind::begin_unwind_inner::h5188f63ed8e7a8f4oCw
   4:     0x7f53d8dc7bae - rt::unwind::begin_unwind::h17431745025147829990
   5:     0x7f53d8f10412 - middle::check_const::CheckCrateVisitor<'a, 'tcx>.Visitor<'v>::visit_fn::h3de92c228569f0fdoQd
   6:     0x7f53d8f1339b - visit::walk_expr::h15305561305326812834
   7:     0x7f53d8f0c168 - middle::check_const::CheckCrateVisitor<'a, 'tcx>.Visitor<'v>::visit_expr::hd1fdf4a4113a51f0rSd
   8:     0x7f53d8f0c168 - middle::check_const::CheckCrateVisitor<'a, 'tcx>.Visitor<'v>::visit_expr::hd1fdf4a4113a51f0rSd
   9:     0x7f53d8ef7100 - middle::check_const::CheckCrateVisitor<'a, 'tcx>::global_expr::h7ceeeec8c2ba1122RFd
  10:     0x7f53d8f17aba - middle::check_const::check_crate::hefefaa79e1429bedoke
  11:     0x7f53db4a8af7 - driver::phase_3_run_analysis_passes::h1f752cba4a23265ftGa
  12:     0x7f53db489eec - driver::compile_input::h4db3fe53fca6bc45Qba
  13:     0x7f53db542b31 - run_compiler::haed29ba6b8e4f89065b
  14:     0x7f53db540382 - boxed::F.FnBox<A>::call_box::h454497668228412666
  15:     0x7f53db53f949 - rt::unwind::try::try_fn::h14350681364391317498
  16:     0x7f53dafd5a78 - rust_try_inner
  17:     0x7f53dafd5a65 - rust_try
  18:     0x7f53db53fbe4 - boxed::F.FnBox<A>::call_box::h1654447988124325378
  19:     0x7f53daf60d81 - sys::thread::Thread::new::thread_start::he88a7f923caab202KIv
  20:     0x7f53d4fec0a4 - start_thread
  21:     0x7f53daba5cfc - clone
  22:                0x0 - <unknown>
