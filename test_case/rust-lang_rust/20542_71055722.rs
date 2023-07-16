
$ cargo build
   Compiling zwave v0.0.1 (file:///Users/david/src/zwave)
ERROR:rbml::reader: failed to find block with tag 7u
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librbml/lib.rs:253


Could not compile `zwave`.

To learn more, run the command again with --verbose.
Davids-Mac-Pro:zwave (master) $ RUST_BACKTRACE=1 cargo build
   Compiling zwave v0.0.1 (file:///Users/david/src/zwave)
ERROR:rbml::reader: failed to find block with tag 7u
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librbml/lib.rs:253

stack backtrace:
   1:        0x11070fb1c - sys::backtrace::write::hf2ebbbc2b1e17d4dPVt
   2:        0x1107317cf - failure::on_fail::h8104d9bfedbdb4c2DZz
   3:        0x11069e1ee - rt::unwind::begin_unwind_inner::ha4d120f202d1295cuHz
   4:        0x10ea5310b - rt::unwind::begin_unwind::h6598034689533309194
   5:        0x10ea51fd9 - reader::get_doc::hfb8e8c99c37c187bYKa
   6:        0x10dabf938 - metadata::decoder::item_type::h3cba7f172d334da0Szj
   7:        0x10dacfddb - metadata::decoder::get_type::h2f6cd61dd7e7d2bb6Lj
   8:        0x10da81f2e - metadata::csearch::get_type::he6c7fdb2dc9b48a0MFm
   9:        0x10d459524 - collect::CollectCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h527c4019d48bb49353t
  10:        0x10d4450f1 - astconv::ast_path_to_ty::haa3570fa44fb6cf98Is
  11:        0x10d448de7 - astconv::ast_ty_to_ty::unboxed_closure.29986
  12:        0x10d3e5c68 - astconv::ast_ty_to_ty::h2e2f224b22cd5dfdJ9s
  13:        0x10d43f0e9 - vec::Vec<T>.FromIterator<T>::from_iter::h14928839551931381196
  14:        0x10d43d3e6 - astconv::convert_angle_bracketed_parameters::hf0682d835e3d0455Bks
  15:        0x10d3e53c8 - astconv::ast_path_substs_for_ty::he91005be89c29bfecas
  16:        0x10d4451ec - astconv::ast_path_to_ty::haa3570fa44fb6cf98Is
  17:        0x10d448de7 - astconv::ast_ty_to_ty::unboxed_closure.29986
  18:        0x10d3e5c68 - astconv::ast_ty_to_ty::h2e2f224b22cd5dfdJ9s
  19:        0x10d43fc3f - astconv::convert_ty_with_lifetime_elision::h24b5c2f0bfa28d0dMos
  20:        0x10d44a8ca - astconv::ty_of_method_or_bare_fn::heeb753278e884d9f0rt
  21:        0x10d449c61 - astconv::ty_of_method::hb5bd6cb93ae1d0ffmqt
  22:        0x10d46d299 - collect::convert_methods::ty_of_method::h7671aa56beff0427asu
  23:        0x10d45fdac - collect::convert::h38fdd2028e060c53hvu
  24:        0x10d457dc2 - visit::walk_mod::h54061686586147098
  25:        0x10d49488e - check_crate::unboxed_closure.31098
  26:        0x10d492951 - check_crate::h05b28be74dfcf5bcayy
  27:        0x10cdfbfce - driver::phase_3_run_analysis_passes::hebe0d39037654bf9aGa
  28:        0x10cde7bde - driver::compile_input::h34c1c11c6899857fAba
  29:        0x10cea5716 - run_compiler::h823f7b3bd358b28alac
  30:        0x10cea2a73 - thunk::F.Invoke<A, R>::invoke::h2577805882533131152
  31:        0x10cea18d8 - rt::unwind::try::try_fn::h7233092926527859983
  32:        0x11079b2a9 - rust_try_inner
  33:        0x11079b296 - rust_try
  34:        0x10cea1f16 - thunk::F.Invoke<A, R>::invoke::h8652838825404240908
  35:        0x11071ee92 - sys::thread::thread_start::h239f7d5cb0adebaaGKw
  36:     0x7fff8cbb32fc - _pthread_body
  37:     0x7fff8cbb3279 - _pthread_body

Could not compile `zwave`.

To learn more, run the command again with --verbose.
