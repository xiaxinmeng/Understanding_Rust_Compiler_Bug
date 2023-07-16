
 cargo +beta build --target=i686-pc-windows-msvc
   Compiling windows_gen v0.4.0 (C:\Users\ryanl\Code\windows-rs\crates\gen)
thread 'rustc' panicked at 'compiler\rustc_resolve\src\imports.rs:904:25: inconsistent resolution for an import', /rustc/4d25f4607015a56d18d7c6c649441608a9298845\library\std\src\panic.rs:59:5
stack backtrace:
   0:     0x7fffb8c8896e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4250b92f779127fa
   1:     0x7fffb8cb47cb - core::fmt::write::h03b5df1724a61ac9
   2:     0x7fffb8c7be68 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hfbc56bd070366dc3
   3:     0x7fffb8c8cb3d - std::panicking::take_hook::h63160ec33aab7934
   4:     0x7fffb8c8c609 - std::panicking::take_hook::h63160ec33aab7934
   5:     0x7fffa1dfd475 - rustc_driver::report_ice::hfe8ada112a6b706d
   6:     0x7fffb8c8d5a2 - std::panicking::rust_panic_with_hook::hdf6c3e39bb7a4662
   7:     0x7fffa47a17d8 - <rustc_resolve::imports::UnresolvedImportError as core::fmt::Debug>::fmt::h2263387dae85b984
   8:     0x7fffa47a165f - <rustc_resolve::imports::UnresolvedImportError as core::fmt::Debug>::fmt::h2263387dae85b984
   9:     0x7fffa47a16e6 - <rustc_resolve::imports::UnresolvedImportError as core::fmt::Debug>::fmt::h2263387dae85b984
  10:     0x7fffa47d1d06 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  11:     0x7fffa47d0e2c - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  12:     0x7fffa47d0ce1 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  13:     0x7fffa47d0c43 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  14:     0x7fffa47d0d89 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  15:     0x7fffa47d0d49 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  16:     0x7fffa478af53 - rustc_resolve::imports::ImportResolver::finalize_imports::h058aecd33a8078be
  17:     0x7fffa4784d0a - rustc_resolve::imports::ImportResolver::finalize_imports::h058aecd33a8078be
  18:     0x7fffa47d1185 - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor as rustc_ast::visit::Visitor>::visit_ty::h7f161ddb75dc5d05
  19:     0x7fffa482ff22 - rustc_resolve::Resolver::resolve_crate::h8a92aa605b0036a3
  20:     0x7fffa1ff61f8 - rustc_interface::passes::BoxedResolver::to_resolver_outputs::h2d92347711680db0
  21:     0x7fffa1fd47ff - rustc_interface::interface::parse_cfgspecs::hd94eda9d41345067
  22:     0x7fffa1fc3146 - rustc_interface::util::commit_date_str::hb81a5a6e14a3c6be
  23:     0x7fffa1ff4647 - rustc_interface::interface::parse_cfgspecs::hd94eda9d41345067
  24:     0x7fffa202c516 - rustc_interface::queries::Queries::expansion::h630bc3eb9a43aab4
  25:     0x7fffa1e21ba0 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h697cc8610eb6932c
  26:     0x7fffa1e005d5 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hf50081b5c60853fc
  27:     0x7fffa1e23229 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h697cc8610eb6932c
  28:     0x7fffa1e02465 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hf50081b5c60853fc
  29:     0x7fffa1e25a15 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h697cc8610eb6932c
  30:     0x7fffa1e4d28d - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h697cc8610eb6932c
  31:     0x7fffb8c9cb8a - std::sys::windows::thread::Thread::new::hbe3a2f8dadfc4075
  32:     0x7ff82e387034 - BaseThreadInitThunk
  33:     0x7ff82f25d241 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-beta.4 (4d25f4607 2021-03-05) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `windows_gen`
