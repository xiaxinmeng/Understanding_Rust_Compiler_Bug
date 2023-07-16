
   Compiling cynon v0.0.1 (/Volumes/server/data/Matthias/prj/rust/cynon)
     Running `rustc --crate-name build_script_build --edition=2018 build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=f8228d0fe8773068 -C extra-filename=-f8228d0fe8773068 --out-dir /Volumes/server/data/Matthias/prj/rust/cynon/target/debug/build/cynon-f8228d0fe8773068 -C incremental=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/incremental -L dependency=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/deps --extern prost_build=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/deps/libprost_build-0de83129d3e4f190.rlib`
error: incremental compilation: could not create session directory lock file: Operation not supported (os error 45)

thread 'rustc' panicked at 'trying to get session directory from `IncrCompSession`: NotInitialized', compiler/rustc_session/src/session.rs:874:48
stack backtrace:
   0:        0x10981baa4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcfc48256a5ab8835
   1:        0x109882590 - core::fmt::write::haf3903118f694c48
   2:        0x10980d0a6 - std::io::Write::write_fmt::h7385463ac87804ed
   3:        0x10982046f - std::panicking::default_hook::{{closure}}::h91bd4c58cf71392b
   4:        0x10982013d - std::panicking::default_hook::h7bd29c87df967048
   5:        0x10eb0bc73 - rustc_driver::report_ice::h36110b17656cf994
   6:        0x109820b9e - std::panicking::rust_panic_with_hook::hae2b05f08a320721
   7:        0x1098206fb - std::panicking::begin_panic_handler::{{closure}}::h72d68d3a77e0b718
   8:        0x10981bf18 - std::sys_common::backtrace::__rust_end_short_backtrace::h7c5e286792f94edb
   9:        0x1098206ba - _rust_begin_unwind
  10:        0x1098a6e4b - std::panicking::begin_panic_fmt::h6826a3ebe3a95a51
  11:        0x113069e39 - rustc_session::session::Session::incr_comp_session_dir::h9e4df312765624b0
  12:        0x111fd4c20 - rustc_incremental::persist::fs::garbage_collect_session_directories::hc278153e85e5a69e
  13:        0x10ecd51e5 - rustc_session::utils::<impl rustc_session::session::Session>::time::hfb85c7a36ca241c4
  14:        0x10ed15384 - rustc_interface::passes::register_plugins::hcbb989c71900c709
  15:        0x10ed48eb8 - rustc_interface::queries::Queries::register_plugins::hbe269a7365ea56fb
  16:        0x10eb4a093 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::heacbf49337f5fb9d
  17:        0x10eb27321 - rustc_span::with_source_map::h3927afb6bcf6a169
  18:        0x10eb457c9 - scoped_tls::ScopedKey<T>::set::h5608e7cc9397961b
  19:        0x10eb4d644 - std::sys_common::backtrace::__rust_begin_short_backtrace::h9b82f31f81e95856
  20:        0x10eae05bc - core::ops::function::FnOnce::call_once{{vtable.shim}}::h5baa2aca5ae17340
  21:        0x10982eb7d - std::sys::unix::thread::Thread::new::thread_start::he3e6719579180a65
  22:     0x7fff5a5cd2eb - __pthread_body
  23:     0x7fff5a5d0249 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0 (7eac88abb 2020-11-16) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: aborting due to previous error

error: could not compile `cynon`

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2018 build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=f8228d0fe8773068 -C extra-filename=-f8228d0fe8773068 --out-dir /Volumes/server/data/Matthias/prj/rust/cynon/target/debug/build/cynon-f8228d0fe8773068 -C incremental=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/incremental -L dependency=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/deps --extern prost_build=/Volumes/server/data/Matthias/prj/rust/cynon/target/debug/deps/libprost_build-0de83129d3e4f190.rlib` (exit code: 101)
