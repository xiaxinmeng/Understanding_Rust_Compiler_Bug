plain
+ gather_profiles Debug,Opt Full syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18
+ cd /checkout/obj
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib ../library/core/src/lib.rs
thread 'rustc' panicked at 'assertion failed: import.imported_module.get().is_none()', compiler/rustc_resolve/src/imports.rs:616:21
stack backtrace:
   0:     0x7fc78fe90f2d - std::backtrace_rs::backtrace::libunwind::trace::h10d9c821301299c7
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc78fe90f2d - std::backtrace_rs::backtrace::trace_unsynchronized::h62bbd4af3b105f2b
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc78fe90f2d - std::sys_common::backtrace::_print_fmt::h67dcf976014ffece
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fc78fe90f2d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb276cbd7eb804d00
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fc78feecbbc - core::fmt::write::h12fc369efba2737e
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/core/src/fmt/mod.rs:1196:17
   5:     0x7fc78fe826b1 - std::io::Write::write_fmt::h5fe73b39bca430ea
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/io/mod.rs:1654:15
   6:     0x7fc78fe93c45 - std::sys_common::backtrace::_print::h8bf9333d9240f4c1
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fc78fe93c45 - std::sys_common::backtrace::print::h2ea33b22e18831d7
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fc78fe93c45 - std::panicking::default_hook::{{closure}}::h1bbca846c3548569
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/panicking.rs:295:22
   9:     0x7fc78fe938b9 - std::panicking::default_hook::h140313aab3794fee
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/panicking.rs:314:9
  10:     0x7fc790803ec9 - rustc_driver[531ed99341e5cdc]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fc78fe94416 - std::panicking::rust_panic_with_hook::ha6363c5e25c75dc3
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/panicking.rs:702:17
  12:     0x7fc78fe941d9 - std::panicking::begin_panic_handler::{{closure}}::h802c112c403903fc
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/panicking.rs:586:13
  13:     0x7fc78fe913e4 - std::sys_common::backtrace::__rust_end_short_backtrace::h3438afdfd9dbd0b4
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7fc78fe93f49 - rust_begin_unwind
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/panicking.rs:584:5
  15:     0x7fc78fe591d3 - core::panicking::panic_fmt::h92255e691d408f7d
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/core/src/panicking.rs:142:14
  16:     0x7fc78fe5909d - core::panicking::panic::h0f080e5b77601592
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/core/src/panicking.rs:48:5
  17:     0x7fc79130f173 - <rustc_resolve[826f4782a7f11e1a]::imports::ImportResolver>::finalize_import
  18:     0x7fc791308cdc - <rustc_resolve[826f4782a7f11e1a]::imports::ImportResolver>::finalize_imports
  19:     0x7fc79132e075 - <rustc_session[90c42aaf65bd0aa9]::session::Session>::time::<(), <rustc_resolve[826f4782a7f11e1a]::Resolver>::resolve_crate::{closure#0}>
  20:     0x7fc7908d0158 - rustc_interface[9be6c5f9b6bf79fe]::passes::configure_and_expand
  21:     0x7fc790985a14 - <rustc_interface[9be6c5f9b6bf79fe]::queries::Queries>::expansion
  22:     0x7fc79079cd48 - <rustc_interface[9be6c5f9b6bf79fe]::interface::Compiler>::enter::<rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}::{closure#2}, core[4a868bd7bb084c2d]::result::Result<core[4a868bd7bb084c2d]::option::Option<rustc_interface[9be6c5f9b6bf79fe]::queries::Linker>, rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>
  23:     0x7fc79080afc3 - rustc_span[1d014ce893e541e6]::with_source_map::<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_interface[9be6c5f9b6bf79fe]::interface::create_compiler_and_run<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}>::{closure#1}>
  24:     0x7fc7907b8221 - rustc_interface[9be6c5f9b6bf79fe]::interface::create_compiler_and_run::<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}>
  25:     0x7fc790796a72 - <scoped_tls[f6285c128543b3c2]::ScopedKey<rustc_span[1d014ce893e541e6]::SessionGlobals>>::set::<rustc_interface[9be6c5f9b6bf79fe]::interface::run_compiler<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}>::{closure#0}, core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>
  26:     0x7fc7907bfbdf - std[c817509785e69c99]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9be6c5f9b6bf79fe]::util::run_in_thread_pool_with_globals<rustc_interface[9be6c5f9b6bf79fe]::interface::run_compiler<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}>::{closure#0}, core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>::{closure#0}, core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>
  27:     0x7fc7907c0d69 - <<std[c817509785e69c99]::thread::Builder>::spawn_unchecked_<rustc_interface[9be6c5f9b6bf79fe]::util::run_in_thread_pool_with_globals<rustc_interface[9be6c5f9b6bf79fe]::interface::run_compiler<core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>, rustc_driver[531ed99341e5cdc]::run_compiler::{closure#1}>::{closure#0}, core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>::{closure#0}, core[4a868bd7bb084c2d]::result::Result<(), rustc_errors[2595838e3716d1bb]::ErrorGuaranteed>>::{closure#1} as core[4a868bd7bb084c2d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7fc78fe9e333 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h27e95500398e40f5
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/alloc/src/boxed.rs:1872:9
  29:     0x7fc78fe9e333 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6ace280cf86ad594
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/alloc/src/boxed.rs:1872:9
  30:     0x7fc78fe9e333 - std::sys::unix::thread::Thread::new::thread_start::hc3687841ff5268c3
                               at /rustc/20dd8008d52592f40dfc3aa396d7f10ce1dcda5c/library/std/src/sys/unix/thread.rs:108:17
  31:     0x7fc78f9d98ca - start_thread
  32:     0x7fc78f537abd - clone
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

