
RUST_BACKTRACE=1 cargo test
   Compiling raft_rs v0.0.1 (file:///home/user/github/waterlink/johnmq/raft-rs)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: *start <= *end', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcore/slice.rs:396

stack backtrace:
   1:     0x7f8136dd9af0 - rt::backtrace::imp::write::h78410bf389422bb4Afq
   2:     0x7f8136ddcbf0 - failure::on_fail::h3d7a049ab1da182dXAq
   3:     0x7f813b31ee30 - unwind::begin_unwind_inner::ha5ca26286b78bd8fdSd
   4:     0x7f813b31e950 - unwind::begin_unwind_fmt::h9ff359c82ebbcec0FPd
   5:     0x7f813b31e910 - rust_begin_unwind
   6:     0x7f813b36a1a0 - panicking::panic_fmt::h6d1e7e05d89e9577h7j
   7:     0x7f813b365a50 - panicking::panic::h9595095b841fe7bal4j
   8:     0x7f813be31050 - middle::typeck::check::vtable::check_object_safety::h909babbc174b8351CBN
   9:     0x7f813be9e600 - middle::typeck::check::FnCtxt<'a, 'tcx>::register_unsize_obligations::h7602815ae022d228GhX
  10:     0x7f813be9e570 - middle::typeck::check::FnCtxt<'a, 'tcx>::register_autoref_obligations::h0590f015c52acbc0EgX
  11:     0x7f813be9e570 - middle::typeck::check::FnCtxt<'a, 'tcx>::register_autoref_obligations::h0590f015c52acbc0EgX
  12:     0x7f813be79a80 - middle::typeck::check::FnCtxt<'a, 'tcx>::write_adjustment::h1396e48fc524a05dIdX
  13:     0x7f813be6a520 - middle::typeck::check::FnCtxt<'a, 'tcx>::mk_assignty::ha58af6f758d897bdxvX
  14:     0x7f813be6a210 - middle::typeck::check::demand::coerce::h48183cae82ca7869RPR
  15:     0x7f813becd000 - middle::typeck::check::check_expr_coercable_to_type::closure.132386
  16:     0x7f813bec3970 - middle::typeck::check::check_expr_with_unifier::h9021eb76dad51ef5dJY
  17:     0x7f813bec1dd0 - middle::typeck::check::check_argument_types::h1928dcb07649510889X
  18:     0x7f813bec1610 - middle::typeck::check::check_method_argument_types::h20978a60653d31e5k8X
  19:     0x7f813bec3970 - middle::typeck::check::check_expr_with_unifier::h9021eb76dad51ef5dJY
  20:     0x7f813bf17500 - middle::typeck::check::check_stmt::h0b590f51bdc43b8bWL0
  21:     0x7f813be8f1a0 - middle::typeck::check::check_block_with_expected::hbcf547bf91ace5705P0
  22:     0x7f813bec3970 - middle::typeck::check::check_expr_with_unifier::h9021eb76dad51ef5dJY
  23:     0x7f813be8f1a0 - middle::typeck::check::check_block_with_expected::hbcf547bf91ace5705P0
  24:     0x7f813be8a770 - middle::typeck::check::check_fn::hc80418f404e5c598pvV
  25:     0x7f813bece970 - middle::typeck::check::check_expr_with_unifier::check_expr_fn::h2ca30ae5f2e571b43dZ
  26:     0x7f813bec3970 - middle::typeck::check::check_expr_with_unifier::h9021eb76dad51ef5dJY
  27:     0x7f813bec1dd0 - middle::typeck::check::check_argument_types::h1928dcb07649510889X
  28:     0x7f813bec3970 - middle::typeck::check::check_expr_with_unifier::h9021eb76dad51ef5dJY
  29:     0x7f813be8f1a0 - middle::typeck::check::check_block_with_expected::hbcf547bf91ace5705P0
  30:     0x7f813be8a770 - middle::typeck::check::check_fn::hc80418f404e5c598pvV
  31:     0x7f813be8a4b0 - middle::typeck::check::check_bare_fn::h1dd0be1cd95d0c63EkV
  32:     0x7f813be861c0 - middle::typeck::check::check_item::hfa9ed324eb120d1cNEV
  33:     0x7f813be89c30 - visit::Visitor::visit_mod::h4776340902026008023
  34:     0x7f813be88760 - visit::walk_item::h1837381033057518270
  35:     0x7f813be89f80 - middle::typeck::check::check_item_types::h7314688a92a7a43fOjV
  36:     0x7f813b985590 - util::common::time::h1086293170624038403
  37:     0x7f813c15cb90 - middle::typeck::check_crate::h707090d761adfc72A3n
  38:     0x7f813c1c6950 - driver::driver::phase_3_run_analysis_passes::h2dc7f682b53f78f9lNA
  39:     0x7f813c1c1200 - driver::driver::compile_input::hb25c14dfa369b7d96tA
  40:     0x7f813c2438b0 - driver::run_compiler::h3e5318d61e864690KgE
  41:     0x7f813c2437a0 - driver::run::closure.145379
  42:     0x7f813b99e370 - task::TaskBuilder<S>::try_future::closure.103636
  43:     0x7f813b99e160 - task::TaskBuilder<S>::spawn_internal::closure.103607
  44:     0x7f813b6691a0 - task::NativeSpawner.Spawner::spawn::closure.8435
  45:     0x7f813b377e30 - rust_try_inner
  46:     0x7f813b377e20 - rust_try
  47:     0x7f813b31c5e0 - unwind::try::h8c75b93d8c87f5c4xGd
  48:     0x7f813b31c470 - task::Task::run::h3855d187b4c0788fnMc
  49:     0x7f813b668ee0 - task::NativeSpawner.Spawner::spawn::closure.8373
  50:     0x7f813b31dcd0 - thread::thread_start::he45c6d58df77cc88C7c
  51:     0x7f8136189ea0 - start_thread
  52:     0x7f813afe5999 - __clone
  53:                0x0 - <unknown>

Could not compile `raft_rs`.

To learn more, run the command again with --verbose.
