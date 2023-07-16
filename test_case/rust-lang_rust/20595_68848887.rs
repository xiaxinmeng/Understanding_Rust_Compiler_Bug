
       Fresh typeable v0.0.4
   Compiling error v0.0.3 (file:///home/melle/Projects/repos/rust-error)
     Running `rustc /home/melle/Projects/repos/rust-error/src/lib.rs --crate-name error --crate-type lib -g -C metadata=cb8c656eca10aad6 -C extra-filename=-cb8c656eca10aad6 --out-dir /home/melle/Projects/repos/rust-error/target --emit=dep-info,link -L /home/melle/Projects/repos/rust-error/target -L /home/melle/Projects/repos/rust-error/target/deps --extern typeable=/home/melle/Projects/repos/rust-error/target/deps/libtypeable-c9d1fd492eab7aee.rlib`
/home/melle/Projects/repos/rust-error/src/lib.rs:4:12: 4:28 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
/home/melle/Projects/repos/rust-error/src/lib.rs:4 #![feature(old_orphan_check)]
                                                              ^~~~~~~~~~~~~~~~
/home/melle/Projects/repos/rust-error/src/lib.rs:4:12: 4:28 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
/home/melle/Projects/repos/rust-error/src/lib.rs:4 #![feature(old_orphan_check)]
                                                              ^~~~~~~~~~~~~~~~
/home/melle/Projects/repos/rust-error/src/lib.rs:84:1: 86:2 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `core::fmt::Show` for the type `Error + 'static`
/home/melle/Projects/repos/rust-error/src/lib.rs:84 impl Show for Error {
/home/melle/Projects/repos/rust-error/src/lib.rs:85     fn fmt(&self, f: &mut Formatter) -> fmt::Result { (*self).fmt(f) }
/home/melle/Projects/repos/rust-error/src/lib.rs:86 }
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:123

stack backtrace:
   1:     0x7f8d04371920 - sys::backtrace::write::h36422e877a8b3158yQs
   2:     0x7f8d04397650 - failure::on_fail::hde5005d61795902da3y
   3:     0x7f8d042fd4a0 - rt::unwind::begin_unwind_inner::ha0668a3a9d73b9a7RHy
   4:     0x7f8cff1b3af0 - rt::unwind::begin_unwind::h9147230717703493418
   5:     0x7f8cff1b3a80 - diagnostic::SpanHandler::span_bug::hee7de6e88971705bDFF
   6:     0x7f8d02692590 - middle::traits::error_reporting::report_fulfillment_error::h923ad3a17f8b31cfaEO
   7:     0x7f8d02565010 - middle::traits::error_reporting::report_fulfillment_errors::ha648826445912c38FDO
   8:     0x7f8d039219b0 - check::vtable::select_all_fcx_obligations_or_error::hb3611e6686523fabvjb
   9:     0x7f8d0399bce0 - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>::with_fcx::h0df2a1129da2a633Hfi
  10:     0x7f8d039a2760 - check::wf::CheckTypeWellFormedVisitor<'ccx, 'tcx>.Visitor<'v>::visit_item::hc72c9294b1b3f21ftoi
  11:     0x7f8d03b90bd0 - check_crate::unboxed_closure.40283
  12:     0x7f8d03b8b820 - check_crate::h4ba306a17295b431iux
  13:     0x7f8d048b2600 - driver::phase_3_run_analysis_passes::h56810662503f92d8Sva
  14:     0x7f8d048a0aa0 - driver::compile_input::h8d0abf631800220evba
  15:     0x7f8d0496c070 - thunk::F.Invoke<A, R>::invoke::h13263903969514368718
  16:     0x7f8d0496ae20 - rt::unwind::try::try_fn::h2272437873322108176
  17:     0x7f8d043fb8b0 - rust_try_inner
  18:     0x7f8d043fb8a0 - rust_try
  19:     0x7f8d0496b170 - thunk::F.Invoke<A, R>::invoke::h17236552692728063371
  20:     0x7f8d04383490 - sys::thread::thread_start::hd19c7749b7af23e8EGv
  21:     0x7f8cfe9d70c0 - start_thread
  22:     0x7f8d03fa7ec9 - __clone
  23:                0x0 - <unknown>

Could not compile `error`.

Caused by:
  Process didn't exit successfully: `rustc /home/melle/Projects/repos/rust-error/src/lib.rs --crate-name error --crate-type lib -g -C metadata=cb8c656eca10aad6 -C extra-filename=-cb8c656eca10aad6 --out-dir /home/melle/Projects/repos/rust-error/target --emit=dep-info,link -L /home/melle/Projects/repos/rust-error/target -L /home/melle/Projects/repos/rust-error/target/deps --extern typeable=/home/melle/Projects/repos/rust-error/target/deps/libtypeable-c9d1fd492eab7aee.rlib` (status=101)


rustc 0.13.0-nightly (ad9e75938 2015-01-05 00:26:28 +0000)
binary: rustc
commit-hash: ad9e759382ad7daed26f86732f41f5f83cd673e2
commit-date: 2015-01-05 00:26:28 +0000
host: x86_64-unknown-linux-gnu
release: 0.13.0-nightly
