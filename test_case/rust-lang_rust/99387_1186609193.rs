`

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f8a9791dc80 - std::backtrace_rs::backtrace::libunwind::trace::hfd97d34f5b9adb5a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f8a9791dc80 - std::backtrace_rs::backtrace::trace_unsynchronized::ha08ef33cdd1a8d14
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f8a9791dc80 - std::sys_common::backtrace::_print_fmt::haf7cabbcd4c09172
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f8a9791dc80 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7e27323de08c589f
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f8a9798b5a8 - core::fmt::write::hfbeb0e19f1f81a2c
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f8a978e64df - std::io::Write::write_fmt::h0c909a73698b88b2
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1672:15
   6:     0x7f8a978ecdce - std::sys_common::backtrace::_print::he5cc0f9d8f290756
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f8a978ecdce - std::sys_common::backtrace::print::h180abf93ce822247
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f8a978ecdce - std::panicking::default_hook::{{closure}}::h0088be320aa53a67
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:295:22
   9:     0x7f8a978eca19 - std::panicking::default_hook::he281460b60075c8f
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:314:9
  10:     0x7f8a986ae16a - <alloc[bb226ae01fa67229]::boxed::Box<dyn for<'a, 'b> core[68c69b1964127ede]::ops::function::Fn<(&'a core[68c69b1964127ede]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[68c69b1964127ede]::marker::Send + core[68c69b1964127ede]::marker::Sync> as core[68c69b1964127ede]::ops::function::Fn<(&core[68c69b1964127ede]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1949:9
  11:     0x7f8a986ae16a - rustc_driver[c951fd703985c6d1]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1153:13
  12:     0x7f8a978ed539 - std::panicking::rust_panic_with_hook::h278c3ffd37674291
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:702:17
  13:     0x7f8a9dcd6f83 - std[a34c85a79a5ff9e1]::panicking::begin_panic::<rustc_errors[6f9c501190a828e4]::ExplicitBug>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:617:9
  14:     0x7f8a9dcd5fa6 - std[a34c85a79a5ff9e1]::sys_common::backtrace::__rust_end_short_backtrace::<std[a34c85a79a5ff9e1]::panicking::begin_panic<rustc_errors[6f9c501190a828e4]::ExplicitBug>::{closure#0}, !>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:138:18
  15:     0x7f8a98630e06 - std[a34c85a79a5ff9e1]::panicking::begin_panic::<rustc_errors[6f9c501190a828e4]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:616:12
  16:     0x7f8a9dd1bd06 - std[a34c85a79a5ff9e1]::panic::panic_any::<rustc_errors[6f9c501190a828e4]::ExplicitBug>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:61:5
  17:     0x7f8a9dd20e83 - <rustc_errors[6f9c501190a828e4]::HandlerInner as core[68c69b1964127ede]::ops::drop::Drop>::drop
  18:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_errors[6f9c501190a828e4]::HandlerInner>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  19:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<core[68c69b1964127ede]::cell::UnsafeCell<rustc_errors[6f9c501190a828e4]::HandlerInner>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  20:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<core[68c69b1964127ede]::cell::RefCell<rustc_errors[6f9c501190a828e4]::HandlerInner>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  21:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_data_structures[5381e28d570373a7]::sync::Lock<rustc_errors[6f9c501190a828e4]::HandlerInner>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  22:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_errors[6f9c501190a828e4]::Handler>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  23:     0x7f8a986c3c48 - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_session[4aa44555e437e6f6]::parse::ParseSess>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  24:     0x7f8a986cf055 - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_session[4aa44555e437e6f6]::session::Session>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  25:     0x7f8a986cf055 - <alloc[bb226ae01fa67229]::rc::Rc<rustc_session[4aa44555e437e6f6]::session::Session> as core[68c69b1964127ede]::ops::drop::Drop>::drop
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/rc.rs:1559:17
  26:     0x7f8a9877254c - core[68c69b1964127ede]::ptr::drop_in_place::<alloc[bb226ae01fa67229]::rc::Rc<rustc_session[4aa44555e437e6f6]::session::Session>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  27:     0x7f8a9877254c - core[68c69b1964127ede]::ptr::drop_in_place::<rustc_interface[544e148730d0046e]::interface::Compiler>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:487:1
  28:     0x7f8a9876fc6b - core[68c69b1964127ede]::mem::drop::<rustc_interface[544e148730d0046e]::interface::Compiler>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/mem/mod.rs:974:24
  29:     0x7f8a9876fc6b - rustc_interface[544e148730d0046e]::interface::create_compiler_and_run::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#1}::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:327:60
  30:     0x7f8a9876fc6b - <rustc_data_structures[5381e28d570373a7]::profiling::TimingGuard>::run::<(), rustc_interface[544e148730d0046e]::interface::create_compiler_and_run<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#1}::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:718:9
  31:     0x7f8a9876fc6b - rustc_interface[544e148730d0046e]::interface::create_compiler_and_run::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:327:9
  32:     0x7f8a9876fc6b - rustc_span[8ecc148ab01e39e]::with_source_map::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_interface[544e148730d0046e]::interface::create_compiler_and_run<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:986:5
  33:     0x7f8a986d7d69 - rustc_interface[544e148730d0046e]::interface::create_compiler_and_run::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:317:5
  34:     0x7f8a986d7d69 - rustc_interface[544e148730d0046e]::interface::run_compiler::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:337:12
  35:     0x7f8a986d7d69 - <scoped_tls[a31a5a0e226b4d9f]::ScopedKey<rustc_span[8ecc148ab01e39e]::SessionGlobals>>::set::<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  36:     0x7f8a9876bb0f - rustc_span[8ecc148ab01e39e]::create_session_globals_then::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:112:5
  37:     0x7f8a9876bb0f - rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals::<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:160:32
  38:     0x7f8a9876bb0f - std[a34c85a79a5ff9e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:122:18
  39:     0x7f8a9875f3fa - <std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_::<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:505:17
  40:     0x7f8a9875f3fa - <core[68c69b1964127ede]::panic::unwind_safe::AssertUnwindSafe<<std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[68c69b1964127ede]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
  41:     0x7f8a9875f3fa - std[a34c85a79a5ff9e1]::panicking::try::do_call::<core[68c69b1964127ede]::panic::unwind_safe::AssertUnwindSafe<<std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:492:40
  42:     0x7f8a9875f3fa - std[a34c85a79a5ff9e1]::panicking::try::<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, core[68c69b1964127ede]::panic::unwind_safe::AssertUnwindSafe<<std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:456:19
  43:     0x7f8a9875f3fa - std[a34c85a79a5ff9e1]::panic::catch_unwind::<core[68c69b1964127ede]::panic::unwind_safe::AssertUnwindSafe<<std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
  44:     0x7f8a9875f3fa - <std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_::<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:504:30
  45:     0x7f8a9875f3fa - <<std[a34c85a79a5ff9e1]::thread::Builder>::spawn_unchecked_<rustc_interface[544e148730d0046e]::util::run_in_thread_pool_with_globals<rustc_interface[544e148730d0046e]::interface::run_compiler<core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>, rustc_driver[c951fd703985c6d1]::run_compiler::{closure#1}>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#0}, core[68c69b1964127ede]::result::Result<(), rustc_errors[6f9c501190a828e4]::ErrorGuaranteed>>::{closure#1} as core[68c69b1964127ede]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:248:5
  46:     0x7f8a978ee405 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h920bbd4619823104
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1935:9
  47:     0x7f8a978ee405 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hdcffa1fbabb23688
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1935:9
  48:     0x7f8a978ee405 - std::sys::unix::thread::Thread::new::thread_start::h1e3eea9b9bf258db
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7f8a9768854d - <unknown>
  50:     0x7f8a9770d874 - clone
  51:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
