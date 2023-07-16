
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'arithmetic operation overflowed', C:/bot/slave/nightly-dist-rustc-win-64/build/src/libsyntax\codemap.rs:73

stack backtrace:
   1:         0x7121a6fc - sys::backtrace::write::h0dbbf7b67c297decSVA
   2:         0x71235b08 - rt::unwind::register::h77e3f3d90b03e98dXpJ
   3:         0x711833b7 - rt::unwind::begin_unwind_inner::h12ceb964528c2b96mnJ
   4:         0x71183b7b - rt::unwind::begin_unwind_fmt::ha1e3b759bf51a254XlJ
   5:         0x7123573a - rust_begin_unwind
   6:         0x7124c2a9 - panicking::panic_fmt::h55855a4db6daddf2IZs
   7:         0x712488e1 - panicking::panic::h4d7abbe447750a26UXs
   8:           0x51260e - codemap::BytePos.Sub::sub::hb4e9bb288333e96exYB
   9:           0xbd7f48 - middle::astencode::_&'a DecodeContext<'b, 'c, 'tcx>.ast_map..FoldOps::new_span::h2c2deff2e015ae41qqa
  10:           0xbe824a - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  11:           0xbe7f77 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  12:           0xbeb70f - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  13:           0xbe848b - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  14:           0xbe7f77 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  15:           0xbeb6dc - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  16:           0xbebd41 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  17:           0xbeb859 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  18:           0xc0a706 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  19:           0xc09ea2 - middle::astencode::rbml..Doc<'a>.doc_decoder_helpers::opt_child::hc02538a1a994b8cb7Tb
  20:           0xbdaa0b - middle::astencode::decode_inlined_item::h4b5ecdf19d937b3aPqa
  21:         0x635f1b0a - trans::base::_InsnCtxt.Drop::drop::h23e8c6aa94978b9eYmr
  22:           0xfa1efe - metadata::decoder::maybe_get_item_ast::hc63081888c5999fbPDi
  23:           0xdbd1a5 - metadata::csearch::maybe_get_item_ast::h01114cd4d0fa035fo3k
  24:         0x635f03fd - back::write::run_assembler::h9b10d62e3415d47666c
  25:         0x6363cd69 - trans::common::ExprOrMethodCall...std..fmt..Debug::fmt::h2639905fc5fb8c20SAl
  26:         0x6363c087 - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  27:         0x63638848 - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  28:         0x6364ea25 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  29:         0x6365b6fb - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  30:         0x6360e087 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  31:         0x6360ee02 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  32:         0x636dea2a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h820ae59272a71f02HTs
  33:         0x635f7ffd - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  34:         0x635f92c0 - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  35:         0x6363cf14 - trans::common::ExprOrMethodCall...std..fmt..Debug::fmt::h2639905fc5fb8c20SAl
  36:         0x6363b5bb - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  37:         0x63638a5e - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  38:         0x6364ea25 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  39:         0x6365b6fb - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  40:         0x6360e087 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  41:         0x63716b3d - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::h460f09245e4f2e80ZGw
  42:         0x6360d4e2 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_ast_cleanup_scope::h2e3d819ba03b0988SJJ
  43:         0x6360eb63 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  44:         0x636dea2a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h820ae59272a71f02HTs
  45:         0x635f7ffd - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  46:         0x635f942c - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  47:         0x6363cf14 - trans::common::ExprOrMethodCall...std..fmt..Debug::fmt::h2639905fc5fb8c20SAl
  48:         0x6363b5bb - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  49:         0x63638a5e - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  50:         0x6364ea25 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  51:         0x6365b6fb - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  52:         0x6360e087 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  53:         0x6360ee02 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  54:         0x636dea2a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h820ae59272a71f02HTs
  55:         0x635f7ffd - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  56:         0x635f942c - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  57:         0x6363cf14 - trans::common::ExprOrMethodCall...std..fmt..Debug::fmt::h2639905fc5fb8c20SAl
  58:         0x6363b5bb - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  59:         0x63651b84 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  60:         0x63650293 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  61:         0x63659f70 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  62:         0x63658d8e - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  63:         0x6360f77f - trans::expr::Dest...std..cmp..PartialEq::eq::ha661cb7533fe26abOmh
  64:         0x6366fc5d - trans::common::erase_regions::RegionEraser<'a, 'tcx>.TypeFolder<'tcx>::fold_substs::hcee31f2ecd0cd8c3Zvk
  65:         0x63658e76 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  66:         0x6360e0a8 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  67:         0x6360ee02 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  68:         0x636dea2a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h820ae59272a71f02HTs
  69:         0x635f7ffd - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  70:         0x635f942c - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  71:         0x6363cf14 - trans::common::ExprOrMethodCall...std..fmt..Debug::fmt::h2639905fc5fb8c20SAl
  72:         0x6363b5bb - trans::context::CrateContext<'b, 'tcx>::tn::head2a03b8133cd76d7l
  73:         0x63651b84 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  74:         0x63650293 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  75:         0x63659f70 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h90e475dde6fd53b6BDl
  76:         0x6360e087 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  77:         0x63716b3d - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::h460f09245e4f2e80ZGw
  78:         0x6360d4e2 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_ast_cleanup_scope::h2e3d819ba03b0988SJJ
  79:         0x6360eb63 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hac8b50cc79571f06wTJ
  80:         0x636dea2a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h820ae59272a71f02HTs
  81:         0x635f7ffd - trans::context::CrateContext<'b, 'tcx>::sess::h1bfe1f4cd70e1d25w4l
  82:         0x635f4d8b - trans::context::CrateContext<'b, 'tcx>::stats::h974340237ce9f5d56fm
