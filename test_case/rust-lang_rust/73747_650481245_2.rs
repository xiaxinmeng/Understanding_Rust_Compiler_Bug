rust
error: internal compiler error: src/librustc_middle/ich/impls_ty.rs:167:9: ty::TyKind::hash_stable() - can't hash a TyVid _#649t.

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:916:9
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
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:217
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:524
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::bug_fmt
  20: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyVid>::hash_stable
  21: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_InferTy::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::InferTy>::hash_stable
  22: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
  23: <T as rustc_query_system::dep_graph::dep_node::DepNodeParams<Ctxt>>::to_fingerprint
  24: rustc_query_system::query::plumbing::get_query_impl
  25: rustc_typeck::check::cast::CastCheck::check
  26: rustc_typeck::check::FnCtxt::check_casts
  27: rustc_middle::ty::context::GlobalCtxt::enter_local
  28: rustc_typeck::check::typeck_tables_of
  29: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_tables_of>::compute
  30: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  31: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  32: rustc_data_structures::stack::ensure_sufficient_stack
  33: rustc_query_system::query::plumbing::get_query_impl
  34: rustc_query_system::query::plumbing::ensure_query_impl
  35: rustc_typeck::check::typeck_item_bodies
  36: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
  37: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  38: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  39: rustc_data_structures::stack::ensure_sufficient_stack
  40: rustc_query_system::query::plumbing::get_query_impl
  41: rustc_typeck::check_crate
  42: rustc_interface::passes::analysis
  43: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  44: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  45: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  46: rustc_data_structures::stack::ensure_sufficient_stack
  47: rustc_query_system::query::plumbing::get_query_impl
  48: rustc_middle::ty::context::tls::enter_global
  49: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  50: rustc_span::with_source_map
  51: rustc_interface::interface::run_compiler_in_existing_thread_pool
  52: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
