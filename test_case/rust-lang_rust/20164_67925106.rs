
   Compiling pitch_calc v0.8.0 (file:///Users/Mitch/Programming/Rust/pitch_calc)
/Users/Mitch/Programming/Rust/pitch_calc/examples/test.rs:27:9: 27:10 error: internal compiler error: debuginfo::create_for_loop_var_metadata() - Referenced variable location is not an alloca!
/Users/Mitch/Programming/Rust/pitch_calc/examples/test.rs:27     for i in range(0u, 12) {
                                                                     ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:        0x112c042f8 - sys::backtrace::write::he7909da9eb6468e9fBt
   2:        0x112c24c03 - failure::on_fail::hb7ba51d37f883f003Qz
   3:        0x112b90cfa - rt::unwind::begin_unwind_inner::hf4af2222e6b64258uyz
   4:        0x110a91d97 - rt::unwind::begin_unwind::h8565474196370893881
   5:        0x110a91d30 - rt::unwind::begin_unwind::h8565474196370893881
   6:        0x10ff0852d - session::Session::span_bug::hf51aebfd6f9bd43e1on
   7:        0x10f6b5904 - middle::pat_util::pat_bindings::closure.49095
   8:        0x110a5ccd1 - ast_util::walk_pat::h8ae3b0f1f213c350lxC
   9:        0x10f5eaa90 - trans::expr::trans_rvalue_stmt_unadjusted::hd7e98e6041ff86d1l4i
  10:        0x10f5a2835 - trans::expr::trans_into::h24b0113a1e4ebf0euGh
  11:        0x10f5a17cb - trans::controlflow::trans_stmt_semi::he012fdd638652fbcgZd
  12:        0x10f5a2e72 - trans::controlflow::trans_block::hc3b3fec5a3fb9ff79Zd
  13:        0x10f64d234 - trans::base::trans_closure::hd46f470562b1daa76au
  14:        0x10f597622 - trans::base::trans_fn::hbf7352ab9dfcd879nmu
  15:        0x10f592e33 - trans::base::trans_item::hf69d9d491dc08772rHu
  16:        0x10f653d38 - trans::base::trans_crate::h82eea2293a06ce55HDv
  17:        0x10f39550e - driver::phase_4_translate_to_llvm::h3268e7acb055bc171Ca
  18:        0x10f36f2d8 - driver::compile_input::ha7c31ba4e10c023avba
  19:        0x10f4e78e6 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h2822247860475515909
  20:        0x10f4e41e9 - rt::unwind::try::try_fn::h6416506685293600697
  21:        0x112c8ad29 - rust_try_inner
  22:        0x112c8ad16 - rust_try
  23:        0x10f4e48e7 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h15537213600061603954
  24:        0x112c141d4 - sys::thread::thread_start::h3cbfc0fe9ecfb222Kww
  25:     0x7fff886662fc - _pthread_body
  26:     0x7fff88666279 - _pthread_body

Build failed, waiting for other jobs to finish...
Could not compile `pitch_calc`.

To learn more, run the command again with --verbose.

shell returned 101
