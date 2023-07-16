
% rustc --version
rustc 1.0.0-nightly (3d0d9bb6f 2015-01-12 22:56:20 +0000)
% rustc -g bad.rs
bad.rs:8:5: 8:10 warning: struct field is never used: `b`, #[warn(dead_code)] on by default
bad.rs:8     b : B,
             ^~~~~
bad.rs:9:5: 9:17 warning: struct field is never used: `b1`, #[warn(dead_code)] on by default
bad.rs:9     b1 : B::Type,
             ^~~~~~~~~~~~
bad.rs:12:9: 12:10 warning: unused variable: `e`, #[warn(unused_variables)] on by default
bad.rs:12     let e = Struct {
                  ^
error: internal compiler error: type_of with ty_projection
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:        0x105ca1a2c - sys::backtrace::write::h27c3ddb426dd9587RUt
   2:        0x105cc3a4f - failure::on_fail::h63fb027d8b3cc19ca1z
   3:        0x105c302de - rt::unwind::begin_unwind_inner::hafe5720113b93d950Iz
   4:        0x10394cdcf - rt::unwind::begin_unwind::h11195957873796278085
   5:        0x10394d73b - diagnostic::Handler::bug::h8c0937a4743d025ctWF
   6:        0x102f4bce0 - metadata::tydecode::parse_existential_bounds_::h1282082665815765083
   7:        0x10268be98 - trans::type_of::type_of::h77cc5a392ec4c7efX1o
   8:        0x10268b3a0 - trans::type_of::type_of::h77cc5a392ec4c7efX1o
   9:        0x1027a391c - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45662
  10:        0x1027a3502 - vec::Vec<T>.FromIterator<T>::from_iter::h6250772183958048706
  11:        0x10279f68d - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::hf67e5668f84ac65cZeF
  12:        0x1027a218d - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h0585555c78b3f769IcF
  13:        0x10279c5b3 - trans::debuginfo::type_metadata::h08ada392314e61a8xYF
  14:        0x10279dc77 - trans::debuginfo::declare_local::hcf0a2c0d72819844iVE
  15:        0x10279d4d3 - ast_util::walk_pat::walk_pat_::h4796394502832644510
  16:        0x102683aef - trans::controlflow::trans_block::h9179bd0e311bbf71B3d
  17:        0x10274d0ac - trans::base::trans_closure::h0a187eeead3ee5f9E0t
  18:        0x10266f387 - trans::base::trans_fn::h5f00e8761486c618jbu
  19:        0x10266a9cb - trans::base::trans_item::hb876c423cf52a967Eyu
  20:        0x10275322c - trans::base::trans_crate::hd8e597ceb97fa3f8luv
  21:        0x102516c44 - driver::phase_4_translate_to_llvm::hf1a267085ea57e05YMa
  22:        0x1024f3a91 - driver::compile_input::ha4f10a5f4de94c11Aba
  23:        0x1025bb380 - run_compiler::h0e39ba09c41b0c3de5b
  24:        0x1025b8723 - thunk::F.Invoke<A, R>::invoke::h7667196022708996715
  25:        0x1025b7588 - rt::unwind::try::try_fn::h602527677678223085
  26:        0x105d2d8e9 - rust_try_inner
  27:        0x105d2d8d6 - rust_try
  28:        0x1025b7bc6 - thunk::F.Invoke<A, R>::invoke::h5116215899158159820
  29:        0x105cb1232 - sys::thread::thread_start::hd67e9dc06fcd0bebIJw
  30:     0x7fff9358c2fc - _pthread_body
  31:     0x7fff9358c279 - _pthread_body
