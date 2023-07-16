
$ RUST_BACKTRACE=1 cargo run --release ~/Library/Application\ Support/minecraft/saves/test
   Compiling image v0.2.0-alpha.10 (https://github.com/PistonDevelopers/image#ffbd2d5e)
   Compiling gl v0.0.8 (https://github.com/bjz/gl-rs#64d26f54)
   Compiling gfx_gl v0.1.1
/Users/fenhl/.cargo/git/checkouts/image-f4d4ca6613133df7/master/src/imageops/colorops.rs:62:46: 62:66 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `traits::Num` for the type `<<I as image::GenericImage>::Pixel as buffer::Pixel>::Subpixel`
/Users/fenhl/.cargo/git/checkouts/image-f4d4ca6613133df7/master/src/imageops/colorops.rs:62     let max: <I::Pixel as Pixel>::Subpixel = Primitive::max_value();
                                                                                                                                         ^~~~~~~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x109ac4cf7 - sys::backtrace::write::h1864de2e29cfcfa3jWA
   2:        0x109aec8f2 - failure::on_fail::hb779450ca026c246YwJ
   3:        0x109a363d8 - rt::unwind::begin_unwind_inner::h252a44c757d4dd84eeJ
   4:        0x109166e5f - rt::unwind::begin_unwind::h4771388940342094321
   5:        0x109166e0c - diagnostic::SpanHandler::span_bug::hd8309936b923a05674E
   6:        0x106ae542c - middle::traits::error_reporting::report_fulfillment_errors::hbe697315f4245bcdsuO
   7:        0x106415ee9 - check::vtable::select_all_fcx_obligations_or_error::h1aab9eca37d5de28oPb
   8:        0x1064c90ad - check::check_bare_fn::h08eeba9e9dfaab5eScn
   9:        0x1064c0821 - check::check_item::hf6e90ce2411e3a51lwn
  10:        0x1064c69c3 - visit::walk_item::h10596481115374198522
  11:        0x1064c69ce - visit::walk_item::h10596481115374198522
  12:        0x1065961de - check_crate::closure.35462
  13:        0x10659154e - check_crate::h59e1f00c8e9e52b5OxB
  14:        0x105f115c5 - driver::phase_3_run_analysis_passes::h5d5a99ee07ec1be1SGa
  15:        0x105ef7441 - driver::compile_input::hd4a314c6edff3df0Eba
  16:        0x105fcd1b0 - run_compiler::hacde47e333c8d7485bc
  17:        0x105fca682 - thunk::F.Invoke<A, R>::invoke::h7196015415497398923
  18:        0x105fc9350 - rt::unwind::try::try_fn::h6570971418868152995
  19:        0x109b63f69 - rust_try_inner
  20:        0x109b63f56 - rust_try
  21:        0x105fc9a49 - thunk::F.Invoke<A, R>::invoke::h16470735320018065311
  22:        0x109ad6623 - sys::thread::thread_start::ha3ace5b5b28cf521mOE
  23:     0x7fff919a6268 - _pthread_body
  24:     0x7fff919a61e5 - _pthread_body

Build failed, waiting for other jobs to finish...
Could not compile `image`.

To learn more, run the command again with --verbose.
