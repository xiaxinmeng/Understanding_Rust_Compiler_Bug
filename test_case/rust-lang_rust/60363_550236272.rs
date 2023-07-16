
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:806: Cannot create local mono-item for DefId(17:2303 ~ amadeus_parquet[2f0c]::internal[0]::record[0]::impls[0]::date_time_from_parquet[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:931:9
stack backtrace:
   0:        0x111cbeb65 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0b2af991c2fe744d
   1:        0x111cf5cb0 - core::fmt::write::h9accbbe8984e0e42
   2:        0x111cb246b - std::io::Write::write_fmt::h043704431db5a03f
   3:        0x111cc2f73 - std::panicking::default_hook::{{closure}}::h6c92fb3ec83e7ead
   4:        0x111cc2c7a - std::panicking::default_hook::hc450333d5d96e7ef
   5:        0x10f16a012 - rustc_driver::report_ice::h0aa6b04ddcb1e761
   6:        0x111cc378c - std::panicking::rust_panic_with_hook::h917f6fb05cbf8c98
   7:        0x110cff621 - std::panicking::begin_panic::h1205e40bc9171afc
   8:        0x110d2c397 - rustc_errors::HandlerInner::bug::haeca66694c11cc59
   9:        0x110d2b28d - rustc_errors::Handler::bug::hb40f33537bef7c32
  10:        0x1105ed47b - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h9f9613638d967607
  11:        0x1105eab36 - rustc::ty::context::tls::with_opt::{{closure}}::ha4d042d104651380
  12:        0x1105eaac4 - rustc::ty::context::tls::with_context_opt::h523faf69415e0011
  13:        0x1105eaaf2 - rustc::ty::context::tls::with_opt::h44fc742babe5f857
  14:        0x1105ed398 - rustc::util::bug::opt_span_bug_fmt::h8873c770af03df25
  15:        0x1105ed2eb - rustc::util::bug::bug_fmt::h1857b12f160422a8
  16:        0x10fbc6ec2 - rustc_mir::monomorphize::collector::should_monomorphize_locally::hd8019171dbcb3619
  17:        0x10fbc6b9b - rustc_mir::monomorphize::collector::visit_instance_use::he5c4d4b84a0d9240
  18:        0x10fbc6486 - <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind::hf743bc76134dcb0a
  19:        0x10fbc4a93 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  20:        0x10fbc53d2 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  21:        0x10fbc53d2 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  22:        0x10fbc53d2 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  23:        0x10fbc53d2 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  24:        0x10fbc53d2 - rustc_mir::monomorphize::collector::collect_items_rec::hfa14f1d35d76256f
  25:        0x10fa88fdd - rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}::h574170026231c7ac
  26:        0x10fa584f3 - rustc::util::common::time::hdca60068c42125a5
  27:        0x10fbc340f - rustc_mir::monomorphize::collector::collect_crate_mono_items::he0c4399775ba1745
  28:        0x10fa580a6 - rustc::util::common::time::h070e019f9ed47a1c
  29:        0x10fc7ebd4 - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::hd2c425623409a37b
  30:        0x11438bf12 - rustc::ty::query::__query_compute::collect_and_partition_mono_items::hf7346204340bdd84
  31:        0x1143bc54a - rustc::dep_graph::graph::DepGraph::with_task_impl::h20d95ec042ad160d
  32:        0x11445c8d0 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hfb83f0160d8e8add
  33:        0x11449d24d - rustc_codegen_ssa::base::codegen_crate::h6850b2d17f847535
  34:        0x1144801a9 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate::hab30d48d4da2e9cd
  35:        0x10f233afe - rustc_interface::passes::start_codegen::{{closure}}::h6bcbe3d097a1e661
  36:        0x10f23bdac - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h04e5fd864970b22a
  37:        0x10f27eeeb - rustc_interface::passes::create_global_ctxt::{{closure}}::h52bddd6fee95c3ac
  38:        0x10f239fc9 - rustc_interface::queries::Query<T>::compute::hede163aafb181870
  39:        0x10f24e04c - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen::h485cc6931e7920e3
  40:        0x10f1a6d8f - rustc_interface::interface::run_compiler_in_existing_thread_pool::hea93b9ac2eadfbf3
  41:        0x10f189f44 - std::thread::local::LocalKey<T>::with::h7066d35c989f2793
  42:        0x10f188212 - scoped_tls::ScopedKey<T>::set::h34475cd5175c854c
  43:        0x10f1b2855 - syntax::with_globals::hf919ce8a65a9b30e
  44:        0x10f1c575d - std::sys_common::backtrace::__rust_begin_short_backtrace::h890ade53b1344e13
  45:        0x111cd2d1f - __rust_maybe_catch_panic
  46:        0x10f1c64c7 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb7eb70cb9597c7f7
  47:        0x111ca4a0e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h9b41d7adac010663
  48:        0x111cd1a5e - std::sys::unix::thread::Thread::new::thread_start::h0725fe9398379348
  49:     0x7fff5c8db2eb - _pthread_body
  50:     0x7fff5c8de249 - _pthread_start

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (1423bec54 2019-11-05) running on x86_64-apple-darwin

note: compiler flags: -C opt-level=3 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
