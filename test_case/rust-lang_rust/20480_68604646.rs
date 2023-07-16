 Shell
Compiling flate2 v0.1.1 (file:///home/imperio/rust/repo_git/flate2-rs)
/home/imperio/rust/repo_git/flate2-rs/src/raw.rs:54:26: 55:78 error: internal compiler error: impl `VtableImpl(impl_def_id=DefId { krate: 0, node: 3981 }:raw::Stream.Deref, substs=Substs[types=[[];[];[]], regions=[[];[];[]]], nested=[[];[];[]])` did not contain projection for `Obligation(predicate=<raw::Stream as TraitRef(raw::Stream, core::ops::Deref)>::Target,depth=0)`
/home/imperio/rust/repo_git/flate2-rs/src/raw.rs:54         try!(self.stream.write(&[], ffi::MZ_FINISH, &mut self.buf,
/home/imperio/rust/repo_git/flate2-rs/src/raw.rs:55                                self.inner.as_mut().unwrap(), ffi::mz_deflate));
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:     0x7f7c9eb433e0 - sys::backtrace::write::h07654db4c803799asit
   2:     0x7f7c9eb68c10 - failure::on_fail::hcaec3177bc42e928Tuz
   3:     0x7f7c9eace2d0 - rt::unwind::begin_unwind_inner::hb226e8b72f3e677ey9y
   4:     0x7f7c99c30e30 - rt::unwind::begin_unwind::h4014735294015536206
   5:     0x7f7c99c30dc0 - diagnostic::SpanHandler::span_bug::hf36643a9ba759b90zFF
   6:     0x7f7c9ce9b670 - middle::traits::project::project_type::h92ecab34bfecdb26UQP
   7:     0x7f7c9ce987d0 - middle::traits::project::opt_normalize_projection_type::h6d8595fd5add6228XIP
   8:     0x7f7c9ce863a0 - middle::traits::project::normalize_projection_type::h570150b42252014cyHP
   9:     0x7f7c9ce99c70 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::heed1bc3ca5d7ef339FP
  10:     0x7f7c9ce99c70 - middle::traits::project::AssociatedTypeNormalizer<'a, 'b, 'tcx>.TypeFolder<'tcx>::fold_ty::heed1bc3ca5d7ef339FP
  11:     0x7f7c9e14ec80 - middle::ty_fold::ty..FnSig<'tcx>.TypeFoldable<'tcx>::fold_with::h11433372155515551456
  12:     0x7f7c9e164040 - check::method::lookup_in_trait_adjusted::hf5477c0e9796df5fJyh
  13:     0x7f7c9e1460e0 - check::try_overloaded_deref::ha1b0c1202063bbdePPl
  14:     0x7f7c9e152d70 - check::method::probe::probe::hc37a8fd9501aafc23If
  15:     0x7f7c9e20e6e0 - check::check_expr_with_unifier::check_method_call::he5ead07f2eb10ed9hPm
  16:     0x7f7c9e202be0 - check::check_expr_with_unifier::h2903680690164758454
  17:     0x7f7c9e0ee3a0 - check::_match::check_match::h2acbb9a8a0ccf0973Aa
  18:     0x7f7c9e22c230 - check::check_expr_with_unifier::h13231159384621416452
  19:     0x7f7c9e1cef30 - check::check_block_with_expected::hd824300850ddcee8CKo
  20:     0x7f7c9e23a540 - check::check_expr_with_unifier::h10793131406227171560
  21:     0x7f7c9e1cef30 - check::check_block_with_expected::hd824300850ddcee8CKo
  22:     0x7f7c9e1b85b0 - check::check_fn::h7223c9c594af73c5Lyj
  23:     0x7f7c9e1c9b20 - check::check_bare_fn::h8ccefd5610a5a146goj
  24:     0x7f7c9e1cfad0 - check::check_method_body::h72b0078144bcb29elQj
  25:     0x7f7c9e1c10c0 - check::check_item::h39fb2bbe81c824a9SGj
  26:     0x7f7c9e1c9690 - visit::walk_mod::h16234166799498673367
  27:     0x7f7c9e3619d0 - check_crate::unboxed_closure.39848
  28:     0x7f7c9e35c620 - check_crate::hb52a41bcb9b55409Vlx
  29:     0x7f7c9f093620 - driver::phase_3_run_analysis_passes::h33329f43054c2979Tva
  30:     0x7f7c9f081ac0 - driver::compile_input::h88016b50923aff2bwba
  31:     0x7f7c9f14ce00 - thunk::F.Invoke<A, R>::invoke::h18199457566815101468
  32:     0x7f7c9f14bbb0 - rt::unwind::try::try_fn::h5958693639131164017
  33:     0x7f7c9ebcf510 - rust_try_inner
  34:     0x7f7c9ebcf500 - rust_try
  35:     0x7f7c9f14bf00 - thunk::F.Invoke<A, R>::invoke::h2060855259774067648
  36:     0x7f7c9eb54f50 - sys::thread::thread_start::hb5d2265d5cf58673y8v
  37:     0x7f7c994540c0 - start_thread
  38:     0x7f7c9e773ec9 - __clone
  39:                0x0 - <unknown>
