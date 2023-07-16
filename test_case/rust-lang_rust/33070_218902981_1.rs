
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:545
stack backtrace:
   1:     0x7f04f913a160 - std::sys::backtrace::tracing::imp::write::h9fb600083204ae7f
   2:     0x7f04f9147d6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hca543c34f11229ac
   3:     0x7f04f914790c - std::panicking::default_hook::hc2c969e7453d080c
   4:     0x7f04f910d338 - std::panicking::rust_panic_with_hook::hfe203e3083c2b544
   5:     0x7f04f56d34c8 - std::panicking::begin_panic::hf2bac3a63b4cf82e
   6:     0x7f04f56d33cf - syntax::errors::Handler::span_bug::h1878d6587b8bfd95
   7:     0x7f04f56d322d - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::hf0fb5697f8f25f40
   8:     0x7f04f56d313d - rustc::session::opt_span_bug_fmt::hc581ba88c4e2ec53
   9:     0x7f04f56d2ff4 - rustc::session::span_bug_fmt::h4591bcbadfc38558
  10:     0x7f04f592287e - _<ty..subst..SubstFolder<'a, 'gcx, 'tcx> as ty..fold..TypeFolder<'gcx, 'tcx>>::fold_region::h04b91b1acd88fd7a
  11:     0x7f04f57c9b97 - _<ty..subst..SubstFolder<'a, 'gcx, 'tcx> as ty..fold..TypeFolder<'gcx, 'tcx>>::fold_ty::h85d957d19ba2e322
  12:     0x7f04f57c966f - rustc::infer::InferCtxt::type_vars_for_defs::h7fb1ca6642fcbdb6
  13:     0x7f04f7ef2206 - rustc_typeck::check::FnCtxt::instantiate_path::hae3b9f0a269ab36f
  14:     0x7f04f7ed50b2 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hf7a336bf3f6d489c
  15:     0x7f04f7efaf91 - rustc_typeck::check::callee::_<impl check..FnCtxt<'a, 'gcx, 'tcx>>::check_call::hb1af81fdd6670b2d
  16:     0x7f04f7ed0920 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::hf7a336bf3f6d489c
  17:     0x7f04f7eacded - rustc_typeck::check::FnCtxt::check_block_with_expected::hb3119871f6a1bbfa
  18:     0x7f04f7ea3d32 - rustc_typeck::check::check_fn::h4bb517663c83e412
  19:     0x7f04f7ea07fe - rustc_typeck::check::check_bare_fn::ha3f2d22e33abb8cf
  20:     0x7f04f7e9c272 - rustc_typeck::check::check_item_body::h7ffec660fc242796
  21:     0x7f04f7e92eb9 - rustc_typeck::check::check_item_bodies::ha729008a2e67410e
  22:     0x7f04f7e8bee6 - rustc_typeck::check_crate::hab54f1dbb6479f2b
  23:     0x7f04f969bf8c - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::he8e0792bce031dff
  24:     0x7f04f9699fda - rustc::ty::context::TyCtxt::create_and_enter::h9e0841510a9ae184
  25:     0x7f04f965d141 - rustc_driver::driver::compile_input::h0629572e6f316b31
  26:     0x7f04f964d41d - rustc_driver::run_compiler::h8902aebf8b1849a8
  27:     0x7f04f964a51e - std::panicking::try::call::hb9e578062982aefa
  28:     0x7f04f915650b - __rust_try
  29:     0x7f04f91564ae - __rust_maybe_catch_panic
  30:     0x7f04f964b004 - _<F as std..boxed..FnBox<A>>::call_box::h27f542a39f1d61ef
  31:     0x7f04f9145e74 - std::sys::thread::Thread::new::thread_start::h6f266e069bf4ec2b
  32:     0x7f04f0dba473 - start_thread
  33:     0x7f04f8d9dacc - clone
  34:                0x0 - <unknown>

error: Could not compile `ice`.

To learn more, run the command again with --verbose.
