
warning: unused import: `SpanExt`
 --> \swc\macros\common\src\lib.rs:3:35
  |
3 | use pmutil::{synom_ext::FromSpan, SpanExt};
  |                                   ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

   Compiling swc_ecma_parser v0.12.0
thread 'rustc' panicked at 'src\librustc_resolve\resolve_imports.rs:880: inconsistent resolution for an import', src\librustc\util\bug.rs:37:26
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSlice as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as syntax::visit::Visitor>::visit_ty
   8: <rustc_resolve::check_unused::UnusedImportCheckVisitor as syntax::visit::Visitor>::visit_use_tree
   9: <rustc_resolve::check_unused::UnusedImportCheckVisitor as syntax::visit::Visitor>::visit_use_tree
  10: <rustc_resolve::check_unused::UnusedImportCheckVisitor as syntax::visit::Visitor>::visit_use_tree
  11: <rustc_resolve::check_unused::UnusedImportCheckVisitor as syntax::visit::Visitor>::visit_use_tree
  12: <rustc_resolve::check_unused::UnusedImportCheckVisitor as syntax::visit::Visitor>::visit_use_tree
  13: rustc_resolve::resolve_imports::ImportResolver::finalize_imports
  14: rustc_resolve::resolve_imports::ImportResolver::finalize_imports
  15: rustc_resolve::Resolver::resolve_crate
  16: rustc_interface::passes::BoxedResolver::to_resolver_outputs
  17: rustc_interface::passes::BoxedResolver::complete
  18: rustc_interface::passes::BoxedResolver::complete
  19: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::compile
  20: rustc_interface::queries::Queries::expansion
  21: rustc_driver::pretty::print_after_hir_lowering
  22: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  23: <rustc_target::spec::Target as rustc_target::spec::HasTargetSpec>::target_spec
  24: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  25: <rustc_target::spec::Target as rustc_target::spec::HasTargetSpec>::target_spec
  26: _rust_maybe_catch_panic
  27: rustc_driver::pretty::print_after_hir_lowering
  28: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  29: std::sys::windows::thread::Thread::new
  30: BaseThreadInitThunk
  31: RtlUserThreadStart

note: rustc 1.41.0-nightly (ae1b871cc 2019-12-06) running on x86_64-pc-windows-msvc

note: compiler flags: -C opt-level=3 --crate-type lib

query stack during panic:
end of query stack
error: could not compile `swc_ecma_parser`.
