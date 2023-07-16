 terminal
   Compiling rustc-serialize v0.3.1
   Compiling libc v0.1.2
   Compiling gcc v0.3.1
   Compiling log v0.2.5
   Compiling rand v0.1.4
   Compiling time v0.1.19
   Compiling rust-crypto v0.2.19 (file:///Users/dragan/Code/OpenSource/RustCrypto)
thread 'rustc' panicked at 'tried to get overflow intrinsic for non-int type', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc_trans/trans/expr.rs:2352

stack backtrace:
   1:        0x1113fdb52 - sys::backtrace::write::h6b0889bc971c1ad3IDA
   2:        0x11142c844 - panicking::on_panic::h0fcd1d74630df38dKsJ
   3:        0x11134ac67 - rt::unwind::begin_unwind_inner::h56b969a14fc61916gbJ
   4:        0x10de4185f - rt::unwind::begin_unwind::h5082286894643109956
   5:        0x10df219d1 - trans::expr::with_overflow_check::hd6793b92ae2aee44ylk
   6:        0x10df1ff05 - trans::expr::trans_eager_binop::h416c33bf491e0717upj
   7:        0x10df03e5f - trans::expr::trans_assign_op::hc0d59ae1da070adeDYj
   8:        0x10deec30f - trans::expr::trans_rvalue_stmt_unadjusted::h450cf67b08184907jui
   9:        0x10de96086 - trans::expr::trans_into::h4a29f680db4b3526znh
  10:        0x10de94fa0 - trans::controlflow::trans_stmt_semi::h318c4f3f34b9c533o4d
  11:        0x10de96ace - trans::controlflow::trans_block::h26a308528bb95051b5d
  12:        0x10dee8050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  13:        0x10de95f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  14:        0x10de94fa0 - trans::controlflow::trans_stmt_semi::h318c4f3f34b9c533o4d
  15:        0x10de96ace - trans::controlflow::trans_block::h26a308528bb95051b5d
  16:        0x10dee8050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  17:        0x10de95f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  18:        0x10df974e0 - trans::_match::trans_match_inner::he768a1b2ccde6888CIw
  19:        0x10dee7fe3 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  20:        0x10de95f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  21:        0x10de96e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  22:        0x10deebf9e - trans::expr::trans_rvalue_stmt_unadjusted::h450cf67b08184907jui
  23:        0x10de96086 - trans::expr::trans_into::h4a29f680db4b3526znh
  24:        0x10df974e0 - trans::_match::trans_match_inner::he768a1b2ccde6888CIw
  25:        0x10dee7fe3 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  26:        0x10de95f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  27:        0x10dfafd26 - trans::_match::mk_binding_alloca::h3448676421097350985
  28:        0x10de95280 - trans::base::init_local::h08a9fcff749ad013czs
  29:        0x10de96b02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  30:        0x10dee8050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  31:        0x10de95f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  32:        0x10de94fa0 - trans::controlflow::trans_stmt_semi::h318c4f3f34b9c533o4d
  33:        0x10de96ace - trans::controlflow::trans_block::h26a308528bb95051b5d
  34:        0x10df75da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  35:        0x10de7e645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  36:        0x10de7aaa6 - trans::base::trans_item::h8ce277cc13b040a3vTt
  37:        0x10de7a8a7 - trans::base::trans_item::h8ce277cc13b040a3vTt
  38:        0x10df7eecb - trans::base::trans_crate::hb69c466e59630a14GPu
  39:        0x10d8bbec7 - driver::phase_4_translate_to_llvm::he4e045cdc3897b4crNa
  40:        0x10d8977f2 - driver::compile_input::h25d96a14ec26932cIba
  41:        0x10d96972e - run_compiler::h15c3ea085a111a6fH5b
  42:        0x10d966ac7 - thunk::F.Invoke<A, R>::invoke::h11905936354613679413
  43:        0x10d96560f - rt::unwind::try::try_fn::h4025213131938170097
  44:        0x1114a9628 - rust_try_inner
  45:        0x1114a9615 - rust_try
  46:        0x10d965e48 - thunk::F.Invoke<A, R>::invoke::h13725635537516470714
  47:        0x1114150a2 - sys::thread::thread_start::h1120d6d2e105321012E
  48:     0x7fff8368d898 - _pthread_body
  49:     0x7fff8368d729 - _pthread_start
