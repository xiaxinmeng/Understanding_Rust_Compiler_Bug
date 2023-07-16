plain
failures:

---- [ui] src/test/ui/lto/issue-11154.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/issue-11154.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-11154" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "prefer-dynamic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-11154/auxiliary"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'shared emitter attempted to translate a diagnostic', compiler/rustc_codegen_ssa/src/back/write.rs:1740:9
stack backtrace:
   0:     0x7f95caa96a7e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb9a388e4c3a42ede
   1:     0x7f95cab005c8 - core::fmt::write::hb99a0874fafc1b2e
   2:     0x7f95caa880c1 - std::io::Write::write_fmt::h7d2e52ee3545fd9d
   3:     0x7f95caa96881 - std::sys_common::backtrace::print::hbf3481e4686c23a7
   4:     0x7f95caa99a94 - std::panicking::default_hook::{{closure}}::h17a4608ae188f297
   5:     0x7f95caa99759 - std::panicking::default_hook::h9b10882ac2f72307
   6:     0x7f95cb48c1c4 - rustc_driver[c24bf75c354981dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f95caa9a1e4 - std::panicking::rust_panic_with_hook::h1e593b345853ada5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   8:     0x7f95caa99f09 - std::panicking::begin_panic_handler::{{closure}}::h84171bc2aaabd2b6
   9:     0x7f95caa96fb4 - std::sys_common::backtrace::__rust_end_short_backtrace::h0e2c24fcc6978482
  10:     0x7f95caa99c12 - rust_begin_unwind
  11:     0x7f95caa4aab3 - core::panicking::panic_fmt::h2fa735c63e2b824a
  12:     0x7f95cd42fe63 - <rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::SharedEmitter as rustc_errors[8cd4c504d3fb9ad]::translation::Translate>::translate_message
  13:     0x7f95cd3b4689 - <alloc[8e9a7bb7a1ed3642]::string::String as core[402a09665d8217ef]::iter::traits::collect::FromIterator<alloc[8e9a7bb7a1ed3642]::borrow::Cow<str>>>::from_iter::<core[402a09665d8217ef]::iter::adapters::map::Map<core[402a09665d8217ef]::slice::iter::Iter<(rustc_error_messages[11bf0caef6750391]::DiagnosticMessage, rustc_errors[8cd4c504d3fb9ad]::snippet::Style)>, <rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::SharedEmitter as rustc_errors[8cd4c504d3fb9ad]::translation::Translate>::translate_messages::{closure#0}>>
  14:     0x7f95cd431be0 - <rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::SharedEmitter as rustc_errors[8cd4c504d3fb9ad]::emitter::Emitter>::emit_diagnostic
  15:     0x7f95ce3c874b - <rustc_errors[8cd4c504d3fb9ad]::HandlerInner>::emit_diagnostic
  16:     0x7f95ce3c21c6 - <rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed as rustc_errors[8cd4c504d3fb9ad]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  17:     0x7f95cb692508 - <rustc_errors[8cd4c504d3fb9ad]::Handler>::emit_err::<rustc_codegen_llvm[9d2f75f782227376]::errors::DynamicLinkingWithLTO>
  18:     0x7f95cb72da8e - rustc_codegen_llvm[9d2f75f782227376]::back::lto::prepare_lto
  19:     0x7f95cb72dfeb - rustc_codegen_llvm[9d2f75f782227376]::back::lto::run_fat
  20:     0x7f95cb7fac76 - <rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend as rustc_codegen_ssa[853dcbbe8f0303d6]::traits::write::WriteBackendMethods>::run_fat_lto
  21:     0x7f95cb6e7106 - rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::generate_lto_work::<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>
  22:     0x7f95cb7eb830 - rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::start_executing_work::<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>::{closure#4}
  23:     0x7f95cb7e8cb6 - std[1b1c08cf12703b1f]::sys_common::backtrace::__rust_begin_short_backtrace::<<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend as rustc_codegen_ssa[853dcbbe8f0303d6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::start_executing_work<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>::{closure#4}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>::{closure#0}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>
  24:     0x7f95cb7e06cc - std[1b1c08cf12703b1f]::panicking::try::<core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>, core[402a09665d8217ef]::panic::unwind_safe::AssertUnwindSafe<<std[1b1c08cf12703b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend as rustc_codegen_ssa[853dcbbe8f0303d6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::start_executing_work<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>::{closure#4}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>::{closure#0}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>::{closure#1}::{closure#0}>>
  25:     0x7f95cb78e5c3 - <<std[1b1c08cf12703b1f]::thread::Builder>::spawn_unchecked_<<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend as rustc_codegen_ssa[853dcbbe8f0303d6]::traits::backend::ExtraBackendMethods>::spawn_thread<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::start_executing_work<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>::{closure#4}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>::{closure#0}, core[402a09665d8217ef]::result::Result<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, ()>>::{closure#1} as core[402a09665d8217ef]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7f95caaa698e - std::sys::unix::thread::Thread::new::thread_start::h69b3b991bedfaae0
  27:     0x7f95ca840b43 - <unknown>
  28:     0x7f95ca8d2a00 - <unknown>
  29:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (e9092cf75 2022-10-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C lto -C prefer-dynamic
query stack during panic:
end of query stack
end of query stack
thread 'rustc' panicked at '/checkout/compiler/rustc_codegen_ssa/src/back/write.rs:1882:17: panic during codegen/LLVM phase', compiler/rustc_middle/src/util/bug.rs:36:26
stack backtrace:
   0:     0x7f95caa96a7e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb9a388e4c3a42ede
   1:     0x7f95cab005c8 - core::fmt::write::hb99a0874fafc1b2e
   2:     0x7f95caa880c1 - std::io::Write::write_fmt::h7d2e52ee3545fd9d
   3:     0x7f95caa96881 - std::sys_common::backtrace::print::hbf3481e4686c23a7
   4:     0x7f95caa99a94 - std::panicking::default_hook::{{closure}}::h17a4608ae188f297
   5:     0x7f95caa99759 - std::panicking::default_hook::h9b10882ac2f72307
   6:     0x7f95cb48c1c4 - rustc_driver[c24bf75c354981dc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f95caa9a1e4 - std::panicking::rust_panic_with_hook::h1e593b345853ada5
   8:     0x7f95ce02fcc1 - std[1b1c08cf12703b1f]::panicking::begin_panic::<alloc[8e9a7bb7a1ed3642]::string::String>::{closure#0}
   9:     0x7f95ce02f01c - std[1b1c08cf12703b1f]::sys_common::backtrace::__rust_end_short_backtrace::<std[1b1c08cf12703b1f]::panicking::begin_panic<alloc[8e9a7bb7a1ed3642]::string::String>::{closure#0}, !>
  10:     0x7f95cb429281 - std[1b1c08cf12703b1f]::panicking::begin_panic::<alloc[8e9a7bb7a1ed3642]::string::String>
  11:     0x7f95ce1924a3 - std[1b1c08cf12703b1f]::panic::panic_any::<alloc[8e9a7bb7a1ed3642]::string::String>
  12:     0x7f95ce1a2128 - rustc_middle[a6cf42eca9dd0bb1]::ty::context::tls::with_context_opt::<rustc_middle[a6cf42eca9dd0bb1]::ty::context::tls::with_opt<rustc_middle[a6cf42eca9dd0bb1]::util::bug::opt_span_bug_fmt<rustc_span[c48270296f196d09]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  13:     0x7f95ce1a3bb9 - rustc_middle[a6cf42eca9dd0bb1]::util::bug::opt_span_bug_fmt::<rustc_span[c48270296f196d09]::span_encoding::Span>
  14:     0x7f95cb430b98 - rustc_middle[a6cf42eca9dd0bb1]::util::bug::bug_fmt
  15:     0x7f95cb774872 - <rustc_session[21ea6d5ee070282e]::session::Session>::time::<rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::CompiledModules, <rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::OngoingCodegen<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>>::join::{closure#0}>
  16:     0x7f95cb6f086c - <rustc_codegen_ssa[853dcbbe8f0303d6]::back::write::OngoingCodegen<rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend>>::join
  17:     0x7f95cb7fb6df - <rustc_codegen_llvm[9d2f75f782227376]::LlvmCodegenBackend as rustc_codegen_ssa[853dcbbe8f0303d6]::traits::backend::CodegenBackend>::join_codegen
  18:     0x7f95cb5cdeab - <rustc_interface[7bfca00f3f047655]::queries::Linker>::link
  19:     0x7f95cb48d9a5 - rustc_span[c48270296f196d09]::with_source_map::<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_interface[7bfca00f3f047655]::interface::run_compiler<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_driver[c24bf75c354981dc]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  20:     0x7f95cb4f153c - <scoped_tls[f2286af1576204c8]::ScopedKey<rustc_span[c48270296f196d09]::SessionGlobals>>::set::<rustc_interface[7bfca00f3f047655]::interface::run_compiler<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_driver[c24bf75c354981dc]::run_compiler::{closure#1}>::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>
  21:     0x7f95cb4acb69 - std[1b1c08cf12703b1f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7bfca00f3f047655]::util::run_in_thread_pool_with_globals<rustc_interface[7bfca00f3f047655]::interface::run_compiler<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_driver[c24bf75c354981dc]::run_compiler::{closure#1}>::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>
  22:     0x7f95cb4f2688 - std[1b1c08cf12703b1f]::panic::catch_unwind::<core[402a09665d8217ef]::panic::unwind_safe::AssertUnwindSafe<<std[1b1c08cf12703b1f]::thread::Builder>::spawn_unchecked_<rustc_interface[7bfca00f3f047655]::util::run_in_thread_pool_with_globals<rustc_interface[7bfca00f3f047655]::interface::run_compiler<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_driver[c24bf75c354981dc]::run_compiler::{closure#1}>::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>
  23:     0x7f95cb49dc4a - <<std[1b1c08cf12703b1f]::thread::Builder>::spawn_unchecked_<rustc_interface[7bfca00f3f047655]::util::run_in_thread_pool_with_globals<rustc_interface[7bfca00f3f047655]::interface::run_compiler<core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>, rustc_driver[c24bf75c354981dc]::run_compiler::{closure#1}>::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[402a09665d8217ef]::result::Result<(), rustc_errors[8cd4c504d3fb9ad]::ErrorGuaranteed>>::{closure#1} as core[402a09665d8217ef]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f95caaa698e - std::sys::unix::thread::Thread::new::thread_start::h69b3b991bedfaae0
  25:     0x7f95ca840b43 - <unknown>
  26:     0x7f95ca8d2a00 - <unknown>
  27:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (e9092cf75 2022-10-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C lto -C prefer-dynamic
query stack during panic:
end of query stack
------------------------------------------

