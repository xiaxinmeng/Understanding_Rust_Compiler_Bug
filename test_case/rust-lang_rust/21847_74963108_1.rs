
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Trying to convert unsized value to lval', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans/datum.rs:495

stack backtrace:
   1:     0x7fcb21cedb70 - sys::backtrace::write::h00fa41ac57b8527bfmB
   2:     0x7fcb21d164f0 - panicking::on_panic::he60a0e1a593653a5TOK
   3:     0x7fcb21c5d960 - rt::unwind::begin_unwind_inner::h3bcb1406fb794dc7etK
   4:     0x7fcb208ed3e0 - rt::unwind::begin_unwind::h14051137956471406385
   5:     0x7fcb209661d0 - trans::datum::Datum<'tcx, Expr>::to_lvalue_datum::hccf6a8b7b600451ebYf
   6:     0x7fcb20998fe0 - trans::expr::trans_index::h0e1e4b8610393db20Ei
   7:     0x7fcb20985bc0 - trans::expr::trans_unadjusted::hfa1c05df16b707e2mqi
   8:     0x7fcb2093acd0 - trans::expr::trans_into::h546d5b1120c5d84ejGh
   9:     0x7fcb2093a1a0 - trans::controlflow::trans_stmt_semi::h4b02d82dbafc77ce6de
  10:     0x7fcb2093b840 - trans::controlflow::trans_block::h231f411ce6a4283aXee
  11:     0x7fcb20a07b80 - trans::base::trans_closure::hd62fcc69afa5e29aFiu
  12:     0x7fcb209271a0 - trans::base::trans_fn::h58b20f0749af6eafWtu
  13:     0x7fcb20922810 - trans::base::trans_item::h6046484f0ec08588PSu
  14:     0x7fcb20a0f0e0 - trans::base::trans_crate::h7924470e1c25a280fQv
  15:     0x7fcb22358cb0 - driver::phase_4_translate_to_llvm::hb5ebc8e27de986a12Oa
  16:     0x7fcb22332070 - driver::compile_input::h5ed7326e88136f7fEba
  17:     0x7fcb22402290 - run_compiler::h12bed89c4496bc43Bbc
  18:     0x7fcb224007f0 - thunk::F.Invoke<A, R>::invoke::h13875370646425574429
  19:     0x7fcb223ff6d0 - rt::unwind::try::try_fn::h5086316350382225766
  20:     0x7fcb21d83050 - rust_try_inner
  21:     0x7fcb21d83040 - rust_try
  22:     0x7fcb223ff980 - thunk::F.Invoke<A, R>::invoke::h12662570625570879817
  23:     0x7fcb21d01e80 - sys::thread::thread_start::h197b42630d52606aYVF
  24:     0x7fcb1be0f0c0 - start_thread
  25:     0x7fcb218d6fd9 - __clone
  26:                0x0 - <unknown>
