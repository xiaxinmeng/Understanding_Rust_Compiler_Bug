
task 'rustc' failed at 'assertion failed: !ty::type_needs_infer(ty)', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/librustc/middle/typeck/mod.rs:280

stack backtrace:
   1:        0x104335c88 - rt::backtrace::imp::write::hae692894f5f1f1ebUbq
   2:        0x104339475 - failure::on_fail::h75f9b8d6f426596adsq
   3:        0x1045e5fe9 - unwind::begin_unwind_inner::h54dfeb5a2036a633AXd
   4:        0x1045e5b77 - unwind::begin_unwind_fmt::hadacf5a3687620033Ud
   5:        0x1015df64c - middle::typeck::write_ty_to_tcx::h44d1c821ecaca6a0RUD
   6:        0x1015d8ef5 - middle::typeck::check::writeback::WritebackCx<'cx>::visit_node_id::h151214da46fae4c2Ey8
   7:        0x1015d603e - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_expr::hdfef6feeff4c2b95ho8
   8:        0x1015d62fe - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_expr::hdfef6feeff4c2b95ho8
   9:        0x1015dba36 - middle::typeck::check::writeback::WritebackCx<'cx>.Visitor<(*>::visit_local::he925f9cb37ed8a8aTt8
  10:        0x1015df0a3 - visit::walk_block::h1604224355725969978
  11:        0x1015d758f - middle::typeck::check::writeback::resolve_type_vars_in_fn::h960409d3de2a563evi8
  12:        0x1016364ef - middle::typeck::check::check_bare_fn::h5b61e7a34e8f1873oVd
  13:        0x10162fae2 - middle::typeck::check::check_item::hd7449b378454e2d6zse
  14:        0x1016361fd - middle::typeck::check::check_item_types::h89c2a47746b27eceGUd
  15:        0x10179b6fe - middle::typeck::check_crate::h86d2524029231b02vfE
  16:        0x101b49599 - driver::driver::phase_3_run_analysis_passes::h858b7dce24b9b32fVEs
  17:        0x101b449c1 - driver::driver::compile_input::h544a5b5acde2ff3bWqs
  18:        0x101bf6a1c - driver::run_compiler::hfcfc73e0e640751fZov
  19:        0x101bf4056 - driver::main_args::closure.Name(134079*
  20:        0x101c074bb - task::TaskBuilder<S>::try_future::closure.Name(135228*
  21:        0x101c073b1 - task::TaskBuilder<S>::spawn_internal::closure.Name(135205*
  22:        0x1012e463c - task::spawn_opts::closure.Name(8249*
  23:        0x1046499ac - rust_try
  24:        0x1045e2d4b - unwind::try::ha8241ded872b0a50ZLd
  25:        0x1045e2ab3 - task::Task::run::hff5823cf81f02aafRXc
  26:        0x1012e44d1 - task::spawn_opts::closure.Name(8194*
  27:        0x1045e4d46 - thread::thread_start::h73513dc0bb2d1001pkd
  28:     0x7fff90266899 - _pthread_body
  29:     0x7fff9026672a - _pthread_struct_init
