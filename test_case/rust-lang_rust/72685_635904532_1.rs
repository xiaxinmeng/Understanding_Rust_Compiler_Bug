log
error: internal compiler error: src/librustc_mir/transform/generator.rs:1244: impossible case reached

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:904:9
stack backtrace:
   0:        0x10b15ff1f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8e33e558833d1956
   1:        0x10b199d0e - core::fmt::write::h9d197fdfdd3a8aba
   2:        0x10b150ee7 - std::io::Write::write_fmt::h489d814bafdb2fc0
   3:        0x10b164a3a - std::panicking::default_hook::{{closure}}::hd5cc0ed2514a340f
   4:        0x10b16477c - std::panicking::default_hook::h7e2bc667d2c8e4db
   5:        0x104bd5588 - rustc_driver::report_ice::hda8844815682ab6b
   6:        0x10b165155 - std::panicking::rust_panic_with_hook::h88adf9321de60790
   7:        0x108c628a6 - std::panicking::begin_panic::h40a3b10abbe0b486
   8:        0x108855147 - rustc_errors::HandlerInner::bug::h5510f64ec98175e7
   9:        0x108853b37 - rustc_errors::Handler::bug::h70245619acd154d8
  10:        0x1082e2a89 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::hf97180823c881ffc
  11:        0x1082d76b6 - rustc_middle::ty::context::tls::with_opt::{{closure}}::hee471a2db74159be
  12:        0x1082d763c - rustc_middle::ty::context::tls::with_opt::ha1f589eb9df81f6e
  13:        0x1082e2998 - rustc_middle::util::bug::opt_span_bug_fmt::hd6250c66736c06a0
  14:        0x108c4115b - rustc_middle::util::bug::bug_fmt::hf584f902a66a7db2
  15:        0x1076bac09 - <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass::hdfea4658ad32071f
  16:        0x1077ba1ce - rustc_mir::transform::run_passes::hf0a88394d36da281
  17:        0x1077bb70a - rustc_mir::transform::run_optimization_passes::h8914b8bf00cb929a
  18:        0x1077bbaad - rustc_mir::transform::optimized_mir::ha7de39a4cf9f4a5b
  19:        0x107b21373 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute::h42faf9beb2921343
  20:        0x107a79149 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::had2b2adaaca5ae3e
  21:        0x107b7ac08 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hdb5c7e7885571969
  22:        0x107b4f5bc - rustc_query_system::query::plumbing::get_query::h7ee3cf724a5d148d
  23:        0x107b16872 - rustc_metadata::rmeta::encoder::EncodeContext::encode_optimized_mir::he774ebbbf385ac39
  24:        0x107b1739a - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h1d09417378a8e4e0
  25:        0x107b860cf - rustc_hir::intravisit::walk_expr::h37385670f6547fc0
  26:        0x107b1717c - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h1d09417378a8e4e0
  27:        0x107b85a25 - rustc_hir::intravisit::walk_fn::h935265b2fbb3e35a
  28:        0x107b8680e - rustc_hir::intravisit::walk_item::ha1cfa26dbfa4ca6f
  29:        0x107b1786f - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item::h8fa511149a8b9845
  30:        0x107ad7046 - rustc_hir::hir::Crate::visit_all_item_likes::hb510a1643aca4d80
  31:        0x107b1da7b - rustc_metadata::rmeta::encoder::encode_metadata_impl::hd10c91e6f0356ac2
  32:        0x107b9fc41 - rustc_data_structures::sync::join::he7dd775e0f210822
  33:        0x107bb54f3 - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::hade3754e7cd7f82e
  34:        0x10838bca4 - rustc_middle::ty::context::TyCtxt::encode_metadata::ha4b006894b0490da
  35:        0x104eef0ad - rustc_interface::passes::start_codegen::h883662f922a42a2b
  36:        0x104e4840b - rustc_middle::ty::context::tls::enter_global::heca5eb3952d054ea
  37:        0x104f25873 - rustc_interface::queries::Queries::ongoing_codegen::h7dce60ab7d36285f
  38:        0x104d310ef - rustc_interface::interface::run_compiler_in_existing_thread_pool::h941b4802c24f9c9f
  39:        0x104bdd499 - scoped_tls::ScopedKey<T>::set::h19bc66da3554c573
  40:        0x104bdabf5 - rustc_ast::attr::with_globals::hc6f50c98b5adfc21
  41:        0x104be3490 - std::sys_common::backtrace::__rust_begin_short_backtrace::hbdd06d733493d2c5
  42:        0x104d32e9c - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbf0703c87d11ebad
  43:        0x10b14137e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h63df79cdef197011
  44:        0x10b173786 - std::sys::unix::thread::Thread::new::thread_start::ha540b6bb74f6bd73
  45:     0x7fff70077109 - _ZL12preoptimized

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (ce93331e2 2020-04-17) running on x86_64-apple-darwin

note: compiler flags: -C panic=abort -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `peer::coordinator::read_known_peers::{{closure}}#0`
end of query stack
error: aborting due to previous error

error: could not compile `MY_CRATE`.
