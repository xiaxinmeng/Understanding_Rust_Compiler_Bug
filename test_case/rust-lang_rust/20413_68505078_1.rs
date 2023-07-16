
foo.rs:10:9: 10:12 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::kinds::Sized` for the type `NoData<T>`
foo.rs:10     let val: NoData<T> = NoData;
                  ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:        0x10d1fa1e5 - sys::backtrace::write::h4cebc8c3e368e8f0Upt
   2:        0x10d21ec33 - failure::on_fail::hd9de93a6058d1a7dwwz
   3:        0x10d184e6a - rt::unwind::begin_unwind_inner::h00d5ebf844629bb50dz
   4:        0x10b09a127 - rt::unwind::begin_unwind::h16325012161302436941
   5:        0x10b09a0bc - diagnostic::SpanHandler::span_bug::h1cb2c7c5c82d285daPF
   6:        0x10a880c1e - middle::traits::error_reporting::report_fulfillment_error::h2148a74f54ee08d9QWO
   7:        0x10a75d6f2 - middle::traits::error_reporting::report_fulfillment_errors::h810f7f45fe3b1cb2lWO
   8:        0x10a08f062 - check::vtable::select_all_fcx_obligations_or_error::h571e4dea5c1362649mb
   9:        0x10a14d73a - check::check_bare_fn::h1812c230f63b7673Qzj
  10:        0x10a152959 - check::check_method_body::h2ef2abe5be0f9daew2j
  11:        0x10a1479fa - check::check_item::hac3080059fe09f34SSj
  12:        0x10a2d8e40 - check_crate::unboxed_closure.39832
  13:        0x10a2d4733 - check_crate::h127b3439cfd8ff83NLx
  14:        0x109afcd52 - driver::phase_3_run_analysis_passes::hadc5fda84afbf325qva
  15:        0x109ae9b88 - driver::compile_input::h9d2d7dddc58375d9wba
  16:        0x109c1b843 - thunk::F.Invoke<A, R>::invoke::h1922614042677645733
  17:        0x109c189a0 - rt::unwind::try::try_fn::h5185751407680132505
  18:        0x10d2865e9 - rust_try_inner
  19:        0x10d2865d6 - rust_try
  20:        0x109c190e6 - thunk::F.Invoke<A, R>::invoke::h18183949975069308340
  21:        0x10d20b544 - sys::thread::thread_start::h0fb21aaa7cc37e0dDfw
  22:     0x7fff8f8de2fc - _pthread_body
  23:     0x7fff8f8de279 - _pthread_body
