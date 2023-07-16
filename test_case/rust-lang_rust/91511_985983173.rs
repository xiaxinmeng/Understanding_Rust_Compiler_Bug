
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/compiler/rustc_hir/src/definitions.rs:452:14
stack backtrace:
   0:        0x10e40a321 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4edbddd26fc45697
   1:        0x10e45b4bb - core::fmt::write::h266c957f12b1655a
   2:        0x10e3fad3a - std::io::Write::write_fmt::h65d4ee621e8d810d
   3:        0x10e40d585 - std::panicking::default_hook::{{closure}}::h5b4886cd9f93dfca
   4:        0x10e40d16f - std::panicking::default_hook::h986d152c7bd7e732
   5:        0x1063977c8 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h997411fe6ff6a285
   6:        0x10e40ddc6 - std::panicking::rust_panic_with_hook::h151f3b3bf37b4f17
   7:        0x10e40d814 - std::panicking::begin_panic_handler::{{closure}}::hf6660086d9ebd48c
   8:        0x10e40a797 - std::sys_common::backtrace::__rust_end_short_backtrace::h34152178ea368a9a
   9:        0x10e40d7aa - _rust_begin_unwind
  10:        0x10e48605f - core::panicking::panic_fmt::hb64a2db862b4aca0
  11:        0x10e485fb7 - core::panicking::panic::h709f8bbcee9c47c1
  12:        0x109bf8fba - <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id::hb730d16bcb9182b2
  13:        0x10a83c49c - rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover::hffc4e6a0830f2648
  14:        0x109aecbe0 - rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node::h1e753a243eb0fce7
  15:        0x109b60a79 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h5ec984268becc2ed
  16:        0x109b60a50 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h5ec984268becc2ed
  17:        0x109b60a50 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h5ec984268becc2ed
  18:        0x109b60a50 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h5ec984268becc2ed
  19:        0x109b20beb - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green::h29c934006e707d96
  20:        0x109877e88 - rustc_query_system::query::plumbing::ensure_must_run::h7a6c9ea58c479dff
  21:        0x109b0f07a - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait::hca51fa11b348897d
  22:        0x108e3c837 - rustc_session::session::Session::track_errors::h3606446a7e54ce1b
  23:        0x108dfad35 - rustc_typeck::check_crate::h639646483ad1f67a
  24:        0x1064b819f - rustc_interface::passes::analysis::h4f85fbb562aaa2d0
  25:        0x109b6d1d8 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h250cbf125cf5c5da
  26:        0x109a8f5d5 - rustc_data_structures::stack::ensure_sufficient_stack::h6bf2db115705a40c
  27:        0x10995448c - rustc_query_system::query::plumbing::try_execute_query::hea85233fa9cc2fd4
  28:        0x109b0a0c5 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis::hda9a7d1ffac3bca0
  29:        0x1063f7211 - rustc_interface::passes::QueryContext::enter::hca7a6b48fcdfcc9c
  30:        0x1063d5be6 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h5dd0e3e5d157764e
  31:        0x1063a620e - rustc_span::with_source_map::h5222a0af0f8eb603
  32:        0x1063d43ac - scoped_tls::ScopedKey<T>::set::h1d3d67352154478d
  33:        0x1063aac32 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb945015c1b1cd51d
  34:        0x1063fc9a5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc43c9a06b1d9c32a
  35:        0x10e417ba7 - std::sys::unix::thread::Thread::new::thread_start::h1ebadf8a3a1817e4
  36:     0x7fff2040a8fc - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `components`
