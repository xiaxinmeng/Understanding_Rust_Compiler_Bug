
task 'rustc' failed at 'No def'n found for syntax::ast::DefId{krate: 0u32, node: 5u32} in tcx.tcache', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librustc/middle/ty.rs:3582

stack backtrace:
   1:     0x7f9a4901ae60 - rt::backtrace::imp::write::h09749635742f42d5t9p
   2:     0x7f9a4901df00 - failure::on_fail::h03c253a2288fc220Guq
   3:     0x7f9a497e5a90 - unwind::begin_unwind_inner::h84cc9a0765812e46M8d
   4:     0x7f9a497e5780 - unwind::begin_unwind_fmt::h5294fb1154f645ceh6d
   5:     0x7f9a49e61c00 - middle::ty::lookup_item_type::h89baf3a5b77a5660yZF
   6:     0x7f9a4a1a58d0 - middle::typeck::check::check_expr_with_unifier::hc5fcf79af1d30b79TqT
   7:     0x7f9a4a1a3230 - middle::typeck::check::check_argument_types::h200806ef1fa9539bVPS
   8:     0x7f9a4a1a1c80 - middle::typeck::check::check_method_argument_types::h021ef003da386d4bYNS
   9:     0x7f9a4a1a58d0 - middle::typeck::check::check_expr_with_unifier::hc5fcf79af1d30b79TqT
  10:     0x7f9a4a16e640 - middle::typeck::check::check_block_with_expected::hc2974a9f9cd6494ePyV
  11:     0x7f9a4a16a4a0 - middle::typeck::check::check_fn::h14c81c9d5f42a6f5dJQ
  12:     0x7f9a4a16a1c0 - middle::typeck::check::check_bare_fn::h5578676740afd8e33xQ
  13:     0x7f9a4a173780 - middle::typeck::check::check_method_body::h3d8a5ba9d77c3eddmiR
  14:     0x7f9a4a163350 - middle::typeck::check::check_item::h58dd0272451cce32R6Q
  15:     0x7f9a4a169fc0 - middle::typeck::check::check_item_types::h3da035c97051e367lxQ
  16:     0x7f9a49bbb440 - util::common::time::h4510992740229746977
  17:     0x7f9a4a375f30 - middle::typeck::check_crate::h3e227d690828d21chNh
  18:     0x7f9a4a446830 - driver::driver::phase_3_run_analysis_passes::hfa39577b52f9ea04nfw
  19:     0x7f9a4a4419a0 - driver::driver::compile_input::h97804601f8a7103fC1v
  20:     0x7f9a4a4e44d0 - driver::run_compiler::h6659b80ce522edebk6y
  21:     0x7f9a4a4e43e0 - driver::main_args::closure.135085
  22:     0x7f9a4a4f63e0 - task::TaskBuilder<S>::try_future::closure.136238
  23:     0x7f9a4a4f61e0 - task::TaskBuilder<S>::spawn_internal::closure.136215
  24:     0x7f9a4af1ae60 - task::spawn_opts::closure.8314
  25:     0x7f9a4983d440 - rust_try_inner
  26:     0x7f9a4983d430 - rust_try
  27:     0x7f9a497e30f0 - unwind::try::h9ef1228fa05d611fhXd
  28:     0x7f9a497e2e90 - task::Task::run::h7037046a47095aa6T4c
  29:     0x7f9a4af1ac20 - task::spawn_opts::closure.8260
  30:     0x7f9a497e4cd0 - thread::thread_start::h6cc33e9539afe5013sd
  31:     0x7f9a48ab0060 - start_thread
  32:     0x7f9a494b4489 - __clone
  33:                0x0 - <unknown>
