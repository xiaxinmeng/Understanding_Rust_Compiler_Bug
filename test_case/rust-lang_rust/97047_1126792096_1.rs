
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:7397 ~ cranial_coitus[a396]::passes::utils::changes::{impl#1}::combine) (NoSolution): could not prove Binder(WellFormed(&[usize; _]), [])
  --> src\passes\utils\changes.rs:75:24
   |
75 |         for _change in &self.changes {}
   |                        ^^^^^^^^^^^^^
   |
   = note: delayed at compiler\rustc_borrowck\src\type_check\canonical.rs:149:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1369:13
stack backtrace:
   0:     0x7ff8fe65988f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h19d347d7cfe5cdb5
   1:     0x7ff8fe69496a - core::fmt::write::ha4c3b8e240caf9f6
   2:     0x7ff8fe64bea9 - <std::io::IoSlice as core::fmt::Debug>::fmt::hc3dee57d919433ff
   3:     0x7ff8fe65d07b - std::panicking::default_hook::h4544884403d9be85
   4:     0x7ff8fe65cc6e - std::panicking::default_hook::h4544884403d9be85
   5:     0x7ff8cb47e8c9 - <rustc_middle[dba4d5507afbab0d]::ty::SymbolName as core[eee2e1d8d36d089]::fmt::Debug>::fmt
   6:     0x7ff8fe65d95a - std::panicking::rust_panic_with_hook::hc1cc4992a4b6ec1b
   7:     0x7ff8cfbe3055 - <rustc_errors[a795d8b482c20b3d]::diagnostic_builder::DiagnosticBuilderInner as core[eee2e1d8d36d089]::ops::drop::Drop>::drop
   8:     0x7ff8cfbe3009 - <rustc_errors[a795d8b482c20b3d]::diagnostic_builder::DiagnosticBuilderInner as core[eee2e1d8d36d089]::ops::drop::Drop>::drop
   9:     0x7ff8cff29399 - rustc_query_system[a0b9f5f0299a8a38]::query::plumbing::incremental_verify_ich_cold
  10:     0x7ff8cfbd5629 - <rustc_feature[ea871e0fc9abb418]::builtin_attrs::AttributeType as core[eee2e1d8d36d089]::fmt::Debug>::fmt
  11:     0x7ff8cfbd9288 - <rustc_errors[a795d8b482c20b3d]::HandlerInner as core[eee2e1d8d36d089]::ops::drop::Drop>::drop
  12:     0x7ff8cb4a5794 - rustc_driver[dba66f49bbdf914a]::pretty::print_after_hir_lowering
  13:     0x7ff8cb4a8bca - rustc_driver[dba66f49bbdf914a]::pretty::print_after_hir_lowering
  14:     0x7ff8cb493ecd - rustc_driver[dba66f49bbdf914a]::pretty::print_after_hir_lowering
  15:     0x7ff8cb491bf9 - rustc_driver[dba66f49bbdf914a]::pretty::print_after_hir_lowering
  16:     0x7ff8cb42f540 - <rustc_codegen_ssa[a427f0aa3144ff71]::back::linker::PtxLinker as rustc_codegen_ssa[a427f0aa3144ff71]::back::linker::Linker>::no_default_libraries
  17:     0x7ff8cb432c27 - <rustc_codegen_ssa[a427f0aa3144ff71]::back::linker::PtxLinker as rustc_codegen_ssa[a427f0aa3144ff71]::back::linker::Linker>::no_default_libraries
  18:     0x7ff8cb43c876 - <rustc_driver[dba66f49bbdf914a]::args::Error as core[eee2e1d8d36d089]::fmt::Debug>::fmt
  19:     0x7ff8cb43d998 - <rustc_driver[dba66f49bbdf914a]::args::Error as core[eee2e1d8d36d089]::fmt::Debug>::fmt
  20:     0x7ff8fe66edec - std::sys::windows::thread::Thread::new::h1883b00da4b4f517
  21:     0x7ff95ba97034 - BaseThreadInitThunk
  22:     0x7ff95d2a2651 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (6dd68402c 2022-05-11) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
