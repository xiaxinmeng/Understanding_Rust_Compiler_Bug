
stack backtrace:
   0:        0x107dae1fc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hddf67b5e68ee5eac
   1:        0x107e1599d - core::fmt::write::hae6418d3f135b639
   2:        0x107d9fe16 - std::io::Write::write_fmt::h822c37b1fbb805d3
   3:        0x107db1e79 - std::panicking::default_hook::{{closure}}::hb8c76ec6b2b5fec2
   4:        0x107db1a00 - std::panicking::default_hook::h9520f36dd50be056
   5:        0x10eac7468 - rustc_driver::report_ice::hab08807b07bece61
   6:        0x107db265e - std::panicking::rust_panic_with_hook::hbb70e1d25c7381a9
   7:        0x107db2139 - std::panicking::begin_panic_handler::{{closure}}::hb72eee9aad2e147c
   8:        0x107dae6b8 - std::sys_common::backtrace::__rust_end_short_backtrace::h372ff87ecb2667f3
   9:        0x107db20ca - _rust_begin_unwind
  10:        0x107e3cc1f - core::panicking::panic_fmt::h261fd45d36f74dfa
  11:        0x107e3cb77 - core::panicking::panic::hee3a097dbdc988d8
  12:        0x112b8c396 - rustc_middle::ty::query::try_load_from_on_disk_cache::h79b5afe1c2e80cd7
  13:        0x1120a5284 - rustc_query_system::dep_graph::graph::DepGraph<K>::exec_cache_promotions::h59848ff5959f2b25
  14:        0x1120ba598 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h6b0f4f58f3fe4fa5
  15:        0x112074663 - rustc_incremental::persist::save::save_in::h95c4af985c810672
  16:        0x11207055f - rustc_data_structures::sync::join::h28cbe6f4856b759a
  17:        0x1120b9f9d - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h417a8e727abe81c6
  18:        0x112073707 - rustc_incremental::persist::save::save_dep_graph::h5168ffbb71fa5484
  19:        0x111f90893 - rustc_codegen_ssa::base::finalize_tcx::h27af4387e3fad8ad
  20:        0x10ee782c0 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h672987f7a5956f16
  21:        0x10ec89dad - rustc_session::utils::<impl rustc_session::session::Session>::time::hdde8570199d8b0d3
  22:        0x10ecc18e2 - rustc_interface::passes::QueryContext::enter::hd55f5917ee1869a7
  23:        0x10ed07690 - rustc_interface::queries::Queries::ongoing_codegen::hd9e56cbce1d99cdb
  24:        0x10ea79b6f - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb3cbdf40d2d583ab
  25:        0x10eaf73de - rustc_span::with_source_map::h4f73b4651f1d8670
  26:        0x10ea7aae4 - rustc_interface::interface::create_compiler_and_run::ha35d2af54e9355e8
  27:        0x10eb0d749 - scoped_tls::ScopedKey<T>::set::h8cb1fe74bd062a4c
  28:        0x10eb13b91 - std::sys_common::backtrace::__rust_begin_short_backtrace::h8e2469a9116ebf76
  29:        0x10ea821c9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hfc39034b1b021f49
  30:        0x107dbefdd - std::sys::unix::thread::Thread::new::thread_start::h93dd3097fa4fa219
  31:     0x7fff6d53a109 - __pthread_start

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: rustc 1.50.0-nightly (3d6705aa5 2020-12-07) running on x86_64-apple-darwin

note: compiler flags: -C linker-plugin-lto -C debuginfo=2 -C incremental -C target-cpu=native --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
