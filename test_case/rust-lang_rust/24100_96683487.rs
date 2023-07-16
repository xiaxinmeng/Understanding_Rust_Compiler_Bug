
error: internal compiler error: translating unsupported cast: TestStruct (cast_other) -> TestStruct (cast_other)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:230

stack backtrace:
   1:        0x10dd99b3f - sys::backtrace::write::h7c4b7b31b5a98994KVr
   2:        0x10dda2242 - panicking::on_panic::h130becb52f065864NVv
   3:        0x10dd5e985 - rt::unwind::begin_unwind_inner::hf2ec506d50a3ea22wDv
   4:        0x10d5692de - rt::unwind::begin_unwind::h10963008848674724601
   5:        0x10d569a82 - diagnostic::Handler::bug::h7f6411f5b5710a68VxB
   6:        0x10af2d887 - session::Session::bug::h67c3e63982ede886xWp
   7:        0x10a767a0c - trans::expr::trans_imm_cast::h56c659d7a3aa3946OkC
   8:        0x10a75bbde - trans::expr::trans_unadjusted::h16ccd3176676f118vxA
   9:        0x10a72fdf7 - trans::expr::trans_into::hb0f7fdd5a2a9abdbj4z
  10:        0x10a6b1109 - trans::controlflow::trans_block::h12567c1fc6ee8337H1u
  11:        0x10a6afa06 - trans::base::trans_closure::ha53aa9eb2a9bf670gEh
  12:        0x10a6b173e - trans::base::trans_fn::h8f58e16c0621df33YOh
  13:        0x10a6f2f25 - trans::monomorphize::monomorphic_fn::h9b2fe76155400f16iAJ
  14:        0x10a6e33b7 - trans::callee::trans_fn_ref_with_substs::ha369033cd198b04eBGn
  15:        0x10a6e1e52 - trans::callee::trans_fn_ref::h20eb8b04123e4545Gun
  16:        0x10a6f6feb - trans::meth::trans_method_callee::h0e2799d89ca0fda9jxI
  17:        0x10a6f5533 - trans::callee::trans_call_inner::h2848324853079840396
  18:        0x10a75c79b - trans::expr::trans_rvalue_dps_unadjusted::hadab5746c39f5e9fY2A
  19:        0x10a72fdd7 - trans::expr::trans_into::hb0f7fdd5a2a9abdbj4z
  20:        0x10a7b0c61 - trans::_match::mk_binding_alloca::h3749212929604063812
  21:        0x10a6a0b39 - trans::base::init_local::hcee21b7a4917a72djXg
  22:        0x10a6b0df3 - trans::controlflow::trans_block::h12567c1fc6ee8337H1u
  23:        0x10a6afa06 - trans::base::trans_closure::ha53aa9eb2a9bf670gEh
  24:        0x10a6b173e - trans::base::trans_fn::h8f58e16c0621df33YOh
  25:        0x10a6b4d38 - trans::base::trans_item::he86179308be326c4adi
  26:        0x10a6c3b80 - trans::base::trans_crate::h350105e1ee0f6a06a2i
  27:        0x10a531c2e - driver::phase_4_translate_to_llvm::h7039638042d67b1enOa
  28:        0x10a50a7a9 - driver::compile_input::h65cad254ac62afa6Qba
  29:        0x10a5c9613 - run_compiler::hbd6a992aacaf1248F4b
  30:        0x10a5c6d7a - boxed::F.FnBox<A>::call_box::h9480122591250505076
  31:        0x10a5c62c7 - rt::unwind::try::try_fn::h5956310425634165804
  32:        0x10de24618 - rust_try_inner
  33:        0x10de24605 - rust_try
  34:        0x10a5c659e - boxed::F.FnBox<A>::call_box::h8195565033996650298
  35:        0x10dda0c3d - sys::thread::Thread::new::thread_start::h8efe8b1c3780385fvYu
  36:     0x7fff92e872fb - _pthread_body
  37:     0x7fff92e87278 - _pthread_start
