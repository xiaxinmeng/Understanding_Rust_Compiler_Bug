
$ RUST_BACKTRACE=1 cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling log v0.2.5
   Compiling rustc-serialize v0.3.1
   Compiling libc v0.1.2
   Compiling gcc v0.3.1
   Compiling rand v0.1.4
   Compiling time v0.1.19
   Compiling rust-crypto v0.2.19 (file:///home/parbo/src/rust-crypto)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'tried to get overflow intrinsic for non-int type', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans/expr.rs:2352

stack backtrace:
   1:     0x7fa61457bf1f - sys::backtrace::write::hf79a3da4fdecb8a0OBA
   2:     0x7fa6145a6c32 - panicking::on_panic::h9f64f4c69e19f194hHJ
   3:     0x7fa6144dbeda - rt::unwind::begin_unwind_inner::h37f4496c980fe936knJ
   4:     0x7fa613b7eddc - rt::unwind::begin_unwind::h265200097513870162
   5:     0x7fa613c55ba6 - trans::expr::with_overflow_check::hb51eee44b3064a1aylk
   6:     0x7fa613c5423a - trans::expr::trans_eager_binop::hd0a96011514b6b88upj
   7:     0x7fa613c38c2a - trans::expr::trans_assign_op::h2b754fac4ce95c82DYj
   8:     0x7fa613c21e5c - trans::expr::trans_rvalue_stmt_unadjusted::ha3dc197598d6fe14jui
   9:     0x7fa613bcf510 - trans::expr::trans_into::h95c6d2681fdd2548znh
  10:     0x7fa613bce594 - trans::controlflow::trans_stmt_semi::h3c27cfa3f0150db6o4d
  11:     0x7fa613bcfed0 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  12:     0x7fa613c1df7e - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  13:     0x7fa613bcf3f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  14:     0x7fa613bce594 - trans::controlflow::trans_stmt_semi::h3c27cfa3f0150db6o4d
  15:     0x7fa613bcfed0 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  16:     0x7fa613c1df7e - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  17:     0x7fa613bcf3f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  18:     0x7fa613cc7df0 - trans::_match::trans_match_inner::hed8323987dcd430bCIw
  19:     0x7fa613c1df22 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  20:     0x7fa613bcf3f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  21:     0x7fa613bd0227 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  22:     0x7fa613c21b19 - trans::expr::trans_rvalue_stmt_unadjusted::ha3dc197598d6fe14jui
  23:     0x7fa613bcf510 - trans::expr::trans_into::h95c6d2681fdd2548znh
  24:     0x7fa613cc7df0 - trans::_match::trans_match_inner::hed8323987dcd430bCIw
  25:     0x7fa613c1df22 - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  26:     0x7fa613bcf3f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  27:     0x7fa613cdf967 - trans::_match::mk_binding_alloca::h13035368140960659810
  28:     0x7fa613bce7fd - trans::base::init_local::h1e7c96bb7077440dczs
  29:     0x7fa613bcff02 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  30:     0x7fa613c1df7e - trans::expr::trans_rvalue_dps_unadjusted::hd47de7ac66e018254zi
  31:     0x7fa613bcf3f6 - trans::expr::trans_into::h95c6d2681fdd2548znh
  32:     0x7fa613bce594 - trans::controlflow::trans_stmt_semi::h3c27cfa3f0150db6o4d
  33:     0x7fa613bcfed0 - trans::controlflow::trans_block::h3e86dfa8c58560e6b5d
  34:     0x7fa613ca7821 - trans::base::trans_closure::hab3cc3c679d5ff23Kkt
  35:     0x7fa613bb8b08 - trans::base::trans_fn::he0569b8eb832adf9Dvt
  36:     0x7fa613bb5241 - trans::base::trans_item::h48fc370b7d259ac7vTt
  37:     0x7fa613bb5078 - trans::base::trans_item::h48fc370b7d259ac7vTt
  38:     0x7fa613cb07ec - trans::base::trans_crate::hc92be67ede893c70GPu
  39:     0x7fa614bb4e83 - driver::phase_4_translate_to_llvm::h9904f5d5fc3fb761rNa
  40:     0x7fa614b9083f - driver::compile_input::h3913ff7013f0c056Iba
  41:     0x7fa614c58cb7 - run_compiler::h28a4446bae1034e7H5b
  42:     0x7fa614c56829 - thunk::F.Invoke<A, R>::invoke::h6503055919709693733
  43:     0x7fa614c554a0 - rt::unwind::try::try_fn::h1384674024000742916
  44:     0x7fa614616de8 - rust_try_inner
  45:     0x7fa614616dd5 - rust_try
  46:     0x7fa614c55c3f - thunk::F.Invoke<A, R>::invoke::h5780663349966142752
  47:     0x7fa614591965 - sys::thread::thread_start::h4ab695857833a5dar8E
  48:     0x7fa60e3ed0a4 - start_thread
  49:     0x7fa61414ccfc - __clone
  50:                0x0 - <unknown>

Could not compile `rust-crypto`.

To learn more, run the command again with --verbose.
