
   Compiling dataflow_join v0.0.1 (file:///Users/mcsherry/Projects/dataflow-join)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: bound_list_is_sorted(&bounds.projection_bounds)', /Users/rustbuild/src/rust-buildbot/slave/stable-dist-rustc-mac/build/src/librustc/middle/ty.rs:3237

stack backtrace:
   1:        0x10f598fff - sys::backtrace::write::h5a94548fe691961bc2r
   2:        0x10f5a1a50 - panicking::on_panic::hc33f9bf1349f5e0bLgw
   3:        0x10f55ccc5 - rt::unwind::begin_unwind_inner::hb24e48e8f7bb8bb0uYv
   4:        0x10c9a956f - rt::unwind::begin_unwind::h16372345962065799137
   5:        0x10ca9a049 - middle::ty::mk_trait::h6533a56ee618f30dTa5
   6:        0x10cd253c1 - metadata::tydecode::parse_ty_::h17233465002289524658
   7:        0x10cd24538 - metadata::tydecode::parse_ty_::h17233465002289524658
   8:        0x10cd26353 - metadata::tydecode::parse_substs_::h7886299839747005652
   9:        0x10cd24b47 - metadata::tydecode::parse_ty_::h17233465002289524658
  10:        0x10cd26353 - metadata::tydecode::parse_substs_::h7886299839747005652
  11:        0x10cd24b47 - metadata::tydecode::parse_ty_::h17233465002289524658
  12:        0x10cd26353 - metadata::tydecode::parse_substs_::h7886299839747005652
  13:        0x10cd24b47 - metadata::tydecode::parse_ty_::h17233465002289524658
  14:        0x10cd249fb - metadata::tydecode::parse_ty_::h17233465002289524658
  15:        0x10ccec9fc - metadata::csearch::get_field_type::h2287f01378fea54d1Gm
  16:        0x10ccac905 - middle::ty::lookup_field_type_unsubstituted::h2b1ef075de9a0a1d7B8
  17:        0x10caf8547 - middle::ty::struct_fields::h264cc4fe3d890140kF8
  18:        0x10cc9e574 - middle::traits::select::SelectionContext<'cx, 'tcx>::builtin_bound::h250758517ef4e73caCS
  19:        0x10cc9993c - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_builtin_bound_candidates::h4d99b58c75912a4bJyS
  20:        0x10cc976cb - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h48ed99cc3014ff51PCR
  21:        0x10cc8d2a7 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h942ec6226a4ce943FgR
  22:        0x10cb5e4e7 - middle::traits::select::SelectionContext<'cx, 'tcx>::select::hfcdde7d6189f9fe2EUQ
  23:        0x10cc6e52c - middle::traits::fulfill::FulfillmentContext<'tcx>::select::hbfd9add7588cde90vMO
  24:        0x10cc6da98 - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h3009bf934d8bcc0aJLO
  25:        0x10c7bcab3 - check::FnCtxt<'a, 'tcx>::select_obligations_where_possible::h195400fd72caaf3aBnp
  26:        0x10c744681 - check::FnCtxt<'a, 'tcx>::resolve_type_vars_if_possible::hbafa97f6f84c0b527Bo
  27:        0x10c71120e - check::structurally_resolved_type::h0d57f96ee12a7090nft
  28:        0x10c7c95db - check::check_expr_with_unifier::check_method_call::h6fbc17f86cb4009c7qq
  29:        0x10c7fd0f8 - check::check_expr_with_unifier::h16198709343475811244
  30:        0x10c7c95a6 - check::check_expr_with_unifier::check_method_call::h6fbc17f86cb4009c7qq
  31:        0x10c7fd0f8 - check::check_expr_with_unifier::h16198709343475811244
  32:        0x10c7c95a6 - check::check_expr_with_unifier::check_method_call::h6fbc17f86cb4009c7qq
  33:        0x10c7e03ea - check::check_expr_with_unifier::h9590468336052747275
  34:        0x10c808e7b - check::check_decl_local::h5ab825b4ae4c4049xYr
  35:        0x10c7b3be4 - check::check_block_with_expected::hcf516cdce8fb2f3cD4r
  36:        0x10c7e09ee - check::check_expr_with_unifier::h9590468336052747275
  37:        0x10c7b431b - check::check_block_with_expected::hcf516cdce8fb2f3cD4r
  38:        0x10c797998 - check::check_fn::hccdd8c8447844807gFn
  39:        0x10c795e9d - check::closure::check_expr_closure::hf6e67e1d806efaf5Ngl
  40:        0x10c7c3ece - check::check_expr_with_unifier::h4656396524897881353
  41:        0x10c79db5c - check::check_argument_types::h70e22fb88692fa7aBRp
  42:        0x10c79f35a - check::check_method_argument_types::h95c2f0efed3f12130Op
  43:        0x10c7ca0d2 - check::check_expr_with_unifier::check_method_call::h6fbc17f86cb4009c7qq
  44:        0x10c7e03ea - check::check_expr_with_unifier::h9590468336052747275
  45:        0x10c808e7b - check::check_decl_local::h5ab825b4ae4c4049xYr
  46:        0x10c7b3be4 - check::check_block_with_expected::hcf516cdce8fb2f3cD4r
  47:        0x10c797998 - check::check_fn::hccdd8c8447844807gFn
  48:        0x10c7aea13 - check::check_bare_fn::hbfad11d5a9f87954Tun
  49:        0x10c7aca89 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hf2b38fe3b992c9bcWrn
  50:        0x10c871aba - check_crate::closure.38929
  51:        0x10c86b54a - check_crate::hcd5b7e66feee2debsJC
  52:        0x10c0c08af - driver::phase_3_run_analysis_passes::h3f61cd0eb3853b54tGa
  53:        0x10c0a52d3 - driver::compile_input::h3d15d32af3f4b6bdQba
  54:        0x10c160f33 - run_compiler::h32c0e6c36780775b75b
  55:        0x10c15e69a - boxed::F.FnBox<A>::call_box::h7208527504866586156
  56:        0x10c15dbf7 - rt::unwind::try::try_fn::h4574368377636742306
  57:        0x10f6297a8 - rust_try_inner
  58:        0x10f629795 - rust_try
  59:        0x10c15decd - boxed::F.FnBox<A>::call_box::h14823742600371436653
  60:        0x10f5a063d - sys::thread::Thread::new::thread_start::hefa4f0e4c64c9336Yjv
  61:     0x7fff98192267 - _pthread_body
  62:     0x7fff981921e4 - _pthread_start

Could not compile `dataflow_join`.

To learn more, run the command again with --verbose.
% rustc --version
rustc 1.1.0 (35ceea399 2015-06-19)
%
