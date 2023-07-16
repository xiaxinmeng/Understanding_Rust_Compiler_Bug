
/tmp $ rustc hi.rs
hi.rs:2:9: 2:12 warning: the `int` type is deprecated; use `isize` or a fixed-sized integer
hi.rs:2     Bar(int, [int]),
                ^~~
hi.rs:2:9: 2:12 help: add #![feature(int_uint)] to the crate attributes to silence this warning
hi.rs:2     Bar(int, [int]),
                ^~~
hi.rs:2:15: 2:18 warning: the `int` type is deprecated; use `isize` or a fixed-sized integer
hi.rs:2     Bar(int, [int]),
                      ^~~
hi.rs:2:15: 2:18 help: add #![feature(int_uint)] to the crate attributes to silence this warning
hi.rs:2     Bar(int, [int]),
                      ^~~
hi.rs:7:15: 7:18 warning: the `int` type is deprecated; use `isize` or a fixed-sized integer
hi.rs:7     let _x: &(int, [int]);
                      ^~~
hi.rs:7:15: 7:18 help: add #![feature(int_uint)] to the crate attributes to silence this warning
hi.rs:7     let _x: &(int, [int]);
                      ^~~
hi.rs:7:21: 7:24 warning: the `int` type is deprecated; use `isize` or a fixed-sized integer
hi.rs:7     let _x: &(int, [int]);
                            ^~~
hi.rs:7:21: 7:24 help: add #![feature(int_uint)] to the crate attributes to silence this warning
hi.rs:7     let _x: &(int, [int]);
                            ^~~
hi.rs:2:5: 2:20 warning: variant is never used: `Bar`, #[warn(dead_code)] on by default
hi.rs:2     Bar(int, [int]),
            ^~~~~~~~~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'unsized_part_of_type failed even though ty is unsized', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc_trans/trans/common.rs:166

stack backtrace:
   1:        0x10cdf4ef7 - sys::backtrace::write::h88d0d05790f5fe63uvy
   2:        0x10ce1b1d2 - failure::on_fail::h254ebaa74a16f795dOF
   3:        0x10cd70828 - rt::unwind::begin_unwind_inner::ha4f653fd236db381FvF
   4:        0x10948a235 - rt::unwind::begin_unwind::h12190706954857531769
   5:        0x10955e7f1 - trans::common::unsized_part_of_type::heb51b171ecadcb48HMk
   6:        0x10958f3b2 - trans::type_of::type_of::type_of_unsize_info::hd1a63e34e2e99b94A2o
   7:        0x1094e2e81 - trans::type_of::type_of::h2372af414ad01c3ag2o
   8:        0x1095114fd - trans::base::alloc_ty::h3d6cf8c39435c8c2yut
   9:        0x1095d95a5 - ast_util::walk_pat::walk_pat_::h12916028195929177049
  10:        0x1094d763f - trans::base::init_local::hfb21acfe815478f3Ggt
  11:        0x1094d8933 - trans::controlflow::trans_block::h924ca481d4b4516asae
  12:        0x1095a486d - trans::base::trans_closure::h818341d4fc835178E5t
  13:        0x1094c45d6 - trans::base::trans_fn::h13435eb2cf011516Ogu
  14:        0x1094bf723 - trans::base::trans_item::hd9986468b8457cd3fFu
  15:        0x1095aab7c - trans::base::trans_crate::h3c7ea32f1a6ebc96ZBv
  16:        0x109360757 - driver::phase_4_translate_to_llvm::h2222bdf86d2278693Oa
  17:        0x10933c772 - driver::compile_input::h024fc27312c2cbcbCba
  18:        0x10940f6fe - run_compiler::h655f217f5987c6497ac
  19:        0x10940c6ef - thunk::F.Invoke<A, R>::invoke::h3444412781138926669
  20:        0x10940b380 - rt::unwind::try::try_fn::h12649063132436107851
  21:        0x10ce923c9 - rust_try_inner
  22:        0x10ce923b6 - rust_try
  23:        0x10940ba40 - thunk::F.Invoke<A, R>::invoke::h6921957054689317792
  24:        0x10ce057d3 - sys::thread::thread_start::h6219f4f9fd58602fhDB
  25:     0x7fff9201f268 - _pthread_body
  26:     0x7fff9201f1e5 - _pthread_body
