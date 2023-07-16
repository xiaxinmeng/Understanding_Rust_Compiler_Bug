plain
---- [codegen] src/test/codegen/cdylib-external-inline-fns.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/cdylib-external-inline-fns.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cdylib-external-inline-fns" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cdylib-external-inline-fns/auxiliary" "-C" "no-prepopulate-passes"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `cdylib` for target `i686-unknown-linux-musl`
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', compiler/rustc_codegen_ssa/src/back/link.rs:233:10
stack backtrace:
stack backtrace:
   0:     0x7f957a7f356a - std::backtrace_rs::backtrace::libunwind::trace::hbcc5f3a9e1cd996a
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f957a7f356a - std::backtrace_rs::backtrace::trace_unsynchronized::h418ebc9dbc261fa4
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f957a7f356a - std::sys_common::backtrace::_print_fmt::h4d80d4e58740b093
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f957a7f356a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d0dce271f936bdf
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f957a85423e - core::fmt::write::h1976fbf26e351fcd
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f957a7e5415 - std::io::Write::write_fmt::hf820e53f8efc3144
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/io/mod.rs:1682:15
   6:     0x7f957a7f3335 - std::sys_common::backtrace::_print::hee87f99d078e746c
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f957a7f3335 - std::sys_common::backtrace::print::ha871a16f97e1caee
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f957a7f60ff - std::panicking::default_hook::{{closure}}::h610717dd42490fae
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/panicking.rs:267:22
   9:     0x7f957a7f5e3b - std::panicking::default_hook::h59ea75b1794e77ec
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/panicking.rs:286:9
  10:     0x7f957b550cd5 - rustc_driver[cc264430102de7b8]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f957a7f68dd - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::he918841609f951e0
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/alloc/src/boxed.rs:2002:9
  12:     0x7f957a7f68dd - std::panicking::rust_panic_with_hook::hea300512bc648d5c
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/panicking.rs:692:13
  13:     0x7f957a7f6659 - std::panicking::begin_panic_handler::{{closure}}::h8f6005df80b4745f
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/panicking.rs:579:13
  14:     0x7f957a7f3a1c - std::sys_common::backtrace::__rust_end_short_backtrace::hb1d6add282356534
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7f957a7f6362 - rust_begin_unwind
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/panicking.rs:575:5
  16:     0x7f957a7b8943 - core::panicking::panic_fmt::h18c7278e248f0bf0
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/core/src/panicking.rs:64:14
  17:     0x7f957a7b8ab2 - core::panicking::panic_bounds_check::h699b72ad05c24658
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/core/src/panicking.rs:159:5
  18:     0x7f95800e351d - rustc_codegen_ssa[817e8c0aefaec8f0]::back::link::each_linked_rlib
  19:     0x7f957b72d3a0 - rustc_codegen_ssa[817e8c0aefaec8f0]::back::write::start_executing_work::<rustc_codegen_llvm[dbc9541bb27bc4b7]::LlvmCodegenBackend>
  20:     0x7f957b72cbc5 - rustc_codegen_ssa[817e8c0aefaec8f0]::back::write::start_async_codegen::<rustc_codegen_llvm[dbc9541bb27bc4b7]::LlvmCodegenBackend>
  21:     0x7f957b744639 - rustc_codegen_ssa[817e8c0aefaec8f0]::base::codegen_crate::<rustc_codegen_llvm[dbc9541bb27bc4b7]::LlvmCodegenBackend>
  22:     0x7f957b700438 - <rustc_codegen_llvm[dbc9541bb27bc4b7]::LlvmCodegenBackend as rustc_codegen_ssa[817e8c0aefaec8f0]::traits::backend::CodegenBackend>::codegen_crate
  23:     0x7f957b6a670f - <rustc_session[7c175f266e5efc77]::session::Session>::time::<alloc[feada68de6120740]::boxed::Box<dyn core[b72dd3c917181b9b]::any::Any>, rustc_interface[bb6ff79c2299f250]::passes::start_codegen::{closure#0}>
  24:     0x7f957b5eca12 - rustc_interface[bb6ff79c2299f250]::passes::start_codegen
  25:     0x7f957b5eba33 - <rustc_interface[bb6ff79c2299f250]::passes::QueryContext>::enter::<<rustc_interface[bb6ff79c2299f250]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b72dd3c917181b9b]::result::Result<alloc[feada68de6120740]::boxed::Box<dyn core[b72dd3c917181b9b]::any::Any>, rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>
  26:     0x7f957b66069d - <rustc_interface[bb6ff79c2299f250]::queries::Queries>::ongoing_codegen
  27:     0x7f957b52676c - <rustc_interface[bb6ff79c2299f250]::interface::Compiler>::enter::<rustc_driver[cc264430102de7b8]::run_compiler::{closure#1}::{closure#2}, core[b72dd3c917181b9b]::result::Result<core[b72dd3c917181b9b]::option::Option<rustc_interface[bb6ff79c2299f250]::queries::Linker>, rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>
  28:     0x7f957b4b383b - rustc_span[3f48495f1bbd1773]::with_source_map::<core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>, rustc_interface[bb6ff79c2299f250]::interface::run_compiler<core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>, rustc_driver[cc264430102de7b8]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  29:     0x7f957b517a0b - <scoped_tls[26d13d110549ea75]::ScopedKey<rustc_span[3f48495f1bbd1773]::SessionGlobals>>::set::<rustc_interface[bb6ff79c2299f250]::interface::run_compiler<core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>, rustc_driver[cc264430102de7b8]::run_compiler::{closure#1}>::{closure#0}, core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>
  30:     0x7f957b4dd540 - std[5bbc4ac3d506bc68]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bb6ff79c2299f250]::util::run_in_thread_pool_with_globals<rustc_interface[bb6ff79c2299f250]::interface::run_compiler<core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>, rustc_driver[cc264430102de7b8]::run_compiler::{closure#1}>::{closure#0}, core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>
  31:     0x7f957b4bf84d - <<std[5bbc4ac3d506bc68]::thread::Builder>::spawn_unchecked_<rustc_interface[bb6ff79c2299f250]::util::run_in_thread_pool_with_globals<rustc_interface[bb6ff79c2299f250]::interface::run_compiler<core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>, rustc_driver[cc264430102de7b8]::run_compiler::{closure#1}>::{closure#0}, core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b72dd3c917181b9b]::result::Result<(), rustc_errors[1aa750ff8fd56315]::ErrorGuaranteed>>::{closure#1} as core[b72dd3c917181b9b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f957a800cc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he5fcae7b8063aaa2
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/alloc/src/boxed.rs:1988:9
  33:     0x7f957a800cc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3a3eeabe581104ec
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/alloc/src/boxed.rs:1988:9
  34:     0x7f957a800cc3 - std::sys::unix::thread::Thread::new::thread_start::h6c2923fce3683bae
                               at /rustc/aaeecca1456e1f5d90a3f5e2cb56686816f3a287/library/std/src/sys/unix/thread.rs:108:17
  35:     0x7f957a5b2b43 - <unknown>
  36:     0x7f957a644a00 - <unknown>
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (aaeecca14 2023-01-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=/musl-i686/bin/musl-gcc -C no-prepopulate-passes
query stack during panic:
end of query stack
warning: 1 warning emitted
------------------------------------------
