
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:649:9
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
   1: std::sys_common::backtrace::print
   2: std::panicking::take_hook
   3: std::panicking::take_hook
   4: <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate> as rustc::ty::ToPredicate>::to_predicate
   5: std::panicking::rust_panic_with_hook
   6: std::io::buffered::BufWriter<W>::flush_buf
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::span_bug_fmt
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::TyCtxt::encode_metadata
  11: rustc::ty::context::tls::with_context::{{closure}}
  12: rustc::util::bug::span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry
  15: rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_type
  16: rustc_metadata::cstore_impl::provide_extern
  17: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder>::read_str
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  20: rustc::ty::instance::Instance::resolve
  21: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const
  22: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  23: rustc::ty::fold::TypeFoldable::fold_with
  24: rustc::traits::project::normalize_with_depth
  25: rustc::traits::select::SelectionContext::match_impl
  26: rustc::traits::object_safety::<impl rustc::ty::context::TyCtxt>::contains_illegal_self_type_reference
  27: rustc::ty::trait_def::<impl rustc::ty::context::TyCtxt>::for_each_relevant_impl
  28: rustc::traits::select::SelectionContext::match_projection
  29: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  30: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  31: rustc::dep_graph::graph::DepGraph::with_anon_task
  32: rustc::traits::select::SelectionContext::coinductive_predicate
  33: rustc::traits::select::SelectionContext::evaluate_stack
  34: rustc::dep_graph::graph::DepGraph::with_anon_task
  35: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  36: rustc::traits::select::SelectionContext::evaluate_predicates_recursively
  37: rustc::infer::InferCtxt::probe
  38: <alloc::vec::IntoIter<T> as core::ops::drop::Drop>::drop
  39: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  40: rustc::dep_graph::graph::DepGraph::with_anon_task
  41: rustc::traits::select::SelectionContext::coinductive_predicate
  42: rustc::traits::select::SelectionContext::evaluate_stack
  43: rustc::dep_graph::graph::DepGraph::with_anon_task
  44: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  45: rustc::traits::select::SelectionContext::evaluate_predicates_recursively
  46: rustc::infer::InferCtxt::probe
  47: <alloc::vec::IntoIter<T> as core::ops::drop::Drop>::drop
  48: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  49: rustc::dep_graph::graph::DepGraph::with_anon_task
  50: rustc::traits::select::SelectionContext::coinductive_predicate
  51: rustc::traits::select::SelectionContext::evaluate_stack
  52: rustc::dep_graph::graph::DepGraph::with_anon_task
  53: rustc::traits::select::SelectionContext::evaluate_predicate_recursively
  54: rustc::infer::InferCtxt::probe
  55: rustc::traits::select::SelectionContext::evaluate_root_obligation
  56: rustc_traits::evaluate_obligation::evaluate_obligation
  57: rustc::ty::query::__query_compute::evaluate_obligation
  58: rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder>::read_str
  59: rustc::dep_graph::graph::DepGraph::with_task_impl
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  61: rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::evaluate_obligation
  62: rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::evaluate_obligation_no_overflow
  63: rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::predicate_may_hold
  64: rustc_typeck::check::method::<impl rustc_typeck::check::FnCtxt>::lookup_method_in_trait
  65: rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_user_unop
  66: rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop
  67: rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  69: rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop
  70: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  71: rustc_typeck::check::FnCtxt::check_block_no_value
  72: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  73: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  74: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  75: rustc_typeck::check::FnCtxt::check_block_no_value
  76: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  77: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  78: rustc_typeck::check::check_fn
  79: rustc::infer::InferCtxtBuilder::enter
  80: rustc_typeck::check::typeck_tables_of
  81: rustc::ty::fold::<impl rustc::ty::context::TyCtxt>::anonymize_late_bound_regions
  82: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  83: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  84: rustc_typeck::check::typeck_item_bodies
  85: rustc::ty::fold::<impl rustc::ty::context::TyCtxt>::anonymize_late_bound_regions
  86: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  87: rustc_typeck::check_crate
  88: rustc_interface::passes::analysis
  89: alloc::slice::merge_sort
  90: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  91: env_logger::init_from_env
  92: rustc_interface::passes::lower_to_hir::{{closure}}
  93: rustc_interface::interface::run_compiler_in_existing_thread_pool
  94: std::thread::local::LocalKey<T>::with
  95: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:649:9
