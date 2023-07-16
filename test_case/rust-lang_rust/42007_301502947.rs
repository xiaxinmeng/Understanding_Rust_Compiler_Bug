
   7: std::panicking::begin_panic_fmt
   8: rustc::lint::context::<impl serialize::serialize::Decodable for rustc::lint::LintId>::decode::{{closure}}
   9: rustc::ty::context::tls::with::{{closure}}
  10: <std::thread::local::LocalKey<T>>::with
  11: rustc::ty::context::tls::with
  12: rustc::lint::context::<impl serialize::serialize::Decodable for rustc::lint::LintId>::decode
  13: core::ops::FnOnce::call_once
  14: serialize::serialize::Decoder::read_struct_field
  15: <rustc::lint::context::EarlyLint as serialize::serialize::Decodable>::decode::{{closure}}
  16: serialize::serialize::Decoder::read_struct
  17: <rustc::lint::context::EarlyLint as serialize::serialize::Decodable>::decode
  18: <collections::vec::Vec<T> as serialize::serialize::Decodable>::decode::{{closure}}::{{closure}}
  19: serialize::serialize::Decoder::read_seq_elt
  20: <collections::vec::Vec<T> as serialize::serialize::Decodable>::decode::{{closure}}
  21: serialize::serialize::Decoder::read_seq
  22: <collections::vec::Vec<T> as serialize::serialize::Decodable>::decode
  23: serialize::collection_impls::<impl serialize::serialize::Decodable for std::collections::hash::map::HashMap<K, V, S>>::decode::{{closure}}::{{closure}}
  24: serialize::serialize::Decoder::read_map_elt_val
  25: serialize::collection_impls::<impl serialize::serialize::Decodable for std::collections::hash::map::HashMap<K, V, S>>::decode::{{closure}}
  26: serialize::serialize::Decoder::read_map
  27: serialize::collection_impls::<impl serialize::serialize::Decodable for std::collections::hash::map::HashMap<K, V, S>>::decode
  28: core::ops::FnOnce::call_once
  29: serialize::serialize::Decoder::read_struct_field
  30: <rustc::lint::table::LintTable as serialize::serialize::Decodable>::decode::{{closure}}
  31: serialize::serialize::Decoder::read_struct
  32: <rustc::lint::table::LintTable as serialize::serialize::Decodable>::decode
  33: core::ops::FnOnce::call_once
  34: serialize::serialize::Decoder::read_struct_field
  35: <rustc::ty::context::TypeckTables<'tcx> as serialize::serialize::Decodable>::decode::{{closure}}
  36: serialize::serialize::Decoder::read_struct
  37: <rustc::ty::context::TypeckTables<'tcx> as serialize::serialize::Decodable>::decode
  38: rustc_metadata::decoder::<impl rustc_metadata::schema::Lazy<T>>::decode
  39: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::item_body_tables
  40: rustc_metadata::cstore_impl::provide::typeck_tables_of
  41: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get_with::{{closure}}
  42: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  43: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get_with
  44: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  45: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  46: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  47: rustc_const_eval::eval::const_eval
  48: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::try_get_with::{{closure}}
  49: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  50: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::try_get_with
  51: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::try_get
  52: rustc::ty::maps::TyCtxtAt::const_eval
  53: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
  54: rustc::ty::AdtDef::discriminants::{{closure}}
  55: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
  56: <core::option::Option<T>>::map
  57: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::next
  58: rustc::ty::layout::Layout::compute_uncached
  59: rustc::ty::util::<impl rustc::ty::TyS<'tcx>>::layout
  60: rustc::ty::layout::Layout::compute_uncached::{{closure}}
  61: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
  62: <core::option::Option<T>>::map
  63: <core::iter::Map<I, F> as core::iter::iterator::Iterator>::next
  64: <<core::result::Result<V, E> as core::iter::traits::FromIterator<core::result::Result<A, E>>>::from_iter::Adapter<Iter, E> as core::iter::iterator::Iterator>::next
  65: <&'a mut I as core::iter::iterator::Iterator>::next
  66: <collections::vec::Vec<T> as collections::vec::SpecExtend<T, I>>::from_iter
  67: <collections::vec::Vec<T> as core::iter::traits::FromIterator<T>>::from_iter
  68: <core::result::Result<V, E> as core::iter::traits::FromIterator<core::result::Result<A, E>>>::from_iter
  69: core::iter::iterator::Iterator::collect
  70: rustc::ty::layout::Layout::compute_uncached
  71: rustc::ty::util::<impl rustc::ty::TyS<'tcx>>::layout
  72: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LateLintPass<'a, 'tcx>>::check_item::{{closure}}
  73: rustc::infer::InferCtxtBuilder::enter::{{closure}}
  74: rustc::ty::context::tls::enter::{{closure}}
  75: <std::thread::local::LocalKey<T>>::with
  76: rustc::ty::context::tls::enter
  77: rustc::ty::context::GlobalCtxt::enter_local
  78: rustc::infer::InferCtxtBuilder::enter
  79: <rustc_lint::types::VariantSizeDifferences as rustc::lint::LateLintPass<'a, 'tcx>>::check_item
  80: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::{{closure}}
  81: rustc::lint::context::LintContext::with_lint_attrs
  82: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  83: rustc::hir::intravisit::Visitor::visit_nested_item
  84: rustc::hir::intravisit::walk_mod
  85: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  86: rustc::hir::intravisit::walk_crate
  87: rustc::lint::context::check_crate::{{closure}}
  88: rustc::lint::context::LintContext::with_lint_attrs
  89: rustc::lint::context::check_crate
  90: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
