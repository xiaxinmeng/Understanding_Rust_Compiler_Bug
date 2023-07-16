
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0:        0x809d0a75c - std::backtrace_rs::backtrace::libunwind::trace::hbe714726001e366a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:        0x809d0a75c - std::backtrace_rs::backtrace::trace_unsynchronized::hdc20954b281fa398
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x809d0a75c - std::sys_common::backtrace::_print_fmt::h851c3bb22d1bd14f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:67:5
   3:        0x809d0a75c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h992bfab2808b9d2c
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:46:22
   4:        0x809d6533c - core::fmt::write::h4beeee5be6027630
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/fmt/mod.rs:1163:17
   5:        0x809cfad35 - std::io::Write::write_fmt::hbd0bd1c6adf0dd3a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/io/mod.rs:1696:15
   6:        0x809d0d970 - std::sys_common::backtrace::_print::h595ca2cf59b93af6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:49:5
   7:        0x809d0d970 - std::sys_common::backtrace::print::ha09b43169c9decf0
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:36:9
   8:        0x809d0d970 - std::panicking::default_hook::{{closure}}::hd9ad3eaee846e849
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:210:50
   9:        0x809d0d54b - std::panicking::default_hook::h7a7c077f048b8792
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:227:9
  10:        0x801eca654 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::he530e1db20d01541
  11:        0x809d0e118 - std::panicking::rust_panic_with_hook::hb040b83acf3f137e
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:628:17
  12:        0x809d0dbc2 - std::panicking::begin_panic_handler::{{closure}}::hfdbe47d5d1814ad3
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:519:13
  13:        0x809d0ac04 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b51ac183bb8560f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:139:18
  14:        0x809d0db59 - rust_begin_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
  15:        0x809cd2871 - core::panicking::panic_fmt::h4915ff406643e66f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
  16:        0x809cd27bd - core::panicking::panic::hceb470b81ee5e370
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
  17:        0x806258f4d - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_session::cstore::CrateStore for rustc_metadata::creader::CStore>::def_path_hash_to_def_id::ha831cdcb8279f8b1
  18:        0x805e5f799 - <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id::h9df2079a14be3f45
  19:        0x806cb6fde - rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::DefId>::recover::h0be1ec7f2369aa4e
  20:        0x805d568dc - rustc_query_impl::query_callbacks::adt_def::force_from_dep_node::h5a827a651ceb7f4b
  21:        0x805dc8a3a - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h246154e175b68f8a
  22:        0x805d8a5f0 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green::hd5d777c24d30e6eb
  23:        0x805a310a9 - rustc_query_system::query::plumbing::ensure_must_run::hd578707044cfa8d0
  24:        0x805d6e57c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_liveness::h9ecccfd91cac56ed
  25:        0x802006b17 - rustc_middle::hir::map::Map::for_each_module::he9354321183add89
  26:        0x8020dfcaf - rustc_session::utils::<impl rustc_session::session::Session>::time::h64ee4179782c7e9d
  27:        0x80201d2a3 - rustc_interface::passes::analysis::hafe53c6f463472d1
  28:        0x805d227f3 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h82ef7db05ecd9df5
  29:        0x805df165a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h87bde42e8f100b15
  30:        0x805cdd2f5 - rustc_data_structures::stack::ensure_sufficient_stack::hfec16daa65da045b
  31:        0x805a79515 - rustc_query_system::query::plumbing::try_execute_query::h2ceb9b21adc7fe6b
  32:        0x805d6a8c5 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::h20b8ecf2e134fdc0
  33:        0x801f47a34 - rustc_interface::passes::QueryContext::enter::h6e9a68780cfa1f49
  34:        0x801f1ae97 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h3c0234fc1ed6e0c7
  35:        0x801eddd21 - rustc_span::with_source_map::h24653b1f6aeb659e
  36:        0x801f1bc34 - rustc_interface::interface::create_compiler_and_run::h050aaf5624589b91
  37:        0x801eea363 - scoped_tls::ScopedKey<T>::set::h092456e7ef4a4dd2
  38:        0x801ede5b9 - std::sys_common::backtrace::__rust_begin_short_backtrace::h4e3f92e0e4c9cd3c
  39:        0x801f49fe0 - core::ops::function::FnOnce::call_once{{vtable.shim}}::ha7b36f8d53cc6378
  40:        0x809d178d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h035c0499ebb170c5
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  41:        0x809d178d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h284449b9578fabfe
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  42:        0x809d178d3 - std::sys::unix::thread::Thread::new::thread_start::he9b38f275cae3559
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys/unix/thread.rs:106:17
  43:        0x80136882b - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-freebsd

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
