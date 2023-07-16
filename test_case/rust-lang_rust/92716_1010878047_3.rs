
    Finished dev [unoptimized] target(s) in 0.02s
     Running `target/debug/xtask install`
$ cargo install --path crates/rust-analyzer --locked --force --features force-always-assert
  Installing rust-analyzer v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/rust-analyzer)
    Updating crates.io index
   Compiling ide_assists v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/ide_assists)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
   2: core::panicking::panic
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
   3: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_session::cstore::CrateStore for rustc_metadata::creader::CStore>::def_path_hash_to_def_id
   4: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   5: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::DefId>::recover
   6: rustc_query_system::query::plumbing::force_query
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  10: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  11: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  12: rustc_query_system::query::plumbing::ensure_must_run
  13: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  14: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  15: rustc_data_structures::sync::par_for_each_in
  16: rustc_interface::passes::analysis
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::try_execute_query
  20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  21: rustc_interface::passes::QueryContext::enter
  22: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  23: rustc_span::with_source_map
  24: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
   2: core::panicking::panic
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
   3: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_session::cstore::CrateStore for rustc_metadata::creader::CStore>::def_path_hash_to_def_id
   4: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   5: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::DefId>::recover
   6: rustc_query_system::query::plumbing::force_query
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   9: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory
  10: rustc_query_system::query::plumbing::try_execute_query
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
  12: <rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue
  13: rustc_mir_transform::check_unsafety::unsafety_check_result
  14: core::ops::function::FnOnce::call_once
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::try_execute_query
  18: rustc_query_system::query::plumbing::force_query_impl
  19: rustc_query_impl::query_callbacks::unsafety_check_result::force_from_dep_node
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  21: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  23: rustc_query_system::query::plumbing::ensure_must_run
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
  25: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  26: rustc_data_structures::sync::par_for_each_in
  27: rustc_interface::passes::analysis
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::try_execute_query
  31: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  32: rustc_interface::passes::QueryContext::enter
  33: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  34: rustc_span::with_source_map
  35: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [unsafety_check_result] unsafety-checking `handlers::generate_documentation_template::generate_documentation_template::{closure#0}`
#1 [unsafety_check_result] unsafety-checking `handlers::generate_documentation_template::generate_documentation_template`
#2 [analysis] running analysis passes on this crate
end of query stack
error: failed to compile `rust-analyzer v0.0.0 (/home/my_username_redacted/temp/rust-analyzer/crates/rust-analyzer)`, intermediate artifacts can be found at `/home/my_username_redacted/temp/rust-analyzer/target`

Caused by:
  could not compile `ide_assists`
Error: install server

Caused by:
    command `cargo install --path crates/rust-analyzer --locked --force --features force-always-assert` failed, exit status: 101
