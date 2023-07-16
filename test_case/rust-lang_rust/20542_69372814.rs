
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librbml/lib.rs:260

stack backtrace:
  1:     0x7f5f9c440410 - sys::backtrace::write::hb159f9850e834894l1s
  2:     0x7f5f9c465150 - failure::on_fail::hd4fb51e7b66e269fxgz
  3:     0x7f5f9c3caf20 - rt::unwind::begin_unwind_inner::he95e3015bc1198c6tVy
  4:     0x7f5f95e067c0 - rt::unwind::begin_unwind::h13575525727365681082
  5:     0x7f5f95e052c0 - reader::get_doc::hd7ae0dfafb4be73c4Ka
  6:     0x7f5f9a9425b0 - metadata::decoder::item_type::hb8f98ef796e04d7cPUi
  7:     0x7f5f9a953030 - metadata::decoder::get_type::h2d8643563367a1c1A6i
  8:     0x7f5f9a784800 - middle::ty::lookup_item_type::h3242b1ceb33ff7f6sv8
  9:     0x7f5f9bcb4e50 - coherence::CoherenceChecker<'a, 'tcx>::check::hbe21b009cdc1d00crfv
  10:     0x7f5f9bccd5c0 - check_crate::unboxed_closure.40370
  11:     0x7f5f9bcc9720 - check_crate::h3f64d3a4a846d0b2Nrx
  12:     0x7f5f9c980cd0 - driver::phase_3_run_analysis_passes::had0a81b4c9f8a9cd6va
  13:     0x7f5f9c96eb50 - driver::compile_input::h4f21f66f6761bb22vba
  14:     0x7f5f9ca37640 - thunk::F.Invoke<A, R>::invoke::h1208443586143045988
  15:     0x7f5f9ca363f0 - rt::unwind::try::try_fn::h4735177180287591181
  16:     0x7f5f9c4c9810 - rust_try_inner
  17:     0x7f5f9c4c9800 - rust_try
  18:     0x7f5f9ca36740 - thunk::F.Invoke<A, R>::invoke::h6623100047367797814
  19:     0x7f5f9c4513d0 - sys::thread::thread_start::h89f54a8cba556c09RSv
  20:     0x7f5f96966250 - start_thread
  21:     0x7f5f9c083219 - clone
  22:                0x0 - <unknown>

Could not compile `playform`.
