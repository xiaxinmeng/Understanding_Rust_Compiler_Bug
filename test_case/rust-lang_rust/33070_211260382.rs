
stack backtrace:
   1:     0x7f5820eff610 - std::sys::backtrace::tracing::imp::write::h9fb600083204ae7f
   2:     0x7f5820f0cd0b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hca543c34f11229ac
   3:     0x7f5820f0c8ac - std::panicking::default_hook::hc2c969e7453d080c
   4:     0x7f5820ed18bf - std::sys_common::unwind::begin_unwind_inner::h30e12d15ce2b2e25
   5:     0x7f581a7a3ed8 - std::sys_common::unwind::begin_unwind::h24c4c0fa2cfe995a
   6:     0x7f581a7a3e7b - syntax::errors::Handler::span_bug::h1c6b996df38ae8d7
   7:     0x7f581a7a3d4a - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::hf0fb5697f8f25f40
   8:     0x7f581a7a3bf2 - rustc::session::opt_span_bug_fmt::hc581ba88c4e2ec53
   9:     0x7f581a7a3b34 - rustc::session::span_bug_fmt::h4591bcbadfc38558
  10:     0x7f581a88518d - _<ty..subst..SubstFolder<'a, 'tcx> as ty..fold..TypeFolder<'tcx>>::fold_ty::hce07f4b40d3cacb2
  11:     0x7f581b66495b - _<check..FnCtxt<'a, 'tcx> as astconv..AstConv<'tcx>>::ty_infer::h36a2788135ab3af5
  12:     0x7f581b7090c6 - _<std..iter..Map<I, F> as std..iter..Iterator>::next::h0128742a87f770ca
  13:     0x7f581b70766b - rustc_typeck::astconv::create_substs_for_ast_path::h6fa21d9ce590a57f
  14:     0x7f581b70dbba - rustc_typeck::astconv::create_substs_for_ast_trait_ref::h03de7676f19db52c
  15:     0x7f581b70cf0c - rustc_typeck::astconv::ast_path_to_mono_trait_ref::h8a557d894b7dff9f
  16:     0x7f581b712ace - rustc_typeck::astconv::qpath_to_ty::h233b0002fda3dbc0
  17:     0x7f581b6af872 - rustc_typeck::astconv::finish_resolving_def_to_ty::hf17fb92b2d6f4861
  18:     0x7f581b6821e3 - rustc_typeck::check::resolve_ty_and_def_ufcs::hb950623f3fdffcaa
  19:     0x7f581b67f9fd - rustc_typeck::check::check_expr_with_expectation_and_lvalue_pref::hdaa9a3d5d214eeb7
  20:     0x7f581b69273a - rustc_typeck::check::callee::check_call::h581c68eea83787eb
  21:     0x7f581b67b71e - rustc_typeck::check::check_expr_with_expectation_and_lvalue_pref::hdaa9a3d5d214eeb7
  22:     0x7f581b6599a7 - rustc_typeck::check::check_block_with_expected::hc05ff1e7fff64a1e
  23:     0x7f581b651459 - rustc_typeck::check::check_fn::hb43882d54094ac4a
  24:     0x7f581b64ed0c - rustc_typeck::check::check_bare_fn::h7e4c2ad58d1814dd
  25:     0x7f581b65e04f - rustc_typeck::check::check_method_body::h7863cacbf117e455
  26:     0x7f581b649d3e - rustc_typeck::check::check_item_body::h7ffec660fc242796
  27:     0x7f581b641ab1 - rustc_typeck::check::check_item_bodies::ha729008a2e67410e
  28:     0x7f581b63901f - rustc_typeck::check_crate::hb08ba31a6a5f65b5
  29:     0x7f582145b850 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h5a042a091cd7658c
  30:     0x7f5821459a5b - rustc::ty::context::TyCtxt::create_and_enter::h3f9051bcccbd93e4
  31:     0x7f582145650e - rustc_driver::driver::phase_3_run_analysis_passes::h9c723484c588a35b
  32:     0x7f5821428f5f - rustc_driver::driver::compile_input::h0629572e6f316b31
  33:     0x7f582140f4d4 - rustc_driver::run_compiler::h8902aebf8b1849a8
  34:     0x7f582140c941 - std::sys_common::unwind::try::try_fn::h4c74456035d0fcc7
  35:     0x7f5820efcdab - __rust_try
  36:     0x7f5820efcd3d - std::sys_common::unwind::inner_try::h47a4d9cd4a369dcd
  37:     0x7f582140d18a - _<F as std..boxed..FnBox<A>>::call_box::h27f542a39f1d61ef
  38:     0x7f5820f0aea4 - std::sys::thread::Thread::new::thread_start::h6f266e069bf4ec2b
  39:     0x7f5818d8a423 - start_thread
  40:     0x7f5820b69cbc - clone
  41:                0x0 - <unknown>
