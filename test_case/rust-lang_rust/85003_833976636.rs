
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(ab12843d6b708f6f-ede3d4341e1f4d21): Ok(EvaluatedToOk)', /rustc/88f19c6dab716c6281af7602e30f413e809c5974/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0 (88f19c6da 2021-05-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `[closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]: std::ops::FnMut<(for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>,)>`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]>], item_def_id: DefId(2:7162 ~ core[b0ed]::iter::traits::iterator::Iterator::Item) } } }`
end of query stack
error: could not compile `nu-command`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
jonathan@spectre ~/S/nushell (syntax_simplification_expr) [101]> RUST_BACKTRACE=1 cargo test --all --features=extra
   Compiling nu-command v0.30.1 (/home/jonathan/Source/nushell/crates/nu-command)
   Compiling nu v0.30.1 (/home/jonathan/Source/nushell)
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(ab12843d6b708f6f-ede3d4341e1f4d21): Ok(EvaluatedToOk)', /rustc/88f19c6dab716c6281af7602e30f413e809c5974/compiler/rustc_query_system/src/query/plumbing.rs:593:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:435:5
   2: rustc_query_system::query::plumbing::incremental_verify_ich
   3: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
   4: rustc_data_structures::stack::ensure_sufficient_stack
   5: rustc_query_system::query::plumbing::get_query_impl
   6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
   7: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
   8: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
   9: rustc_trait_selection::traits::fulfill::FulfillProcessor::process_trait_obligation
  10: rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations
  11: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
  12: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  13: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query
  14: rustc_traits::normalize_projection_ty::normalize_projection_ty
  15: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::normalize_projection_ty>::compute
  16: rustc_query_system::query::config::QueryVtable<CTX,K,V>::compute
  17: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_ignore
  19: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  20: rustc_data_structures::stack::ensure_sufficient_stack
  21: rustc_query_system::query::plumbing::get_query_impl
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_projection_ty
  23: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  24: rustc_middle::ty::fold::TypeFoldable::fold_with
  25: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  26: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  27: rustc_middle::ty::fold::TypeFoldable::fold_with
  28: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
  29: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  30: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  31: rustc_infer::infer::InferCtxtBuilder::enter
  32: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
  33: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  34: rustc_query_system::dep_graph::graph::DepGraph<K>::with_ignore
  35: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  36: rustc_data_structures::stack::ensure_sufficient_stack
  37: rustc_query_system::query::plumbing::get_query_impl
  38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_generic_arg_after_erasing_regions
  39: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
  40: rustc_middle::ty::instance::Instance::subst_mir_and_normalize_erasing_regions
  41: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
  42: rustc_mir::monomorphize::collector::collect_neighbours
  43: rustc_mir::monomorphize::collector::collect_items_rec
  44: rustc_mir::monomorphize::collector::collect_items_rec
  45: rustc_mir::monomorphize::collector::collect_items_rec
  46: rustc_mir::monomorphize::collector::collect_items_rec
  47: rustc_mir::monomorphize::collector::collect_crate_mono_items
  48: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  49: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
  50: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  51: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  52: rustc_data_structures::stack::ensure_sufficient_stack
  53: rustc_query_system::query::plumbing::force_query_with_job
  54: rustc_query_system::query::plumbing::get_query_impl
  55: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  56: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  57: rustc_interface::passes::QueryContext::enter
  58: rustc_interface::queries::Queries::ongoing_codegen
  59: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  60: rustc_span::with_source_map
  61: rustc_interface::interface::create_compiler_and_run
  62: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0 (88f19c6da 2021-05-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `[closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]: std::ops::FnMut<(for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>,)>`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]>], item_def_id: DefId(2:7162 ~ core[b0ed]::iter::traits::iterator::Iterator::Item) } } }`
#2 [normalize_generic_arg_after_erasing_regions] normalizing `fn(std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]>) -> std::vec::Vec<<std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]> as std::iter::Iterator>::Item> {<std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]> as std::iter::Iterator>::collect::<std::vec::Vec<<std::iter::Map<std::vec::IntoIter<for<'r, 's> fn(&'r [nu_protocol::Value], &'s nu_source::Tag) -> std::result::Result<nu_protocol::Value, nu_errors::ShellError>>, [closure@crates/nu-command/src/commands/math/command.rs:179:22: 179:63]> as std::iter::Iterator>::Item>>}`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: could not compile `nu-command`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
