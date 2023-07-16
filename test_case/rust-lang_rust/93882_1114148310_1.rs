
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<Self as common::protocol::Packet>, [])` during codegen
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:65:43

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/compiler/rustc_middle/src/ty/consts.rs:269:52

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1358:13
stack backtrace:
   0:     0x7fd3a3948e9d - std::backtrace_rs::backtrace::libunwind::trace::h7f24d1483cf528fe
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd3a3948e9d - std::backtrace_rs::backtrace::trace_unsynchronized::h03050cc97ffe75ec
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd3a3948e9d - std::sys_common::backtrace::_print_fmt::h63b8836aefdc015a
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fd3a3948e9d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha4ac7bfb36697a5f
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fd3a39a4bdc - core::fmt::write::hd292b1f4d2c479fd
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fd3a393a591 - std::io::Write::write_fmt::h068e7531e204a916
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/io/mod.rs:1655:15
   6:     0x7fd3a394bbb5 - std::sys_common::backtrace::_print::hfabb833a6740a056
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fd3a394bbb5 - std::sys_common::backtrace::print::hb1429a0d0f928b6f
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fd3a394bbb5 - std::panicking::default_hook::{{closure}}::h34adfbc911ccf68a
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/panicking.rs:295:22
   9:     0x7fd3a394b829 - std::panicking::default_hook::h73863fded179358b
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/panicking.rs:314:9
  10:     0x7fd3a40f0861 - rustc_driver[cb28940aba8fd456]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd395a99d83 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h13befed6a3cb0bed
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/alloc/src/boxed.rs:1880:9
  12:     0x7fd395a8f48d - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hd48358bc8aa95090
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/proc_macro/src/bridge/client.rs:335:21
  13:     0x7fd3a394c386 - std::panicking::rust_panic_with_hook::h92f4e80da881a184
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/panicking.rs:702:17
  14:     0x7fd3a5252ac1 - std[3296cd2a775c5f89]::panicking::begin_panic::<rustc_errors[e23cc0f33216169e]::ExplicitBug>::{closure#0}
  15:     0x7fd3a5252a26 - std[3296cd2a775c5f89]::sys_common::backtrace::__rust_end_short_backtrace::<std[3296cd2a775c5f89]::panicking::begin_panic<rustc_errors[e23cc0f33216169e]::ExplicitBug>::{closure#0}, !>
  16:     0x7fd3a525d936 - std[3296cd2a775c5f89]::panicking::begin_panic::<rustc_errors[e23cc0f33216169e]::ExplicitBug>
  17:     0x7fd3a525a9e6 - std[3296cd2a775c5f89]::panic::panic_any::<rustc_errors[e23cc0f33216169e]::ExplicitBug>
  18:     0x7fd3a6a3cb37 - <rustc_errors[e23cc0f33216169e]::HandlerInner as core[2f0ee908fba151d6]::ops::drop::Drop>::drop
  19:     0x55c63976bd98 - core[2f0ee908fba151d6]::ptr::drop_in_place::<rustc_session[929515d6f5de6318]::parse::ParseSess>
  20:     0x55c63978310e - <alloc[3da1e44802e29726]::rc::Rc<rustc_session[929515d6f5de6318]::session::Session> as core[2f0ee908fba151d6]::ops::drop::Drop>::drop
  21:     0x55c6394dd22d - core[2f0ee908fba151d6]::ptr::drop_in_place::<rustc_interface[bc2cf492e306d845]::interface::Compiler>
  22:     0x55c6394ccb6f - rustc_span[e1069567ec31099c]::with_source_map::<core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>, rustc_interface[bc2cf492e306d845]::interface::create_compiler_and_run<core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>, rustdoc[9da64564816089ce]::main_options::{closure#0}>::{closure#1}>
  23:     0x55c63966b8f4 - rustc_interface[bc2cf492e306d845]::interface::create_compiler_and_run::<core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>, rustdoc[9da64564816089ce]::main_options::{closure#0}>
  24:     0x55c6394cf8e7 - <scoped_tls[f0d43e271921f837]::ScopedKey<rustc_span[e1069567ec31099c]::SessionGlobals>>::set::<rustdoc[9da64564816089ce]::main_args::{closure#0}, core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>>
  25:     0x55c63966be0f - std[3296cd2a775c5f89]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bc2cf492e306d845]::util::run_in_thread_pool_with_globals<rustdoc[9da64564816089ce]::main_args::{closure#0}, core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>>::{closure#0}, core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>>
  26:     0x55c6397198a9 - <<std[3296cd2a775c5f89]::thread::Builder>::spawn_unchecked_<rustc_interface[bc2cf492e306d845]::util::run_in_thread_pool_with_globals<rustdoc[9da64564816089ce]::main_args::{closure#0}, core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>>::{closure#0}, core[2f0ee908fba151d6]::result::Result<(), rustc_errors[e23cc0f33216169e]::ErrorGuaranteed>>::{closure#1} as core[2f0ee908fba151d6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fd3a39562d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h241575d295eb6c8b
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/alloc/src/boxed.rs:1866:9
  28:     0x7fd3a39562d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hddbc683041c235fd
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/alloc/src/boxed.rs:1866:9
  29:     0x7fd3a39562d3 - std::sys::unix::thread::Thread::new::thread_start::had3eb13d294e3b29
                               at /rustc/7c4b47696907d64eff5621a64eb3c6e795a9ec77/library/std/src/sys/unix/thread.rs:108:17
  30:     0x7fd3a387feae - start_thread
                               at /builddir/glibc-2.32/nptl/pthread_create.c:463:8
  31:     0x7fd3a363d2ff - __GI___clone
                               at /builddir/glibc-2.32/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  32:                0x0 - <unknown>
