
test.rs:5:9: 5:22 error: internal compiler error: break to unknown label
test.rs:5         break 'outer;
                  ^~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:121

stack backtrace:
   1:        0x10f5f0582 - rt::backtrace::imp::write::hbe96aaedb9f233e5Tgt
   2:        0x10f5f363d - failure::on_fail::h00a79ab4c301b861MCt
   3:        0x10f855d55 - unwind::begin_unwind_inner::h8917945a32f94139I2c
   4:        0x10d5e9827 - unwind::begin_unwind::h4934850694264909217
   5:        0x10d5e97c0 - unwind::begin_unwind::h4934850694264909217
   6:        0x10c61eaed - session::Session::span_bug::h9ddd28e9be98f23drt1
   7:        0x10c6f9668 - middle::liveness::Liveness<'a, 'tcx>::propagate_through_expr::h5e435a49904f6dc7Wtx
   8:        0x10c6fb0d0 - middle::liveness::Liveness<'a, 'tcx>::propagate_through_loop::closure.93735
   9:        0x10c6f91f8 - middle::liveness::Liveness<'a, 'tcx>::with_loop_nodes::h5123349144306321434
  10:        0x10c6fae18 - middle::liveness::Liveness<'a, 'tcx>::propagate_through_loop::haf8ebd0e0f942e44LQx
  11:        0x10c6f9b70 - middle::liveness::Liveness<'a, 'tcx>::propagate_through_expr::h5e435a49904f6dc7Wtx
  12:        0x10c6f9d65 - middle::liveness::Liveness<'a, 'tcx>::propagate_through_expr::h5e435a49904f6dc7Wtx
  13:        0x10c6f92b4 - middle::liveness::Liveness<'a, 'tcx>::compute::closure.93665
  14:        0x10c6f91f8 - middle::liveness::Liveness<'a, 'tcx>::with_loop_nodes::h5123349144306321434
  15:        0x10c6ed5bc - middle::liveness::visit_fn::hf74c7937fefd55a2Snw
  16:        0x10c6f5ef7 - visit::walk_expr::h15555431973934265096
  17:        0x10c6ee8bb - middle::liveness::visit_expr::h8d08d15138a5d6deLAw
  18:        0x10c6f5f67 - visit::walk_expr::h15555431973934265096
  19:        0x10c6ee397 - middle::liveness::visit_expr::h8d08d15138a5d6deLAw
  20:        0x10c6f4890 - visit::walk_block::h10136710710042042084
  21:        0x10c6ee397 - middle::liveness::visit_expr::h8d08d15138a5d6deLAw
  22:        0x10c6f498d - visit::walk_block::h10136710710042042084
  23:        0x10c6ed1b7 - middle::liveness::visit_fn::hf74c7937fefd55a2Snw
  24:        0x10c6ef811 - visit::walk_item::h12794621036516313043
  25:        0x10c6eedeb - middle::liveness::check_crate::h6b252a0b5d782dd5q2v
  26:        0x10c0b0dd8 - util::common::time::h15747003379871098135
  27:        0x10c23f93d - driver::driver::phase_3_run_analysis_passes::h03c31230e01512340jS
  28:        0x10c2331cc - driver::driver::compile_input::hd1278c9ccfcd13e860R
  29:        0x10c2b1135 - driver::run_compiler::h77df7302b6201815eZT
  30:        0x10c2af90e - driver::run::closure.59711
  31:        0x10c0c845e - task::TaskBuilder::try_future::closure.38968
  32:        0x10f57e264 - task::TaskBuilder::spawn_internal::closure.24385
  33:        0x10f8535bd - task::Task::spawn::closure.5925
  34:        0x10f8b846c - rust_try_inner
  35:        0x10f8b8456 - rust_try
  36:        0x10f853697 - unwind::try::h87a1bf4088a7340a3Qc
  37:        0x10f85346c - task::Task::run::he655a378e4b8f8b893b
  38:        0x10f85316f - task::Task::spawn::closure.5901
  39:        0x10f854e07 - thread::thread_start::h83a5d756a5e3479fWnc
  40:     0x7fff8f8ab2fc - _pthread_body
  41:     0x7fff8f8ab279 - _pthread_body
