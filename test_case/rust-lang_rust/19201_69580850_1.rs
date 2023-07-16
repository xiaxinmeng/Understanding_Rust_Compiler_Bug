
bad2.rs:2:1: 2:24 warning: function is never used: `foo`, #[warn(dead_code)] on by default
bad2.rs:2 fn foo(a: [[u8]; 5]) {}
          ^~~~~~~~~~~~~~~~~~~~~~~
bad2.rs:2:8: 2:9 warning: unused variable: `a`, #[warn(unused_variables)] on by default
bad2.rs:2 fn foo(a: [[u8]; 5]) {}
                 ^
error: internal compiler error: trying to take the sizing type of [u8], an unsized type
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:        0x1127f14b5 - sys::backtrace::write::h64ca2fb259c4ae97lCt
   2:        0x11281394f - failure::on_fail::h64ff5ae6887860cc0Hz
   3:        0x11277f5ca - rt::unwind::begin_unwind_inner::h1d4bc098fd350446Qpz
   4:        0x1104930b7 - rt::unwind::begin_unwind::h1123425023578864118
   5:        0x110493a28 - diagnostic::Handler::bug::h6d2bfe842975d6e0tWF
   6:        0x10fa93788 - session::Session::bug::h5da8d79b0a6f5276iRq
   7:        0x10f1ca848 - trans::type_of::sizing_type_of::h9a9144f41b95ef26nUo
   8:        0x10f1c9bb4 - trans::type_of::sizing_type_of::h9a9144f41b95ef26nUo
   9:        0x10f1de841 - trans::common::type_is_immediate::h6bb1c0a049852e45tUk
  10:        0x10f25c889 - trans::type_of::type_of_rust_fn::hbbe004d99dfebef2xQo
  11:        0x10f2640e1 - trans::base::decl_rust_fn::h1e048b4bd74ce022s1r
  12:        0x10f27561e - trans::base::register_fn::h0ee69c538bdaf7b2dEu
  13:        0x10f192908 - trans::base::get_item_val::h697b171366de28ecc1u
  14:        0x10f18fce7 - trans::base::trans_item::hdbddd5abd5742b0cEwu
  15:        0x10f27902c - trans::base::trans_crate::h6077c32a4f7ba4e9lsv
  16:        0x10f035b3e - driver::phase_4_translate_to_llvm::ha92297f2357a645fPFa
  17:        0x10f011f4b - driver::compile_input::h31580cbd7ea87613xba
  18:        0x10f0dcfda - monitor::unboxed_closure.22557
  19:        0x10f0db735 - thunk::F.Invoke<A, R>::invoke::h6367419564961841226
  20:        0x10f0da510 - rt::unwind::try::try_fn::h7763956589852599824
  21:        0x11287a2a9 - rust_try_inner
  22:        0x11287a296 - rust_try
  23:        0x10f0dac0c - thunk::F.Invoke<A, R>::invoke::h16724184168577887652
  24:        0x112801154 - sys::thread::thread_start::hbd8f2f8bdd3a3baadrw
  25:     0x7fff9358c2fc - _pthread_body
  26:     0x7fff9358c279 - _pthread_body
