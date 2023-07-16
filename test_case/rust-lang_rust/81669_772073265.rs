
error: internal compiler error: failed to process buffered lint here
...
    = note: delayed at /rustc/04caa632dd10c2bf64b69524c7f9c4c30a436877/compiler/rustc_lint/src/early.rs:384:18

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
stack backtrace:
   0:        0x11390b14c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd841328aa7dc06cc
   1:        0x11396f64d - core::fmt::write::hefe95a44532fe6ed
   2:        0x1138fd566 - std::io::Write::write_fmt::haeb49d4f9ecd789d
   3:        0x11390f019 - std::panicking::default_hook::{{closure}}::h465a961bccd4d390
   4:        0x11390eba9 - std::panicking::default_hook::h7e189743c9acab0a
   5:        0x10bc02b18 - rustc_driver::report_ice::h560f670b897314ed
   6:        0x11390f7fe - std::panicking::rust_panic_with_hook::he74e9eac8174d6dd
   7:        0x11390f305 - std::panicking::begin_panic_handler::{{closure}}::h122c1e0e85c150a9
   8:        0x11390b608 - std::sys_common::backtrace::__rust_end_short_backtrace::he103784f07f740ac
   9:        0x11390f26a - _rust_begin_unwind
  10:        0x11399725b - std::panicking::begin_panic_fmt::h9385c4006b676237
  11:        0x1101fb1e6 - rustc_errors::HandlerInner::flush_delayed::ha814ae8fc66f459e
  12:        0x1101f73cb - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h604d573a9d19f76b
  13:        0x10bc45d0d - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::he1042ff96c3a2292
  14:        0x10bc4c9a8 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hbbedb60f235c87aa
  15:        0x10bc47095 - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::h32ce24398f080a07
  16:        0x10bc320b1 - rustc_span::with_source_map::hb15b187d8726f95a
  17:        0x10bbb3ed4 - rustc_interface::interface::create_compiler_and_run::h676e33808641f5cb
  18:        0x10bc3e713 - std::sys_common::backtrace::__rust_begin_short_backtrace::h17f5da3f25073b11
  19:        0x10bbdd9a9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h099c825c1a0d0ef5
  20:        0x11391ca0d - std::sys::unix::thread::Thread::new::thread_start::h23116ad77b076564
  21:     0x7fff72a08109 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (04caa632d 2021-01-30) running on x86_64-apple-darwin

note: compiler flags: -Z macro-backtrace -Z instrument-coverage -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: build failed
