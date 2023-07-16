
assoc.rs:22:18: 22:23 note: cat_expr Errd during dtor check
assoc.rs:22     let b: u32 = a.a();
                             ^~~~~
assoc.rs:22:9: 22:10 warning: unused variable: `b`, #[warn(unused_variables)] on by default
assoc.rs:22     let b: u32 = a.a();
                    ^
error: internal compiler error: fictitious type [type error] in sizing_type_of()
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:189

stack backtrace:
   1:     0x7f3b0334a210 - sys::backtrace::write::h58f3688f47ea9825KlC
   2:     0x7f3b03372ac0 - panicking::on_panic::h6a53d3220988f2eebYL
   3:     0x7f3b032b3720 - rt::unwind::begin_unwind_inner::h72a0c6e103bfcdc3zCL
   4:     0x7f3b0057dde0 - rt::unwind::begin_unwind::h2955836616559227712
   5:     0x7f3b0057e5a0 - diagnostic::Handler::bug::h3ef6141b1fad14d7r5E
   6:     0x7f3b012cc2c0 - session::Session::bug::h912125adfec44a01Snt
   7:     0x7f3b01f928e0 - trans::type_of::sizing_type_of::h71ec04b972aedb71W3o
   8:     0x7f3b02058d70 - trans::base::trans_closure::h8de4dc5484c0f3bephu
   9:     0x7f3b01f77b30 - trans::base::trans_fn::h734239fd8d4c689fGsu
  10:     0x7f3b01f72d50 - trans::base::trans_item::h653e553ae00342d4zRu
  11:     0x7f3b020602e0 - trans::base::trans_crate::hffa644676f991a591Ov
  12:     0x7f3b039d14f0 - driver::phase_4_translate_to_llvm::h7bbc12961d2d7c58qPa
  13:     0x7f3b039ab2d0 - driver::compile_input::hfb1bb38c59866d0fGba
  14:     0x7f3b03a7a510 - run_compiler::hf2a7d9848e51e3b5Zbc
  15:     0x7f3b03a78e10 - thunk::F.Invoke<A, R>::invoke::h12698859262113329290
  16:     0x7f3b03a77d00 - rt::unwind::try::try_fn::h10854600533917764358
  17:     0x7f3b033df880 - rust_try_inner
  18:     0x7f3b033df870 - rust_try
  19:     0x7f3b03a78000 - thunk::F.Invoke<A, R>::invoke::h4174228679526982116
  20:     0x7f3b0335eb60 - sys::thread::thread_start::h60ae7cb39d135ef7R4G
  21:     0x7f3afd23c0c0 - start_thread
  22:     0x7f3b02f2bfd9 - __clone
  23:                0x0 - <unknown>
