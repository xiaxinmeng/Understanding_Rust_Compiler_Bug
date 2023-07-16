
error: internal compiler error: ../src/librustc/traits/project.rs:1010: Tried to project an inherited associated type during coherence checking, which is currently not supported.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:656
stack backtrace:
   1:     0x7f14b8dc89df - std::sys::backtrace::tracing::imp::write::h22f199c1dbb72ba2
   2:     0x7f14b8dd7f0d - std::panicking::default_hook::{{closure}}::h9a389c462b6a22dd
   3:     0x7f14b8dd5386 - std::panicking::default_hook::h852b4223c1c00c59
   4:     0x7f14b8dd5a68 - std::panicking::rust_panic_with_hook::hcd9d05f53fa0dafc
   5:     0x7f14b520c607 - std::panicking::begin_panic::h2f463d37998ebeba
   6:     0x7f14b521c768 - rustc_errors::Handler::bug::haca77c19c882b432
   7:     0x7f14b6271a9a - rustc::session::opt_span_bug_fmt::{{closure}}::hfeb850fbe828b399
   8:     0x7f14b61b0ba5 - rustc::session::opt_span_bug_fmt::h46e45438a860a75e
   9:     0x7f14b61b09e2 - rustc::session::bug_fmt::hde22f071bf5a80ea
  10:     0x7f14b61c9644 - rustc::traits::project::opt_normalize_projection_type::h3c60084765367e72
  11:     0x7f14b61c6d84 - rustc::traits::project::normalize_projection_type::h7c95c77a633add3c
  12:     0x7f14b61c6a53 - <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty::h6991154722ef72a6
  13:     0x7f14b61ef6e3 - rustc::ty::fold::TypeFolder::fold_substs::he628ffdb19736c82
  14:     0x7f14b6223080 - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for rustc::ty::Predicate<'tcx>>::super_fold_with::hdc2172d9ec2b3531
  15:     0x7f14b61c673e - rustc::traits::project::AssociatedTypeNormalizer::fold::h9bbd40855a494cd7
  16:     0x7f14b626853b - rustc::traits::select::SelectionContext::impl_or_trait_obligations::{{closure}}::h19b9bcd4ac2db5e3
  17:     0x7f14b608f38f - <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next::h3a3d48b1ae7d6800
  18:     0x7f14b6085d38 - <collections::vec::Vec<T> as core::iter::traits::FromIterator<T>>::from_iter::h4d047740cab4e83f
  19:     0x7f14b61dd84b - rustc::traits::select::SelectionContext::impl_or_trait_obligations::h9644f264b94a8200
  20:     0x7f14b62735d4 - rustc::traits::select::SelectionContext::confirm_impl_candidate::{{closure}}::h3d7fa611bcdee667
  21:     0x7f14b6126ecb - rustc::infer::InferCtxt::in_snapshot::haeae699068632827
  22:     0x7f14b61d7650 - rustc::traits::select::SelectionContext::confirm_candidate::h540501d672b9cbca
  23:     0x7f14b6273b5a - rustc::traits::select::SelectionContext::evaluate_candidate::{{closure}}::h04cd1468ff1eb8be
  24:     0x7f14b61ce3ea - rustc::traits::select::SelectionContext::probe::h2194aba9d8520c64
  25:     0x7f14b61cfe7b - rustc::traits::select::SelectionContext::evaluate_stack::h5c91229b58e6a943
  26:     0x7f14b61cf770 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::h71916bff78ccea77
  27:     0x7f14b61cf161 - rustc::traits::select::SelectionContext::evaluate_obligation_conservatively::h7dc149b5b7563089
  28:     0x7f14b61e716e - rustc::traits::type_known_to_meet_builtin_bound::hcd45d38728909c13
  29:     0x7f14b62122b2 - rustc::ty::util::<impl rustc::ty::TyS<'tcx>>::impls_bound::h753935dd0b4a114a
  30:     0x7f14b6212967 - rustc::ty::util::<impl rustc::ty::TyS<'tcx>>::moves_by_default::h72ff582de112af70
  31:     0x7f14b612e6ee - rustc::infer::InferCtxt::type_moves_by_default::h3b16d949850faded
  32:     0x7f14b61538d2 - rustc::middle::expr_use_visitor::ExprUseVisitor::consume_expr::h8aa20fc3357ddc6e
  33:     0x7f14b6153855 - rustc::middle::expr_use_visitor::ExprUseVisitor::walk_fn::h16bdd4f745899501
  34:     0x7f14b7eba54c - rustc_passes::consts::CheckCrateVisitor::fn_like::{{closure}}::h794ef39f49ac8a19
  35:     0x7f14b7eaf106 - rustc_passes::consts::CheckCrateVisitor::fn_like::h85700fc8102b6ef0
  36:     0x7f14b7eb013e - <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_impl_item::hbb6de125b301836f
  37:     0x7f14b7eaff81 - <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_item::h16b1592c63b20096
  38:     0x7f14b7eb1b42 - rustc_passes::consts::check_crate::h6d941508cdb757e1
  39:     0x7f14b918bce5 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h1928c4704cfe9c61
  40:     0x7f14b915b6ed - rustc_driver::driver::phase_3_run_analysis_passes::he578df6b8805151c
  41:     0x7f14b9147f69 - rustc_driver::driver::compile_input::h5b63ccd49eeeb98b
  42:     0x7f14b91712ba - rustc_driver::run_compiler::h98c7274e7cb1d11d
  43:     0x7f14b90aaf0b - std::panicking::try::do_call::h99ed0da044e497c3
  44:     0x7f14b8ddfe06 - __rust_maybe_catch_panic
  45:     0x7f14b90c9461 - <F as alloc::boxed::FnBox<A>>::call_box::hbdd5a14cd8e33b97
  46:     0x7f14b8dd3de0 - std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2
  47:     0x7f14b10b76f9 - start_thread
  48:     0x7f14b8a97b5c - clone
  49:                0x0 - <unknown>
