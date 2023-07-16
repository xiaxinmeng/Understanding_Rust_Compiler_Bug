
error: internal compiler error: type_of with ty_projection
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:190

stack backtrace:
   1:        0x1134698f7 - sys::backtrace::write::h1feed23329bb1716DJC
   2:        0x113497813 - panicking::on_panic::hf92b99c53fdf643bPxI
   3:        0x1133bbc5e - rt::unwind::begin_unwind_inner::h0bdcab28492dff0aqfI
   4:        0x112b8e12e - rt::unwind::begin_unwind::h15271973076642037093
   5:        0x112b8e979 - diagnostic::Handler::bug::h690b37fb3bd82960EsB
   6:        0x110493077 - session::Session::bug::h605b11f6a18d940bFPq
   7:        0x10ff1c157 - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
   8:        0x10ff1c860 - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
   9:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  10:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  11:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  12:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  13:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  14:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  15:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  16:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  17:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  18:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  19:        0x110029776 - vec::Vec<T>.FromIterator<T>::from_iter::h10825907506224819013
  20:        0x10ff1ccbd - trans::type_of::in_memory_type_of::h292c6bad712c2420fRo
  21:        0x10ff8305f - trans::type_of::type_of_rust_fn::hda44212bef235e44UEo
  22:        0x10ff8c3a4 - trans::base::decl_rust_fn::h4a3a494c31b34005CFr
  23:        0x10ff9f421 - trans::base::register_fn::h17a29763f3b2616cobu
  24:        0x10ffa23c1 - trans::base::register_method::hd8dd1d8c09735d22yOu
  25:        0x10fea141b - trans::base::get_item_val::h0ce72c8ec41fa0c2ayu
  26:        0x10fe9e79f - trans::base::trans_item::h0b33044d08b3fdbfP2t
  27:        0x10ffa4977 - trans::base::trans_crate::hbf9c1b509aba1c64lZu
  28:        0x10f8c647c - driver::phase_4_translate_to_llvm::h6cfd3a151775345eaOa
  29:        0x10f89ddce - driver::compile_input::hf63e92284d149049Qba
  30:        0x10f95b985 - run_compiler::h8e6dd56ee6a375c9S4b
  31:        0x10f959152 - boxed::F.FnBox<A>::call_box::h12642792261833469410
  32:        0x10f958647 - rt::unwind::try::try_fn::h10069384595608373564
  33:        0x113521b58 - rust_try_inner
  34:        0x113521b45 - rust_try
  35:        0x10f95893a - boxed::F.FnBox<A>::call_box::h1065465868264452274
  36:        0x113480cfd - sys::thread::create::thread_start::h056e16758443c21fweH
  37:     0x7fff94323267 - _pthread_body
  38:     0x7fff943231e4 - _pthread_start
