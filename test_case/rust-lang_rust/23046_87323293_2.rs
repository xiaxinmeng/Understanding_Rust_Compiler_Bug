 console
$ rustc src/lib.rs --crate-name sequence_trie --crate-type lib -g --test -C metadata=0808f6f1b57c7e83 -C extra-filename=-0808f6f1b57c7e83 --out-dir ./overrides/rust_sequence_trie/target/debug --emit=dep-info,link -L dependency=./overrides/rust_sequence_trie/target/debug -L dependency=./overrides/rust_sequence_trie/target/debug/deps
src/benchmark.rs:23:36: 23:44 error: type annotations required: cannot resolve `collections::vec::Vec<u32> : core::convert::AsRef<_>` [E0283]
src/benchmark.rs:23                     map.insert(key.as_ref(), 7u32);
                                                       ^~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/option.rs:364

stack backtrace:
   1:     0x7fefb1602319 - sys::backtrace::write::hce2635debe4f9308BBD
   2:     0x7fefb162cc3c - panicking::on_panic::h684283a75db74f52HRJ
   3:     0x7fefb1552553 - rt::unwind::begin_unwind_inner::h5840ea28733d246foxJ
   4:     0x7fefb155293f - rt::unwind::begin_unwind_fmt::hb23a1ee6fe217791ZvJ
   5:     0x7fefb162c857 - rust_begin_unwind
   6:     0x7fefb167ccb4 - panicking::panic_fmt::he15400e7c822c131brB
   7:     0x7fefb1677a44 - panicking::panic::h143696e2c54ef9a4IpB
   8:     0x7fefb022a4f5 - check::regionck::type_must_outlive::h7cf4171efdcb16b9n1e
   9:     0x7fefb0270eab - check::regionck::type_of_node_must_outlive::h3b07938907cd8203Xqe
  10:     0x7fefb026d6c2 - check::regionck::visit_expr::h9151012217c93467RHd
  11:     0x7fefb0269f30 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::hf123615d02fb21bf4jd
  12:     0x7fefb0301601 - check::check_bare_fn::h0c144619db1f6b78Rnn
  13:     0x7fefb02f9235 - check::check_item::hfc51974e981a39ceCGn
  14:     0x7fefb02ff822 - visit::walk_item::h16090704992959972715
  15:     0x7fefb03db271 - check_crate::closure.36029
  16:     0x7fefb03d5633 - check_crate::h4947b34bb4ea3242HlC
  17:     0x7fefb1c653fd - driver::phase_3_run_analysis_passes::ha98c2f61eed776b9pGa
  18:     0x7fefb1c49325 - driver::compile_input::hc85b3d46d8787bc0Rba
  19:     0x7fefb1d00b25 - run_compiler::hcc9e5d244ab00cc3q2b
  20:     0x7fefb1cfe5da - thunk::F.Invoke<A, R>::invoke::h15610588257015573231
  21:     0x7fefb1cfd829 - rt::unwind::try::try_fn::h11610778690058047059
  22:     0x7fefb16a6eb8 - rust_try_inner
  23:     0x7fefb16a6ea5 - rust_try
  24:     0x7fefb1cfdb87 - thunk::F.Invoke<A, R>::invoke::h2134375656366038107
  25:     0x7fefb1617c21 - sys::thread::create::thread_start::had4589224c69738barI
  26:     0x7fefab43b0a4 - start_thread
  27:     0x7fefb11b8cfc - __clone
  28:                0x0 - <unknown>
