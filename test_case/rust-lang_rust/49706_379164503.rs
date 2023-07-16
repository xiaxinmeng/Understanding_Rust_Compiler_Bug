
   9: core::option::expect_failed
             at libcore/option.rs:916
  10: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::macro_def_scope
  11: rustc_resolve::Resolver::resolve_crate_root
  12: rustc_resolve::Resolver::resolve_path
  13: rustc_resolve::Resolver::smart_resolve_path_fragment
  14: rustc_resolve::Resolver::resolve_expr
  15: <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_block
  16: <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_fn
  17: syntax::visit::walk_item
  18: rustc_resolve::Resolver::resolve_item
  19: rustc_resolve::Resolver::resolve_crate
  20: rustc::util::common::time
  21: rustc_driver::driver::phase_2_configure_and_expand
  22: rustc_driver::driver::compile_input
  23: rustc_driver::run_compiler_impl
  24: syntax::with_globals
  25: rustc_driver::run
  26: rustc_driver::main
