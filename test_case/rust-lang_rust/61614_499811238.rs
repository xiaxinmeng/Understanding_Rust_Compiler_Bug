
error: internal compiler error: src/librustc_mir/transform/generator.rs:532: Broken MIR: generator contains type std::option::Option<data::CookieData> in MIR, but typeck only knows about for<'r> {data::CookieData, std::sync::Arc<std::sync::RwLock<cookie::jar::CookieJar>>, std::pin::Pin<std::boxed::Box<(dyn core::future::future::Future<Output = http::response::Response<http_service::Body>> + std::marker::Send + 'r)>>, ()}
  --> tide-cookies/src/middleware.rs:39:37
   |
39 |           FutureExt::boxed(async move {
   |  _____________________________________^
40 | |             let cookie_data = cx
41 | |                 .extensions_mut()
42 | |                 .remove()
...  |
63 | |             res
64 | |         })
   | |_________^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:573:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::span_bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::span_bug_fmt
  16: <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass
  17: rustc_mir::transform::run_passes::{{closure}}
  18: rustc_mir::transform::run_passes
  19: rustc_mir::transform::optimized_mir
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  23: rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt>::layout_raw_uncached
  24: rustc::ty::layout::layout_raw
  25: rustc::ty::query::__query_compute::layout_raw
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::layout_raw>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: <rustc::ty::layout::LayoutCx<rustc::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
  30: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  31: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  32: rustc_mir::transform::run_passes::{{closure}}
  33: rustc_mir::transform::run_passes
  34: rustc_mir::transform::optimized_mir
  35: rustc::ty::query::__query_compute::optimized_mir
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc_metadata::encoder::EncodeContext::encode_optimized_mir
  40: <rustc_metadata::encoder::EncodeContext as rustc::hir::intravisit::Visitor>::visit_item
  41: rustc::hir::Crate::visit_all_item_likes
  42: rustc_metadata::encoder::EncodeContext::encode_crate_root
  43: rustc::dep_graph::graph::DepGraph::with_ignore
  44: rustc_metadata::encoder::encode_metadata
  45: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  46: rustc::ty::context::TyCtxt::encode_metadata
  47: rustc_interface::passes::encode_and_write_metadata
  48: rustc::util::common::time
  49: rustc_interface::passes::start_codegen
  50: rustc::ty::context::tls::enter_global
  51: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  52: rustc_interface::passes::create_global_ctxt::{{closure}}
  53: rustc_interface::passes::BoxedGlobalCtxt::enter
  54: rustc_interface::queries::Query<T>::compute
  55: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  56: rustc_interface::interface::run_compiler_in_existing_thread_pool
  57: std::thread::local::LocalKey<T>::with
  58: scoped_tls::ScopedKey<T>::set
  59: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [optimized_mir] processing `<middleware::CookiesMiddleware as tide_core::middleware::Middleware<State>>::handle::{{closure}}#0`
#1 [layout_raw] computing layout of `[static generator@tide-cookies/src/middleware.rs:39:37: 64:10 cx:tide_core::context::Context<State>, next:tide_core::middleware::Next<State> for<'r> {data::CookieData, std::sync::Arc<std::sync::RwLock<cookie::jar::CookieJar>>, std::pin::Pin<std::boxed::Box<(dyn core::future::future::Future<Output = http::response::Response<http_service::Body>> + std::marker::Send + 'r)>>, ()}]`
#2 [optimized_mir] processing `<middleware::CookiesMiddleware as tide_core::middleware::Middleware<State>>::handle`
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (5eeb567a2 2019-06-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `tide-cookies`.
