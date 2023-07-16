
 150:     0x3020cb33a0fe - rustc_middle[139fd9a513792497]::ty::context::tls::enter_context::<<rustc_interface[dbf0b3379fbff258]::passes::QueryContext>::enter<rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/compiler/rustc_middle/src/ty/context.rs:1929:9
 151:     0x3020cb33a0fe - <rustc_interface[dbf0b3379fbff258]::passes::QueryContext>::enter::<rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/compiler/rustc_interface/src/passes.rs:765:9
 152:     0x3020cb351da9 - rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}::{closure#2}
                               at rustc-1.66.0-src/compiler/rustc_driver/src/lib.rs:375:13
 153:     0x3020cb351da9 - <rustc_interface[dbf0b3379fbff258]::interface::Compiler>::enter::<rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}::{closure#2}, core[7b9ebadd81a13581]::result::Result<core[7b9ebadd81a13581]::option::Option<rustc_interface[dbf0b3379fbff258]::queries::Linker>, rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/compiler/rustc_interface/src/queries.rs:381:19
 154:     0x3020cb2b87d6 - rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}
                               at rustc-1.66.0-src/compiler/rustc_driver/src/lib.rs:307:22
 155:     0x3020cb2b87d6 - rustc_interface[dbf0b3379fbff258]::interface::run_compiler::<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}::{closure#1}
                               at rustc-1.66.0-src/compiler/rustc_interface/src/interface.rs:327:21
 156:     0x3020cb2b87d6 - rustc_span[dc2a830b5230907e]::with_source_map::<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
                               at rustc-1.66.0-src/compiler/rustc_span/src/lib.rs:1009:5
 157:     0x3020cb340363 - rustc_interface[dbf0b3379fbff258]::interface::run_compiler::<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}
                               at rustc-1.66.0-src/compiler/rustc_interface/src/interface.rs:321:13
 158:     0x3020cb340363 - <scoped_tls[82712a8f7601681b]::ScopedKey<rustc_span[dc2a830b5230907e]::SessionGlobals>>::set::<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/vendor/scoped-tls/src/lib.rs:137:9
 159:     0x3020cb2e9710 - rustc_span[dc2a830b5230907e]::create_session_globals_then::<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}>
                               at rustc-1.66.0-src/compiler/rustc_span/src/lib.rs:111:5
 160:     0x3020cb2e9710 - rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals::<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at rustc-1.66.0-src/compiler/rustc_interface/src/util.rs:147:38
 161:     0x3020cb2e9710 - std[131a27a50fada2d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/library/std/src/sys_common/backtrace.rs:121:18
 162:     0x3020cb2cfcf9 - <std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_::<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at rustc-1.66.0-src/library/std/src/thread/mod.rs:551:17
 163:     0x3020cb2cfcf9 - <core[7b9ebadd81a13581]::panic::unwind_safe::AssertUnwindSafe<<std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[7b9ebadd81a13581]::ops::function::FnOnce<()>>::call_once
                               at rustc-1.66.0-src/library/core/src/panic/unwind_safe.rs:271:9
 164:     0x3020cb2cfcf9 - std[131a27a50fada2d5]::panicking::try::do_call::<core[7b9ebadd81a13581]::panic::unwind_safe::AssertUnwindSafe<<std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/library/std/src/panicking.rs:483:40
 165:     0x3020cb2cfcf9 - std[131a27a50fada2d5]::panicking::try::<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, core[7b9ebadd81a13581]::panic::unwind_safe::AssertUnwindSafe<<std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at rustc-1.66.0-src/library/std/src/panicking.rs:447:19
 166:     0x3020cb2cfcf9 - std[131a27a50fada2d5]::panic::catch_unwind::<core[7b9ebadd81a13581]::panic::unwind_safe::AssertUnwindSafe<<std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>
                               at rustc-1.66.0-src/library/std/src/panic.rs:137:14
 167:     0x3020cb2cfcf9 - <std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_::<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1}
                               at rustc-1.66.0-src/library/std/src/thread/mod.rs:550:30
 168:     0x3020cb2cfcf9 - <<std[131a27a50fada2d5]::thread::Builder>::spawn_unchecked_<rustc_interface[dbf0b3379fbff258]::util::run_in_thread_pool_with_globals<rustc_interface[dbf0b3379fbff258]::interface::run_compiler<core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>, rustc_driver[41081130a5d71f28]::run_compiler::{closure#1}>::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[7b9ebadd81a13581]::result::Result<(), rustc_errors[ff286ed6934e19c3]::ErrorGuaranteed>>::{closure#1} as core[7b9ebadd81a13581]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at rustc-1.66.0-src/library/core/src/ops/function.rs:251:5
 169:     0x3020c7ce1778 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6520efe9e24ffeed
                               at rustc-1.66.0-src/library/alloc/src/boxed.rs:1987:9
 170:     0x3020c7ce1778 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf35bd7b6d086be0e
                               at rustc-1.66.0-src/library/alloc/src/boxed.rs:1987:9
 171:     0x3020c7cf6e85 - std::sys::unix::thread::Thread::new::thread_start::h2f60a0fb9a4bae82
                               at rustc-1.66.0-src/library/std/src/sys/unix/thread.rs:108:17
 172:     0x3020d1d7f9ba - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0 (69f9c33d7 2022-12-12) (built from a source tarball) running on x86_64-unknown-freebsd

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=1 -C debug-assertions=on -Z unstable-options -C linker=cc -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [lint_mod] linting module `de::impls`
#1 [analysis] running analysis passes on this crate
end of query stack

