
> 
RUST_BACKTRACE=1 rustc +stage1 -Zinstrument-coverage -Ztreat-err-as-bug src/test/ui/issues/issue-33287.rs
error: this operation will panic at runtime
 --> src/test/ui/issues/issue-33287.rs:7:17
  |
7 |     let range = A[1]..;
  |                 ^^^^ index out of bounds: the length is 1 but the index is 1
  |
  = note: `#[deny(unconditional_panic)]` on by default

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
             at ./library/std/src/panicking.rs:519:12
   1: rustc_errors::HandlerInner::panic_if_treat_err_as_bug
             at ./compiler/rustc_errors/src/lib.rs:990:27
   2: rustc_errors::HandlerInner::bump_err_count
             at ./compiler/rustc_errors/src/lib.rs:980:9
   3: rustc_errors::HandlerInner::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:796:13
   4: rustc_errors::Handler::emit_diagnostic
             at ./compiler/rustc_errors/src/lib.rs:715:9
   5: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at ./compiler/rustc_errors/src/diagnostic_builder.rs:101:9
   6: rustc_mir::transform::const_prop::ConstPropagator::report_assert_as_lint::{{closure}}
             at ./compiler/rustc_mir/src/transform/const_prop.rs:522:13
   7: core::ops::function::FnOnce::call_once{{vtable.shim}}
             at ./library/core/src/ops/function.rs:227:5
   8: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
             at ./library/alloc/src/boxed.rs:1328:9
   9: rustc_middle::lint::struct_lint_level::struct_lint_level_impl
             at ./compiler/rustc_middle/src/lint.rs:362:9
  10: rustc_middle::lint::struct_lint_level
             at ./compiler/rustc_middle/src/lint.rs:364:5
  11: rustc_middle::ty::context::TyCtxt::struct_span_lint_hir
             at ./compiler/rustc_middle/src/ty/context.rs:2575:9
  12: rustc_mir::transform::const_prop::ConstPropagator::report_assert_as_lint
             at ./compiler/rustc_mir/src/transform/const_prop.rs:519:9
  13: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_terminator
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1216:29
  14: <rustc_mir::transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
             at ./compiler/rustc_mir/src/transform/const_prop.rs:1050:13
  15: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
             at ./compiler/rustc_mir/src/transform/const_prop.rs:150:9
  16: rustc_mir::transform::run_passes::{{closure}}
             at ./compiler/rustc_mir/src/transform/mod.rs:186:9
  17: rustc_mir::transform::run_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:202:13
  18: rustc_mir::transform::run_optimization_passes
             at ./compiler/rustc_mir/src/transform/mod.rs:446:5
  19: rustc_mir::transform::inner_optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:486:5
  20: rustc_mir::transform::optimized_mir
             at ./compiler/rustc_mir/src/transform/mod.rs:462:25
  21: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  23: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
  24: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  25: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  26: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  27: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  28: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  29: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  30: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  31: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  32: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  33: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  34: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  35: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  36: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  37: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  38: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  39: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  40: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  41: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  42: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  43: <rustc_query_system::query::caches::DefaultCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:114:79
  44: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  45: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  46: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  47: rustc_middle::ty::query::TyCtxtAt::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  48: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::optimized_mir
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  49: rustc_mir::transform::coverage::query::covered_file_name
             at ./compiler/rustc_mir/src/transform/coverage/query.rs:132:20
  50: rustc_mir::transform::coverage::query::provide::{{closure}}
             at ./compiler/rustc_mir/src/transform/coverage/query.rs:14:49
  51: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
  52: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::covered_file_name>::compute
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:377:17
  53: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  54: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}::{{closure}}
  55: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:74
  56: stacker::maybe_grow
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  57: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  58: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:73:17
  59: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  60: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  61: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  62: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:72:13
  63: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1777:13
  64: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1761:40
  65: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1750:22
  66: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1761:9
  67: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1774:9
  68: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:61:9
  69: rustc_query_system::query::plumbing::force_query_with_job::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:598:9
  70: rustc_query_system::query::plumbing::with_diagnostics
             at ./compiler/rustc_query_system/src/query/plumbing.rs:302:18
  71: rustc_query_system::query::plumbing::force_query_with_job
             at ./compiler/rustc_query_system/src/query/plumbing.rs:597:51
  72: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:426:16
  73: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:645:23
  74: <rustc_query_system::query::caches::ArenaCache<K,V> as rustc_query_system::query::caches::QueryCache>::lookup
             at ./compiler/rustc_query_system/src/query/caches.rs:198:13
  75: rustc_query_system::query::plumbing::try_get_cached
             at ./compiler/rustc_query_system/src/query/plumbing.rs:379:5
  76: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:637:5
  77: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:739:5
  78: rustc_middle::ty::query::TyCtxtAt::covered_file_name
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:487:17
  79: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::covered_file_name
             at ./compiler/rustc_middle/src/ty/query/plumbing.rs:448:17
  80: rustc_codegen_llvm::coverageinfo::mapgen::add_unreachable_coverage
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs:292:49
  81: rustc_codegen_llvm::coverageinfo::mapgen::finalize
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs:51:5
  82: rustc_codegen_llvm::coverageinfo::<impl rustc_codegen_ssa::traits::coverageinfo::CoverageInfoMethods for rustc_codegen_llvm::context::CodegenCx>::coverageinfo_finalize
             at ./compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs:46:9
  83: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
             at ./compiler/rustc_codegen_llvm/src/base.rs:149:17
  84: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:301:14
  85: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:200:9
  86: rustc_codegen_llvm::base::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/base.rs:104:9
  87: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/lib.rs:106:9
  88: rustc_codegen_ssa::base::codegen_crate
             at ./compiler/rustc_codegen_ssa/src/base.rs:642:38
  89: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at ./compiler/rustc_codegen_llvm/src/lib.rs:267:18
  90: rustc_interface::passes::start_codegen::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:996:9
  91: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:570:9
  92: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:9:9
  93: rustc_interface::passes::start_codegen
             at ./compiler/rustc_interface/src/passes.rs:995:19
  94: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:282:20
  95: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:725:42
  96: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1733:50
  97: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1717:9
  98: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1733:9
  99: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:725:9
 100: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:273:13
 101: rustc_interface::queries::Query<T>::compute
             at ./compiler/rustc_interface/src/queries.rs:35:28
 102: rustc_interface::queries::Queries::ongoing_codegen
             at ./compiler/rustc_interface/src/queries.rs:271:9
 103: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:446:13
 104: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:413:19
 105: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:341:22
 106: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:196:13
 107: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:765:5
 108: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:190:5
 109: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:212:12
 110: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:152:13
 111: scoped_tls::ScopedKey<T>::set
             at /usr/local/google/home/tmandry/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 112: rustc_span::with_session_globals
             at ./compiler/rustc_span/src/lib.rs:94:5
 113: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:150:9
 114: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:125:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage -Z treat-err-as-bug

query stack during panic:
#0 [optimized_mir] optimizing MIR for `test`
#1 [covered_file_name] retrieving the covered file name, if instrumented, for `test`
end of query stack
