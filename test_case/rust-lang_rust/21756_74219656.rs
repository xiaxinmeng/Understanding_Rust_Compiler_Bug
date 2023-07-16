
/root/.cargo/git/checkouts/image-f4d4ca6613133df7/master/src/imageops/colorops.rs:62:46: 62:66 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `traits::Num` for the type `<<I as image::GenericImage>::Pixel as buffer::Pixel>::Subpixel`
/root/.cargo/git/checkouts/image-f4d4ca6613133df7/master/src/imageops/colorops.rs:62     let max: <I::Pixel as Pixel>::Subpixel = Primitive::max_value();
                                                                                                                                  ^~~~~~~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /rustc-nightly/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7f53cd4f14b0 - sys::backtrace::write::h7f470cc4f24154a6ZTA
   2:     0x7f53cd516140 - failure::on_fail::h61642dd3c2c5b7f9JFJ
   3:     0x7f53cd46e0b0 - rt::unwind::begin_unwind_inner::hd970c8feb3e9463d4jJ
   4:     0x7f53ca812410 - rt::unwind::begin_unwind::h3673386443430583256
   5:     0x7f53ca8123a0 - diagnostic::SpanHandler::span_bug::h5ed17cc567e332df74E
   6:     0x7f53cb585c60 - middle::traits::error_reporting::report_fulfillment_errors::hc272c1e020beb1a9suO
   7:     0x7f53ccbb7ad0 - check::vtable::select_all_fcx_obligations_or_error::h34bbe66b3fb3edf0oPb
   8:     0x7f53ccc64410 - check::check_bare_fn::h727526b64e64c055Scn
   9:     0x7f53ccc5bb10 - check::check_item::h2ba91f101eebedf8lwn
  10:     0x7f53ccc62100 - visit::walk_item::h7118923990314441683
  11:     0x7f53ccc62100 - visit::walk_item::h7118923990314441683
  12:     0x7f53ccd2f480 - check_crate::closure.35395
  13:     0x7f53ccd29d50 - check_crate::hd29eb55a52d61a01OxB
  14:     0x7f53cdb1ec80 - driver::phase_3_run_analysis_passes::h6d35bf1217485c05SGa
  15:     0x7f53cdb04d60 - driver::compile_input::h782a70ecc086dc68Eba
  16:     0x7f53cdbd5f30 - run_compiler::had38ec949ad218b25bc
  17:     0x7f53cdbd4590 - thunk::F.Invoke<A, R>::invoke::h12150908996860458254
  18:     0x7f53cdbd3480 - rt::unwind::try::try_fn::h16942177747375777417
  19:     0x7f53cd58b980 - rust_try_inner
  20:     0x7f53cd58b970 - rust_try
  21:     0x7f53cdbd3730 - thunk::F.Invoke<A, R>::invoke::h6840892055334168534
  22:     0x7f53cd501db0 - sys::thread::thread_start::h4240f311c9074a7baOE
  23:     0x7f53c764ffe0 - start_thread
  24:     0x7f53cd0ecc99 - __clone
  25:                0x0 - <unknown>
