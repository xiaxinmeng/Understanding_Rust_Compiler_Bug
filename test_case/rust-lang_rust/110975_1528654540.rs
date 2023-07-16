plain
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... FAILED

tests/fail/panic/double_panic.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "tests/fail/panic/double_panic.rs"
fail test got exit status: 101, but expected 1

actual output differed from expected
--- tests/fail/panic/double_panic.stderr
--- tests/fail/panic/double_panic.stderr
+++ <stderr output>
... 66 lines skipped ...
   31: std::rt::lang_start
  at RUSTLIB/std/src/rt.rs:LL:CC
-thread panicked while panicking. aborting.
-error: abnormal termination: the program aborted execution
-  --> RUSTLIB/std/src/sys/PLATFORM/mod.rs:LL:CC
+thread 'rustc' panicked at 'the panic runtime should avoid double-panics', src/tools/miri/src/shims/panic.rs:LL:CC
+   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb341d153fb909bc6
+   1: core::fmt::write::h019bc813d31bd4a7
+   2: std::io::Write::write_fmt::h1b900118e7ff9edb
+   3: std::sys_common::backtrace::print::h70e007d1edb65af2
+   3: std::sys_common::backtrace::print::h70e007d1edb65af2
+   4: std::panicking::default_hook::{{closure}}::h37879ae9ebe83737
+   5: std::panicking::default_hook::h5e78e2c01bdcf170
+   6: rustc_driver_impl[18302c18ee201253]::DEFAULT_HOOK::{closure#0}::{closure#0}
+   7: std::panicking::rust_panic_with_hook::h9a3de04da103346c
+   8: std::panicking::begin_panic_handler::{{closure}}::ha914bf392dc2074a
+   9: std::sys_common::backtrace::__rust_end_short_backtrace::he0854533685abcbd
+  11: core::panicking::panic_fmt::h01968f1840baef26
+  11: core::panicking::panic_fmt::h01968f1840baef26
+  12: <rustc_const_eval[b5545eb9aecc4f18]::interpret::eval_context::InterpCx<miri[e8ae0fae90148189]::machine::MiriMachine>>::eval_fn_call
+  13: <rustc_const_eval[b5545eb9aecc4f18]::interpret::eval_context::InterpCx<miri[e8ae0fae90148189]::machine::MiriMachine> as miri[e8ae0fae90148189]::concurrency::thread::EvalContextExt>::run_threads
+  14: miri[e8ae0fae90148189]::eval::eval_entry
+  15: <rustc_interface[f1d030898e59702d]::queries::QueryResult<&rustc_middle[222233f2b60e4710]::ty::context::GlobalCtxt>>::enter
+  16: <miri[678468416af3e5d8]::MiriCompilerCalls as rustc_driver_impl[18302c18ee201253]::Callbacks>::after_analysis
+  17: <rustc_interface[f1d030898e59702d]::interface::Compiler>::enter
+  18: rustc_span[7b254133821462c1]::set_source_map
+  19: <scoped_tls[8f4d813dc3227d5a]::ScopedKey<rustc_span[7b254133821462c1]::SessionGlobals>>::set
+  20: std[265d1c747912c40]::sys_common::backtrace::__rust_begin_short_backtrace
+  21: <<std[265d1c747912c40]::thread::Builder>::spawn_unchecked_<rustc_interface[f1d030898e59702d]::util::run_in_thread_pool_with_globals<rustc_interface[f1d030898e59702d]::interface::run_compiler<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}>::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>::{closure#1} as core[64565efd276796ca]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  22: std::sys::PLATFORM::thread::Thread::new::thread_start::hafb305a714020f44
+  23: <unknown>
+  24: <unknown>
+  25: <unknown>
+error: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+
+
+note: rustc 1.71.0-nightly (b2ac2a56f 2023-04-29) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -Z ui-testing
+
+query stack during panic:
+end of query stack
+
+Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
+note: the place in the program where the ICE was triggered
    |
-LL |     ABORT();
-   | ^ the program aborted execution
-   | ^ the program aborted execution
+LL |     miri_start_panic(Box::into_raw(payload_box) as *mut u8)
    |
-   = note: inside `std::sys::PLATFORM::abort_internal` at RUSTLIB/std/src/sys/PLATFORM/mod.rs:LL:CC
+   = note: inside `panic_unwind::imp::panic` at RUSTLIB/panic_unwind/src/miri.rs:LL:CC
+   = note: inside `panic_unwind::__rust_start_panic` at RUSTLIB/panic_unwind/src/lib.rs:LL:CC
---
   3:           0xb8ee67 - std::sys_common::backtrace::_print_fmt
                               at /checkout/library/std/src/sys_common/backtrace.rs:65:5
   4:           0xb8ca22 - <std::sys_common::backtrace::_print::DisplayBacktrace as std::fmt::Display>::fmt
                               at /checkout/library/std/src/sys_common/backtrace.rs:44:22
   5:           0xbb8a08 - core::fmt::rt::Argument::<'_>::fmt
                               at /checkout/library/core/src/fmt/rt.rs:138:9
                               at /checkout/library/core/src/fmt/mod.rs:1105:17
   7:           0x21f90c - <std::sys::unix::stdio::Stderr as std::io::Write>::write_fmt
                               at /checkout/library/std/src/io/mod.rs:1712:15
   8:           0xb8eb72 - std::sys_common::backtrace::_print
---
  30:           0x14e1d4 - std::rt::lang_start_internal
                               at /checkout/library/std/src/rt.rs:148:20
  31:           0x14e3bb - std::rt::lang_start::<()>
                               at /checkout/library/std/src/rt.rs:165:17
thread 'rustc' panicked at 'the panic runtime should avoid double-panics', src/tools/miri/src/shims/panic.rs:66:9
   0:     0x7f079379bc01 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb341d153fb909bc6
   1:     0x7f0793801ed8 - core::fmt::write::h019bc813d31bd4a7
   2:     0x7f07937901c1 - std::io::Write::write_fmt::h1b900118e7ff9edb
   3:     0x7f079379ba15 - std::sys_common::backtrace::print::h70e007d1edb65af2
   3:     0x7f079379ba15 - std::sys_common::backtrace::print::h70e007d1edb65af2
   4:     0x7f079379e9d4 - std::panicking::default_hook::{{closure}}::h37879ae9ebe83737
   5:     0x7f079379e731 - std::panicking::default_hook::h5e78e2c01bdcf170
   6:     0x7f07941e4b25 - rustc_driver_impl[18302c18ee201253]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f079379f0d2 - std::panicking::rust_panic_with_hook::h9a3de04da103346c
   8:     0x7f079379ee22 - std::panicking::begin_panic_handler::{{closure}}::ha914bf392dc2074a
   9:     0x7f079379c0c6 - std::sys_common::backtrace::__rust_end_short_backtrace::he0854533685abcbd
  10:     0x7f079379eb82 - rust_begin_unwind
  11:     0x7f0793758c23 - core::panicking::panic_fmt::h01968f1840baef26
  12:     0x557267cf60dd - <rustc_const_eval[b5545eb9aecc4f18]::interpret::eval_context::InterpCx<miri[e8ae0fae90148189]::machine::MiriMachine>>::eval_fn_call
  13:     0x557267d41a60 - <rustc_const_eval[b5545eb9aecc4f18]::interpret::eval_context::InterpCx<miri[e8ae0fae90148189]::machine::MiriMachine> as miri[e8ae0fae90148189]::concurrency::thread::EvalContextExt>::run_threads
  14:     0x557267c92396 - miri[e8ae0fae90148189]::eval::eval_entry
  15:     0x557267bcb11c - <rustc_interface[f1d030898e59702d]::queries::QueryResult<&rustc_middle[222233f2b60e4710]::ty::context::GlobalCtxt>>::enter::<(), <miri[678468416af3e5d8]::MiriCompilerCalls as rustc_driver_impl[18302c18ee201253]::Callbacks>::after_analysis::{closure#0}>
  16:     0x557267bd3f1a - <miri[678468416af3e5d8]::MiriCompilerCalls as rustc_driver_impl[18302c18ee201253]::Callbacks>::after_analysis
  17:     0x7f079423f535 - <rustc_interface[f1d030898e59702d]::interface::Compiler>::enter::<rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}::{closure#2}, core[64565efd276796ca]::result::Result<core[64565efd276796ca]::option::Option<rustc_interface[f1d030898e59702d]::queries::Linker>, rustc_span[7b254133821462c1]::ErrorGuaranteed>>
  18:     0x7f07942040ff - rustc_span[7b254133821462c1]::set_source_map::<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_interface[f1d030898e59702d]::interface::run_compiler<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  19:     0x7f07941f3851 - <scoped_tls[8f4d813dc3227d5a]::ScopedKey<rustc_span[7b254133821462c1]::SessionGlobals>>::set::<rustc_interface[f1d030898e59702d]::interface::run_compiler<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}>::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>
  20:     0x7f07942104a9 - std[265d1c747912c40]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f1d030898e59702d]::util::run_in_thread_pool_with_globals<rustc_interface[f1d030898e59702d]::interface::run_compiler<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}>::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>
  21:     0x7f07941faf81 - <<std[265d1c747912c40]::thread::Builder>::spawn_unchecked_<rustc_interface[f1d030898e59702d]::util::run_in_thread_pool_with_globals<rustc_interface[f1d030898e59702d]::interface::run_compiler<core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>, rustc_driver_impl[18302c18ee201253]::run_compiler::{closure#1}>::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[64565efd276796ca]::result::Result<(), rustc_span[7b254133821462c1]::ErrorGuaranteed>>::{closure#1} as core[64565efd276796ca]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f07937ab95e - std::sys::unix::thread::Thread::new::thread_start::hafb305a714020f44
  23:     0x7f0793441b43 - <unknown>
  24:     0x7f07934d3a00 - <unknown>
  25:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (b2ac2a56f 2023-04-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing

query stack during panic:
end of query stack

Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
note: the place in the program where the ICE was triggered
   |
   |
LL |     miri_start_panic(Box::into_raw(payload_box) as *mut u8)
   |
   = note: inside `panic_unwind::imp::panic` at /checkout/library/panic_unwind/src/miri.rs:18:5: 18:60
   = note: inside `panic_unwind::__rust_start_panic` at /checkout/library/panic_unwind/src/lib.rs:105:5: 105:24
   = note: inside `std::panicking::rust_panic` at /checkout/library/std/src/panicking.rs:755:25: 755:48
