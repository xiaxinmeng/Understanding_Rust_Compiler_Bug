log
error: internal compiler error: src/librustc_mir/transform/generator.rs:1225: impossible case reached

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
stack backtrace:
   0:        0x10b0da7ae - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h663935c3ade921d4
   1:        0x10b113f0c - core::fmt::write::h45a9bd04db15c24f
   2:        0x10b0cc2a7 - std::io::Write::write_fmt::h0e6cabb1f767eb66
   3:        0x10b0df2e5 - std::panicking::default_hook::{{closure}}::h45f8ee2074ad82b4
   4:        0x10b0df022 - std::panicking::default_hook::hd48caf78d696bd9a
   5:        0x10b80d9c8 - rustc_driver::report_ice::h20209cc7d5e9b721
   6:        0x10b0df935 - std::panicking::rust_panic_with_hook::h468218923d76a6d7
   7:        0x10fc37886 - std::panicking::begin_panic::h6f663bec8c63636d
   8:        0x10f8459f7 - rustc_errors::HandlerInner::bug::hcf0983b48acbbacf
   9:        0x10f844337 - rustc_errors::Handler::bug::h2d11699c9caf6729
  10:        0x10f4801f9 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h6df69a2a3dd903a9
  11:        0x10f4781e6 - rustc_middle::ty::context::tls::with_opt::{{closure}}::h08953dcab9277843
  12:        0x10f47816b - rustc_middle::ty::context::tls::with_opt::h3ea04b0ab6091eff
  13:        0x10f480108 - rustc_middle::util::bug::opt_span_bug_fmt::h5a2e586f8e52e067
  14:        0x10fc2130b - rustc_middle::util::bug::bug_fmt::h0e03d94765b73e41
  15:        0x10e4c0bcc - <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass::h3ef22c1710d372cd
  16:        0x10e66e996 - rustc_mir::transform::run_passes::hd42a628b56c379fa
  17:        0x10e670048 - rustc_mir::transform::run_optimization_passes::h7aaac24e50e622f9
  18:        0x10e670337 - rustc_mir::transform::optimized_mir::h42c99d0a316f684f
  19:        0x10e9348fb - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute::ha47e246c174bdb06
  20:        0x10ea182c8 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h6415c816156b8a28
  21:        0x10eabb2f2 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc594217cb7a84d83
  22:        0x10eae3254 - rustc_data_structures::stack::ensure_sufficient_stack::h127eb72d4b2f7a5e
  23:        0x10eaa0dfe - rustc_query_system::query::plumbing::get_query_impl::hf23da1e9f1218c25
  24:        0x10e945f99 - rustc_metadata::rmeta::encoder::EncodeContext::encode_optimized_mir::h1f1755936c71f94a
  25:        0x10e946d39 - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h930b558cca40452a
  26:        0x10eafbf9f - rustc_hir::intravisit::walk_expr::h61856d0f7e90f3d9
  27:        0x10e946a2a - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h930b558cca40452a
  28:        0x10eafb7b5 - rustc_hir::intravisit::walk_fn::h16f7cd7c49739edc
  29:        0x10eafc754 - rustc_hir::intravisit::walk_item::h5c459e923d44349a
  30:        0x10e9472ef - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item::h677f6dc9c4004353
  31:        0x10eafd546 - rustc_hir::hir::Crate::visit_all_item_likes::h804ca1112e64a269
  32:        0x10e94eee5 - rustc_metadata::rmeta::encoder::encode_metadata_impl::he7196de0938d43b1
  33:        0x10ea2de0f - rustc_data_structures::sync::join::h4f6a3d1460736199
  34:        0x10e97bb54 - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::h9b154f4f413b549a
  35:        0x10f476c24 - rustc_middle::ty::context::TyCtxt::encode_metadata::h9900cc31cdbd1748
  36:        0x10b98528a - rustc_interface::passes::start_codegen::he1887f2bb9a360fe
  37:        0x10b9a3648 - rustc_middle::ty::context::tls::enter_global::h5234796652c04bbe
  38:        0x10b98f2ab - rustc_interface::queries::Queries::ongoing_codegen::h8127070cc894d6fb
  39:        0x10b81693d - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb63f149f7d127d23
  40:        0x10b877c91 - rustc_span::with_source_map::hff02123913f3c9fc
  41:        0x10b818b17 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hfade9e67fe3dc88a
  42:        0x10b80f499 - scoped_tls::ScopedKey<T>::set::ha23c73df790e47d2
  43:        0x10b819bbd - std::sys_common::backtrace::__rust_begin_short_backtrace::hebfd2a8940a25d1c
  44:        0x10b7f9f3c - core::ops::function::FnOnce::call_once{{vtable.shim}}::h302f2854e9369da4
  45:        0x10b0edb0d - std::sys::unix::thread::Thread::new::thread_start::h15652ee16771ed61
  46:     0x7fff67dba109 - _ZL12preoptimized

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (feb3536eb 2020-06-09) running on x86_64-apple-darwin

note: compiler flags: -C panic=abort -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `module::{{closure}}#0`
end of query stack
error: aborting due to previous error

error: could not compile `MY_CRATE`.

To learn more, run the command again with --verbose.
