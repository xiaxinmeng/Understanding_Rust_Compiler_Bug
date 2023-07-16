
$ env RUST_BACKTRACE=1 rustc test.rs 
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'no entry found for key', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libcore/option.rs:330

stack backtrace:
   1:        0x107e7bef7 - sys::backtrace::write::h88d0d05790f5fe63uvy
   2:        0x107ea21d2 - failure::on_fail::h254ebaa74a16f795dOF
   3:        0x107df7828 - rt::unwind::begin_unwind_inner::ha4f653fd236db381FvF
   4:        0x107df7f91 - rt::unwind::begin_unwind_fmt::h0c200fcff22cbdd4buF
   5:        0x107ea1cde - rust_begin_unwind
   6:        0x107eef6b7 - panicking::panic_fmt::hc869633448df28a2yow
   7:        0x1049d4b36 - astconv::ast_ty_to_ty::closure.33331
   8:        0x10496fc2d - astconv::ast_ty_to_ty::ha8210893eee3ae24U5u
   9:        0x1049fe639 - collect::ty_generics::he1a5ea70ef819dfbggx
  10:        0x1049e22b0 - collect::trait_def_of_item::h68e4e76c00f507a3LPw
  11:        0x1049e0daf - collect::CollectTraitDefVisitor<'a, 'tcx>.visit..Visitor<'v>::visit_item::h38121d7695ef20c1C0v
  12:        0x104a20edf - check_crate::closure.34409
  13:        0x104a1eef5 - check_crate::ha95de63736329c3dZKA
  14:        0x1043d1825 - driver::phase_3_run_analysis_passes::hc0bb154a7b2eb50apGa
  15:        0x1043b8197 - driver::compile_input::h024fc27312c2cbcbCba
  16:        0x10448b6fe - run_compiler::h655f217f5987c6497ac
  17:        0x1044886ef - thunk::F.Invoke<A, R>::invoke::h3444412781138926669
  18:        0x104487380 - rt::unwind::try::try_fn::h12649063132436107851
  19:        0x107f193c9 - rust_try_inner
  20:        0x107f193b6 - rust_try
  21:        0x104487a40 - thunk::F.Invoke<A, R>::invoke::h6921957054689317792
  22:        0x107e8c7d3 - sys::thread::thread_start::h6219f4f9fd58602fhDB
  23:     0x7fff8c6c0268 - _pthread_body
  24:     0x7fff8c6c01e5 - _pthread_body
