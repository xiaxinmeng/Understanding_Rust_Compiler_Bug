
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/compiler/rustc_hir/src/definitions.rs:452:14
stack backtrace:
   0:     0x7f99b00e3a9c - std::backtrace_rs::backtrace::libunwind::trace::hf6a6dfd7da937cb0
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f99b00e3a9c - std::backtrace_rs::backtrace::trace_unsynchronized::hc596a19e4891f7f3
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f99b00e3a9c - std::sys_common::backtrace::_print_fmt::hb16700db31584325
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f99b00e3a9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h231c4190cfa75162
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f99b0140fdc - core::fmt::write::h2a1462b5f8eea807
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/fmt/mod.rs:1163:17
   5:     0x7f99b00d3c05 - std::io::Write::write_fmt::h71ddfebc68685972
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/io/mod.rs:1696:15
   6:     0x7f99b00e6f60 - std::sys_common::backtrace::_print::hcc197d4bebf2b369
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f99b00e6f60 - std::sys_common::backtrace::print::h335a66af06738c7c
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f99b00e6f60 - std::panicking::default_hook::{{closure}}::h6fac9ac9c8b79e52
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:210:50
   9:     0x7f99b00e6b15 - std::panicking::default_hook::h341c1030c6a1161b
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:227:9
  10:     0x7f99b08d32b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h932547f60770f26a
  11:     0x7f999814c273 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h9ecfe8ad3730f2bf
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1705:9
  12:     0x7f99981cb28d - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::heed0aea2b0a83910
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7f99b00e7779 - std::panicking::rust_panic_with_hook::h50680ff4b44510c6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:628:17
  14:     0x7f99b00e7202 - std::panicking::begin_panic_handler::{{closure}}::h9371c0fbb1e8465a
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:519:13
  15:     0x7f99b00e3f44 - std::sys_common::backtrace::__rust_end_short_backtrace::h9b3efa22a5768c0f
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys_common/backtrace.rs:139:18
  16:     0x7f99b00e7199 - rust_begin_unwind
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
  17:     0x7f99b00ab441 - core::panicking::panic_fmt::h23b9203e89cc61cf
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
  18:     0x7f99b00ab38d - core::panicking::panic::h0ba7146865b2f9d6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
  19:     0x7f99b20d6b47 - <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id::hb925ab58899dddff
  20:     0x7f99b2442b21 - rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover::h2e76aa40aab4da19
  21:     0x7f99b2acd52c - rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node::hc7fd02f51ac51090
  22:     0x7f99b20c1d6a - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h14b50df9818693f3
  23:     0x7f99b20c1d41 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h14b50df9818693f3
  24:     0x7f99b20c1d41 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h14b50df9818693f3
  25:     0x7f99b20c1d41 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h14b50df9818693f3
  26:     0x7f99b2020234 - rustc_query_system::query::plumbing::ensure_must_run::h5874d260d5ffefca
  27:     0x7f99b2adb430 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait::hef06870853770eff
  28:     0x7f99b1c9abf6 - rustc_session::session::Session::track_errors::hd7a6334bb75371c9
  29:     0x7f99b277e448 - rustc_typeck::check_crate::h6215961d94aab927
  30:     0x7f99b24dd0a0 - rustc_interface::passes::analysis::h9fda1a8ae44d53e7
  31:     0x7f99b2af73cf - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h26e86d963a73dec8
  32:     0x7f99b2a9c609 - rustc_data_structures::stack::ensure_sufficient_stack::h135baf7277c4d193
  33:     0x7f99b29c0a7b - rustc_query_system::query::plumbing::try_execute_query::h0bf7639d3f58bfbd
  34:     0x7f99b2ad9232 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::ha2144cf05b40d10d
  35:     0x7f99b24d1ca9 - rustc_interface::passes::QueryContext::enter::h0523b23606206a0b
  36:     0x7f99b24b8977 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hf84cd18c24bd5171
  37:     0x7f99b24a60ee - rustc_span::with_source_map::h6ab8a240e103b5b9
  38:     0x7f99b24b82ac - scoped_tls::ScopedKey<T>::set::hd1fbd64c6f645895
  39:     0x7f99b24a6ef5 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0a1328c9fa7f7448
  40:     0x7f99b24d2962 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h4ea1ced06d6b3e97
  41:     0x7f99b00f2933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7bd677a5dc988be6
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  42:     0x7f99b00f2933 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7b1c1ba11c4db785
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/alloc/src/boxed.rs:1691:9
  43:     0x7f99b00f2933 - std::sys::unix::thread::Thread::new::thread_start::h9c58c0d12d84e854
                               at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/sys/unix/thread.rs:106:17
  44:     0x7f99b0007259 - start_thread
  45:     0x7f99aff1c5e3 - __GI___clone
  46:                0x0 - <unknown>

error: internal compiler error: unexpected panic
