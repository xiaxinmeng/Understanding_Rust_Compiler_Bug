
<anon>:2:5: 2:18 warning: path statement with no effect, #[warn(path_statements)] on by default
<anon>:2     std::u8::MAX;
             ^~~~~~~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: type_is_zero_size(bcx.ccx(), bcx.tcx().node_id_to_type(expr.id))', ../src/librustc_trans/expr.rs:159
stack backtrace:
   1:     0x7f937149b610 - std::sys::backtrace::tracing::imp::write::h9fb600083204ae7f
   2:     0x7f93714a8d5b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hca543c34f11229ac
   3:     0x7f93714a88fc - std::panicking::default_hook::hc2c969e7453d080c
   4:     0x7f937146e28f - std::sys_common::unwind::begin_unwind_inner::h30e12d15ce2b2e25
   5:     0x7f936ff6cecf - std::sys_common::unwind::begin_unwind::h65390c63b501d6ed
   6:     0x7f936ffda208 - rustc_trans::expr::trans_into::he1315719b2222c7c
   7:     0x7f937005624b - rustc_trans::controlflow::trans_stmt_semi::h1cfba097d89a5175
   8:     0x7f936ffd1ea0 - rustc_trans::controlflow::trans_block::h4fad28c271095568
   9:     0x7f936ffd0c85 - rustc_trans::base::trans_closure::h83a389352ac0be25
  10:     0x7f936ffd28f3 - rustc_trans::base::trans_fn::h9423938066432dd9
  11:     0x7f936ffdce91 - rustc_trans::base::trans_item::hdc785ae68a9e6500
  12:     0x7f936fff6ecb - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::h22c1c36dec64effa
  13:     0x7f936ffe57cb - rustc_trans::base::trans_crate::h6ed1dbd6e572a8c2
  14:     0x7f9371a006ff - rustc_driver::driver::phase_4_translate_to_llvm::hd7579aae98641824
  15:     0x7f93719fed8f - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::he3c6b6ab9afe28dd
  16:     0x7f93719fb630 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h06c776ef4d1c70b6
  17:     0x7f93719f4f5e - rustc::ty::context::TyCtxt::create_and_enter::hb0e0e916c7274a64
  18:     0x7f93719c790d - rustc_driver::driver::compile_input::h0629572e6f316b31
  19:     0x7f93719a853c - rustc_driver::run_compiler::h8902aebf8b1849a8
  20:     0x7f93719a5c41 - std::sys_common::unwind::try::try_fn::h4c74456035d0fcc7
  21:     0x7f9371498deb - __rust_try
  22:     0x7f9371498d7d - std::sys_common::unwind::inner_try::h47a4d9cd4a369dcd
  23:     0x7f93719a648a - _<F as std..boxed..FnBox<A>>::call_box::h27f542a39f1d61ef
  24:     0x7f93714a6ef4 - std::sys::thread::Thread::new::thread_start::h6f266e069bf4ec2b
  25:     0x7f936922e423 - start_thread
  26:     0x7f9371105cbc - clone
  27:                0x0 - <unknown>
