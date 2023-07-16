log
error: internal compiler error: src/librustc_mir/transform/generator.rs:1244: impossible case reached

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:904:9
stack backtrace:
   0:        0x1085a0f0f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5f94faac0175417e
   1:        0x1085dacfe - core::fmt::write::h6df182023d9a3b95
   2:        0x108591ed7 - std::io::Write::write_fmt::h01d91a9263b8b567
   3:        0x1085a5a2a - std::panicking::default_hook::{{closure}}::h224eeb9bf4f57784
   4:        0x1085a576c - std::panicking::default_hook::h075ef9f55ab1c87b
   5:        0x1020119d8 - rustc_driver::report_ice::h839668b6f6ee766f
   6:        0x1085a6145 - std::panicking::rust_panic_with_hook::h073aa5d47393da95
   7:        0x106090596 - std::panicking::begin_panic::hd440e6b088740f3c
   8:        0x105c85547 - rustc_errors::HandlerInner::bug::h9e8ac091d8db35b5
   9:        0x105c83f37 - rustc_errors::Handler::bug::h36ed9541a04885fc
  10:        0x105713570 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::hbddccb624994634d
  11:        0x105708076 - rustc_middle::ty::context::tls::with_opt::{{closure}}::ha492a72378e6816b
  12:        0x105707ffc - rustc_middle::ty::context::tls::with_opt::he38cce9a6e4c498c
  13:        0x105713478 - rustc_middle::util::bug::opt_span_bug_fmt::he731295bda299d43
  14:        0x10607120b - rustc_middle::util::bug::bug_fmt::h7ff1313c812cc524
  15:        0x104aef609 - <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass::ha35fc414cb2df94a
  16:        0x104bf082e - rustc_mir::transform::run_passes::hd827a4539f07877a
  17:        0x104bf18aa - rustc_mir::transform::run_optimization_passes::h2ce109868df11098
  18:        0x104bf1ab3 - rustc_mir::transform::optimized_mir::h35bf6652bd5a6764
  19:        0x104f57393 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute::h3cc19ec9d9ea6a15
  20:        0x104eacc39 - rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps::h64cb36ddad128472
  21:        0x104fb1548 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf1ebe5defb932a5c
  22:        0x104f7458c - rustc_query_system::query::plumbing::get_query::h6671a0528e0184a0
  23:        0x104f4c892 - rustc_metadata::rmeta::encoder::EncodeContext::encode_optimized_mir::h7c0bfeb60ffe9485
  24:        0x104f4d3ba - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h17a738155897d005
  25:        0x104fbc0ef - rustc_hir::intravisit::walk_expr::h5c5d575bc47a8b59
  26:        0x104f4d19c - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_expr::h17a738155897d005
  27:        0x104fbba45 - rustc_hir::intravisit::walk_fn::hbfa2c41a46c256db
  28:        0x104fbc82e - rustc_hir::intravisit::walk_item::h37fc828aa3395968
  29:        0x104f4d88f - <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item::h42b42f8ddccf1e2a
  30:        0x104f0bce6 - rustc_hir::hir::Crate::visit_all_item_likes::h142036d1dede4a40
  31:        0x104f53a9b - rustc_metadata::rmeta::encoder::encode_metadata_impl::h36ec3570fac288ed
  32:        0x104fd6461 - rustc_data_structures::sync::join::h1884a05288217c60
  33:        0x104febec3 - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::hae3a892f144578cd
  34:        0x1057bc554 - rustc_middle::ty::context::TyCtxt::encode_metadata::h753a833bb8acea4f
  35:        0x1023174cc - rustc_interface::passes::start_codegen::h2cca674743f6e9f4
  36:        0x1022803eb - rustc_middle::ty::context::tls::enter_global::h6a39e126a61c050f
  37:        0x1023621c3 - rustc_interface::queries::Queries::ongoing_codegen::h16dfba2279957de2
  38:        0x10216cdcf - rustc_interface::interface::run_compiler_in_existing_thread_pool::h28920202dc32f459
  39:        0x10201a229 - scoped_tls::ScopedKey<T>::set::h72ccfda3e4bc20d9
  40:        0x102017125 - rustc_ast::attr::with_globals::hb7cd589acd2b132f
  41:        0x10201f6e0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h8439916d1c647f2e
  42:        0x10216f66c - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd9041fd5dcffbd35
  43:        0x10858236e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hca40cbe33b61149d
  44:        0x1085b4776 - std::sys::unix::thread::Thread::new::thread_start::hbe6d5a098d8adb9e
  45:     0x7fff70077109 - _ZL12preoptimized

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (52fa23add 2020-04-18) running on x86_64-apple-darwin

note: compiler flags: -C panic=abort -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `peer::coordinator::read_known_peers::{{closure}}#0`
end of query stack
error: aborting due to previous error

error: could not compile `MY_CRATE`.
