
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `DefId(0:8)`,
 right: `DefId(0:9)`', compiler/rustc_ast_lowering/src/index.rs:77:9
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:495:5
   1: core::panicking::panic_fmt
             at ./library/core/src/panicking.rs:107:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
             at ./library/core/src/panicking.rs:145:5
   4: rustc_ast_lowering::index::NodeCollector::insert
             at ./compiler/rustc_ast_lowering/src/index.rs:77:9
   5: <rustc_ast_lowering::index::NodeCollector as rustc_hir::intravisit::Visitor>::visit_ty
             at ./compiler/rustc_ast_lowering/src/index.rs:257:9
   6: rustc_hir::intravisit::walk_generic_args
             at ./compiler/rustc_hir/src/intravisit.rs:804:5
   7: rustc_hir::intravisit::walk_path
             at ./compiler/rustc_hir/src/intravisit.rs:783:9
   8: rustc_hir::intravisit::Visitor::visit_path
             at ./compiler/rustc_hir/src/intravisit.rs:478:9
   9: rustc_hir::intravisit::walk_use
             at ./compiler/rustc_hir/src/intravisit.rs:687:5
  10: rustc_hir::intravisit::Visitor::visit_use
             at ./compiler/rustc_hir/src/intravisit.rs:406:9
  11: rustc_hir::intravisit::walk_item
             at ./compiler/rustc_hir/src/intravisit.rs:576:13
  12: <rustc_ast_lowering::index::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at ./compiler/rustc_ast_lowering/src/index.rs:169:13
  13: rustc_ast_lowering::index::NodeCollector::with_parent
             at ./compiler/rustc_ast_lowering/src/index.rs:108:9
  14: <rustc_ast_lowering::index::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item
             at ./compiler/rustc_ast_lowering/src/index.rs:162:9
  15: rustc_ast_lowering::index::index_hir
             at ./compiler/rustc_ast_lowering/src/index.rs:66:34
  16: rustc_ast_lowering::LoweringContext::make_owner_info
             at ./compiler/rustc_ast_lowering/src/lib.rs:502:13
  17: rustc_ast_lowering::LoweringContext::with_hir_id_owner
             at ./compiler/rustc_ast_lowering/src/lib.rs:465:20
  18: <rustc_ast_lowering::item::ItemLowerer as rustc_ast::visit::Visitor>::visit_item
             at ./compiler/rustc_ast_lowering/src/item.rs:44:22
  19: rustc_ast::visit::walk_stmt
             at ./compiler/rustc_ast/src/visit.rs:692:37
  20: rustc_ast::visit::Visitor::visit_stmt
             at ./compiler/rustc_ast/src/visit.rs:103:9
  21: rustc_ast::visit::walk_block
             at ./compiler/rustc_ast/src/visit.rs:686:5
  22: rustc_ast::visit::Visitor::visit_block
             at ./compiler/rustc_ast/src/visit.rs:100:9
  23: rustc_ast::visit::walk_fn
             at ./compiler/rustc_ast/src/visit.rs:637:13
  24: <rustc_ast_lowering::item::ItemLowerer as rustc_ast::visit::Visitor>::visit_fn
             at ./compiler/rustc_ast_lowering/src/item.rs:68:18
  25: rustc_ast::visit::walk_item
             at ./compiler/rustc_ast/src/visit.rs:291:13
  26: <rustc_ast_lowering::item::ItemLowerer as rustc_ast::visit::Visitor>::visit_item::{{closure}}
             at ./compiler/rustc_ast_lowering/src/item.rs:55:22
  27: rustc_ast_lowering::item::<impl rustc_ast_lowering::LoweringContext>::with_parent_item_lifetime_defs
             at ./compiler/rustc_ast_lowering/src/item.rs:115:19
  28: <rustc_ast_lowering::item::ItemLowerer as rustc_ast::visit::Visitor>::visit_item
             at ./compiler/rustc_ast_lowering/src/item.rs:49:9
  29: rustc_ast::visit::walk_crate
             at ./compiler/rustc_ast/src/visit.rs:235:5
  30: rustc_ast_lowering::LoweringContext::lower_crate
             at ./compiler/rustc_ast_lowering/src/lib.rs:396:9
  31: rustc_ast_lowering::lower_crate
             at ./compiler/rustc_ast_lowering/src/lib.rs:297:5
  32: rustc_interface::passes::lower_to_hir
             at ./compiler/rustc_interface/src/passes.rs:464:21
  33: rustc_interface::passes::create_global_ctxt
             at ./compiler/rustc_interface/src/passes.rs:807:17
  34: rustc_interface::queries::Queries::global_ctxt::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:226:16
  35: rustc_interface::queries::Query<T>::compute
             at ./compiler/rustc_interface/src/queries.rs:38:28
  36: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:377:13
  37: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:390:19
  38: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:312:22
  39: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:203:13
  40: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:976:5
  41: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:197:5
  42: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:219:12
  43: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:145:13
  44: scoped_tls::ScopedKey<T>::set
             at .cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  45: rustc_span::create_session_globals_then
             at ./compiler/rustc_span/src/lib.rs:109:5
  46: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:143:9
