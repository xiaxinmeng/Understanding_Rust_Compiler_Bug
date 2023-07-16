bash
thread 'rustc' panicked at 'places should be checked on creation: InterpErrorInfo { kind: can't access mutable globals in ConstProp, backtrace: None }', src/librustc_mir/interpret/operand.rs:237:25
stack backtrace:
   0:     0x7f8826db9ba4 - backtrace::backtrace::libunwind::trace::he144fab28a4aed2d
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7f8826db9ba4 - backtrace::backtrace::trace_unsynchronized::h54d2de49d4561d5b
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7f8826db9ba4 - std::sys_common::backtrace::_print_fmt::hc03d55f811cceef4
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f8826db9ba4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbc44d25334fa89cd
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f8826df81bc - core::fmt::write::hdf236390fbd68d3d
                               at src/libcore/fmt/mod.rs:1069
   5:     0x7f8826dab5b3 - std::io::Write::write_fmt::h4ee562ef1f300991
                               at src/libstd/io/mod.rs:1532
   6:     0x7f8826dbeba5 - std::sys_common::backtrace::_print::hc4186a5ac838159c
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f8826dbeba5 - std::sys_common::backtrace::print::h61fb789361bb8109
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f8826dbeba5 - std::panicking::default_hook::{{closure}}::hcd876594d255c932
                               at src/libstd/panicking.rs:198
   9:     0x7f8826dbe8e2 - std::panicking::default_hook::hfc205bc5ce834a89
                               at src/libstd/panicking.rs:218
  10:     0x7f882735c9d3 - rustc_driver::report_ice::h425ff32f2997b1b0
  11:     0x7f8826dbf325 - std::panicking::rust_panic_with_hook::h98fcd55ca23bf6e3
                               at src/libstd/panicking.rs:481
  12:     0x7f8826dbee3b - rust_begin_unwind
                               at src/libstd/panicking.rs:385
  13:     0x7f8826df4e51 - core::panicking::panic_fmt::hd101a87121fa411f
                               at src/libcore/panicking.rs:89
  14:     0x7f8826df4ac3 - core::option::expect_none_failed::h4f0e89faa3179bf7
                               at src/libcore/option.rs:1272
  15:     0x7f88286c4ee0 - rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate::h9e5d69914fbd3305
  16:     0x7f88286ef1df - rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place::hecfa8b086a9d5954
  17:     0x7f88283e5fac - <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement::h80dfc90527f8a7a5
  18:     0x7f88283e590c - <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body::hd6bf873cbd2afe4a
  19:     0x7f88283e362a - <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass::h9fa8545e88cf213f
  20:     0x7f88287c4bcc - rustc_mir::transform::run_passes::h86f06e2b0f09c282
  21:     0x7f88287c5f34 - rustc_mir::transform::run_optimization_passes::h1069117249cb0b38
  22:     0x7f88287c616f - rustc_mir::transform::optimized_mir::h4ec71b75d2a6bf1a
  23:     0x7f8829795de1 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h5b1025130185b1f3
  24:     0x7f88297dcc67 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h2e128a35d7b49968
  25:     0x7f88299c5ab3 - rustc_query_system::query::plumbing::get_query::h828844368b6e29fa
  26:     0x7f882866fb4d - rustc_mir::monomorphize::collector::collect_items_rec::h09e92850b75eeb5a
  27:     0x7f88285b1553 - rustc_session::utils::<impl rustc_session::session::Session>::time::hd67751bc12e8f569
  28:     0x7f882866e7ac - rustc_mir::monomorphize::collector::collect_crate_mono_items::he88f6c023e895d70
  29:     0x7f8828704942 - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::hcddfe42969bb55db
  30:     0x7f88288fe322 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute::h74a163faf8fa9fa4
  31:     0x7f88289486a9 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h33492783e319a700
  32:     0x7f8828933e24 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc1a904633c4769f0
  33:     0x7f8828911b11 - rustc_query_system::query::plumbing::get_query::h62e5953c75e544e4
  34:     0x7f88288d3378 - rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local::h4c53d11aabad0ab4
  35:     0x7f8828be0eee - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::exported_symbols>::compute::hd61caafb6202bfed
  36:     0x7f8828b162d3 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::hd1c4539b0ad19f6b
  37:     0x7f8828c3aab5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h393ecb9f06efde8d
  38:     0x7f8828c0fac9 - rustc_query_system::query::plumbing::get_query::h824de524a506526f
  39:     0x7f8828bdcbcc - rustc_metadata::rmeta::encoder::encode_metadata_impl::hd04d894906804c23
  40:     0x7f8828c7510c - rustc_data_structures::sync::join::h2e3de323c6c055ae
  41:     0x7f8828ca82b0 - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::hbbf7378b0f89abc0
  42:     0x7f882966ef60 - rustc_middle::ty::context::TyCtxt::encode_metadata::h4390b9d0eedad1d1
  43:     0x7f88276f5b2e - rustc_interface::passes::start_codegen::hf3eb84427919bf3f
  44:     0x7f8827602b49 - rustc_middle::ty::context::tls::enter_global::h1712f97c17be1c1a
  45:     0x7f88277004c6 - rustc_interface::queries::Queries::ongoing_codegen::h97f0905c2e49e248
  46:     0x7f88274b1731 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hc5186ff1ae47163e
  47:     0x7f882736651d - scoped_tls::ScopedKey<T>::set::h8d9d472e8b3cd3fa
  48:     0x7f8827363a14 - rustc_ast::attr::with_globals::hc6db6365f3099bf0
  49:     0x7f882736e76e - std::sys_common::backtrace::__rust_begin_short_backtrace::h8b9b12138841a09f
  50:     0x7f88274b364e - core::ops::function::FnOnce::call_once{{vtable.shim}}::h90c923a0b1c604b3
  51:     0x7f8826dcf3aa - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h314c63ae4ebdec44
                               at /rustc/fa51f810e5b9254904b92660e7280b7d6a46f112/src/liballoc/boxed.rs:1034
  52:     0x7f8826dcf3aa - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd1e953b7b8bd01ff
                               at /rustc/fa51f810e5b9254904b92660e7280b7d6a46f112/src/liballoc/boxed.rs:1034
  53:     0x7f8826dcf3aa - std::sys::unix::thread::Thread::new::thread_start::hb0fc8ed9cb1fd36f
                               at src/libstd/sys/unix/thread.rs:87
  54:     0x7f8826b336db - start_thread
                               at /build/glibc-OTsEL5/glibc-2.27/nptl/pthread_create.c:463
  55:     0x7f882645088f - __clone
  56:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-nightly (fa51f810e 2020-04-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C bitcode-in-rlib=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `new`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
