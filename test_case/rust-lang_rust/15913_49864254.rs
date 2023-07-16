
$ RUST_BACKTRACE=1 rustc src/sodiumoxide/lib.rs
error: internal compiler error: adt::represent_type called on non-ADT type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/ast_util.rs:784
stack backtrace:
   1:     0x7f83d9603f00 - rt::backtrace::imp::write::h93db0ef72e375f6eu7p
   2:     0x7f83d96072c0 - <unknown>
   3:     0x7f83dd5e3eb0 - unwind::begin_unwind_inner::h524249902aa36692s1d
   4:     0x7f83da3b29f0 - <unknown>
   5:     0x7f83da3b3460 - diagnostic::Handler::bug::he7407140be962954H9C
   6:     0x7f83ddc27780 - driver::session::Session::bug::hf704959287c7d212kBx
   7:     0x7f83de000350 - middle::trans::adt::represent_type::haae18780ee897b83pqr
   8:     0x7f83de0268d0 - middle::trans::base::trans_named_tuple_constructor::h9bf7c1f3693bdde31Rc
   9:     0x7f83de01abb0 - middle::trans::callee::trans_call_inner::h0453f759b5273437zl0
  10:     0x7f83de01e260 - middle::trans::callee::trans_call::hcce5cbe6fcff0cf4Hf0
  11:     0x7f83de02f0c0 - <unknown>
  12:     0x7f83de02e2b0 - <unknown>
  13:     0x7f83ddff0680 - middle::trans::expr::trans::h5d5357a078d45fefOi1
  14:     0x7f83de03a3a0 - <unknown>
  15:     0x7f83de02e2b0 - <unknown>
  16:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  17:     0x7f83de0c95a0 - <unknown>
  18:     0x7f83de0c9240 - <unknown>
  19:     0x7f83de097840 - middle::trans::_match::store_local::ha45caeae0b2b712aX2f
  20:     0x7f83ddfee3c0 - middle::trans::base::init_local::h5818fcef1f2cbc80lQb
  21:     0x7f83ddfed9a0 - middle::trans::controlflow::trans_stmt::h66c43e395626280ceqX
  22:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  23:     0x7f83de09d680 - middle::trans::base::trans_closure::hc94081271c0d25c4RAc
  24:     0x7f83ddfe3330 - middle::trans::base::trans_fn::hf8f0cabd10dd337axNc
  25:     0x7f83ddfde220 - middle::trans::base::trans_item::h69574bc80cb65706T5c
  26:     0x7f83ddfd93b0 - middle::trans::inline::maybe_instantiate_inline::he825e49d75727fd64IW
  27:     0x7f83de01e4c0 - <unknown>
  28:     0x7f83de01abb0 - middle::trans::callee::trans_call_inner::h0453f759b5273437zl0
  29:     0x7f83de01e260 - middle::trans::callee::trans_call::hcce5cbe6fcff0cf4Hf0
  30:     0x7f83de02f0c0 - <unknown>
  31:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  32:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  33:     0x7f83de09d680 - middle::trans::base::trans_closure::hc94081271c0d25c4RAc
  34:     0x7f83ddfe3330 - middle::trans::base::trans_fn::hf8f0cabd10dd337axNc
  35:     0x7f83ddfde220 - middle::trans::base::trans_item::h69574bc80cb65706T5c
  36:     0x7f83ddfd93b0 - middle::trans::inline::maybe_instantiate_inline::he825e49d75727fd64IW
  37:     0x7f83de01e4c0 - <unknown>
  38:     0x7f83de01abb0 - middle::trans::callee::trans_call_inner::h0453f759b5273437zl0
  39:     0x7f83de01e260 - middle::trans::callee::trans_call::hcce5cbe6fcff0cf4Hf0
  40:     0x7f83de02f0c0 - <unknown>
  41:     0x7f83de02e2b0 - <unknown>
  42:     0x7f83ddff0680 - middle::trans::expr::trans::h5d5357a078d45fefOi1
  43:     0x7f83de03a3a0 - <unknown>
  44:     0x7f83de02e2b0 - <unknown>
  45:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  46:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  47:     0x7f83de02f0c0 - <unknown>
  48:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  49:     0x7f83ddfef810 - middle::trans::controlflow::trans_if::hab0fef4fd5086e0d7yX
  50:     0x7f83de02f0c0 - <unknown>
  51:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  52:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  53:     0x7f83de09d680 - middle::trans::base::trans_closure::hc94081271c0d25c4RAc
  54:     0x7f83ddfe3330 - middle::trans::base::trans_fn::hf8f0cabd10dd337axNc
  55:     0x7f83ddfe3960 - middle::trans::monomorphize::monomorphic_fn::hef7bfa5bc3657781kTW
  56:     0x7f83de0124c0 - middle::trans::callee::trans_fn_ref_with_vtables::hbfb866dbab3ad07bkUZ
  57:     0x7f83de00fae0 - middle::trans::callee::trans_fn_ref::hfdd0462e8c9bc23bHGZ
  58:     0x7f83de01e4c0 - <unknown>
  59:     0x7f83de01abb0 - middle::trans::callee::trans_call_inner::h0453f759b5273437zl0
  60:     0x7f83de01e260 - middle::trans::callee::trans_call::hcce5cbe6fcff0cf4Hf0
  61:     0x7f83de02f0c0 - <unknown>
  62:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  63:     0x7f83de031d40 - <unknown>
  64:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  65:     0x7f83ddfee2a0 - middle::trans::controlflow::trans_stmt_semi::h660644549fa5ef45BuX
  66:     0x7f83ddfed9a0 - middle::trans::controlflow::trans_stmt::h66c43e395626280ceqX
  67:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  68:     0x7f83de02f0c0 - <unknown>
  69:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  70:     0x7f83ddfee2a0 - middle::trans::controlflow::trans_stmt_semi::h660644549fa5ef45BuX
  71:     0x7f83ddfed9a0 - middle::trans::controlflow::trans_stmt::h66c43e395626280ceqX
  72:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  73:     0x7f83ddfef810 - middle::trans::controlflow::trans_if::hab0fef4fd5086e0d7yX
  74:     0x7f83de02f0c0 - <unknown>
  75:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  76:     0x7f83ddfee2a0 - middle::trans::controlflow::trans_stmt_semi::h660644549fa5ef45BuX
  77:     0x7f83ddfed9a0 - middle::trans::controlflow::trans_stmt::h66c43e395626280ceqX
  78:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  79:     0x7f83de09d680 - middle::trans::base::trans_closure::hc94081271c0d25c4RAc
  80:     0x7f83ddfe3330 - middle::trans::base::trans_fn::hf8f0cabd10dd337axNc
  81:     0x7f83ddfe3960 - middle::trans::monomorphize::monomorphic_fn::hef7bfa5bc3657781kTW
  82:     0x7f83de0124c0 - middle::trans::callee::trans_fn_ref_with_vtables::hbfb866dbab3ad07bkUZ
  83:     0x7f83de00fae0 - middle::trans::callee::trans_fn_ref::hfdd0462e8c9bc23bHGZ
  84:     0x7f83de01f6d0 - middle::trans::meth::trans_method_callee::h3e6083c3e4f7619crfi
  85:     0x7f83de01f640 - <unknown>
  86:     0x7f83de01abb0 - middle::trans::callee::trans_call_inner::h0453f759b5273437zl0
  87:     0x7f83de01ef50 - middle::trans::callee::trans_method_call::hc97b1bb139b43543Pg0
  88:     0x7f83de02f0c0 - <unknown>
  89:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  90:     0x7f83ddfee2a0 - middle::trans::controlflow::trans_stmt_semi::h660644549fa5ef45BuX
  91:     0x7f83ddfed9a0 - middle::trans::controlflow::trans_stmt::h66c43e395626280ceqX
  92:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  93:     0x7f83de02f0c0 - <unknown>
  94:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  95:     0x7f83ddfef480 - middle::trans::controlflow::trans_block::h4c308ba8a257ffa2uvX
  96:     0x7f83de02f0c0 - <unknown>
  97:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1
  98:     0x7f83de0412d0 - middle::trans::_match::trans_match::hd8d5500b2757b5cdzTf
  99:     0x7f83de02f0c0 - <unknown>
  100:     0x7f83ddfeee70 - middle::trans::expr::trans_into::h779f39ecdca2f861Te1


 $ RUST_LOG=debug rustc src/sodiumoxide/lib.rs | tail -n 40
 DEBUG:syntax::print::pp: pp ~[0,0]
 DEBUG:syntax::print::pp: pp String('je_rallocx')/buffer ~[0,0]
 DEBUG:syntax::print::pp: check_stream ~[0, 1] with left_total=1, right_total=11
 DEBUG:syntax::print::pp: pp ~[0,1]
 DEBUG:syntax::print::pp: pp End/buffer ~[0,1]
 DEBUG:syntax::print::pp: scan_push 2
 DEBUG:syntax::print::pp: pp ~[0,2]
 DEBUG:syntax::print::pp: advance_left ~[0,2], sizeof(0)=10
 DEBUG:syntax::print::pp: print BEGIN 10 (remaining line space=78)
 DEBUG:syntax::print::pp: [10=BEGIN, 10=STR(je_rallocx,10)]
 DEBUG:syntax::print::pp: print Begin -> push fitting block
 DEBUG:syntax::print::pp: advance_left ~[1,2], sizeof(1)=10
 DEBUG:syntax::print::pp: print STR(je_rallocx,10) 10 (remaining line space=78)
 DEBUG:syntax::print::pp: [10=STR(je_rallocx,10)]
 DEBUG:syntax::print::pp: print String(je_rallocx)
 DEBUG:syntax::print::pp: advance_left ~[2,2], sizeof(2)=1
 DEBUG:syntax::print::pp: print END 1 (remaining line space=68)
 DEBUG:syntax::print::pp: []
 DEBUG:syntax::print::pp: print End -> pop End
 DEBUG:syntax::print::pp: INDENT 0
 DEBUG:rustc::middle::trans::callee: trans_def(def=DefFn(syntax::ast::DefId{krate: 2u32, node: 269u32}, UnsafeFn), ref_expr=expr(6606: je_rallocx))
 DEBUG:rustc::middle::trans::base: new InsnCtxt: maybe_instantiate_inline
 DEBUG:rustc::metadata::decoder: Looking up item: 269
 DEBUG:rustc::middle::trans::cleanup: pop_custom_cleanup_scope(5)
 DEBUG:rustc::middle::trans::cleanup: popping cleanup scope _custom_, 5 scopes remaining
 DEBUG:rustc::middle::trans::adt: Representing: *mut libc::types::common::c95::c_void
 error: internal compiler error: adt::represent_type called on non-ADT type
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 DEBUG:rustc::util::common: <<
 note: the compiler hit an unexpected failure path. this is a bug.
 note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
 note: run with `RUST_BACKTRACE=1` for a backtrace
 task 'rustc' failed at 'Box<Any>', /build/rust-git/src/rust/src/libsyntax/ast_util.rs:784
