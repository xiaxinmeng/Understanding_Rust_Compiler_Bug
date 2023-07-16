plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.348
[RUSTC-TIMING] build_script_build test:false 0.381
[RUSTC-TIMING] build_script_build test:false 0.603
thread 'rustc' panicked at 'source slice length (8) does not match destination slice length (4)', compiler/rustc_metadata/src/rmeta/encoder.rs:2236:32
stack backtrace:
   0:     0x7f12808da89c - std::backtrace_rs::backtrace::libunwind::trace::h601f837da2118a7e
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f12808da89c - std::backtrace_rs::backtrace::trace_unsynchronized::ha9363a09ff4bbb3c
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f12808da89c - std::sys_common::backtrace::_print_fmt::hd738cd944656a2e3
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f12808da89c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hddf32bb00fd9bf30
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f128093c2ac - core::fmt::write::hc24d306431d6ebcc
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/core/src/fmt/mod.rs:1190:17
   5:     0x7f12808cab08 - std::io::Write::write_fmt::h5a30def43d02e888
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/io/mod.rs:1657:15
   6:     0x7f12808de7c7 - std::sys_common::backtrace::_print::hc4aabe6a9d305913
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f12808de7c7 - std::sys_common::backtrace::print::h061c37239aeb5b74
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f12808de7c7 - std::panicking::default_hook::{{closure}}::h4c487b48946aca7f
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/panicking.rs:295:22
   9:     0x7f12808de48f - std::panicking::default_hook::h6529532158d4f9dc
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/panicking.rs:314:9
  10:     0x7f12811ebbd9 - rustc_driver[ae73c39b1f267ad6]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f12808df0a5 - std::panicking::rust_panic_with_hook::h386de7a57fc01b6d
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/panicking.rs:702:17
  12:     0x7f12808ded57 - std::panicking::begin_panic_handler::{{closure}}::h30b5456c86bf62eb
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/panicking.rs:588:13
  13:     0x7f12808dad44 - std::sys_common::backtrace::__rust_end_short_backtrace::h04abff4029c47a36
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f12808dea59 - rust_begin_unwind
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/panicking.rs:584:5
  15:     0x7f12808a6b23 - core::panicking::panic_fmt::heafed04001a5595e
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/core/src/panicking.rs:143:14
  16:     0x7f12808a6eb2 - core::slice::<impl [T]>::copy_from_slice::len_mismatch_fail::h026c266e55a9b0db
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/core/src/slice/mod.rs:3161:13
  17:     0x7f1282dce1b2 - rustc_metadata[5999c35fe2e3f279]::rmeta::encoder::encode_metadata_impl
  18:     0x7f1282e189d4 - rustc_data_structures[d6e4078dca688a27]::sync::join::<rustc_metadata[5999c35fe2e3f279]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[5999c35fe2e3f279]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[5999c35fe2e3f279]::rmeta::encoder::EncodedMetadata, ()>
  19:     0x7f1282dcd3ae - rustc_metadata[5999c35fe2e3f279]::rmeta::encoder::encode_metadata
  20:     0x7f1281346711 - <rustc_interface[8dfe78949744dc04]::passes::QueryContext>::enter::<<rustc_interface[8dfe78949744dc04]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ef73def78a08f8fd]::result::Result<alloc[cae7aa995e66e3da]::boxed::Box<dyn core[ef73def78a08f8fd]::any::Any>, rustc_errors[768f14375179e26c]::ErrorReported>>
  21:     0x7f1281324684 - <rustc_interface[8dfe78949744dc04]::queries::Queries>::ongoing_codegen
  22:     0x7f128118c625 - <rustc_interface[8dfe78949744dc04]::interface::Compiler>::enter::<rustc_driver[ae73c39b1f267ad6]::run_compiler::{closure#1}::{closure#2}, core[ef73def78a08f8fd]::result::Result<core[ef73def78a08f8fd]::option::Option<rustc_interface[8dfe78949744dc04]::queries::Linker>, rustc_errors[768f14375179e26c]::ErrorReported>>
  23:     0x7f1281197f76 - rustc_span[8ec7e4eaff5694dc]::with_source_map::<core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>, rustc_interface[8dfe78949744dc04]::interface::create_compiler_and_run<core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>, rustc_driver[ae73c39b1f267ad6]::run_compiler::{closure#1}>::{closure#1}>
  24:     0x7f128118ac99 - rustc_interface[8dfe78949744dc04]::interface::create_compiler_and_run::<core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>, rustc_driver[ae73c39b1f267ad6]::run_compiler::{closure#1}>
  25:     0x7f1281174911 - std[5c633598124c2504]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8dfe78949744dc04]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[8dfe78949744dc04]::interface::run_compiler<core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>, rustc_driver[ae73c39b1f267ad6]::run_compiler::{closure#1}>::{closure#0}, core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>>::{closure#0}, core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>>
  26:     0x7f1281209749 - <<std[5c633598124c2504]::thread::Builder>::spawn_unchecked_<rustc_interface[8dfe78949744dc04]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[8dfe78949744dc04]::interface::run_compiler<core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>, rustc_driver[ae73c39b1f267ad6]::run_compiler::{closure#1}>::{closure#0}, core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>>::{closure#0}, core[ef73def78a08f8fd]::result::Result<(), rustc_errors[768f14375179e26c]::ErrorReported>>::{closure#1} as core[ef73def78a08f8fd]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f12808eaae3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb951606d61d3022e
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/alloc/src/boxed.rs:1854:9
  28:     0x7f12808eaae3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4ba79070544bee45
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/alloc/src/boxed.rs:1854:9
  29:     0x7f12808eaae3 - std::sys::unix::thread::Thread::new::thread_start::hab4dea554897abed
                               at /rustc/3e62ae8cce9be671b29a9715b3d390e4ad0c92d2/library/std/src/sys/unix/thread.rs:108:17
  30:     0x7f12804298ca - start_thread

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (3e62ae8cc 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z split-metadata -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
