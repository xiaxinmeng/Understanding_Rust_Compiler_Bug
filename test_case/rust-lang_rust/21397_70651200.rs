
ice.rs:1:35: 1:51 error: internal compiler error: impossible case reached: bad combination of types for cast
ice.rs:1 pub static mut fail: *const str = "" as *const str;
                                           ^~~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/nathan/prj/rust/rust/src/libsyntax/diagnostic.rs:126

stack backtrace:
   1:     0x7ffbb1e686d0 - sys::backtrace::write::hfb11ddfd0e7f0658eRt
   2:     0x7ffbb1e8a5a0 - failure::on_fail::hbcb8cd329aec6ff6J4z
   3:     0x7ffbb1df8e20 - rt::unwind::begin_unwind_inner::haafd3e91071bc3e1XJz
   4:     0x7ffbaca18810 - rt::unwind::begin_unwind::h6185406380551317121
   5:     0x7ffbaca187a0 - diagnostic::SpanHandler::span_bug::h6c67dceaab6289dcEpF
   6:     0x7ffbb01eb3b0 - session::Session::impossible_case::h3d86ed69bec12c961fp
   7:     0x7ffbb16033e0 - trans::consts::const_expr_unadjusted::h7bbb3fd0d30587f7ado
   8:     0x7ffbb1601640 - trans::consts::const_expr::h62680ecd661789d2h2n
   9:     0x7ffbb153bfe0 - trans::base::get_item_val::heeb56e6127c59fe8IYu
  10:     0x7ffbb1608ce0 - trans::consts::trans_static::h24dedd3ba4c51141fHo
  11:     0x7ffbb1539af0 - trans::base::trans_item::h386d32b9df74fe59iuu
  12:     0x7ffbb1624ce0 - trans::base::trans_crate::h777b6fde95f212b0Lpv
  13:     0x7ffbb23d2d40 - driver::phase_4_translate_to_llvm::he23925e56602061eTNa
  14:     0x7ffbb23b2c30 - driver::compile_input::hb4efdf66e0f650f5Cba
  15:     0x7ffbb247d9a0 - run_compiler::h9d3e14b442c4d365l9b
  16:     0x7ffbb247c110 - thunk::F.Invoke<A, R>::invoke::h14209112065483412921
  17:     0x7ffbb247b070 - rt::unwind::try::try_fn::h1021622462190646014
  18:     0x7ffbb1efa120 - rust_try_inner
  19:     0x7ffbb1efa110 - rust_try
  20:     0x7ffbb247b320 - thunk::F.Invoke<A, R>::invoke::h14035172896832106325
  21:     0x7ffbb1e78210 - sys::thread::thread_start::he2d36a2cbcaf629dzGw
  22:     0x7ffbac233250 - start_thread
  23:     0x7ffbb1aa9219 - clone
  24:                0x0 - <unknown>
