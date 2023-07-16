
   Compiling inconsistent_import v0.1.0 (C:\Users\Alexandre\Developpement\rust_bugs\inconsistent_import)
thread 'rustc' panicked at 'src\librustc_resolve\resolve_imports.rs:970: inconsistent resolution for an import', src\librustc\util\bug.rs:47:26
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate<'tcx>> as rustc::ty::ToPredicate<'tcx>>::to_predicate
   4: std::panicking::rust_panic_with_hook
   5: <rustc_resolve::CrateLint as core::fmt::Debug>::fmt
   6: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
   7: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
   8: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
   9: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
  10: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
  11: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a> for (rustc::hir::def::Def, rustc::ty::Visibility, syntax_pos::span_encoding::Span, syntax_pos::hygiene::Mark, rustc_resolve::build_reduced_graph::IsMacroExport)>::to_name_binding
  12: rustc_resolve::resolve_imports::<impl rustc_resolve::Resolver<'a>>::try_define
  13: rustc_resolve::resolve_imports::ImportResolver::finalize_imports
  14: rustc_resolve::Resolver::resolve_crate
  15: rustc_driver::driver::count_nodes
  16: rustc_driver::driver::compile_input
  17: rustc_driver::run_compiler
  18: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  19: rustc_driver::run_compiler
  20: <rustc_driver::CompilationFailure as core::fmt::Debug>::fmt
  21: <humantime::duration::Error as std::error::Error>::cause
  22: _rust_maybe_catch_panic
  23: <humantime::duration::Error as std::error::Error>::cause
  24: std::sys::windows::thread::Thread::new
  25: BaseThreadInitThunk
  26: RtlUserThreadStart
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (f960f377f 2018-12-24) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `inconsistent_import`.
