
$ RUST_BACKTRACE=1 rustc foo.rs
foo.rs:6:9: 6:14 error: internal compiler error: this path should not cause illegal move
foo.rs:6         *temp;
                 ^~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7f2c8bc66870 - sys::backtrace::write::h26b40214a7757117H8t
   2:     0x7f2c8bc89be0 - failure::on_fail::h7a79bf921b6662fbzkB
   3:     0x7f2c8bbf61f0 - rt::unwind::begin_unwind_inner::hfe1c0e12f5e834cfAZA
   4:     0x7f2c89011ac0 - rt::unwind::begin_unwind::h18088714452159156163
   5:     0x7f2c89011a50 - diagnostic::SpanHandler::span_bug::hb14d16d0c0973c4aeFE
   6:     0x7f2c89b0c7b0 - session::Session::span_bug::h25b7c4dc09b94f23P3o
   7:     0x7f2c8add3640 - borrowck::build_borrowck_dataflow_data::h15eb45fb12515356kPe
   8:     0x7f2c8add00e0 - borrowck::borrowck_fn::hb69937ff45b88f0dHMe
   9:     0x7f2c8add10d0 - borrowck::borrowck_item::hf7ef73cfdfac21f5FLe
  10:     0x7f2c8add1770 - borrowck::check_crate::h7ef96074ee6de484xGe
  11:     0x7f2c8c1f1f50 - driver::phase_3_run_analysis_passes::h3a3da09b2eae9713NFa
  12:     0x7f2c8c1d9330 - driver::compile_input::h802d2a6759ac022fBba
  13:     0x7f2c8c29f230 - run_compiler::h4517938d3296cfc5l9b
  14:     0x7f2c8c29d8c0 - thunk::F.Invoke<A, R>::invoke::h5452441173088021345
  15:     0x7f2c8c29c7f0 - rt::unwind::try::try_fn::h8490101008730471441
  16:     0x7f2c8bcf4f60 - rust_try_inner
  17:     0x7f2c8bcf4f50 - rust_try
  18:     0x7f2c8c29caa0 - thunk::F.Invoke<A, R>::invoke::h13338196279056651375
  19:     0x7f2c8bc763b0 - sys::thread::thread_start::h395b8c6815673c5cN3w
  20:     0x7f2c85e9afe0 - start_thread
  21:     0x7f2c8b878859 - __clone
  22:                0x0 - <unknown>
