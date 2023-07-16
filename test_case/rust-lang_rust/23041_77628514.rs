
stack backtrace:
   1:     0x7f6892f6f87f - sys::backtrace::write::hd4e8cc47a8584fc6OBA
   2:     0x7f6892f9a552 - panicking::on_panic::hfee0f86819ae23d5hHJ
   3:     0x7f6892ecfbda - rt::unwind::begin_unwind_inner::h37f420a62e49c8c7knJ
   4:     0x7f689047903d - rt::unwind::begin_unwind::h8415944015202791066
   5:     0x7f6890478fe3 - diagnostic::SpanHandler::span_bug::h75e79c0a54af89b4VBD
   6:     0x7f6890cdcd33 - session::Session::span_bug::hfb84cab0434b695ayHp
   7:     0x7f6891c91251 - check::regionck::visit_expr::h5138703d9a0086d4WBd
   8:     0x7f6891c8ca60 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::hd94272955e2bdf089dd
   9:     0x7f6891d19b36 - check::check_bare_fn::he5ad48bfa8a0f4ccVan
  10:     0x7f6891d11cf1 - check::check_item::h70d8c4d186aed50bztn
  11:     0x7f6891de3ae3 - check_crate::closure.35216
  12:     0x7f6891dde88a - check_crate::h385944248f94b2dcTZB
  13:     0x7f68935240d8 - driver::phase_3_run_analysis_passes::h46bd0dc62476cdecxFa
  14:     0x7f689350b4ed - driver::compile_input::hd3b97ca9cf852f24Iba
  15:     0x7f68935d2a47 - run_compiler::hdea2287c7bb22b2fF5b
  16:     0x7f68935d05d9 - thunk::F.Invoke<A, R>::invoke::h6737654279709288520
  17:     0x7f68935cf250 - rt::unwind::try::try_fn::h8325543558165455024
  18:     0x7f689300a258 - rust_try_inner
  19:     0x7f689300a245 - rust_try
  20:     0x7f68935cf9ef - thunk::F.Invoke<A, R>::invoke::h1677976001982228969
  21:     0x7f6892f85295 - sys::thread::thread_start::hb74e8c873814d22br8E
  22:     0x7f688cf7e223 - start_thread
  23:     0x7f6892b5b5ec - __clone
  24:                0x0 - <unknown>
