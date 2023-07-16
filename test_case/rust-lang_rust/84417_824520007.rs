
   Compiling bedFilter v0.1.0 (/Volumes/3dPrint/bedFilter)
error: incremental compilation: could not create session directory lock file: Operation not supported (os error 45)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:945:48
stack backtrace:
   0: _rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_session::session::Session::incr_comp_session_dir
   3: rustc_incremental::persist::fs::garbage_collect_session_directories
   4: rustc_session::utils::<impl rustc_session::session::Session>::time
   5: rustc_interface::passes::register_plugins
   6: rustc_interface::queries::Queries::register_plugins
   7: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
   8: rustc_span::with_source_map
   9: rustc_interface::interface::create_compiler_and_run
  10: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `bedFilter`

To learn more, run the command again with --verbose.
