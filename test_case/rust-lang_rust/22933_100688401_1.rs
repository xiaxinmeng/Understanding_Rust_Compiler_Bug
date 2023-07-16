
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'path not fully resolved: PathResolution { base_def: DefPrimTy(TyUint(u8)), last_private: LastMod(AllPublic), depth: 1 }', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/middle/def.rs:81

stack backtrace:
   1:     0x7fef208d95a9 - sys::backtrace::write::h602e10b8f312e9d3uhs
   2:     0x7fef208e1529 - panicking::on_panic::h7ec3fe45867bef8cxXw
   3:     0x7fef208a1702 - rt::unwind::begin_unwind_inner::hf035f7930114ad73HCw
   4:     0x7fef208a24a7 - rt::unwind::begin_unwind_fmt::hedeb5f4f1f270aabNBw
   5:     0x7fef1e566f4e - middle::const_eval::eval_const_expr_partial::h6bbfeed9b465166aQii
   6:     0x7fef1e5646e3 - middle::const_eval::eval_const_expr_partial::h6bbfeed9b465166aQii
   7:     0x7fef1fd94395 - astconv::ast_ty_to_ty::h3ba2c06d5e12ed37cXv
   8:     0x7fef1fe12ea9 - collect::type_scheme_of_item::h8830d9c9c58170aeYry
   9:     0x7fef1fe0b51a - collect::convert_typed_item::had5b9e2e37f2991fBwy
  10:     0x7fef1fdf2e34 - collect::convert_item::hf1bc7731f28083aaZFx
  11:     0x7fef1fe385c7 - check_crate::closure.38744
  12:     0x7fef1fe3659b - check_crate::ha4b79837beadc77cQHC
  13:     0x7fef20e2f238 - driver::phase_3_run_analysis_passes::h20da97b9b20a51aetGa
  14:     0x7fef20e1003c - driver::compile_input::h67ec65619cbee40aQba
  15:     0x7fef20ec9c81 - run_compiler::h248a70f938b953f865b
  16:     0x7fef20ec74d2 - boxed::F.FnBox<A>::call_box::h6589873407031420697
  17:     0x7fef20ec6a99 - rt::unwind::try::try_fn::h13793979998646453366
  18:     0x7fef20959828 - rust_try_inner
  19:     0x7fef20959815 - rust_try
  20:     0x7fef20ec6d34 - boxed::F.FnBox<A>::call_box::h12061239028765835470
  21:     0x7fef208e02c1 - sys::thread::Thread::new::thread_start::h8f6e6836281b1b4f3Iv
  22:     0x7fef1a638373 - start_thread
  23:     0x7fef2053127c - clone
  24:                0x0 - <unknown>
