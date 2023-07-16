
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'unsized_part_of_type failed even though ty is unsized', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans/common.rs:168

stack backtrace:
   1:     0x7f79f57642f2 - sys::backtrace::write::hd861c9cad5664affQBA
   2:     0x7f79f578a342 - panicking::on_panic::h2d36a117ae395cb9jHJ
   3:     0x7f79f56d0a89 - rt::unwind::begin_unwind_inner::h020c424433824dc2mnJ
   4:     0x7f79f4decf8c - rt::unwind::begin_unwind::h13525811864842182597
   5:     0x7f79f4ea8a1b - trans::common::unsized_part_of_type::h58de00457dffddfcVxk
   6:     0x7f79f4e4f5b2 - trans::type_of::in_memory_type_of::hf3dc8a7490292449DHo
   7:     0x7f79f4ee8543 - trans::base::trans_closure::h70b835bf44b08f62ent
   8:     0x7f79f4e9fe2e - trans::closure::trans_closure_expr::he9af68f514bf9e21Gnx
   9:     0x7f79f4ecde63 - trans::consts::const_expr_unadjusted::hf36c0cdf480a0885ZTn
  10:     0x7f79f4ed1667 - trans::consts::const_expr::h29a9ed2c5e375d82tHn
  11:     0x7f79f4e6bcde - trans::consts::get_const_expr_as_global::hcd5ecd875acacf4bEEn
  12:     0x7f79f4e2ebfc - trans::expr::trans::h66ceb310b9edf1f9bwh
  13:     0x7f79f4e6086b - trans::callee::trans_args::hf2cfa069a4970bdbG3g
  14:     0x7f79f4e6624c - trans::callee::trans_call_inner::h17056753389190948462
  15:     0x7f79f4e6f077 - trans::expr::trans_rvalue_dps_unadjusted::hbd2cd1fd54de27cayCi
  16:     0x7f79f4e6d1bb - trans::expr::trans_unadjusted::h98b3a24d34abc9f436h
  17:     0x7f79f4e2f0e0 - trans::expr::trans::h66ceb310b9edf1f9bwh
  18:     0x7f79f4e2d625 - trans::expr::trans_to_lvalue::h8c97fd7e77062732c6h
  19:     0x7f79f4e2d18d - trans::base::init_local::h6b5a17c439dc15bfGBs
  20:     0x7f79f4e2e662 - trans::controlflow::trans_block::h47e6e8902755cf9cv7d
  21:     0x7f79f4ee9ed0 - trans::base::trans_closure::h70b835bf44b08f62ent
  22:     0x7f79f4e1df4b - trans::base::trans_fn::h88672d255f7695867xt
  23:     0x7f79f4e19ef6 - trans::base::trans_item::h1cf2dcffa3cf86bfZVt
  24:     0x7f79f4ef067c - trans::base::trans_crate::h88960ef764a955bcaSu
  25:     0x7f79f5d88f92 - driver::phase_4_translate_to_llvm::h5d3ca9bd44c7f33agOa
  26:     0x7f79f5d648ea - driver::compile_input::h124aae68147ae8b8Nba
  27:     0x7f79f5e2915d - run_compiler::hffd5283ae5d6d3abG6b
  28:     0x7f79f5e26ecc - thunk::F.Invoke<A, R>::invoke::h16438916220137010031
  29:     0x7f79f5e25b20 - rt::unwind::try::try_fn::h17325604874211535233
  30:     0x7f79f57f6118 - rust_try_inner
  31:     0x7f79f57f6105 - rust_try
  32:     0x7f79f5e262ab - thunk::F.Invoke<A, R>::invoke::h5807416775295653773
  33:     0x7f79f5777b45 - sys::thread::thread_start::h2188587ec04914c6t8E
  34:     0x7f79ef7e4373 - start_thread
  35:     0x7f79f535927c - __clone
  36:                0x0 - <unknown>

