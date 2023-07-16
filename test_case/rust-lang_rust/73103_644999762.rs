
    Checking annotate-snippets v0.8.0
thread 'rustc' panicked at 'DefId::expect_local: `DefId(1:2858 ~ std[af34]::io[0]::stdio[0]::_print[0])` isn't local', /home/manishearth/mozilla/rust/src/libstd/macros.rs:16:9
stack backtrace:
    Checking rustc_fs_util v0.0.0 (/home/manishearth/mozilla/rust/src/librustc_fs_util)
thread 'rustc' panicked at 'DefId::expect_local: `DefId(1:2858 ~ std[af34]::io[0]::stdio[0]::_print[0])` isn't local', /home/manishearth/mozilla/rust/src/libstd/macros.rs:16:9
stack backtrace:
   Compiling rustc_attr v0.0.0 (/home/manishearth/mozilla/rust/src/librustc_attr)
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: std::panicking::begin_panic_fmt
  10: rustc_span::def_id::DefId::expect_local::{{closure}}
  11: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover
  12: rustc_middle::ty::query::force_from_dep_node
thread 'rustc' panicked at 'DefId::expect_local: `DefId(1:2858 ~ std[af34]::io[0]::stdio[0]::_print[0])` isn't local', /home/manishearth/mozilla/rust/src/libstd/macros.rs:16:9
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
stack backtrace:
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
  15: rustc_query_system::query::plumbing::ensure_query_impl
  16: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  17: std::panicking::try
  18: rustc_session::utils::<impl rustc_session::session::Session>::time
  19: rustc_interface::passes::analysis
  20: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  21: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_middle::ty::context::tls::enter_global
  27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  28: rustc_span::with_source_map
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: scoped_tls::ScopedKey<T>::set
  31: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `rustc_target`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: std::panicking::begin_panic_fmt
  10: rustc_span::def_id::DefId::expect_local::{{closure}}
  11: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover
  12: rustc_middle::ty::query::force_from_dep_node
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
  15: rustc_query_system::query::plumbing::ensure_query_impl
  16: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  17: std::panicking::try
  18: rustc_session::utils::<impl rustc_session::session::Session>::time
  19: rustc_interface::passes::analysis
  20: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  21: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_middle::ty::context::tls::enter_global
  27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  28: rustc_span::with_source_map
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: scoped_tls::ScopedKey<T>::set
  31: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: std::panicking::begin_panic_fmt
  10: rustc_span::def_id::DefId::expect_local::{{closure}}
  11: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover
  12: rustc_middle::ty::query::force_from_dep_node
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
  15: rustc_query_system::query::plumbing::ensure_query_impl
  16: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  17: std::panicking::try
  18: rustc_session::utils::<impl rustc_session::session::Session>::time
  19: rustc_interface::passes::analysis
  20: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  21: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_middle::ty::context::tls::enter_global
  27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  28: rustc_span::with_source_map
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: scoped_tls::ScopedKey<T>::set
  31: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: build failed
