
error: internal compiler error: type_of with ty_projection
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', C:\bot\slave\nightly-dist-rustc-win-64\build\src\libsyntax\diagnostic.rs:182

stack backtrace:
   1:         0x69beba0b - sys::backtrace::write::h2c4bbe6a26c9ec2cw9t
   2:         0x69bff0a3 - rt::unwind::register::hcae04167a6124eaaeTz
   3:         0x69b8363f - rt::unwind::begin_unwind_inner::h08078ef443debc4eNQz
   4:         0x6f89baec - diagnostic::SpanHandler::span_bug::h1e8e97aea6f54bd6M0F
   5:         0x6f89c421 - diagnostic::Handler::bug::h60d532eb9dd277d9S6F
   6:           0x711f59 - session::Session::bug::h046df32b27f83592cbr
   7:           0xfb9d68 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope::h464d2d3abcefaeb9VWK
   8:           0xfba2ba - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope::h464d2d3abcefaeb9VWK
   9:           0xfb9fd0 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope::h464d2d3abcefaeb9VWK
  10:           0xfb9aec - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope::h464d2d3abcefaeb9VWK
  11:          0x10cff05 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  12:          0x10cfb6a - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  13:          0x10cc76f - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  14:          0x10ce790 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  15:          0x10c9442 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  16:          0x10cff30 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  17:          0x10cfb6a - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  18:          0x10cc76f - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  19:          0x10ce790 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  20:          0x10c9442 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  21:          0x10d5996 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  22:          0x10c8958 - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2fdc29f4a21f53d3PTD
  23:          0x10723ea - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h2e47fdc8586e307bWut
  24:           0xfd7608 - trans::base::StatRecorder<'a, 'tcx>.Drop::drop::h2388b1c31fcc139emWr
  25:          0x1077252 - trans::base::IsUnboxedClosureFlag...std..clone..Clone::clone::h5e97d8ec7e80ca9e60t
  26:           0xf9df82 - trans::context::CrateContext<'b, 'tcx>::sess::hd6cb1cf9ebd5d9301tm
  27:           0xf9f54a - trans::context::CrateContext<'b, 'tcx>::sess::hd6cb1cf9ebd5d9301tm
  28:           0xfee9ab - trans::common::ExprOrMethodCall...std..fmt..Show::fmt::haf69276d9baa0715WWl
  29:           0xfec47e - trans::context::CrateContext<'b, 'tcx>::tn::hd0cb0f6fe17abe9eSwm
  30:           0xfca539 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::hfe6e646bf95998a716K
  31:           0xff45e3 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  32:           0xff88ac - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  33:           0xff76b9 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  34:           0xfb2d75 - trans::expr::Dest...std..cmp..PartialEq::eq::h3d06ba9fd9435851Mvh
  35:           0xfc3b08 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_custom_cleanup_scope::h7a9d35dce2d79c8834K
  36:           0xff553a - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  37:           0xff88ac - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  38:           0xff76b9 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  39:           0xfb2d75 - trans::expr::Dest...std..cmp..PartialEq::eq::h3d06ba9fd9435851Mvh
  40:           0xfff0fd - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  41:           0xfb2006 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  42:           0xfb1158 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_ast_cleanup_scope::hb93f7672cd708912NPK
  43:           0xfb262d - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  44:          0x1079562 - trans::base::IsUnboxedClosureFlag...std..clone..Clone::clone::h5e97d8ec7e80ca9e60t
  45:           0xf9df82 - trans::context::CrateContext<'b, 'tcx>::sess::hd6cb1cf9ebd5d9301tm
  46:           0xf9f54a - trans::context::CrateContext<'b, 'tcx>::sess::hd6cb1cf9ebd5d9301tm
  47:           0xfee9ab - trans::common::ExprOrMethodCall...std..fmt..Show::fmt::haf69276d9baa0715WWl
  48:           0xfccab3 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_custom_cleanup_scope::hfe6e646bf95998a716K
  49:           0xff45e3 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  50:           0xff88ac - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  51:           0xfb1db1 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  52:           0xfb291f - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  53:           0xff9944 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  54:           0xfb1db1 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  55:          0x1095233 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::schedule_drop_and_zero_mem::hed712752bdc94a84KeL
  56:           0xff9614 - trans::type_::Type...std..cmp..PartialEq::ne::h12a4f5f13238ba78UZJ
  57:           0xfb1db1 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  58:           0xfb291f - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::h571d6783ced12feft0K
  59:          0x1079562 - trans::base::IsUnboxedClosureFlag...std..clone..Clone::clone::h5e97d8ec7e80ca9e60t
  60:           0xf9df82 - trans::context::CrateContext<'b, 'tcx>::sess::hd6cb1cf9ebd5d9301tm
  61:           0xf9a231 - trans::context::CrateContext<'b, 'tcx>::stats::h064e64c108f865e3XGm
  62:          0x107af88 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_custom_cleanup_scope_with_debug_loc::h76b9726feef793e0NYK
  63:           0xf99722 - trans::context::CrateContext<'b, 'tcx>::stats::h064e64c108f865e3XGm
  64:          0x107f6c8 - trans::base::trans_crate::h3ae41358cc04fdd1cvv
  65:         0x70b240ca - driver::phase_4_translate_to_llvm::h1e5993987ab6d536YMa
  66:         0x70b0264d - driver::compile_input::ha5dff1685b5f89bbAba
  67:         0x70bc0037 - run::he5c8e3699f0298dcM4b
  68:         0x70bbe3b8 - run::he5c8e3699f0298dcM4b
  69:         0x70bbd23a - run::he5c8e3699f0298dcM4b
  70:         0x69c28c0c - rust_try
  71:         0x69c28be9 - rust_try
  72:         0x70bbd86f - run::he5c8e3699f0298dcM4b
  73:         0x69bf1002 - sys::tcp::TcpListener::bind::he3cbb57691b44c3fZGw
  74:         0x774f59ed - BaseThreadInitThunk
