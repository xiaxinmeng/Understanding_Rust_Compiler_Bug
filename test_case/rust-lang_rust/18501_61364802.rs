 rust
task 'rustc' panicked at 'assertion failed: `(left == right) && (right == left)` (left: `11`, right: `0`)', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/librustc/middle/ty.rs:2569

stack backtrace:
   1:        0x1053ac929 - rt::backtrace::imp::write::hfe87245730e40e498nq
   2:        0x1053afb6c - failure::on_fail::h250240fc1d2cc13cHEq
   3:        0x10560b395 - unwind::begin_unwind_inner::h629bbfd5c5c7ad4bSJd
   4:        0x10560b02f - unwind::begin_unwind_fmt::h26eed6b8c9614d01kHd
   5:        0x1024f5487 - middle::ty::type_contents::tc_ty::h4f516816f0265f20JIH
   6:        0x1024f473e - middle::ty::type_contents::tc_ty::h4f516816f0265f20JIH
   7:        0x1024f4c7f - middle::ty::type_contents::tc_ty::h4f516816f0265f20JIH
   8:        0x1022ada4f - middle::ty::type_contents::hcbff11a2d2064355XHH
   9:        0x102497834 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::consume_expr::h12011496269642837587
  10:        0x1024916af - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_expr::h6289422402745590059
  11:        0x102497889 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::consume_expr::h12011496269642837587
  12:        0x1024916af - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_expr::h6289422402745590059
  13:        0x102497889 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::consume_expr::h12011496269642837587
  14:        0x1024916af - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_expr::h6289422402745590059
  15:        0x102497ae3 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_local::h480003012053056122
  16:        0x102497a15 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_block::h3271632192568197791
  17:        0x10249180e - middle::expr_use_visitor::ExprUseVisitor<'d, 't, TYPER>::walk_expr::h6289422402745590059
  18:        0x10249d180 - middle::trans::_match::trans_match_inner::closure.126642
  19:        0x10242094b - middle::trans::_match::trans_match::h812670c167266a78prk
  20:        0x1024100fd - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  21:        0x1023d5cf9 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  22:        0x1023d604e - middle::trans::controlflow::trans_block::hb6579525f1a9c0e0M30
  23:        0x10247633a - middle::trans::base::trans_closure::h73d516938fc276e1Zah
  24:        0x1023c995f - middle::trans::base::trans_fn::ha1261357d96dceccpmh
  25:        0x1023cb592 - middle::trans::monomorphize::monomorphic_fn::h3beeee8af9d8e21acu0
  26:        0x1023fc857 - middle::trans::callee::trans_fn_ref_with_substs::he50eea051fdbe5a9sH3
  27:        0x1023e0aae - middle::trans::meth::trans_method_callee::ha6757eb79d64f196aAm
  28:        0x10240211b - middle::trans::callee::trans_method_call::closure.122853
  29:        0x1023dd895 - middle::trans::callee::trans_call_inner::hecb2b66e77f79065c33
  30:        0x102401f4d - middle::trans::callee::trans_method_call::hd6c33d4672989d74EY3
  31:        0x10240fb8f - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  32:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  33:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  34:        0x102419a98 - middle::trans::expr::trans_addr_of::hfcdfbe87e717518flZ6
  35:        0x10240f120 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  36:        0x1023d5d19 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  37:        0x102422651 - middle::trans::expr::trans_adt::h9723c7611e8b803aFJ6
  38:        0x1024106ff - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  39:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  40:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  41:        0x1024206e2 - middle::trans::_match::trans_match::h812670c167266a78prk
  42:        0x1024100fd - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  43:        0x1023d5cf9 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  44:        0x1023d604e - middle::trans::controlflow::trans_block::hb6579525f1a9c0e0M30
  45:        0x10247633a - middle::trans::base::trans_closure::h73d516938fc276e1Zah
  46:        0x1023c995f - middle::trans::base::trans_fn::ha1261357d96dceccpmh
  47:        0x1023cb592 - middle::trans::monomorphize::monomorphic_fn::h3beeee8af9d8e21acu0
  48:        0x1023fc857 - middle::trans::callee::trans_fn_ref_with_substs::he50eea051fdbe5a9sH3
  49:        0x1023e0aae - middle::trans::meth::trans_method_callee::ha6757eb79d64f196aAm
  50:        0x10240211b - middle::trans::callee::trans_method_call::closure.122853
  51:        0x1023dd895 - middle::trans::callee::trans_call_inner::hecb2b66e77f79065c33
  52:        0x102401f4d - middle::trans::callee::trans_method_call::hd6c33d4672989d74EY3
  53:        0x10240fb8f - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  54:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  55:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  56:        0x102419a98 - middle::trans::expr::trans_addr_of::hfcdfbe87e717518flZ6
  57:        0x10240f120 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  58:        0x1023d5d19 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  59:        0x102422651 - middle::trans::expr::trans_adt::h9723c7611e8b803aFJ6
  60:        0x1024106ff - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  61:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  62:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  63:        0x1024206e2 - middle::trans::_match::trans_match::h812670c167266a78prk
  64:        0x1024100fd - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  65:        0x1023d5cf9 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  66:        0x10249d8ee - middle::trans::_match::store_local::closure.126680
  67:        0x10249d7a0 - middle::trans::_match::mk_binding_alloca::h3000336027245779899
  68:        0x10246dd5f - middle::trans::_match::store_local::h8039e9afecf4d4cdKEk
  69:        0x1023d52d4 - middle::trans::base::init_local::h33c895e862085d0aQig
  70:        0x1023d4741 - middle::trans::controlflow::trans_stmt::h85c129a56de52a90GY0
  71:        0x1023d5f48 - middle::trans::controlflow::trans_block::hb6579525f1a9c0e0M30
  72:        0x10247633a - middle::trans::base::trans_closure::h73d516938fc276e1Zah
  73:        0x1023c995f - middle::trans::base::trans_fn::ha1261357d96dceccpmh
  74:        0x1023cb42c - middle::trans::monomorphize::monomorphic_fn::h3beeee8af9d8e21acu0
  75:        0x1023fc857 - middle::trans::callee::trans_fn_ref_with_substs::he50eea051fdbe5a9sH3
  76:        0x1023e0aae - middle::trans::meth::trans_method_callee::ha6757eb79d64f196aAm
  77:        0x10240211b - middle::trans::callee::trans_method_call::closure.122853
  78:        0x1023dd895 - middle::trans::callee::trans_call_inner::hecb2b66e77f79065c33
  79:        0x102401f4d - middle::trans::callee::trans_method_call::hd6c33d4672989d74EY3
  80:        0x10240fb8f - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  81:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  82:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  83:        0x102407fe4 - middle::trans::callee::trans_args::h6215d14867a1f36bno4
  84:        0x1023de92d - middle::trans::callee::trans_call_inner::hecb2b66e77f79065c33
  85:        0x102401f4d - middle::trans::callee::trans_method_call::hd6c33d4672989d74EY3
  86:        0x10240fb8f - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  87:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  88:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  89:        0x102419a98 - middle::trans::expr::trans_addr_of::hfcdfbe87e717518flZ6
  90:        0x10240f120 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  91:        0x1023d5d19 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  92:        0x102422651 - middle::trans::expr::trans_adt::h9723c7611e8b803aFJ6
  93:        0x1024106ff - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  94:        0x10240e911 - middle::trans::expr::trans_unadjusted::hcc11a8a8a1a648574x5
  95:        0x1023d7272 - middle::trans::expr::trans::h13b6cdaa2b8246660Q4
  96:        0x1024206e2 - middle::trans::_match::trans_match::h812670c167266a78prk
  97:        0x1024100fd - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
  98:        0x1023d5cf9 - middle::trans::expr::trans_into::h6b9cbd6894a0b3dd6M4
  99:        0x1023d604e - middle::trans::controlflow::trans_block::hb6579525f1a9c0e0M30
  100:        0x102410168 - middle::trans::expr::trans_rvalue_dps_unadjusted::hea6dad8c49515284vb6
 ... <frames omitted>
