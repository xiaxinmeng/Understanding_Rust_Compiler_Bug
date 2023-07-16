rust
   Compiling playground v0.0.1 (/playground)
error: missing type for `const` item
 --> src/main.rs:1:7
  |
1 | const e = & & 39;
  |       ^ help: provide a type for the item: `e: &&i32`

error: internal compiler error: src/librustc_infer/infer/lexical_region_resolve/mod.rs:497: cannot relate region: LUB(ReErased, ReErased)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
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
             at src/libstd/io/mod.rs:1439
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
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::bug_fmt
  20: rustc_infer::infer::lexical_region_resolve::LexicalResolver::lub_concrete_regions
  21: rustc_infer::infer::lexical_region_resolve::LexicalResolver::sub_concrete_regions
  22: rustc_infer::infer::lexical_region_resolve::LexicalResolver::infer_variable_values
  23: rustc_infer::infer::lexical_region_resolve::resolve
  24: rustc_infer::infer::InferCtxt::resolve_regions_and_report_errors
  25: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt>::regionck_item
  26: rustc_middle::ty::context::GlobalCtxt::enter_local
  27: rustc_typeck::check::wfcheck::check_item_type
  28: rustc_typeck::check::wfcheck::check_item_well_formed
  29: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_item_well_formed>::compute
  30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  31: rustc_query_system::query::plumbing::get_query
  32: rustc_query_system::query::plumbing::ensure_query
  33: rustc_data_structures::sync::par_for_each_in
  34: rustc_hir::hir::Crate::par_visit_all_item_likes
  35: rustc_session::session::Session::track_errors
  36: rustc_typeck::check_crate
  37: rustc_interface::passes::analysis
  38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  40: rustc_query_system::query::plumbing::get_query
  41: rustc_middle::ty::context::tls::enter_global
  42: rustc_interface::interface::run_compiler_in_existing_thread_pool
  43: scoped_tls::ScopedKey<T>::set
  44: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (76b11980a 2020-04-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_item_well_formed] processing `e`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

error: could not compile `playground`.

To learn more, run the command again with --verbose.
