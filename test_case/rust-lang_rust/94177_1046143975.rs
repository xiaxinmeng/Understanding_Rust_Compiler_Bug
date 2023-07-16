plain
   Compiling md-5 v0.10.0
   Compiling sha-1 v0.10.0
   Compiling sha2 v0.10.1
   Compiling tempfile v3.2.0
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:787 ~ aho_corasick[8478]::packed::api::{impl#7}::next), const_param_did: None }) (end of phase transition to Optimization) at bb4[5]:
                                use of local _13, which has no storage here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/aho-corasick-0.7.18/src/packed/api.rs:628:17
628 |                 Some(c)
    |                 ^^^^^^^
    |
    = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:121:36
    = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:121:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1176:13
stack backtrace:
   0:     0x7f165aa5a93c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2a4427bc194f90c
   1:     0x7f165aac933e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7f165aa498e1 - std::io::Write::write_fmt::h33bb4b10b956f418
   3:     0x7f165aa5a76b - std::sys_common::backtrace::print::h498d219314722bf2
   4:     0x7f165aa5ef34 - std::panicking::default_hook::{{closure}}::h953e478500a4611d
   5:     0x7f165aa5eb16 - std::panicking::default_hook::h8a205d841121ef74
   6:     0x7f165b53e931 - rustc_driver[9b70d8386d30c36e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f165aa5f653 - std::panicking::rust_panic_with_hook::h25cd42f1b126a642
   8:     0x7f165aa5f467 - std::panicking::begin_panic_handler::{{closure}}::h8ceaa7ba9397f567
   9:     0x7f165aa5ae54 - std::sys_common::backtrace::__rust_end_short_backtrace::h3ceb6be7f27379ea
  10:     0x7f165aa5f129 - rust_begin_unwind
  11:     0x7f165aa159d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7f165de32218 - core[ba5cb6be30a93795]::panicking::panic_display::<&str>
  13:     0x7f165de2e53a - <rustc_errors[e2f3c60da98b9720]::HandlerInner>::flush_delayed
  14:     0x7f165de2adb6 - <rustc_errors[e2f3c60da98b9720]::HandlerInner as core[ba5cb6be30a93795]::ops::drop::Drop>::drop
  15:     0x7f165b4b79f5 - core[ba5cb6be30a93795]::ptr::drop_in_place::<rustc_session[68dd9ce62d21586]::parse::ParseSess>
  16:     0x7f165b4bd60a - <alloc[73103391425eea9e]::rc::Rc<rustc_session[68dd9ce62d21586]::session::Session> as core[ba5cb6be30a93795]::ops::drop::Drop>::drop
  17:     0x7f165b523dac - core[ba5cb6be30a93795]::ptr::drop_in_place::<rustc_interface[4208f5f86dbe46f0]::interface::Compiler>
  18:     0x7f165b52d92f - rustc_span[59fba679e0fd44db]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_interface[4208f5f86dbe46f0]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#1}>
  19:     0x7f165b4ede4d - rustc_interface[4208f5f86dbe46f0]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>
  20:     0x7f165b4cae92 - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[59fba679e0fd44db]::SessionGlobals>>::set::<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  21:     0x7f165b4c85e9 - std[3e634f71eb1fd3c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  22:     0x7f165b53082e - std[3e634f71eb1fd3c7]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[3e634f71eb1fd3c7]::thread::Builder>::spawn_unchecked_<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1}::{closure#0}>>
  23:     0x7f165b4cab80 - <<std[3e634f71eb1fd3c7]::thread::Builder>::spawn_unchecked_<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f165aa6e6b3 - std::sys::unix::thread::Thread::new::thread_start::ha58e33afaf9bcd1a
  25:     0x7f1654de0609 - start_thread
  26:     0x7f165a8d7293 - clone
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (b36e2c551 2022-02-20) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
