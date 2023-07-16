
stack backtrace:
   1:        0x104be15d5 - rt::backtrace::imp::write::hae68a22ce56435beoiq
   2:        0x104be4793 - failure::on_fail::hf8967f369f5e74c5Tyq
   3:        0x104ea2055 - unwind::begin_unwind_inner::h1f48c1ec631c1dc8S5d
   4:        0x102c8e882 - unwind::begin_unwind::h5926505211794541144
   5:        0x102c8e815 - diagnostic::SpanHandler::span_bug::he2a6db0f09da46ebsTD
   6:        0x10195f445 - driver::session::Session::span_bug::h0e901731857e85fbBKx
   7:        0x101a3b056 - middle::subst::SubstFolder<'a>.TypeFolder::fold_ty::h7eac83746cb1333bixW
   8:        0x101a3a72e - middle::subst::T.Subst::subst_spanned::h15305841671447511745
   9:        0x101c7c8fc - middle::ty::lookup_field_type::h9895f97838f2e255WgG
  10:        0x101cd4236 - middle::typeck::check::_match::check_struct_pat_fields::hc26eb96e6850c1e1WfI
  11:        0x101cd6d3d - middle::typeck::check::_match::check_struct_pat::h5dfa3b3fa3d630e2EmI
  12:        0x101ccc424 - middle::typeck::check::_match::check_pat::h1f95cf980fc0f59bhrI
  13:        0x101cc9880 - middle::typeck::check::_match::check_match::h3b8792269b558fddrWH
  14:        0x101d7fc90 - middle::typeck::check::check_expr_with_unifier::hdcfe0833f5648e0bzyT
  15:        0x101dd918c - middle::typeck::check::check_decl_local::hbcdc38469e486e90dAV
  16:        0x101dd93df - middle::typeck::check::check_stmt::h9388d42ea0e31e12kCV
  17:        0x101d48b0b - middle::typeck::check::check_block_with_expected::h311c10d48ccca09cvGV
  18:        0x101d803cf - middle::typeck::check::check_expr_with_unifier::hdcfe0833f5648e0bzyT
  19:        0x101cc99ee - middle::typeck::check::_match::check_match::h3b8792269b558fddrWH
  20:        0x101d7fc90 - middle::typeck::check::check_expr_with_unifier::hdcfe0833f5648e0bzyT
  21:        0x101d48e99 - middle::typeck::check::check_block_with_expected::h311c10d48ccca09cvGV
  22:        0x101d44dfe - middle::typeck::check::check_fn::h75db1c8be68fa789TQQ
  23:        0x101d44782 - middle::typeck::check::check_bare_fn::h5d961d8a6b77d5d0JFQ
  24:        0x101d3dbd2 - middle::typeck::check::check_item::h43514e2feccb4bb4xeR
