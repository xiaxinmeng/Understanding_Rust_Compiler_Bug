
error: internal compiler error: symbol `_ZN4main3foo10__rust_abiE` already defined
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:209

stack backtrace:
   1:     0x7fa5345c8bb9 - sys::backtrace::write::he1173a406c2335a83JC
   2:     0x7fa5345ed4b7 - panicking::on_panic::hfd4f10113591c07f9aJ
   3:     0x7fa534537322 - rt::unwind::begin_unwind_inner::h5dd2f870d997f851hQI
   4:     0x7fa531aef8ad - rt::unwind::begin_unwind::h11948444363592342521
   5:     0x7fa531aeff6b - diagnostic::Handler::bug::h8b610b99ced770c3AtB
   6:     0x7fa5324cae6b - session::Session::bug::h9b57e3cf31a5ccf02Sq
   7:     0x7fa533c977df - trans::foreign::trans_rust_fn_with_foreign_abi::h5b35927ae8d3ccbae1D
   8:     0x7fa533c912b6 - trans::base::trans_item::h5f366b9cf5915577Yci
   9:     0x7fa533c97f85 - visit::walk_block::h6053193963480568458
  10:     0x7fa533c91ae4 - trans::base::trans_item::h5f366b9cf5915577Yci
  11:     0x7fa533c9fadd - trans::base::trans_crate::h4d51da86e796d34d31i
  12:     0x7fa534c410fa - driver::phase_4_translate_to_llvm::hb8e4f02386630e0djOa
  13:     0x7fa534c18f4a - driver::compile_input::h7fb56cda9a0f227dQba
  14:     0x7fa534cd7711 - run_compiler::hbdd9b2c4cacce291L4b
  15:     0x7fa534cd5362 - boxed::F.FnBox<A>::call_box::h9433597520612747
  16:     0x7fa534cd4899 - rt::unwind::try::try_fn::h10263588931191114294
  17:     0x7fa534663348 - rust_try_inner
  18:     0x7fa534663335 - rust_try
  19:     0x7fa534cd4b48 - boxed::F.FnBox<A>::call_box::h17081069488870149893
  20:     0x7fa5345dae41 - sys::thread::create::thread_start::hb84d91b7ff5c0ddcrLH
  21:     0x7fa52e5d8373 - start_thread
  22:     0x7fa5341b127c - __clone
  23:                0x0 - <unknown>
