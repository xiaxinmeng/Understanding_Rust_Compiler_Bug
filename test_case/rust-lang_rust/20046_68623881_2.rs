
error: internal compiler error: adt::represent_type called on non-ADT type: &std::io::IoError
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1: 0xb73441f0 - sys::backtrace::write::h8c2ed683dda82704ist
   2: 0xb7367e70 - failure::on_fail::hebe4057108e7dc03xLz
   3: 0xb72d27d0 - rt::unwind::begin_unwind_inner::h420b24b59a4d3530ypz
   4: 0xb38c29b0 - rt::unwind::begin_unwind::h2881239313039487497
   5: 0xb38c31c0 - diagnostic::Handler::bug::he2275dda9c97b39dzWF
   6: 0xb617ba90 - session::Session::bug::hfd10fa89d9dc35a1zMp
   7: 0xb6b40ed0 - trans::adt::represent_type_uncached::h5e681bb076af3e1fzQH
   8: 0xb6a16b40 - trans::adt::represent_type::hb9fc6ac08f11ee404MH
   9: 0xb6aee400 - trans::_match::compile_submatch_continue::hd67afd18709f7004dax
  10: 0xb6aebde0 - trans::_match::compile_submatch::h7bf039279d178001J4w
  11: 0xb6aee400 - trans::_match::compile_submatch_continue::hd67afd18709f7004dax
  12: 0xb6aebde0 - trans::_match::compile_submatch::h7bf039279d178001J4w
  13: 0xb6af4e70 - trans::_match::trans_match_inner::h3e8caaa43b5a6eafoDx
  14: 0xb6a46770 - trans::expr::trans_rvalue_dps_unadjusted::hc875dbef972abc0a69i
  15: 0xb69f9890 - trans::expr::trans_into::hecc47d3a84ed0b18nIh
  16: 0xb69fa0d0 - trans::controlflow::trans_block::h0356beb9de683d99F6d
  17: 0xb6acdae0 - trans::base::trans_closure::h327ac755b34be94apfu
  18: 0xb69e8f20 - trans::base::trans_fn::heb29d5b7a30a15ebequ
  19: 0xb69e2a90 - trans::base::trans_item::ha87127dde51fdff3INu
  20: 0xb6ad6af0 - trans::base::trans_crate::ha55c3849272518cfNKv
  21: 0xb768c710 - driver::phase_4_translate_to_llvm::hed237fb0f43bd599IFa
  22: 0xb766b2f0 - driver::compile_input::h05aad328a73a6525wba
  23: 0xb77c5160 - thunk::F.Invoke<A, R>::invoke::h118516395371660300
  24: 0xb77c3f70 - rt::unwind::try::try_fn::h14176178842317427281
  25: 0xb73d7cd0 - rust_try_inner
  26: 0xb73d7ca0 - rust_try
  27: 0xb77c42b0 - thunk::F.Invoke<A, R>::invoke::h7075950477878468722
  28: 0xb7355520 - sys::thread::thread_start::hf2d9dfd6d613d3d2dlw
  29:        0x0 - <unknown>
