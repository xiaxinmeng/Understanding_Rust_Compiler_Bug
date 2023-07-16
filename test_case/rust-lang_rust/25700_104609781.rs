

error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Unexpected type returned from struct_tail: core::option::Option<parser_combinators::primitives::Consumed<parser_combinators::primitives::ParseError>> for ty=parser_combinators::combinator::Iter<&'static mut parser_combinators::combinator::Map<parser_combinators::combinator::Value<&'static str, collections::string::String>, [closure((<collections::string::String as AstId>::Untyped,)) -> <collections::string::String as AstId>::Untyped]>>', C:/bot/slave/nightly-dist-rustc-win-64/build/src/librustc_trans\trans\type_of.rs:354

stack backtrace:
   1:         0x63cb8722 - sys::backtrace::write::h17240dc0eaceafe7Rgs
   2:         0x63cc1b96 - rt::unwind::register::hf97832460d72cd16gOv
   3:         0x63c85390 - rt::unwind::begin_unwind_inner::hcf167426fb12ee63qLv
   4:         0x63c85d87 - rt::unwind::begin_unwind_fmt::heb10265bd18220dfwKv
   5:         0x6abc0bc4 - trans::context::CrateContext<'b, 'tcx>::report_overbig_object::hdc4d78a07437d834GXt
   6:         0x6abd5a15 - trans::context::CrateContext<'b, 'tcx>::get_intrinsic::h06903648e28e2397lKt
   7:         0x6acd54fa - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::hc923a533a27d041b5qI
   8:         0x6abd45e7 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::needs_invoke::h867326256dc0c0ab0xp
   9:         0x6abe397d - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h65206fced3061d72K2o
  10:         0x6abe2073 - trans::datum::Rvalue.KindOps::post_store::h62afad44cee1556apFv
  11:         0x6abe4234 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::h4e8777762ff791d0yap
  12:         0x6ac23851 - trans::context::CrateContext<'b, 'tcx>::fn_pointer_shims::h6740281c1906aee88Nt
  13:         0x6ac13bcf - trans::datum::Expr.KindOps::is_by_ref::he67898a485dd01554Hv
  14:         0x6ac28db5 - trans::debuginfo::ast..Expr.ToDebugLoc::debug_loc::h2b8f89cc044a8ffalGz
  15:         0x6ac25d1c - trans::debuginfo::ast..Expr.ToDebugLoc::debug_loc::h2b8f89cc044a8ffalGz
  16:         0x6ac85402 - trans::expr::Dest...std..clone..Clone::clone::h2d8a6879a209d263z9z
  17:         0x6ac5cf95 - trans::context::CrateContext<'b, 'tcx>::check_drop_flag_for_sanity::h93f0f1f9855d31d6CYt
  18:         0x6abe3c30 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h65206fced3061d72K2o
  19:         0x6abe2073 - trans::datum::Rvalue.KindOps::post_store::h62afad44cee1556apFv
  20:         0x6abe4234 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::h4e8777762ff791d0yap
  21:         0x6ac23851 - trans::context::CrateContext<'b, 'tcx>::fn_pointer_shims::h6740281c1906aee88Nt
  22:         0x6ac13bcf - trans::datum::Expr.KindOps::is_by_ref::he67898a485dd01554Hv
  23:         0x6ac12533 - trans::datum::Expr.KindOps::is_by_ref::he67898a485dd01554Hv
  24:         0x6ac2763e - trans::debuginfo::ast..Expr.ToDebugLoc::debug_loc::h2b8f89cc044a8ffalGz
  25:         0x6ac25d1c - trans::debuginfo::ast..Expr.ToDebugLoc::debug_loc::h2b8f89cc044a8ffalGz
  26:         0x6ac85402 - trans::expr::Dest...std..clone..Clone::clone::h2d8a6879a209d263z9z
  27:         0x6ac5cf95 - trans::context::CrateContext<'b, 'tcx>::check_drop_flag_for_sanity::h93f0f1f9855d31d6CYt
  28:         0x6abe3c30 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h65206fced3061d72K2o
  29:         0x6abe2073 - trans::datum::Rvalue.KindOps::post_store::h62afad44cee1556apFv
  30:         0x6ac30671 - trans::context::CrateContext<'b, 'tcx>::closure_vals::hf48aea36c2fc6b90aVt
  31:         0x6ac8630c - trans::expr::Dest...std..clone..Clone::clone::h2d8a6879a209d263z9z
  32:         0x6ac84713 - trans::expr::Dest...std..clone..Clone::clone::h2d8a6879a209d263z9z
  33:         0x6abc58e5 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_custom_cleanup_scope::h70d9afa64f76b22fG8o
  34:         0x6ac1fe7f - trans::context::CrateContext<'b, 'tcx>::fn_pointer_shims::h6740281c1906aee88Nt
  35:         0x6ac25286 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::hc61d8507405d927clds
  36:         0x6ac8626a - trans::expr::Dest...std..clone..Clone::clone::h2d8a6879a209d263z9z
  37:         0x6ac5cf95 - trans::context::CrateContext<'b, 'tcx>::check_drop_flag_for_sanity::h93f0f1f9855d31d6CYt
  38:         0x6ac5cb28 - trans::context::CrateContext<'b, 'tcx>::check_drop_flag_for_sanity::h93f0f1f9855d31d6CYt
  39:         0x6abe3952 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h65206fced3061d72K2o
  40:         0x6abe2073 - trans::datum::Rvalue.KindOps::post_store::h62afad44cee1556apFv
  41:         0x6abe4234 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::h4e8777762ff791d0yap
  42:         0x6abe6b99 - trans::base::TransItemVisitor<'a, 'tcx>.Visitor<'v>::visit_item::h0ea287790196486dCbi
  43:         0x6abf4816 - trans::base::trans_crate::h467a88bab46e74c484i
  44:         0x6fca8ce6 - driver::phase_4_translate_to_llvm::hac244219d972fac5nOa
  45:         0x6fc82d31 - driver::compile_input::h8ffd252a42277d1cQba
  46:         0x6fd352e3 - run_compiler::h7a90a6d5e81af66175b
  47:         0x6fd32e6f - run::h27aab490c2c97a88N5b
  48:         0x6fd32429 - run::h27aab490c2c97a88N5b
  49:         0x63cfb2dc - rust_try
  50:         0x63cfb2b9 - rust_try
  51:         0x6fd326a5 - run::h27aab490c2c97a88N5b
  52:         0x63cbfe54 - sys::process::Command::cwd::h8d65332726b1cb469uu
  53:         0x776559cd - BaseThreadInitThunk

Could not compile `rust-error`.

To learn more, run the command again with --verbose.
