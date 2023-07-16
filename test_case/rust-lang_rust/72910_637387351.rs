
>     Checking smq v0.1.0 (/home/sherlock/git/smq)
> error: internal compiler error: src/librustc_middle/ich/impls_ty.rs:94: StableHasher: unexpected region '_#9r
> 
> thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
> stack backtrace:
> error: internal compiler error: src/librustc_middle/ich/impls_ty.rs:94: StableHasher: unexpected region '_#9r
> 
> thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
> stack backtrace:
>    0: backtrace::backtrace::libunwind::trace
>              at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
>    1: backtrace::backtrace::trace_unsynchronized
>              at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
>    2: std::sys_common::backtrace::_print_fmt
>              at src/libstd/sys_common/backtrace.rs:78
>    3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
>              at src/libstd/sys_common/backtrace.rs:59
>    0: backtrace::backtrace::libunwind::trace
>              at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
>    1: backtrace::backtrace::trace_unsynchronized
>              at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
>    2: std::sys_common::backtrace::_print_fmt
>              at src/libstd/sys_common/backtrace.rs:78
>    3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
>              at src/libstd/sys_common/backtrace.rs:59
>    4: core::fmt::write
>              at src/libcore/fmt/mod.rs:1076
>    5: std::io::Write::write_fmt
>              at src/libstd/io/mod.rs:1537
>    6: std::sys_common::backtrace::_print
>              at src/libstd/sys_common/backtrace.rs:62
>    7: std::sys_common::backtrace::print
>              at src/libstd/sys_common/backtrace.rs:49
>    8: std::panicking::default_hook::{{closure}}
>              at src/libstd/panicking.rs:198
>    9: std::panicking::default_hook
>              at src/libstd/panicking.rs:218
>   10: rustc_driver::report_ice
>   11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
>              at /SSD-speed/sherlock/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liballoc/boxed.rs:1090
>   12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
>              at /SSD-speed/sherlock/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libproc_macro/bridge/client.rs:318
>   13: std::panicking::rust_panic_with_hook
>              at src/libstd/panicking.rs:490
>   14: std::panicking::begin_panic
>   15: rustc_errors::HandlerInner::bug
>   16: rustc_errors::Handler::bug
>   17: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
>   18: rustc_middle::ty::context::tls::with_opt::{{closure}}
>   19: rustc_middle::ty::context::tls::with_opt
>   20: rustc_middle::util::bug::opt_span_bug_fmt
>   21: rustc_middle::util::bug::bug_fmt
>   22: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::RegionKind>::hash_stable
>   23: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
>   24: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for &rustc_middle::ty::list::List<T>>::hash_stable
>   25: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
>   26: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
>   27: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for &rustc_middle::ty::list::List<T>>::hash_stable
>   28: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
>   29: <T as rustc_query_system::dep_graph::dep_node::DepNodeParams<Ctxt>>::to_fingerprint
>   30: rustc_query_system::query::plumbing::get_query_impl
>   31: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::suggest_await_before_try
>   32: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
>   33: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
>   34: rustc_typeck::check::FnCtxt::check_argument_types
>   35: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
>   36: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
>   37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   39: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
>   40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   42: rustc_typeck::check::FnCtxt::check_stmt
>   43: rustc_typeck::check::FnCtxt::check_block_with_expected
>   44: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   45: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   46: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   47: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   48: rustc_typeck::check::FnCtxt::check_block_with_expected
>   49: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
>   52: rustc_typeck::check::check_fn
>   53: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
>   54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   55: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   56: rustc_typeck::check::FnCtxt::check_argument_types
>   57: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
>   58: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
>   59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   60: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   61: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
>   62: rustc_typeck::check::check_fn
>   63: rustc_middle::ty::context::GlobalCtxt::enter_local
>   64: rustc_typeck::check::typeck_tables_of
>   65: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_tables_of>::compute
>   66: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   67: rustc_data_structures::stack::ensure_sufficient_stack
>   68: rustc_query_system::query::plumbing::get_query_impl
>   69: rustc_mir_build::hair::cx::Cx::new
>   70: rustc_middle::ty::context::GlobalCtxt::enter_local
>   71: rustc_mir_build::build::mir_built
>   72: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
>   73: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>   74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   75: rustc_data_structures::stack::ensure_sufficient_stack
>   76: rustc_query_system::query::plumbing::force_query_impl
>   77: rustc_middle::ty::query::force_from_dep_node
>   78: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   79: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   80: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   81: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   82: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
>   83: rustc_data_structures::stack::ensure_sufficient_stack
>   84: rustc_query_system::query::plumbing::get_query_impl
>   85: rustc_typeck::collect::type_of::type_of
>   86: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>   87: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   88: rustc_data_structures::stack::ensure_sufficient_stack
>   89: rustc_query_system::query::plumbing::force_query_impl
>   90: rustc_middle::ty::query::force_from_dep_node
>   91: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   92: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   93: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   94: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
>   95: rustc_data_structures::stack::ensure_sufficient_stack
>   96: rustc_query_system::query::plumbing::get_query_impl
>   97: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
>    4: core::fmt::write
>   98: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
>   99: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
>              at src/libcore/fmt/mod.rs:1076
>  100: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
>  101: rustc_typeck::check::FnCtxt::resolve_generator_interiors
>  102: rustc_middle::ty::context::GlobalCtxt::enter_local
>  103: rustc_typeck::check::typeck_tables_of
>  104: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>    5: std::io::Write::write_fmt
>  105: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
> note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
> 
>              at src/libstd/io/mod.rs:1537
>    6: std::sys_common::backtrace::_print
>              at src/libstd/sys_common/backtrace.rs:62
>    7: std::sys_common::backtrace::print
>              at src/libstd/sys_common/backtrace.rs:49
>    8: std::panicking::default_hook::{{closure}}
>              at src/libstd/panicking.rs:198
>    9: std::panicking::default_hook
> note: the compiler unexpectedly panicked. this is a bug.
>              at src/libstd/panicking.rs:218
>   10: rustc_driver::report_ice
> 
> note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
> 
> note: rustc 1.45.0-nightly (ad4bc3323 2020-06-01) running on x86_64-unknown-linux-gnu
> 
> note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld --crate-type lib
> 
> note: some of the compiler flags provided by cargo are hidden
> 
> query stack during panic:
> #0 [typeck_tables_of] type-checking `node::handle_client`
> #1 [mir_built] building MIR for `node::handle_client`
> #2 [mir_borrowck] borrow-checking `node::handle_client`
> #3 [type_of] computing type of `node::handle_client::{{opaque}}#0`
> #4 [evaluate_obligation] evaluating trait selection obligation `{std::future::ResumeTy, async_std::net::tcp::stream::TcpStream, std::sync::Arc<guid::GuidBuilder>, std::sync::Arc<async_std::sync::mutex::Mutex<std::collections::HashMap<std::string::String, std::sync::Arc<topic::Topic>>>>, impl std::future::Future, ()}: std::marker::Send`
> #5 [typeck_tables_of] type-checking `node::Node::run`
> #6 [type_of] computing type of `node::Node::run::{{opaque}}#0`
> #7 [check_mod_item_types] checking item types in top-level module
> #8 [analysis] running analysis passes on this crate
> end of query stack
>   11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
>              at /SSD-speed/sherlock/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/liballoc/boxed.rs:1090
>   12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
>              at /SSD-speed/sherlock/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libproc_macro/bridge/client.rs:318
>   13: std::panicking::rust_panic_with_hook
>              at src/libstd/panicking.rs:490
>   14: std::panicking::begin_panic
>   15: rustc_errors::HandlerInner::bug
>   16: rustc_errors::Handler::bug
>   17: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
>   18: rustc_middle::ty::context::tls::with_opt::{{closure}}
>   19: rustc_middle::ty::context::tls::with_opt
>   20: rustc_middle::util::bug::opt_span_bug_fmt
>   21: rustc_middle::util::bug::bug_fmt
>   22: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::RegionKind>::hash_stable
>   23: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
>   24: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for &rustc_middle::ty::list::List<T>>::hash_stable
>   25: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
>   26: <[T] as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable
>   27: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for &rustc_middle::ty::list::List<T>>::hash_stable
>   28: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
>   29: <T as rustc_query_system::dep_graph::dep_node::DepNodeParams<Ctxt>>::to_fingerprint
>   30: rustc_query_system::query::plumbing::get_query_impl
>   31: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::suggest_await_before_try
>   32: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
>   33: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
>   34: rustc_typeck::check::FnCtxt::check_argument_types
>   35: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
>   36: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
>   37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   39: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
>   40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   42: rustc_typeck::check::FnCtxt::check_stmt
>   43: rustc_typeck::check::FnCtxt::check_block_with_expected
>   44: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   45: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   46: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   47: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   48: rustc_typeck::check::FnCtxt::check_block_with_expected
>   49: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
>   52: rustc_typeck::check::check_fn
>   53: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
>   54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   55: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   56: rustc_typeck::check::FnCtxt::check_argument_types
>   57: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
>   58: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
>   59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
>   60: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
>   61: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
>   62: rustc_typeck::check::check_fn
>   63: rustc_middle::ty::context::GlobalCtxt::enter_local
>   64: rustc_typeck::check::typeck_tables_of
>   65: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_tables_of>::compute
>   66: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   67: rustc_data_structures::stack::ensure_sufficient_stack
>   68: rustc_query_system::query::plumbing::get_query_impl
>   69: rustc_mir_build::hair::cx::Cx::new
>   70: rustc_middle::ty::context::GlobalCtxt::enter_local
>   71: rustc_mir_build::build::mir_built
>   72: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
>   73: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>   74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   75: rustc_data_structures::stack::ensure_sufficient_stack
>   76: rustc_query_system::query::plumbing::force_query_impl
>   77: rustc_middle::ty::query::force_from_dep_node
>   78: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   79: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   80: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   81: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   82: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
>   83: rustc_data_structures::stack::ensure_sufficient_stack
>   84: rustc_query_system::query::plumbing::get_query_impl
>   85: rustc_typeck::collect::type_of::type_of
>   86: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>   87: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
>   88: rustc_data_structures::stack::ensure_sufficient_stack
>   89: rustc_query_system::query::plumbing::force_query_impl
>   90: rustc_middle::ty::query::force_from_dep_node
>   91: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   92: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   93: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
>   94: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
>   95: rustc_data_structures::stack::ensure_sufficient_stack
>   96: rustc_query_system::query::plumbing::get_query_impl
>   97: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
>   98: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
>   99: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
>  100: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
>  101: rustc_typeck::check::FnCtxt::resolve_generator_interiors
>  102: rustc_middle::ty::context::GlobalCtxt::enter_local
>  103: rustc_typeck::check::typeck_tables_of
>  104: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
>  105: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
> note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
> 
> error: aborting due to previous error
> 
> note: the compiler unexpectedly panicked. this is a bug.
> 
> note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
> 
> note: rustc 1.45.0-nightly (ad4bc3323 2020-06-01) running on x86_64-unknown-linux-gnu
> 
> note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld
> 
> note: some of the compiler flags provided by cargo are hidden
> 
> query stack during panic:
> #0 [typeck_tables_of] type-checking `node::handle_client`
> #1 [mir_built] building MIR for `node::handle_client`
> #2 [mir_borrowck] borrow-checking `node::handle_client`
> #3 [type_of] computing type of `node::handle_client::{{opaque}}#0`
> #4 [evaluate_obligation] evaluating trait selection obligation `{std::future::ResumeTy, async_std::net::tcp::stream::TcpStream, std::sync::Arc<guid::GuidBuilder>, std::sync::Arc<async_std::sync::mutex::Mutex<std::collections::HashMap<std::string::String, std::sync::Arc<topic::Topic>>>>, impl std::future::Future, ()}: std::marker::Send`
> #5 [typeck_tables_of] type-checking `node::Node::run`
> #6 [type_of] computing type of `node::Node::run::{{opaque}}#0`
> #7 [check_mod_item_types] checking item types in top-level module
> #8 [analysis] running analysis passes on this crate
> end of query stack
> error: aborting due to previous error
> 
> error: could not compile `smq`.
> 
> To learn more, run the command again with --verbose.
> warning: build failed, waiting for other jobs to finish...
> error: build failed
> 