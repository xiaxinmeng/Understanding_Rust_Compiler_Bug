
   Compiling ice v0.0.0 (/home/ubuntu/dev/ice)
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> src/lib.rs:1:1
  |
1 | #![feature(min_specialization, rustc_attrs)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'region constraints already solved', src/librustc_infer/infer/mod.rs:209:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1504
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:515
  12: rust_begin_unwind
             at src/libstd/panicking.rs:419
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:111
  14: core::option::expect_failed
             at src/libcore/option.rs:1260
  15: rustc_infer::infer::InferCtxt::start_snapshot
  16: rustc_infer::infer::InferCtxt::commit_if_ok
  17: rustc_trait_selection::traits::project::opt_normalize_projection_type
  18: rustc_trait_selection::traits::project::normalize_projection_type
  19: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  20: rustc_middle::ty::fold::TypeFoldable::fold_with
  21: rustc_trait_selection::traits::project::normalize_to
  22: rustc_trait_selection::traits::wf::WfPredicates::normalize
  23: rustc_trait_selection::traits::wf::obligations
  24: rustc_typeck::impl_wf_check::min_specialization::check_always_applicable
  25: rustc_infer::infer::InferCtxtBuilder::enter
  26: rustc_typeck::impl_wf_check::min_specialization::check_min_specialization
  27: <rustc_typeck::impl_wf_check::ImplWfCheck as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
  28: rustc_middle::hir::map::Map::visit_item_likes_in_module
  29: rustc_typeck::impl_wf_check::check_mod_impl_wf
  30: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_impl_wf>::compute
  31: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  32: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  33: rustc_query_system::query::plumbing::get_query
  34: rustc_query_system::query::plumbing::ensure_query
  35: rustc_typeck::impl_wf_check::impl_wf_check
  36: rustc_typeck::check_crate
  37: rustc_interface::passes::analysis
  38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  39: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  40: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  41: rustc_query_system::query::plumbing::get_query
  42: rustc_middle::ty::context::tls::enter_global
  43: rustc_interface::interface::run_compiler_in_existing_thread_pool
  44: scoped_tls::ScopedKey<T>::set
  45: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.1 (c7087fe00 2020-06-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_mod_impl_wf] checking that impls are well-formed in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0554`.
error: could not compile `ice`.

To learn more, run the command again with --verbose.
