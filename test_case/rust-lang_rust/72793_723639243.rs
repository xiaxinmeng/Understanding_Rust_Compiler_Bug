`
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:78:17: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@./72793.rs:21:19: 21:25] as std::ops::Fn<(impl T,)>>), Binder(<[closure@./72793.rs:21:19: 21:25] as std::ops::Fn<(&S,)>>), Sorts(ExpectedFound { expected: &S, found: impl T }))` selecting `Binder(<[closure@./72793.rs:21:19: 21:25] as std::ops::Fn<(&S,)>>)` during codegen

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0:     0x7f714147aa50 - std::backtrace_rs::backtrace::libunwind::trace::h448c56741b6011d3
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/../../backtrace/src/backtrace/libunwind.rs:100:5
   1:     0x7f714147aa50 - std::backtrace_rs::backtrace::trace_unsynchronized::h2d26397c5720fdbb
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f714147aa50 - std::sys_common::backtrace::_print_fmt::h720a2f61f75b9f58
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f714147aa50 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8d62bbfda6d5c836
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f71414ea94c - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/core/src/fmt/mod.rs:1078:17
   5:     0x7f714146c6b2 - std::io::Write::write_fmt::h16a9fe7680ac245c
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/io/mod.rs:1518:15
   6:     0x7f714147fa05 - std::sys_common::backtrace::_print::h9ba51a6db618de7d
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f714147fa05 - std::sys_common::backtrace::print::h5f20e41c85e91716
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f714147fa05 - std::panicking::default_hook::{{closure}}::h5b7294ca19e8edab
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/panicking.rs:208:50
   9:     0x7f714147f6a8 - std::panicking::default_hook::hb3948d1f74b6ff4c
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/panicking.rs:227:9
  10:     0x7f7141d5a2d4 - rustc_driver::report_ice::h021ea70c3b174afa
  11:     0x7f7141480306 - std::panicking::rust_panic_with_hook::h55d23fef0ad751bc
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/panicking.rs:597:17
  12:     0x7f7144da3f7d - std::panicking::begin_panic::{{closure}}::h2d2fd75c8f2a2341
  13:     0x7f7144da3de6 - std::sys_common::backtrace::__rust_end_short_backtrace::h463d6160b8d38e51
  14:     0x7f7144da3f1f - std::panicking::begin_panic::h7716ff6593f9c4fe
  15:     0x7f7144ddcc9c - rustc_errors::HandlerInner::bug::hcaaf59cd7aedcc3b
  16:     0x7f7144ddb290 - rustc_errors::Handler::bug::h709d540b8038ac8d
  17:     0x7f71448b1324 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::hd783d6169452aa9a
  18:     0x7f71448b05db - rustc_middle::ty::context::tls::with_opt::{{closure}}::hb843c547a56f3e04
  19:     0x7f71448b0582 - rustc_middle::ty::context::tls::with_opt::h3f9cf1292158aa7f
  20:     0x7f71448b1249 - rustc_middle::util::bug::opt_span_bug_fmt::hbec46092491724ec
  21:     0x7f71448b11be - rustc_middle::util::bug::bug_fmt::hd9854762eb11f3b1
  22:     0x7f71440d0667 - rustc_infer::infer::InferCtxtBuilder::enter::hd27c91343128654f
  23:     0x7f71442fb38f - rustc_trait_selection::traits::codegen::codegen_fulfill_obligation::hbb8f1b498b15c1ea
  24:     0x7f7142351a2b - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fulfill_obligation>::compute::h1cd7b59c9a9732c0
  25:     0x7f7142340028 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h6569c8e990e04dd1
  26:     0x7f7142369ed6 - rustc_data_structures::stack::ensure_sufficient_stack::h4d0f969f57d162bc
  27:     0x7f71422f677c - rustc_query_system::query::plumbing::get_query_impl::h225377355e8b4967
  28:     0x7f714235a448 - rustc_ty::instance::inner_resolve_instance::hcd318f1ced437469
  29:     0x7f714235a0aa - rustc_ty::instance::resolve_instance::h3bc1bc12db816545
  30:     0x7f7144c4102a - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::resolve_instance>::compute::he88c4b2daaf352f7
  31:     0x7f7144c703f0 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h66ffbab719a699ee
  32:     0x7f71448518c6 - rustc_data_structures::stack::ensure_sufficient_stack::h5d70dde26801a158
  33:     0x7f71445b46bb - rustc_query_system::query::plumbing::get_query_impl::h0f9977ba7047dd61
  34:     0x7f7144c322c4 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::he0415d3539e95d51
  35:     0x7f7144c31f80 - rustc_middle::ty::instance::Instance::resolve::h7b74795a5a17f452
  36:     0x7f7143340c3f - <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator::hecf45b8e4c8ef86f
  37:     0x7f71433438cf - rustc_mir::monomorphize::collector::collect_neighbours::h007bc9dac1822f58
  38:     0x7f714333e192 - rustc_mir::monomorphize::collector::collect_items_rec::h8b80aa1d8c5baccb
  39:     0x7f714333e404 - rustc_mir::monomorphize::collector::collect_items_rec::h8b80aa1d8c5baccb
  40:     0x7f7143607841 - rustc_session::utils::<impl rustc_session::session::Session>::time::h0769395397f4e773
  41:     0x7f714333d10d - rustc_mir::monomorphize::collector::collect_crate_mono_items::h16fd359bc82e9cba
  42:     0x7f71433f727c - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::hd62bc8ab380467e4
  43:     0x7f714217ebc2 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute::ha94f6ecf95919685
  44:     0x7f71420f00c9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h11581c1c2273af7b
  45:     0x7f714215a25a - rustc_data_structures::stack::ensure_sufficient_stack::h5cc442ccbebc74cd
  46:     0x7f714204da4b - rustc_query_system::query::plumbing::get_query_impl::h4e7fa7005ff50500
  47:     0x7f714217f9cc - rustc_codegen_ssa::base::codegen_crate::h90cf2d1d07dcdf6e
  48:     0x7f71421d6a15 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h2a59653aff37ec7b
  49:     0x7f7141f94e3f - rustc_interface::passes::QueryContext::enter::hae4ed674370a9e23
  50:     0x7f7141fc52d3 - rustc_interface::queries::Queries::ongoing_codegen::h54d5e3fae50fcc04
  51:     0x7f7141d6ee51 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hefaa7b63637d8d8e
  52:     0x7f7141d5ce7e - rustc_span::with_source_map::h829c6eccedcb37ca
  53:     0x7f7141d70252 - rustc_interface::interface::create_compiler_and_run::h5c2da4096753e17e
  54:     0x7f7141d5da5a - rustc_span::with_session_globals::h3b2e3a8e7e59ecdc
  55:     0x7f7141d75be1 - std::sys_common::backtrace::__rust_begin_short_backtrace::hf69e25d23954f1a0
  56:     0x7f7141cdcc18 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hed3ab0291d940426
  57:     0x7f714148f30a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9e7afb7a0a438236
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/alloc/src/boxed.rs:1307:9
  58:     0x7f714148f30a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h70c646c4271337a1
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/alloc/src/boxed.rs:1307:9
  59:     0x7f714148f30a - std::sys::unix::thread::Thread::new::thread_start::h35d2b8d36f210d02
                               at /rustc/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/sys/unix/thread.rs:89:17
  60:     0x7f714138a3e9 - start_thread
  61:     0x7f71412a7293 - __GI___clone
  62:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (b1277d04d 2020-11-08) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Fn` fulfills its obligations
#1 [resolve_instance] resolving instance `<[closure@./72793.rs:21:19: 21:25] as std::ops::Fn<(&S,)>>::call`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

