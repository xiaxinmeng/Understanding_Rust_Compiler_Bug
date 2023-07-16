
$ rustc --version
rustc 1.11.0 (9b21dcd6a 2016-08-15)
$ cat ~/.cargo/config
[build]
rustflags = ["-Zorbit"]
$ cd parity && cargo clean && cargo build --release -j 1
  [ truncated irrelevant output]
  Compiling dtoa v0.2.2
warning: the option `Z` is unstable and should only be used on the nightly compiler, but it is currently accepted for backwards compatibility; this will soon change, see issue #31847 for more details
error: internal compiler error: ../src/librustc_trans/mir/lvalue.rs:100: using operand local var4 as lvalue
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:619
stack backtrace:
   1:     0x7f531ab8be2f - std::sys::backtrace::tracing::imp::write::h46e546df6e4e4fe6
   2:     0x7f531ab9a13b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h077deeda8b799591
   3:     0x7f531ab99cd8 - std::panicking::default_hook::heb8b6fd640571a4f
   4:     0x7f531ab5fade - std::panicking::rust_panic_with_hook::hd7b83626099d3416
   5:     0x7f5313714187 - std::panicking::begin_panic::h3e029d5f110b4661
   6:     0x7f5313713ad1 - rustc_errors::Handler::bug::he60f76b829c68950
   7:     0x7f531712ceb4 - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::h11fa08f71c1e9d84
   8:     0x7f531712ccbd - rustc::session::opt_span_bug_fmt::hdc2517bf24a762d0
   9:     0x7f5317147886 - rustc::session::bug_fmt::h4bff3cf11871f37a
  10:     0x7f5319b948ae - rustc_trans::mir::lvalue::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_lvalue::h2a6d486404ea098d
  11:     0x7f5319b93c35 - rustc_trans::mir::lvalue::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_lvalue::h2a6d486404ea098d
  12:     0x7f5319b956c6 - rustc_trans::mir::operand::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_consume::h73d5de2d873e0881
  13:     0x7f5319b939bd - rustc_trans::mir::operand::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_operand::hc01879dc825e1a6b
  14:     0x7f5319ba2403 - rustc_trans::mir::rvalue::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_rvalue_operand::h4cf1fcaf74fe9313
  15:     0x7f5319b8a2ef - rustc_trans::mir::block::_<impl rustc_trans..mir..MirContext<'bcx, 'tcx>>::trans_block::h62f4aec8e51f60c5
  16:     0x7f5319a89fe2 - rustc_trans::mir::trans_mir::hae8e3e3c01dc03cf
  17:     0x7f5319a82d07 - rustc_trans::base::trans_closure::h1ab33a1b60511d91
  18:     0x7f5319a8bde3 - rustc_trans::base::trans_fn::he8613d5f36dd799b
  19:     0x7f5319a952be - rustc_trans::base::trans_item::hcfc6f2f68b9918b0
  20:     0x7f5319aad893 - _<rustc_trans..base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::hfffd713d2b3de363
  21:     0x7f5319aad045 - rustc::hir::intravisit::Visitor::visit_stmt::hb5f2d3beb07f428e
  22:     0x7f5319aad27e - rustc::hir::intravisit::Visitor::visit_fn::h80e8da1b47487d41
  23:     0x7f5319aac14d - rustc::hir::intravisit::walk_item::hd4e84b3fe58a4a0c
  24:     0x7f5319aad8ac - _<rustc_trans..base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::hfffd713d2b3de363
  25:     0x7f5319a9e337 - rustc_trans::base::trans_crate::h75826f6271b49faf
  26:     0x7f531b0e007f - rustc_driver::driver::phase_4_translate_to_llvm::hbc7e9672529bb439
  27:     0x7f531b0dd06c - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h7168080c5b7e33b9
  28:     0x7f531b0d977d - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::hded790081e457a76
  29:     0x7f531b0d2f49 - rustc::ty::context::TyCtxt::create_and_enter::h7622c0f52ea2e7fe
  30:     0x7f531b09080f - rustc_driver::driver::compile_input::hdfe4405d66704c31
  31:     0x7f531b07cf44 - rustc_driver::run_compiler::h581448fb74257353
  32:     0x7f531b07a04e - std::panicking::try::call::hf081e8ea5e252d1a
  33:     0x7f531aba863b - __rust_try
  34:     0x7f531aba85de - __rust_maybe_catch_panic
  35:     0x7f531b07ab34 - _<F as alloc..boxed..FnBox<A>>::call_box::h2d5dcb354b3ff8db
  36:     0x7f531ab98264 - std::sys::thread::Thread::new::thread_start::hf2eed4b6f7149599
  37:     0x7f5312e01183 - start_thread
  38:     0x7f531a7d237c - clone
  39:                0x0 - <unknown>

error: Could not compile `dtoa`.
