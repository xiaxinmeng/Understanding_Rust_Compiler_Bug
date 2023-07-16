
error: internal compiler error: translating unsupported cast: usize (cast_integral) -> *const str (cast_other)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/nathan/prj/rust/rust/src/libsyntax/diagnostic.rs:185

stack backtrace:
   1:     0x7f373294c6d0 - sys::backtrace::write::hfb11ddfd0e7f0658eRt
   2:     0x7f373296e5a0 - failure::on_fail::hbcb8cd329aec6ff6J4z
   3:     0x7f37328dce20 - rt::unwind::begin_unwind_inner::haafd3e91071bc3e1XJz
   4:     0x7f372d4fc810 - rt::unwind::begin_unwind::h6185406380551317121
   5:     0x7f372d4fd0b0 - diagnostic::Handler::bug::h22dd58059bfb9b94EvF
   6:     0x7f3730a00710 - session::Session::bug::ha3b9a2f7768518e0Ucp
   7:     0x7f373209a930 - trans::expr::trans_imm_cast::h6237da60fefe9c376ek
   8:     0x7f373207e5a0 - trans::expr::trans_unadjusted::he8968cac83a4541c4ii
   9:     0x7f3732037ed0 - trans::expr::trans::h8455960f4e8fd42dCBh
  10:     0x7f3732036d30 - trans::expr::trans_into::h83699b23dbd8bfcf8xh
  11:     0x7f37321378a0 - trans::_match::mk_binding_alloca::h10237289352598662242
  12:     0x7f37320363a0 - trans::base::init_local::h919eb93f221f2a7065s
  13:     0x7f3732037330 - trans::controlflow::trans_block::h2d166cd595ca56b6Z2d
  14:     0x7f3732101a70 - trans::base::trans_closure::h7bc248b8281c1222wWt
  15:     0x7f3732022930 - trans::base::trans_fn::h79df99adfc5fa22276t
  16:     0x7f373201daf0 - trans::base::trans_item::h386d32b9df74fe59iuu
  17:     0x7f3732108ce0 - trans::base::trans_crate::h777b6fde95f212b0Lpv
  18:     0x7f3732eb6d40 - driver::phase_4_translate_to_llvm::he23925e56602061eTNa
  19:     0x7f3732e96c30 - driver::compile_input::hb4efdf66e0f650f5Cba
  20:     0x7f3732f619a0 - run_compiler::h9d3e14b442c4d365l9b
  21:     0x7f3732f60110 - thunk::F.Invoke<A, R>::invoke::h14209112065483412921
  22:     0x7f3732f5f070 - rt::unwind::try::try_fn::h1021622462190646014
  23:     0x7f37329de120 - rust_try_inner
  24:     0x7f37329de110 - rust_try
  25:     0x7f3732f5f320 - thunk::F.Invoke<A, R>::invoke::h14035172896832106325
  26:     0x7f373295c210 - sys::thread::thread_start::he2d36a2cbcaf629dzGw
  27:     0x7f372cd17250 - start_thread
  28:     0x7f373258d219 - clone
  29:                0x0 - <unknown>
