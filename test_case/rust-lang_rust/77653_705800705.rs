
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: ErrorReported', src/librustc_mir/monomorphize/collector.rs:731:84
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0 (18bf6b4f0 2020-10-07) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

warning: 1 warning emitted

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<dyn Der<()> as Base<()>>)` during codegen
  |
  = note: delayed at src/librustc_trait_selection/traits/codegen/mod.rs:65:32

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:369:17
stack backtrace:
   0:        0x10b3c5754 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd4b962ed89f71a03
   1:        0x10b42d2e0 - core::fmt::write::h94ae1e793baa7a00
   2:        0x10b3b700b - std::io::Write::write_fmt::h5c716758fdc3057f
   3:        0x10b3ca075 - std::panicking::default_hook::{{closure}}::hc6119c7d16548caf
   4:        0x10b3c9db7 - std::panicking::default_hook::heae8b62897b351dc
   5:        0x10bc00ee8 - rustc_driver::report_ice::h4c5f6debea1a3bdd
   6:        0x10b3ca73d - std::panicking::rust_panic_with_hook::hc36596b4257bea99
   7:        0x11013573d - std::panicking::begin_panic::{{closure}}::ha531c012d8ea503d
   8:        0x1101353c8 - std::sys_common::backtrace::__rust_end_short_backtrace::h81cde96df7db3e51
   9:        0x11057f2ee - std::panicking::begin_panic::h8547fa8f4ce5bdf8
  10:        0x110101cf2 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hd0f245604385b2c4
  11:        0x10bc39fea - core::ptr::drop_in_place::he23c867c5bdbde24
  12:        0x10bc3f2c8 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::he8bd6e057da40850
  13:        0x10bbcd422 - core::ptr::drop_in_place::h5ba472cc3f0eeee2
  14:        0x10bbc6921 - rustc_span::with_source_map::he14c41eca1fb99da
  15:        0x10bc1f421 - rustc_interface::interface::create_compiler_and_run::ha27975ab759bd229
  16:        0x10bc04463 - scoped_tls::ScopedKey<T>::set::ha597f9a01cfb0fbb
  17:        0x10bc13070 - std::sys_common::backtrace::__rust_begin_short_backtrace::hec10f7c8c3dfd9bb
  18:        0x10bbb2dbc - core::ops::function::FnOnce::call_once{{vtable.shim}}::h4702140ae68aebcd
  19:        0x10b3d8e5d - std::sys::unix::thread::Thread::new::thread_start::hd4805e9612a32deb
  20:     0x7fff727bb109 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0 (18bf6b4f0 2020-10-07) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

thread panicked while panicking. aborting.
error: could not compile `rust-issue-77653-matthewjasper`.

Caused by:
  process didn't exit successfully: `rustc --crate-name rust_issue_77653_matthewjasper --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=781d53d2f99ed599 --out-dir /Users/user/rust-issue-77653-matthewjasper/target/debug/deps -C incremental=/Users/user/rust-issue-77653-matthewjasper/target/debug/incremental -L dependency=/Users/user/rust-issue-77653-matthewjasper/target/debug/deps` (signal: 4, SIGILL: illegal instruction)
