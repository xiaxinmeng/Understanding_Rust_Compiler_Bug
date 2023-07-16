
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling alloc v0.0.0 (/home/wesley/code/rust/rust3/src/liballoc)
error: internal compiler error: src/librustc/ty/subst.rs:565: type parameter `T/#1` (T/1) out of range when substituting (root type=Some(T)) substs=[[T]]

thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1052
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1428
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at ./src/liballoc/boxed.rs:1030
  11: rustc_driver::report_ice
             at src/librustc_driver/lib.rs:1157
  12: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:474
  13: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:397
  14: rustc_errors::HandlerInner::span_bug
             at ./<::std::macros::panic macros>:2
  15: rustc_errors::Handler::span_bug
             at ./src/librustc_errors/lib.rs:615
  16: rustc::util::bug::opt_span_bug_fmt::{{closure}}
             at src/librustc/util/bug.rs:35
  17: rustc::ty::context::tls::with_opt::{{closure}}
             at src/librustc/ty/context.rs:1806
  18: rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1758
  19: rustc::ty::context::tls::with_opt
             at src/librustc/ty/context.rs:1806
  20: rustc::util::bug::opt_span_bug_fmt
             at src/librustc/util/bug.rs:32
  21: rustc::util::bug::span_bug_fmt
             at src/librustc/util/bug.rs:23
  22: rustc::ty::subst::SubstFolder::ty_for_param
             at src/librustc/ty/subst.rs:565
  23: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
             at src/librustc/ty/subst.rs:517
  24: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::fold_with
             at ./src/librustc/ty/structural_impls.rs:927
  25: rustc::mir::tcx::_DERIVE_rustc_ty_fold_TypeFoldable_tcx_FOR_PlaceTy::<impl rustc::ty::fold::TypeFoldable for rustc::mir::tcx::PlaceTy>::super_fold_with
             at ./src/librustc/mir/tcx.rs:13
  26: rustc::ty::fold::TypeFoldable::fold_with
             at ./src/librustc/ty/fold.rs:48
  27: <T as rustc::ty::subst::Subst>::subst_spanned
             at ./src/librustc/ty/subst.rs:441
  28: rustc::ty::subst::Subst::subst
             at ./src/librustc/ty/subst.rs:421
  29: rustc_mir::transform::inline::Inliner::should_inline
             at src/librustc_mir/transform/inline.rs:327
  30: rustc_mir::transform::inline::Inliner::consider_optimizing
             at src/librustc_mir/transform/inline.rs:209
  31: rustc_mir::transform::inline::Inliner::run_pass
             at src/librustc_mir/transform/inline.rs:131
  32: <rustc_mir::transform::inline::Inline as rustc_mir::transform::MirPass>::run_pass
             at src/librustc_mir/transform/inline.rs:41
  33: rustc_mir::transform::run_passes::{{closure}}
             at src/librustc_mir/transform/mod.rs:167
  34: rustc_mir::transform::run_passes
             at src/librustc_mir/transform/mod.rs:174
  35: rustc_mir::transform::run_optimization_passes
             at src/librustc_mir/transform/mod.rs:272
  36: rustc_mir::transform::optimized_mir
             at src/librustc_mir/transform/mod.rs:343
  37: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:951
  38: rustc::ty::query::__query_compute::optimized_mir
             at ./src/librustc/ty/query/plumbing.rs:902
  39: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
             at ./src/librustc/ty/query/plumbing.rs:943
  40: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:286
  41: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1697
  42: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1682
  43: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1697
  44: rustc::dep_graph::graph::DepGraph::with_task_impl::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:286
  45: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1769
  46: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1758
  47: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1769
  48: rustc::dep_graph::graph::DepGraph::with_task_impl
             at ./src/librustc/dep_graph/graph.rs:282
  49: rustc::dep_graph::graph::DepGraph::with_task
             at ./src/librustc/dep_graph/graph.rs:209
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:557
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  52: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1697
  53: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1682
  54: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1697
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:278
  56: rustc::ty::context::tls::with_related_context::{{closure}}
             at ./src/librustc/ty/context.rs:1785
  57: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1769
  58: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1758
  59: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1769
  60: rustc::ty::context::tls::with_related_context
             at ./src/librustc/ty/context.rs:1782
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at ./src/librustc/ty/query/plumbing.rs:267
  62: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at ./src/librustc/ty/query/plumbing.rs:547
  63: rustc::ty::query::plumbing::with_diagnostics
             at ./src/librustc/ty/query/plumbing.rs:212
  64: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at ./src/librustc/ty/query/plumbing.rs:546
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at ./src/librustc/ty/query/plumbing.rs:433
  66: rustc::ty::query::TyCtxtAt::optimized_mir
             at ./src/librustc/ty/query/plumbing.rs:1057
  67: rustc::ty::query::<impl rustc::ty::context::TyCtxt>::optimized_mir
             at ./src/librustc/ty/query/plumbing.rs:1020
  68: rustc_metadata::rmeta::encoder::EncodeContext::encode_optimized_mir
             at src/librustc_metadata/rmeta/encoder.rs:966
  69: rustc_metadata::rmeta::encoder::EncodeContext::encode_info_for_impl_item
             at src/librustc_metadata/rmeta/encoder.rs:944
  70: rustc_metadata::rmeta::encoder::EncodeContext::encode_addl_info_for_item
             at src/librustc_metadata/rmeta/encoder.rs:1650
  71: <rustc_metadata::rmeta::encoder::EncodeContext as rustc_hir::intravisit::Visitor>::visit_item
             at src/librustc_metadata/rmeta/encoder.rs:1543
  72: <rustc_hir::intravisit::DeepVisitor<V> as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
             at ./src/librustc_hir/intravisit.rs:56
  73: rustc_hir::hir::Crate::visit_all_item_likes
             at ./src/librustc_hir/hir.rs:662
  74: rustc_metadata::rmeta::encoder::EncodeContext::encode_info_for_items
             at src/librustc_metadata/rmeta/encoder.rs:336
  75: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
             at src/librustc_metadata/rmeta/encoder.rs:448
  76: rustc_metadata::rmeta::encoder::encode_metadata::{{closure}}
             at src/librustc_metadata/rmeta/encoder.rs:1734
  77: rustc::dep_graph::graph::DepGraph::with_ignore::{{closure}}::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:167
  78: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1697
  79: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1682
  80: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1697
  81: rustc::dep_graph::graph::DepGraph::with_ignore::{{closure}}
             at ./src/librustc/dep_graph/graph.rs:167
  82: rustc::ty::context::tls::with_context::{{closure}}
             at ./src/librustc/ty/context.rs:1769
  83: rustc::ty::context::tls::with_context_opt
             at ./src/librustc/ty/context.rs:1758
  84: rustc::ty::context::tls::with_context
             at ./src/librustc/ty/context.rs:1769
  85: rustc::dep_graph::graph::DepGraph::with_ignore
             at ./src/librustc/dep_graph/graph.rs:164
  86: rustc_metadata::rmeta::encoder::encode_metadata
             at src/librustc_metadata/rmeta/encoder.rs:1716
  87: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
             at src/librustc_metadata/rmeta/decoder/cstore_impl.rs:537
  88: rustc::ty::context::TyCtxt::encode_metadata
             at src/librustc/ty/context.rs:1321
  89: rustc_interface::passes::encode_and_write_metadata
             at src/librustc_interface/passes.rs:914
  90: rustc_interface::passes::start_codegen
             at src/librustc_interface/passes.rs:961
  91: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}::{{closure}}
             at src/librustc_interface/queries.rs:280
  92: rustc_interface::passes::QueryContext::enter::{{closure}}
             at src/librustc_interface/passes.rs:696
  93: rustc::ty::context::tls::enter_global::{{closure}}
             at ./src/librustc/ty/context.rs:1720
  94: rustc::ty::context::tls::enter_context::{{closure}}
             at ./src/librustc/ty/context.rs:1697
  95: rustc::ty::context::tls::set_tlv
             at ./src/librustc/ty/context.rs:1682
  96: rustc::ty::context::tls::enter_context
             at ./src/librustc/ty/context.rs:1697
  97: rustc::ty::context::tls::enter_global
             at ./src/librustc/ty/context.rs:1720
  98: rustc_interface::passes::QueryContext::enter
             at src/librustc_interface/passes.rs:696
  99: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}
             at src/librustc_interface/queries.rs:274
 100: rustc_interface::queries::Query<T>::compute
             at src/librustc_interface/queries.rs:33
 101: rustc_interface::queries::Queries::ongoing_codegen
             at src/librustc_interface/queries.rs:272
 102: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:405
 103: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./src/librustc_interface/queries.rs:339
 104: rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:295
 105: rustc_interface::interface::run_compiler_in_existing_thread_pool
             at ./src/librustc_interface/interface.rs:186
 106: rustc_interface::interface::run_compiler::{{closure}}
             at ./src/librustc_interface/interface.rs:200
 107: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:155
 108: scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 109: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at ./src/librustc_interface/util.rs:151
 110: scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 111: syntax::attr::with_globals::{{closure}}
             at ./src/libsyntax/attr/mod.rs:44
 112: scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
 113: syntax::attr::with_globals
             at ./src/libsyntax/attr/mod.rs:44
 114: rustc_interface::util::spawn_thread_pool::{{closure}}
             at ./src/librustc_interface/util.rs:150
 115: rustc_interface::util::scoped_thread::{{closure}}
             at ./src/librustc_interface/util.rs:125
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.43.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=2 -C incremental -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=y --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `<<collections::vec_deque::VecDeque<T> as core::ops::Drop>::drop::Dropper<'a, T> as core::ops::Drop>::drop`
end of query stack
error: aborting due to previous error

error: could not compile `alloc`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/wesley/code/rust/rust3/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/wesley/code/rust/rust3/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /home/wesley/code/rust/rust3/build/bootstrap/debug/bootstrap build --stage 1 -i src/libstd
Build completed unsuccessfully in 0:00:05
