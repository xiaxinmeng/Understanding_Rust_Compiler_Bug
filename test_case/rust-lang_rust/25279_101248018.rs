
src/compile.rs:71:22: 71:23 error: internal compiler error: cannot relate bound region: '_#3r <= ReEarlyBound(1625, TypeSpace, 0, 'a)
src/compile.rs:71             let ty = <Self as Compile<'a>>::get_type();
                                       ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:170

stack backtrace:
   1:     0x7f12bb71d569 - sys::backtrace::write::h1b67d998b7cb67b8uhs
   2:     0x7f12bb7254e9 - panicking::on_panic::h20750656e1ba63bdxXw
   3:     0x7f12bb6e56c2 - rt::unwind::begin_unwind_inner::h76c988894888b37eHCw
   4:     0x7f12b8aa6edd - rt::unwind::begin_unwind::h10772134248123395683
   5:     0x7f12b8aa6e72 - diagnostic::SpanHandler::span_bug::h1606ba1c4247a82cD2B
   6:     0x7f12b979be7a - middle::infer::region_inference::RegionVarBindings<'a, 'tcx>::make_subregion::hdf97136da1ef4d73Xfv
   7:     0x7f12b97577b3 - middle::infer::equate::Equate<'a, 'tcx>.TypeRelation<'a, 'tcx>::regions::h9e8171d15724a8e6Ioq
   8:     0x7f12b9754a82 - middle::ty_relate::super_relate_tys::h9438736136149808282
   9:     0x7f12b9751faa - middle::infer::equate::Equate<'a, 'tcx>.TypeRelation<'a, 'tcx>::tys::hb5f85851302ce7b0ykq
  10:     0x7f12b9752d47 - middle::infer::sub::Sub<'a, 'tcx>.TypeRelation<'a, 'tcx>::relate_with_variance::h14897340968304215671
  11:     0x7f12b975abc6 - iter::_&'a mut I.Iterator::next::h9535714511878633230
  12:     0x7f12b9759ce7 - middle::ty_relate::relate_substs::h10897699673523313923
  13:     0x7f12b97598fc - middle::ty_relate::relate_item_substs::h16203734268933392915
  14:     0x7f12b9759648 - middle::ty_relate::ty..TraitRef<'tcx>.Relate<'a, 'tcx>::relate::h6251723073212296460
  15:     0x7f12b97b5096 - middle::infer::InferCtxt<'a, 'tcx>::sub_trait_refs::hfb3c9e3089dd3ea8VUy
  16:     0x7f12b98473fe - middle::traits::select::SelectionContext<'cx, 'tcx>::match_impl::hb8a39a5fae73ce11iwT
  17:     0x7f12b984d616 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates_from_impls::closure.89550
  18:     0x7f12b984cdb8 - middle::ty::TraitDef<'tcx>::for_each_relevant_impl::h15649903191793266366
  19:     0x7f12b9849742 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h245c559c2afc6e8dDxR
  20:     0x7f12b983f830 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h80930074eb089163tbR
  21:     0x7f12b9720577 - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h4d8e9f6685524470sPQ
  22:     0x7f12b98227c0 - middle::traits::fulfill::FulfillmentContext<'tcx>::select::h4789bdf1a5238276RHO
  23:     0x7f12b98220fe - middle::traits::fulfill::FulfillmentContext<'tcx>::select_new_obligations::ha2169d32a8a0fd26wGO
  24:     0x7f12bae6c788 - check::vtable::select_new_fcx_obligations::h8912903c32f84b61Ybc
  25:     0x7f12baedd1e4 - check::check_argument_types::he9d55dce9b707ac274p
  26:     0x7f12baedacab - check::callee::confirm_builtin_call::hbea428fce883a4c4v9l
  27:     0x7f12baed976c - check::callee::check_call::he8a09fbff59eb19ad0l
  28:     0x7f12baf1c1db - check::check_expr_with_unifier::h3459509644534603543
  29:     0x7f12baf4266b - check::check_decl_local::h91591b4e3997ba254bs
  30:     0x7f12baef1b21 - check::check_block_with_expected::h2365ceaadf2563c7ais
  31:     0x7f12baf1c836 - check::check_expr_with_unifier::h3459509644534603543
  32:     0x7f12baef1e8c - check::check_block_with_expected::h2365ceaadf2563c7ais
  33:     0x7f12baed74f6 - check::check_fn::h480f463cc14c6aed6Wn
  34:     0x7f12baeed517 - check::check_bare_fn::h8504b606dbf00369FMn
  35:     0x7f12baef8f3c - check::check_method_body::h28cd463eb779d2f6Foo
  36:     0x7f12baeeb475 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::ha713990fb80d0d32IJn
  37:     0x7f12baeeb8af - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::ha713990fb80d0d32IJn
  38:     0x7f12bafa6a1a - check_crate::closure.38873
  39:     0x7f12baf9fd10 - check_crate::hc9e46cf04476d28cQHC
  40:     0x7f12bbc73208 - driver::phase_3_run_analysis_passes::hfb834cc0a373d316tGa
  41:     0x7f12bbc5400c - driver::compile_input::h2d3eeee40984f8a3Qba
  42:     0x7f12bbd0dc41 - run_compiler::h0f1bb669f4cb36db75b
  43:     0x7f12bbd0b492 - boxed::F.FnBox<A>::call_box::h4045195144511792878
  44:     0x7f12bbd0aa59 - rt::unwind::try::try_fn::h11015246275424985574
  45:     0x7f12bb79d818 - rust_try_inner
  46:     0x7f12bb79d805 - rust_try
  47:     0x7f12bbd0acf4 - boxed::F.FnBox<A>::call_box::h7159985203131778702
  48:     0x7f12bb724281 - sys::thread::Thread::new::thread_start::h1499b5aaf5a925cc3Iv
  49:     0x7f12b57a1e99 - start_thread
  50:     0x7f12bb3648bc - <unknown>
