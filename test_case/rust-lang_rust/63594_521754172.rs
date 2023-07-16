
DEBUG 2019-08-15T18:50:19Z: rustc_typeck::astconv: impl_trait_ty_to_ty: substs=[]
DEBUG 2019-08-15T18:50:19Z: rustc_typeck::collect: predicates_defined_on(DefId(0:25 ~ file7[317d]::{{impl}}[1]::Out[0]::{{opaque}}[0]))
DEBUG 2019-08-15T18:50:19Z: rustc_typeck::collect: explicit_predicates_of(def_id=DefId(0:25 ~ file7[317d]::{{impl}}[1]::Out[0]::{{opaque}}[0]))
DEBUG 2019-08-15T18:50:19Z: rustc_typeck::astconv: instantiate_poly_trait_ref(TraitRef { path: path(Copy), hir_ref_id: HirId { owner: DefIndex(25), local_id: 2 } }, def_id=DefId(2:1542 ~ core[d402]::marker[0]::Copy[0]))
DEBUG 2019-08-15T18:50:19Z: rustc_typeck::astconv: create_substs_for_ast_trait_ref(trait_segment=PathSegment { ident: Copy#0, hir_id: Some(HirId { owner: DefIndex(25), local_id: 1 }), res: Some(Def(Trait, DefId(2:1542 ~ core[d402]::marker[0]::Copy[0]))), args: None, infer_args: false })
error[E0391]: cycle detected when processing `<impl at file7.rs:15:1: 23:2>::Out::{{opaque}}#0`
  |
  = note: ...which requires processing `<impl at file7.rs:15:1: 23:2>::Out::{{opaque}}#0`...
  = note: ...which requires processing `<impl at file7.rs:15:1: 23:2>::Out::{{opaque}}#0`...
  = note: ...which again requires processing `<impl at file7.rs:15:1: 23:2>::Out::{{opaque}}#0`, completing the cycle
note: cycle used when processing `<impl at file7.rs:15:1: 23:2>::Out`
 --> file7.rs:16:5
  |
16|     type Out = Box<dyn Bar<Assoc: Copy>>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:541:13
stack backtrace:
   0: std::sys_common::backtrace::print
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: rustc::util::common::panic_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::emit_db
   7: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   8: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::predicates_of>::handle_cycle_error
   9: rustc_data_structures::cold_path
  10: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  11: rustc::ty::print::pretty::PrettyPrinter::pretty_print_type
  12: rustc::ty::print::pretty::<impl core::fmt::Display for &rustc::ty::TyS>::fmt
  13: core::fmt::builders::DebugTuple::field
  14: <core::option::Option<T> as core::fmt::Debug>::fmt
  15: core::fmt::write
  16: <core::fmt::Arguments as core::fmt::Display>::fmt
  17: core::fmt::write
  18: std::io::Write::write_fmt
  19: env_logger::Format::into_boxed_fn::{{closure}}
  20: std::thread::local::LocalKey<T>::with
  21: <env_logger::Logger as log::Log>::log
  22: log::__private_api_log
  23: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path
  24: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref
  25: <dyn rustc_typeck::astconv::AstConv>::instantiate_poly_trait_ref_inner
  26: <dyn rustc_typeck::astconv::AstConv>::add_bounds
  27: <dyn rustc_typeck::astconv::AstConv>::compute_bounds
  28: rustc_typeck::collect::explicit_predicates_of
  29: rustc::ty::query::__query_compute::explicit_predicates_of
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::explicit_predicates_of>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_typeck::collect::predicates_defined_on
  34: rustc::ty::query::__query_compute::predicates_defined_on
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::predicates_defined_on>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc_typeck::collect::predicates_of
  39: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::predicates_of>::compute
  40: rustc::dep_graph::graph::DepGraph::with_task_impl
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  42: rustc::ty::print::pretty::PrettyPrinter::pretty_print_type
  43: rustc::ty::print::pretty::<impl core::fmt::Display for &rustc::ty::TyS>::fmt
  44: core::fmt::write
  45: <core::fmt::Arguments as core::fmt::Display>::fmt
  46: core::fmt::write
  47: std::io::Write::write_fmt
  48: env_logger::Format::into_boxed_fn::{{closure}}
  49: std::thread::local::LocalKey<T>::with
  50: <env_logger::Logger as log::Log>::log
  51: log::__private_api_log
  52: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  53: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  54: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path
  55: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref
  56: <dyn rustc_typeck::astconv::AstConv>::instantiate_poly_trait_ref_inner
  57: <dyn rustc_typeck::astconv::AstConv>::conv_object_ty_poly_trait_ref
  58: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  59: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args
  60: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path
  61: <dyn rustc_typeck::astconv::AstConv>::ast_path_substs_for_ty
  62: <dyn rustc_typeck::astconv::AstConv>::ast_path_to_ty
  63: <dyn rustc_typeck::astconv::AstConv>::res_to_ty
  64: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  65: rustc_typeck::collect::checked_type_of
  66: rustc_typeck::collect::type_of
  67: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  68: rustc::dep_graph::graph::DepGraph::with_task_impl
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  70: rustc::traits::project::opt_normalize_projection_type
  71: rustc::traits::project::normalize_projection_type
  72: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  73: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  74: rustc::ty::fold::TypeFoldable::fold_with
  75: rustc::traits::project::normalize
  76: rustc::infer::InferCtxt::partially_normalize_associated_types_in
  77: rustc::ty::context::tls::with_context::{{closure}}
  78: rustc::ty::context::GlobalCtxt::enter_local
  79: rustc_typeck::check::typeck_tables_of
  80: rustc::ty::query::__query_compute::typeck_tables_of
  81: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  82: rustc::dep_graph::graph::DepGraph::with_task_impl
  83: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  84: rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator::check
  85: <rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator as rustc::hir::intravisit::Visitor>::visit_impl_item
  86: rustc::hir::intravisit::walk_item
  87: <rustc_typeck::collect::find_opaque_ty_constraints::ConstraintLocator as rustc::hir::intravisit::Visitor>::visit_item
  88: rustc_typeck::collect::find_opaque_ty_constraints
  89: rustc_typeck::collect::checked_type_of
  90: rustc_typeck::collect::type_of
  91: rustc::ty::query::__query_compute::type_of
  92: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  93: rustc::dep_graph::graph::DepGraph::with_task_impl
  94: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  95: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  96: rustc::hir::map::Map::visit_item_likes_in_module
  97: rustc_typeck::collect::collect_mod_item_types
  98: rustc::ty::query::__query_compute::collect_mod_item_types
  99: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  100: rustc::dep_graph::graph::DepGraph::with_task_impl
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [explicit_predicates_of] processing `<AssocNoCopy as Thing>::Out::{{opaque}}#0`
#1 [predicates_defined_on] processing `<AssocNoCopy as Thing>::Out::{{opaque}}#0`
#2 [predicates_of] processing `<AssocNoCopy as Thing>::Out::{{opaque}}#0`
#3 [type_of] processing `<AssocNoCopy as Thing>::Out`
#4 [typeck_tables_of] processing `<AssocNoCopy as Thing>::func`
#5 [type_of] processing `<AssocNoCopy as Thing>::Out::{{opaque}}#0`
#6 [collect_mod_item_types] collecting item types in top-level module
#7 [analysis] running analysis passes on this crate
end of query stack
