bash
       Fresh unicode-xid v0.1.0
       Fresh semver-parser v0.7.0
       Fresh autocfg v0.1.4
       Fresh version_check v0.1.5
       Fresh rand_core v0.4.0
       Fresh lazy_static v1.3.0
       Fresh stable_deref_trait v1.1.1
       Fresh cfg-if v0.1.9
       Fresh nodrop v0.1.13
       Fresh futures-core-preview v0.3.0-alpha.16
       Fresh smallvec v0.6.9
       Fresh scopeguard v0.3.3
       Fresh siphasher v0.2.3
       Fresh either v1.5.2
       Fresh memoffset v0.2.1
       Fresh futures v0.1.27
       Fresh slab v0.4.2
       Fresh fnv v1.0.6
       Fresh pin-utils v0.1.0-alpha.4
       Fresh itoa v0.4.4
       Fresh matches v0.1.8
       Fresh try-lock v0.2.2
       Fresh safemem v0.2.0
       Fresh remove_dir_all v0.5.1
       Fresh percent-encoding v1.0.1
       Fresh dtoa v0.4.4
       Fresh safemem v0.3.0
       Fresh quick-error v1.2.2
       Fresh route-recognizer v0.1.12
       Fresh semver v0.9.0
       Fresh rand_core v0.3.1
       Fresh rand_jitter v0.1.4
       Fresh futures-channel-preview v0.3.0-alpha.16
       Fresh crossbeam-utils v0.6.5
       Fresh owning_ref v0.4.0
       Fresh futures-io-preview v0.3.0-alpha.16
       Fresh tokio-trace-core v0.2.0
       Fresh tokio-sync v0.1.6
       Fresh unicode-normalization v0.1.8
       Fresh unicode-bidi v0.3.4
       Fresh proc-macro2 v0.4.30
       Fresh libc v0.2.58
       Fresh rustc_version v0.2.3
       Fresh rand_xorshift v0.1.1
       Fresh rand_isaac v0.1.1
       Fresh rand_hc v0.1.0
       Fresh byteorder v1.3.1
       Fresh arrayvec v0.4.10
       Fresh memchr v2.2.0
       Fresh lock_api v0.1.5
       Fresh futures-sink-preview v0.3.0-alpha.16
       Fresh tokio-executor v0.1.7
       Fresh crossbeam-queue v0.1.2
       Fresh quote v0.6.12
       Fresh rand_pcg v0.1.2
       Fresh rand_os v0.1.3
       Fresh iovec v0.1.2
       Fresh net2 v0.2.33
       Fresh num_cpus v1.10.0
       Fresh time v0.1.42
       Fresh httparse v1.3.3
       Fresh idna v0.1.5
       Fresh ryu v0.2.8
       Fresh rand_chacha v0.1.1
       Fresh unicase v1.4.2
       Fresh syn v0.15.34
       Fresh crossbeam-epoch v0.7.1
       Fresh futures-util-preview v0.3.0-alpha.16
       Fresh tokio-current-thread v0.1.6
       Fresh tokio-timer v0.2.11
       Fresh twoway v0.1.8
       Fresh buf_redux v0.8.1
       Fresh bytes v0.4.12
       Fresh futures-cpupool v0.1.8
       Fresh url v1.7.2
       Fresh serde_derive v1.0.92
       Fresh rand v0.6.5
       Fresh phf_shared v0.7.24
       Fresh crossbeam-deque v0.7.1
       Fresh futures-executor-preview v0.3.0-alpha.16
       Fresh serde v1.0.92
       Fresh parking_lot_core v0.4.0
       Fresh tokio-buf v0.1.1
       Fresh string v0.2.0
       Fresh http v0.1.17
       Fresh tempfile v3.0.8
       Fresh cookie v0.12.0
       Fresh log v0.4.6
       Fresh phf_generator v0.7.24
       Fresh parking_lot v0.7.1
       Fresh indexmap v1.0.2
       Fresh futures-preview v0.3.0-alpha.16
       Fresh http-body v0.1.0
       Fresh phf v0.7.24
       Fresh serde_json v1.0.39
       Fresh serde_urlencoded v0.5.5
       Fresh mio v0.6.19
       Fresh tokio-io v0.1.12
       Fresh tokio-threadpool v0.1.14
       Fresh phf_codegen v0.7.24
       Fresh log v0.3.9
       Fresh want v0.0.6
       Fresh http-service v0.2.0 (https://github.com/rustasync/http-service#07b6780f)
       Fresh tokio-reactor v0.1.9
       Fresh h2 v0.1.23
       Fresh mime v0.2.6
       Fresh tokio v0.1.21
       Fresh tokio-tcp v0.1.3
       Fresh hyper v0.12.29
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
             at src/libstd/panicking.rs:197
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
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

note: rustc 1.37.0-nightly (7cdaffd79 2019-06-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `tide-cookies`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name tide_cookies tide-cookies/src/lib.rs --color always --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=93de9ddf6be004d0 -C extra-filename=-93de9ddf6be004d0 --out-dir /home/dcjanus/code/tide/target/debug/deps -C incremental=/home/dcjanus/code/tide/target/debug/incremental -L dependency=/home/dcjanus/code/tide/target/debug/deps --extern cookie=/home/dcjanus/code/tide/target/debug/deps/libcookie-0dadb67651d8ceb1.rlib --extern futures=/home/dcjanus/code/tide/target/debug/deps/libfutures-5210a5437a2c4b1f.rlib --extern http=/home/dcjanus/code/tide/target/debug/deps/libhttp-9aa1c72a98b0692a.rlib --extern http_service=/home/dcjanus/code/tide/target/debug/deps/libhttp_service-8e32f05474f3dd93.rlib --extern tide_core=/home/dcjanus/code/tide/target/debug/deps/libtide_core-1796718b146e011a.rlib` (exit code: 101)
