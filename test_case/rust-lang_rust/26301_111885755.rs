
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: slice_layout_is_correct(cx, &member_llvm_types[..], element_type)', /home/rustbuild/src/rust-buildbot/slave/stable-dist-rustc-linux/build/src/librustc_trans/trans/debuginfo.rs:2810

stack backtrace:
   1:     0x7f141650d449 - sys::backtrace::write::hbc46dc0cfb3b9537d4r
   2:     0x7f1416515156 - panicking::on_panic::h74d3c14d86c58ac8jrw
   3:     0x7f14164d8462 - rt::unwind::begin_unwind_inner::h382cea404b11eb00t6v
   4:     0x7f1415bf237e - rt::unwind::begin_unwind::h11527883117899559464
   5:     0x7f1415cf3f33 - trans::debuginfo::vec_slice_metadata::hf94e0b5f0c2f2a50cgy
   6:     0x7f1415cdf7fc - trans::debuginfo::type_metadata::h61fa8cb587a59ac0Ioy
   7:     0x7f1415ceff16 - trans::debuginfo::VariantMemberDescriptionFactory<'tcx>::create_member_descriptions::closure.48841
   8:     0x7f1415cef8ea - vec::Vec<T>.FromIterator<T>::from_iter::h15487270944988851890
   9:     0x7f1415ce9603 - trans::debuginfo::EnumMemberDescriptionFactory<'tcx>::create_member_descriptions::h7d1cc21665fa719b2Mx
  10:     0x7f1415cea92d - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::hba0b3ce3a1bb00e75Cx
  11:     0x7f1415cdf229 - trans::debuginfo::type_metadata::h61fa8cb587a59ac0Ioy
  12:     0x7f1415cdf856 - trans::debuginfo::type_metadata::h61fa8cb587a59ac0Ioy
  13:     0x7f1415cf44c0 - trans::debuginfo::subroutine_type_metadata::h0a8adba26e117853iky
  14:     0x7f1415cdf1d4 - trans::debuginfo::type_metadata::h61fa8cb587a59ac0Ioy
  15:     0x7f1415c4b8b6 - trans::debuginfo::create_function_debug_context::hce33e3e89c0b549eG0w
  16:     0x7f1415c49d16 - trans::base::new_fn_ctxt::hc47263521fb5ebc7Djh
  17:     0x7f1415c52f10 - trans::base::trans_closure::h8e5687f4ccb96c5eLCh
  18:     0x7f1415c570ea - trans::base::trans_fn::h0bc8fb6dbe17d6adtNh
  19:     0x7f1415c5a177 - trans::base::trans_item::h4f9c99b1e4474396Fbi
  20:     0x7f1415c6856d - trans::base::trans_crate::haa02506df24d5efcF0i
  21:     0x7f1416a5b0fa - driver::phase_4_translate_to_llvm::h86d6fb84c5c936d5hOa
  22:     0x7f1416a32faa - driver::compile_input::hb78754f2f33c01efQba
  23:     0x7f1416af44d1 - run_compiler::h258d36d5501c1cdfz4b
  24:     0x7f1416af2122 - boxed::F.FnBox<A>::call_box::h7239693171334256553
  25:     0x7f1416af1659 - rt::unwind::try::try_fn::h14329119008520845439
  26:     0x7f1416587ac8 - rust_try_inner
  27:     0x7f1416587ab5 - rust_try
  28:     0x7f1416af1908 - boxed::F.FnBox<A>::call_box::h17332056298259451807
  29:     0x7f1416514041 - sys::thread::create::thread_start::h490278b5c3c0b49faqv
  30:     0x7f1410db9353 - start_thread
  31:     0x7f141616ebfc - __clone
  32:                0x0 - <unknown>