stack backtrace:
   0:      0x7fed652c909 - backtrace::backtrace::trace_unsynchronized::hc2b4972ce642e7fe
   1:      0x7fed65247a1 - std::sys_common::backtrace::print::hba526b61977e2223
   2:      0x7fed6535d1c - std::panicking::take_hook::h6d0ee51fc94ebbd4
   3:      0x7fed65359bd - std::panicking::take_hook::h6d0ee51fc94ebbd4
   4:      0x7fed3e638de - <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate> as rustc::ty::ToPredicate>::to_predicate::h5b2689191e79fa73
   5:      0x7fed6536572 - std::panicking::rust_panic_with_hook::h862a758d47ebc191
   6:      0x7feeb36d514 - std::io::buffered::BufWriter<W>::flush_buf::hd1be867c39240e62
   7:      0x7feeb37e03d - rustc_errors::Handler::bug::hdb28144644066777
   8:      0x7fed4424801 - rustc::util::bug::span_bug_fmt::h8aedd1203896ec64
   9:      0x7fed44117e6 - rustc::ty::context::tls::with_opt::{{closure}}::h6d26eb9168b6b9b5
  10:      0x7fed440667e - rustc::ty::context::TyCtxt::encode_metadata::h1ede840ac64af31d
  11:      0x7fed4410ffa - rustc::ty::context::tls::with_context::{{closure}}::h0ace010b40183159
  12:      0x7fed442470f - rustc::util::bug::span_bug_fmt::h8aedd1203896ec64
  13:      0x7fed442466d - rustc::util::bug::bug_fmt::h4a1b1655d756f408
  14:      0x7fed3a1ab4b - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::h4c420f9ddf3c045a
  15:      0x7fed3a1ad47 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::def_kind::hcf2ffc0f2ba0adf3
  16:      0x7fed3a2bb27 - rustc_metadata::cstore_impl::provide_extern::hd96e7e400536b80a
  17:      0x7fed443a0cc - rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder>::read_str::had5f64a164851791
  18:      0x7fed3f36092 - rustc::dep_graph::graph::DepGraph::with_task_impl::h286bce8e830096a2
  19:      0x7fed429ea73 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h9dcbed4f958e5e23
  20:      0x7fed4009fe7 - <rustc::ty::print::pretty::FmtPrinter<F> as rustc::ty::print::Printer>::print_const::h7be1190548b3c34a
  21:      0x7fed4011d7e - rustc::ty::print::Printer::default_print_def_path::h079b2044023798a0
  22:      0x7fed400363d - <rustc::ty::print::pretty::FmtPrinter<F> as rustc::ty::print::Printer>::print_def_path::h2909686884571440
  23:      0x7fed400bb89 - <rustc::ty::print::pretty::FmtPrinter<F> as rustc::ty::print::Printer>::print_const::h7be1190548b3c34a
  24:      0x7fed401141a - rustc::ty::print::Printer::default_print_def_path::h079b2044023798a0
  25:      0x7fed400363d - <rustc::ty::print::pretty::FmtPrinter<F> as rustc::ty::print::Printer>::print_def_path::h2909686884571440
  26:      0x7fed4011085 - rustc::ty::print::Printer::default_print_def_path::h079b2044023798a0
  27:      0x7fed400363d - <rustc::ty::print::pretty::FmtPrinter<F> as rustc::ty::print::Printer>::print_def_path::h2909686884571440
  28:      0x7fed41cbcc8 - rustc::ty::print::pretty::<impl rustc::ty::context::TyCtxt>::def_path_str::h67b039cf94e16052
  29:      0x7fed3e9e261 - rustc::ty::query::Query::describe::h59069403c7ef4446
  30:      0x7fed41d0e5a - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::he8d291ed46625428
  31:      0x7fed3e6394e - <rustc::ty::sty::Binder<rustc::ty::ProjectionPredicate> as rustc::ty::ToPredicate>::to_predicate::h5b2689191e79fa73
  32:      0x7fed6536572 - std::panicking::rust_panic_with_hook::h862a758d47ebc191
  33:      0x7feeb36d514 - std::io::buffered::BufWriter<W>::flush_buf::hd1be867c39240e62
  34:      0x7feeb37e03d - rustc_errors::Handler::bug::hdb28144644066777
  35:      0x7fed4424801 - rustc::util::bug::span_bug_fmt::h8aedd1203896ec64
  36:      0x7fed44117e6 - rustc::ty::context::tls::with_opt::{{closure}}::h6d26eb9168b6b9b5
  37:      0x7fed440667e - rustc::ty::context::TyCtxt::encode_metadata::h1ede840ac64af31d
  38:      0x7fed4410ffa - rustc::ty::context::tls::with_context::{{closure}}::h0ace010b40183159
  39:      0x7fed442470f - rustc::util::bug::span_bug_fmt::h8aedd1203896ec64
  40:      0x7fed442466d - rustc::util::bug::bug_fmt::h4a1b1655d756f408
  41:      0x7fed3a1ab4b - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::h4c420f9ddf3c045a
  42:      0x7fed3a1c22d - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::get_type::hd28542e6c6f75b76
  43:      0x7fed3a26f8d - rustc_metadata::cstore_impl::provide_extern::hd96e7e400536b80a
  44:      0x7fed443954c - rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder>::read_str::had5f64a164851791
  45:      0x7fed3f5f7e6 - rustc::dep_graph::graph::DepGraph::with_task_impl::hd4a18fb0e0ba478e
  46:      0x7fed42b8e63 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hbcd1616f31b94c00
  47:      0x7fed4023cf9 - rustc::ty::instance::Instance::resolve::h7721cb5b61a618e6
  48:      0x7fed3d1dc21 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const::h70cc1cd1f66dc9ea
  49:      0x7fed3ebfd82 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h76bea04cfbe999cb
  50:      0x7fed41bcd29 - rustc::ty::fold::TypeFoldable::fold_with::h533ed27275e8cf45
  51:      0x7fed3d1d546 - rustc::traits::project::normalize_with_depth::had388a52f84bf0dd
  52:      0x7fed3e3d6e4 - rustc::traits::select::SelectionContext::match_impl::hae01edf6571567a5
  53:      0x7fed41b20c5 - rustc::traits::object_safety::<impl rustc::ty::context::TyCtxt>::contains_illegal_self_type_reference::h52361ba093cc8db4
  54:      0x7fed43f4e7e - rustc::ty::trait_def::<impl rustc::ty::context::TyCtxt>::for_each_relevant_impl::hd737eed25188af89
  55:      0x7fed3e34d88 - rustc::traits::select::SelectionContext::match_projection::h64f2f9e89dfb329b
  56:      0x7fed3e32998 - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h69335986cbdf213c
  57:      0x7fed3e304bc - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h69335986cbdf213c
  58:      0x7fed3f6c48f - rustc::dep_graph::graph::DepGraph::with_anon_task::h145f853e5ea70328
  59:      0x7fed3e2f7a6 - rustc::traits::select::SelectionContext::coinductive_predicate::hb059aacf0b8b2a18
  60:      0x7fed3e2ea28 - rustc::traits::select::SelectionContext::evaluate_stack::h071ccf0858841b50
  61:      0x7fed3f6cf84 - rustc::dep_graph::graph::DepGraph::with_anon_task::ha56e59d9eac32052
  62:      0x7fed3e2d4fc - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::h728009a1728cd097
  63:      0x7fed3e2bda1 - rustc::traits::select::SelectionContext::evaluate_predicates_recursively::h76392a7559cd95fd
  64:      0x7fed416ce0b - rustc::infer::InferCtxt::probe::h0ab8d17b87062b58
  65:      0x7fed3d9f88b - <alloc::vec::IntoIter<T> as core::ops::drop::Drop>::drop::hc2129ff4ef5ee1ef
  66:      0x7fed3e30665 - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h69335986cbdf213c
  67:      0x7fed3f6c48f - rustc::dep_graph::graph::DepGraph::with_anon_task::h145f853e5ea70328
  68:      0x7fed3e2f7a6 - rustc::traits::select::SelectionContext::coinductive_predicate::hb059aacf0b8b2a18
  69:      0x7fed3e2ea28 - rustc::traits::select::SelectionContext::evaluate_stack::h071ccf0858841b50
  70:      0x7fed3f6cf84 - rustc::dep_graph::graph::DepGraph::with_anon_task::ha56e59d9eac32052
  71:      0x7fed3e2d4fc - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::h728009a1728cd097
  72:      0x7fed3e2bda1 - rustc::traits::select::SelectionContext::evaluate_predicates_recursively::h76392a7559cd95fd
  73:      0x7fed416ce0b - rustc::infer::InferCtxt::probe::h0ab8d17b87062b58
  74:      0x7fed3d9f88b - <alloc::vec::IntoIter<T> as core::ops::drop::Drop>::drop::hc2129ff4ef5ee1ef
  75:      0x7fed3e30665 - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h69335986cbdf213c
  76:      0x7fed3f6c48f - rustc::dep_graph::graph::DepGraph::with_anon_task::h145f853e5ea70328
  77:      0x7fed3e2f7a6 - rustc::traits::select::SelectionContext::coinductive_predicate::hb059aacf0b8b2a18
  78:      0x7fed3e2ea28 - rustc::traits::select::SelectionContext::evaluate_stack::h071ccf0858841b50
  79:      0x7fed3f6cf84 - rustc::dep_graph::graph::DepGraph::with_anon_task::ha56e59d9eac32052
  80:      0x7fed3e2d4fc - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::h728009a1728cd097
  81:      0x7fed416db79 - rustc::infer::InferCtxt::probe::he64460955cd4f3b9
  82:      0x7fed3e2bb30 - rustc::traits::select::SelectionContext::evaluate_root_obligation::h84336ed4461425ef
  83:      0x7fed8d4624c - rustc_traits::evaluate_obligation::evaluate_obligation::h1ad57efb84317ead
  84:      0x7fed40438b3 - rustc::ty::query::__query_compute::evaluate_obligation::h120b1a8d1e1547ea
  85:      0x7fed443ad3f - rustc::ty::query::on_disk_cache::__ty_decoder_impl::<impl serialize::serialize::Decoder for rustc::ty::query::on_disk_cache::CacheDecoder>::read_str::had5f64a164851791
  86:      0x7fed3f4b999 - rustc::dep_graph::graph::DepGraph::with_task_impl::h7b794a67a15077b6
  87:      0x7fed42865ef - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h8a267b4a8b6a58a1
  88:      0x7fed41b5d8a - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::evaluate_obligation::h7d2692681e00d9c4
  89:      0x7fed41b5e8a - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::evaluate_obligation_no_overflow::h9c841e8cf8850972
  90:      0x7fed41b5b49 - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt>::predicate_may_hold::he4414490bffaed69
  91:      0x7fed214c9f1 - rustc_typeck::check::method::<impl rustc_typeck::check::FnCtxt>::lookup_method_in_trait::hf26fd0a8beccf618
  92:      0x7fed216d3ad - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_user_unop::h29f9ce648e6b3c31
  93:      0x7fed2169883 - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop::h0f20ce481e2b06f0
  94:      0x7fed216950e - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop::h0f20ce481e2b06f0
  95:      0x7fed2134825 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hdb66f052b26f0185
  96:      0x7fed216946b - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop::h0f20ce481e2b06f0
  97:      0x7fed2134825 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hdb66f052b26f0185
  98:      0x7fed218c114 - rustc_typeck::check::FnCtxt::check_block_no_value::h10a5c2115c8b8ce8
  99:      0x7fed21347bc - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::hdb66f052b26f0185
  100:      0x7fed2119207 - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match::h051fab31983aba00
