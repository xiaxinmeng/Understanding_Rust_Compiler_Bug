
$ RUST_BACKTRACE=1 rustc --crate-type=lib test.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'path not fully resolved: PathResolution { base_def: DefTy(DefId { krate: 0, node: 4 }, true), last_private: LastMod(AllPublic), depth: 1 }', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc/middle/def.rs:79

stack backtrace:
   1:        0x10863226d - sys::backtrace::write::h4102628905e599dfKDA
   2:        0x10865bc34 - panicking::on_panic::hc3e357b7ac1634e3MsJ
   3:        0x10858d279 - rt::unwind::begin_unwind_inner::h53ac351c4bf63bfaibJ
   4:        0x10858d6fe - rt::unwind::begin_unwind_fmt::had971e57ff3bf48aT9I
   5:        0x1057a0fd0 - middle::const_eval::eval_const_expr_partial::hd2bf46b839f7abc47lh
   6:        0x10579f0dd - middle::const_eval::eval_const_expr_partial::hd2bf46b839f7abc47lh
   7:        0x10579f5f3 - middle::const_eval::eval_const_expr_partial::hd2bf46b839f7abc47lh
   8:        0x10595dc94 - iter::Map<I, F>.Iterator::next::h5439266479542136741
   9:        0x1057b7007 - middle::ty::enum_variants::h533e784536d4c911iP7
  10:        0x105955c9b - middle::ty::type_is_c_like_enum::h1ce3cb9b64f503a3co6
  11:        0x104f2c5fc - check::check_cast::hfe487139a864af4aPao
  12:        0x104f690b1 - check::check_expr_with_unifier::h10849947522957101540
  13:        0x104f3a892 - check::check_expr_with_unifier::check_binop::h7f85f182a25c1bd9Xgq
  14:        0x104f51da9 - check::check_expr_with_unifier::h10953368268981294451
  15:        0x104f70809 - check::check_const_with_ty::hf19906d71ca9eb4ez9r
  16:        0x104f1d66a - check::check_item::hd818d764a5174b83zun
  17:        0x104fdff92 - check_crate::closure.35673
  18:        0x104fdbb40 - check_crate::hf2d34adf6a37b8a3hfC
  19:        0x104d2ccba - driver::phase_3_run_analysis_passes::hc0848bc6836a8617mGa
  20:        0x104d11fb3 - driver::compile_input::hb5f376d824cdbd3fNba
  21:        0x104ddfbe6 - run_compiler::h771e8a13e5d39fa8G6b
  22:        0x104ddd2d0 - thunk::F.Invoke<A, R>::invoke::h7762305853454838659
  23:        0x104ddbe0f - rt::unwind::try::try_fn::h12334561919016735610
  24:        0x1086d5ca8 - rust_try_inner
  25:        0x1086d5c95 - rust_try
  26:        0x104ddc62f - thunk::F.Invoke<A, R>::invoke::h11009727036119889909
  27:        0x1086473e2 - sys::thread::thread_start::h45ac84d363b9c45b32E
  28:     0x7fff95f7a267 - _pthread_body
  29:     0x7fff95f7a1e4 - _pthread_start
