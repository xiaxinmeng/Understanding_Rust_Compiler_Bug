
task 'rustc' panicked at 'assertion failed: `(left == right) && (right == left)` (left: `None`, right: `Some(())`)', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librustc_typeck/check/method/confirm.rs:153

stack backtrace:
   1:     0x7f5d6bb9d540 - rt::backtrace::imp::write::heed5901f9756dd23BOx
   2:     0x7f5d6bba0780 - failure::on_fail::ha44d264deefff4481fy
   3:     0x7f5d6b7f1a60 - unwind::begin_unwind_inner::hbf982fb3a6122da9CJc
   4:     0x7f5d6b7f1560 - unwind::begin_unwind_fmt::h821ad37001759425ZGc
   5:     0x7f5d6ac4bc50 - check::method::confirm::ConfirmContext<'a, 'tcx>::confirm::hf2e945d9b945d32dwkf
   6:     0x7f5d6ad3a310 - check::check_expr_with_unifier::check_method_call::hee2b27d8726de86drFn
   7:     0x7f5d6ad307a0 - check::check_expr_with_unifier::h422faee1cd496c41EAn
   8:     0x7f5d6ad39260 - check::check_expr_with_unifier::check_binop::h9b948f9bb2e85989qRn
   9:     0x7f5d6ad307a0 - check::check_expr_with_unifier::h422faee1cd496c41EAn
  10:     0x7f5d6ad307a0 - check::check_expr_with_unifier::h422faee1cd496c41EAn
  11:     0x7f5d6ad307a0 - check::check_expr_with_unifier::h422faee1cd496c41EAn
  12:     0x7f5d6ace8810 - check::check_block_with_expected::hf38c5063b6b232d0vtp
  13:     0x7f5d6acb9c20 - check::check_fn::h0ffd34e82269a488jhk
  14:     0x7f5d6ace59f0 - check::check_bare_fn::h80d05a38b73232b5K6j
  15:     0x7f5d6acdda80 - check::check_item::hbaff9fee6f6cec45aqk
  16:     0x7f5d6b095930 - check_crate::closure.43673
  17:     0x7f5d6b092200 - util::common::time::h16516841138721642264
  18:     0x7f5d6b0914d0 - check_crate::h601dd3cedc4d9d9bDUy
  19:     0x7f5d6bfd03f0 - driver::phase_3_run_analysis_passes::h89150347d25ccb80Cta
  20:     0x7f5d6bfbf0b0 - driver::compile_input::hdc87225d47b03d10pba
  21:     0x7f5d6c05de20 - run_compiler::hac4563508ec17f56EYb
  22:     0x7f5d6c05dd10 - run::closure.21548
  23:     0x7f5d6c06f750 - task::TaskBuilder::try_future::closure.23002
  24:     0x7f5d6bb759c0 - task::TaskBuilder::spawn_internal::closure.30612
  25:     0x7f5d6b7ef700 - task::Task::spawn::closure.5729
  26:     0x7f5d6b84cf50 - rust_try_inner
  27:     0x7f5d6b84cf40 - rust_try
  28:     0x7f5d6b7ef7e0 - unwind::try::hd503f9309b2b1ec2Tyc
  29:     0x7f5d6b7ef5a0 - task::Task::run::h7ea5fe2114c20f68fKb
  30:     0x7f5d6b7ef190 - task::Task::spawn::closure.5705
  31:     0x7f5d6b7f0be0 - thread::thread_start::h5e2446fe7cf0a4f1w1b
  32:     0x7f5d66653250 - start_thread
  33:     0x7f5d6b4c5589 - clone
  34:                0x0 - <unknown>
