
D:\projects\kzvcore>cargo test --jobs 1 --all
warning: could not canonicalize path: 'D:\projects\kzvcore'
warning: could not canonicalize path: 'D:\projects\kzvcore'
warning: could not canonicalize path: 'D:\projects'
warning: could not canonicalize path: 'D:\'
   Compiling owning_ref v0.3.3
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/m
aster/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0 (766bd11c8 2018-01-01) running on i686-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
 repr: Os { code: 1, message: "Incorrect function." } }', src\libcore\result.rs:
906:4
stack backtrace:
   0: std::panicking::Location::column
   1: std::panicking::Location::column
   2: std::panicking::rust_panic_with_hook
   3: std::panicking::begin_panic_fmt
   4: std::panicking::begin_panic_fmt
   5: rust_begin_unwind
   6: core::panicking::panic_fmt
   7: <rustc_metadata::creader::ExternCrateInfo as core::fmt::Debug>::fmt
   8: rustc_metadata::locator::Context::maybe_load_library_crate
   9: rustc_metadata::creader::CrateLoader::new
  10: rustc_metadata::creader::CrateLoader::new
  11: <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateL
oader>::process_item
  12: rustc_resolve::build_reduced_graph::<impl rustc_resolve::ToNameBinding<'a>
 for (&'a rustc_resolve::ModuleData<'a>, rustc::ty::Visibility, syntax_pos::span
_encoding::Span, syntax_pos::hygiene::Mark)>::to_name_binding
  13: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as s
yntax::visit::Visitor<'a>>::visit_item
  14: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as s
yntax::visit::Visitor<'a>>::visit_ty
  15: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as s
yntax::visit::Visitor<'a>>::visit_item
  16: <rustc_resolve::resolve_imports::SingleImports<'a> as core::default::Defau
lt>::default
  17: syntax::ext::expand::MacroExpander::expand_crate
  18: syntax::ext::expand::MacroExpander::expand_crate
  19: syntax::ext::expand::MacroExpander::expand_crate
  20: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  21: <rustc_driver::Compilation as core::fmt::Debug>::fmt
  22: rustc_driver::driver::compile_input
  23: rustc_driver::run_compiler
  24: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  25: _rust_maybe_catch_panic
  26: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::Item
LikeVisitor<'v>>::visit_trait_item
  27: std::sys::imp::thread::Thread::new
  28: BaseThreadInitThunk
  29: RtlInitializeExceptionChain

error: Could not compile `owning_ref`.

To learn more, run the command again with --verbose.
