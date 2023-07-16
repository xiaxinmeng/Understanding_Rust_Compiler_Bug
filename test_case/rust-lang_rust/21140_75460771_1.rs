
thread 'rustc' panicked at 'no entry found for key', /Users/shep/Projects/rust/src/libcore/option.rs:330

stack backtrace:
   1:        0x112b78003 - sys::backtrace::write::ha0f424154546b7c40nC
   2:        0x112ba6005 - panicking::on_panic::h3b5dbc92e5fd084enPL
   3:        0x112acf838 - rt::unwind::begin_unwind_inner::h37e4198786058d62AwL
   4:        0x112ad004e - rt::unwind::begin_unwind_fmt::h82fc55bfa0e33b196uL
   5:        0x112ba5ade - rust_begin_unwind
   6:        0x112bf7ca7 - panicking::panic_fmt::h781a4eae67125670cSv
   7:        0x10f4c87dd - astconv::ast_ty_to_ty::closure.34581
   8:        0x10f460cab - astconv::ast_ty_to_ty::hfe9ef7dae67bf45dNcw
   9:        0x10f4bca2d - astconv::ast_ty_arg_to_ty::hd688f06078a4948asbw
  10:        0x10f4bc5d0 - vec::Vec<T>.FromIterator<T>::from_iter::h17676191886426164953
  11:        0x10f4b98c0 - astconv::convert_angle_bracketed_parameters::h4086c3680493bbb0biv
  12:        0x10f4602bf - astconv::ast_path_substs_for_ty::h3b3263700c531214G0u
  13:        0x10f4c3317 - astconv::ast_path_to_ty::h4e66c6cbfa9be1cfqKv
  14:        0x10f4c7d48 - astconv::ast_ty_to_ty::closure.34581
  15:        0x10f460cab - astconv::ast_ty_to_ty::hfe9ef7dae67bf45dNcw
  16:        0x10f4f18d2 - collect::ty_generics::hf5d450d36371d407EGy
  17:        0x10f4cf767 - collect::trait_def_of_item::h7572f38569b82ba4Q2x
  18:        0x10f4ce1ef - collect::CollectTraitDefVisitor<'a, 'tcx>.visit..Visitor<'v>::visit_item::hf820b3250e32a099S8w
  19:        0x10f51a65f - check_crate::closure.35775
  20:        0x10f51860e - check_crate::h583bc81808bd2320VjC
  21:        0x10ee88e2b - driver::phase_3_run_analysis_passes::h507eee96de6c00b5gHa
  22:        0x10ee6e0ed - driver::compile_input::h7a8da1baf9f8a9c6Gba
  23:        0x10ef46bf7 - run_compiler::h179d8ac424d7e401Zbc
  24:        0x10ef44071 - thunk::F.Invoke<A, R>::invoke::h16471643475163732258
  25:        0x10ef42cd0 - rt::unwind::try::try_fn::h1724851838442748004
  26:        0x112c22509 - rust_try_inner
  27:        0x112c224f6 - rust_try
  28:        0x10ef43435 - thunk::F.Invoke<A, R>::invoke::h1291584261435236911
  29:        0x112b8eee3 - sys::thread::thread_start::h3887c6efdcfe71b9U4G
  30:     0x7fff83ba1268 - _pthread_body
