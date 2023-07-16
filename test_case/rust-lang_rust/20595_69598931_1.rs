
ice.rs:13:1: 17:2 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::marker::Sized` for the type `Range<A>`
ice.rs:13 impl<A> Clone for RangeInclusive<A> where Range<A>: Clone {
ice.rs:14     fn clone(&self) -> RangeInclusive<A> {
ice.rs:15         panic!()
ice.rs:16     }
ice.rs:17 }
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:        0x112b474b5 - sys::backtrace::write::h64ca2fb259c4ae97lCt
   2:        0x112b6994f - failure::on_fail::h64ff5ae6887860cc0Hz
   3:        0x112ad55ca - rt::unwind::begin_unwind_inner::h1d4bc098fd350446Qpz
   4:        0x1107e90b7 - rt::unwind::begin_unwind::h1123425023578864118
   5:        0x1107e904c - diagnostic::SpanHandler::span_bug::he8deb1e61e0360e4nQF
   6:        0x10ff7c664 - middle::traits::error_reporting::report_fulfillment_error::h16099ad7ea752bc9hQO
   7:        0x10fe6ee52 - middle::traits::error_reporting::report_fulfillment_errors::h681a99cf8909bc49MPO
   8:        0x10f8d1cad - check::vtable::select_all_fcx_obligations_or_error::h185f88f1cc7da26fdkb
   9:        0x10f939294 - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>::check_item_well_formed::h8ce2ab3c40901df62Ai
  10:        0x10fa14fce - check_crate::unboxed_closure.30671
  11:        0x10fa10873 - check_crate::h32fc7f8788be683523x
  12:        0x10f37e982 - driver::phase_3_run_analysis_passes::h4c5cf0d8fe5d55degwa
  13:        0x10f365420 - driver::compile_input::h31580cbd7ea87613xba
  14:        0x10f430fda - monitor::unboxed_closure.22557
  15:        0x10f42f735 - thunk::F.Invoke<A, R>::invoke::h6367419564961841226
  16:        0x10f42e510 - rt::unwind::try::try_fn::h7763956589852599824
  17:        0x112bd02a9 - rust_try_inner
  18:        0x112bd0296 - rust_try
  19:        0x10f42ec0c - thunk::F.Invoke<A, R>::invoke::h16724184168577887652
  20:        0x112b57154 - sys::thread::thread_start::hbd8f2f8bdd3a3baadrw
  21:     0x7fff9358c2fc - _pthread_body
  22:     0x7fff9358c279 - _pthread_body
% rustc --version
rustc 1.0.0-nightly (44a287e6e 2015-01-08 17:03:40 -0800)
