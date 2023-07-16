
warning: enum is never used: `Bug`
  --> test_wo_abi.rs:37:6
   |
37 | enum Bug<T = [(); Closure.call_once(&0) ]> {
   |      ^^^
   |
   = note: `#[warn(dead_code)]` on by default

thread 'rustc' panicked at 'DefId::expect_local: `DefId(1:7173 ~ std[ff7f]::process::{impl#46}::SUCCESS)` isn't local', /Users/bn/Documents/rust-local-fork/another-debug/compiler/rustc_span/src/def_id.rs:234:43
stack backtrace:
   0: rust_begin_unwind
             at ./library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at ./library/std/src/panicking.rs:460:5
   2: rustc_span::def_id::DefId::expect_local::{{closure}}
             at ./compiler/rustc_span/src/def_id.rs:234:43
   3: core::option::Option<T>::unwrap_or_else
             at ./library/core/src/option.rs:776:21
   4: rustc_span::def_id::DefId::expect_local
             at ./compiler/rustc_span/src/def_id.rs:234:9
   5: rustc_middle::ty::consts::kind::ConstKind::try_eval
             at ./compiler/rustc_middle/src/ty/consts/kind.rs:163:26
   6: rustc_middle::ty::consts::Const::eval
             at ./compiler/rustc_middle/src/ty/consts.rs:181:28
   7: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:370:25
   8: rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::consts::Const>::fold_with
             at ./compiler/rustc_middle/src/ty/structural_impls.rs:1054:9
   9: rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with
             at ./compiler/rustc_middle/src/mir/type_foldable.rs:358:53
  10: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:376:9
  11: rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::fold_with
             at ./compiler/rustc_middle/src/mir/type_foldable.rs:353:9
  12: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
             at ./compiler/rustc_trait_selection/src/traits/query/normalize.rs:91:22
  13: rustc_traits::normalize_erasing_regions::normalize_after_erasing_regions::{{closure}}
             at ./compiler/rustc_traits/src/normalize_erasing_regions.rs:35:15
  14: rustc_infer::infer::InferCtxtBuilder::enter
             at ./compiler/rustc_infer/src/infer/mod.rs:620:9
  15: rustc_traits::normalize_erasing_regions::normalize_after_erasing_regions
             at ./compiler/rustc_traits/src/normalize_erasing_regions.rs:33:5
  16: rustc_traits::normalize_erasing_regions::provide::{{closure}}
             at ./compiler/rustc_traits/src/normalize_erasing_regions.rs:21:13
  17: core::ops::function::FnOnce::call_once
             at ./library/core/src/ops/function.rs:227:5
  18: rustc_query_system::query::plumbing::try_execute_query::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:462:55
  19: stacker::maybe_grow
             at /Users/bn/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  20: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  21: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:137:17
  22: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1782:50
  23: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1766:9
  24: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1782:9
  25: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:136:13
  26: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1826:13
  27: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1810:40
  28: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1799:22
  29: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1810:9
  30: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1823:9
  31: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:125:9
  32: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:462:22
  33: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:730:5
  34: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:853:17
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
             at ./compiler/rustc_query_impl/src/plumbing.rs:563:17
  36: rustc_middle::ty::query::TyCtxtAt::normalize_mir_const_after_erasing_regions
             at ./compiler/rustc_middle/src/ty/query.rs:203:17
  37: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::normalize_mir_const_after_erasing_regions
             at ./compiler/rustc_middle/src/ty/query.rs:184:17
  38: <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
             at ./compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:118:9
  39: rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::fold_with
             at ./compiler/rustc_middle/src/mir/type_foldable.rs:353:9
  40: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
             at ./compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:38:13
  41: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions
             at ./compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:82:9
  42: rustc_middle::ty::instance::Instance::subst_mir_and_normalize_erasing_regions
             at ./compiler/rustc_middle/src/ty/instance.rs:557:13
  43: rustc_mir::monomorphize::collector::MirNeighborCollector::monomorphize
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:619:9
  44: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_constant
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:718:23
  45: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_operand
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:826:9
  46: rustc_middle::mir::visit::Visitor::super_terminator
             at ./compiler/rustc_middle/src/mir/visit.rs:542:29
  47: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:822:9
  48: rustc_middle::mir::visit::Visitor::super_basic_block_data
             at ./compiler/rustc_middle/src/mir/visit.rs:318:21
  49: rustc_middle::mir::visit::Visitor::visit_basic_block_data
             at ./compiler/rustc_middle/src/mir/visit.rs:84:17
  50: rustc_middle::mir::visit::Visitor::super_body
             at ./compiler/rustc_middle/src/mir/visit.rs:261:21
  51: rustc_middle::mir::visit::Visitor::visit_body
             at ./compiler/rustc_middle/src/mir/visit.rs:78:17
  52: rustc_mir::monomorphize::collector::collect_neighbours
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:1397:5
  53: rustc_mir::monomorphize::collector::collect_items_rec::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:426:17
  54: stacker::maybe_grow
             at /Users/bn/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  55: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  56: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:425:13
  57: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:467:9
  58: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:467:9
  59: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:467:9
  60: rustc_mir::monomorphize::collector::collect_items_rec
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:467:9
  61: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:307:17
  62: core::iter::traits::iterator::Iterator::for_each::call::{{closure}}
             at ./library/core/src/iter/traits/iterator.rs:730:29
  63: core::iter::traits::iterator::Iterator::fold
             at ./library/core/src/iter/traits/iterator.rs:2170:21
  64: core::iter::traits::iterator::Iterator::for_each
             at ./library/core/src/iter/traits/iterator.rs:733:9
  65: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:305:13
  66: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:611:9
  67: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:16:9
  68: rustc_mir::monomorphize::collector::collect_crate_mono_items
             at ./compiler/rustc_mir/src/monomorphize/collector.rs:304:9
  69: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
             at ./compiler/rustc_mir/src/monomorphize/partitioning/mod.rs:344:33
  70: rustc_query_system::query::plumbing::try_execute_query::{{closure}}
             at ./compiler/rustc_query_system/src/query/plumbing.rs:462:55
  71: stacker::maybe_grow
             at /Users/bn/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.12/src/lib.rs:55:9
  72: rustc_data_structures::stack::ensure_sufficient_stack
             at ./compiler/rustc_data_structures/src/stack.rs:16:5
  73: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:137:17
  74: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1782:50
  75: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1766:9
  76: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1782:9
  77: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::{{closure}}
             at ./compiler/rustc_query_impl/src/plumbing.rs:136:13
  78: rustc_middle::ty::context::tls::with_related_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1826:13
  79: rustc_middle::ty::context::tls::with_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1810:40
  80: rustc_middle::ty::context::tls::with_context_opt
             at ./compiler/rustc_middle/src/ty/context.rs:1799:22
  81: rustc_middle::ty::context::tls::with_context
             at ./compiler/rustc_middle/src/ty/context.rs:1810:9
  82: rustc_middle::ty::context::tls::with_related_context
             at ./compiler/rustc_middle/src/ty/context.rs:1823:9
  83: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query
             at ./compiler/rustc_query_impl/src/plumbing.rs:125:9
  84: rustc_query_system::query::plumbing::try_execute_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:462:22
  85: rustc_query_system::query::plumbing::get_query_impl
             at ./compiler/rustc_query_system/src/query/plumbing.rs:730:5
  86: rustc_query_system::query::plumbing::get_query
             at ./compiler/rustc_query_system/src/query/plumbing.rs:853:17
  87: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
             at ./compiler/rustc_query_impl/src/plumbing.rs:563:17
  88: rustc_middle::ty::query::TyCtxtAt::collect_and_partition_mono_items
             at ./compiler/rustc_middle/src/ty/query.rs:203:17
  89: rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::collect_and_partition_mono_items
             at ./compiler/rustc_middle/src/ty/query.rs:184:17
  90: rustc_codegen_ssa::base::codegen_crate
             at ./compiler/rustc_codegen_ssa/src/base.rs:507:25
  91: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at ./compiler/rustc_codegen_llvm/src/lib.rs:256:18
  92: rustc_interface::passes::start_codegen::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:1053:9
  93: rustc_data_structures::profiling::VerboseTimingGuard::run
             at ./compiler/rustc_data_structures/src/profiling.rs:611:9
  94: rustc_session::utils::<impl rustc_session::session::Session>::time
             at ./compiler/rustc_session/src/utils.rs:16:9
  95: rustc_interface::passes::start_codegen
             at ./compiler/rustc_interface/src/passes.rs:1052:19
  96: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:254:20
  97: rustc_interface::passes::QueryContext::enter::{{closure}}
             at ./compiler/rustc_interface/src/passes.rs:779:42
  98: rustc_middle::ty::context::tls::enter_context::{{closure}}
             at ./compiler/rustc_middle/src/ty/context.rs:1782:50
  99: rustc_middle::ty::context::tls::set_tlv
             at ./compiler/rustc_middle/src/ty/context.rs:1766:9
 100: rustc_middle::ty::context::tls::enter_context
             at ./compiler/rustc_middle/src/ty/context.rs:1782:9
 101: rustc_interface::passes::QueryContext::enter
             at ./compiler/rustc_interface/src/passes.rs:779:9
 102: rustc_interface::queries::Queries::ongoing_codegen::{{closure}}
             at ./compiler/rustc_interface/src/queries.rs:245:13
 103: rustc_interface::queries::Query<T>::compute
             at ./compiler/rustc_interface/src/queries.rs:38:28
 104: rustc_interface::queries::Queries::ongoing_codegen
             at ./compiler/rustc_interface/src/queries.rs:243:9
 105: rustc_driver::run_compiler::{{closure}}::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:407:13
 106: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at ./compiler/rustc_interface/src/queries.rs:390:19
 107: rustc_driver::run_compiler::{{closure}}
             at ./compiler/rustc_driver/src/lib.rs:312:22
 108: rustc_interface::interface::create_compiler_and_run::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:209:13
 109: rustc_span::with_source_map
             at ./compiler/rustc_span/src/lib.rs:913:5
 110: rustc_interface::interface::create_compiler_and_run
             at ./compiler/rustc_interface/src/interface.rs:203:5
 111: rustc_interface::interface::run_compiler::{{closure}}
             at ./compiler/rustc_interface/src/interface.rs:225:12
 112: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:157:13
 113: scoped_tls::ScopedKey<T>::set
             at /Users/bn/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 114: rustc_span::create_session_globals_then
             at ./compiler/rustc_span/src/lib.rs:107:5
 115: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:155:9
 116: rustc_interface::util::scoped_thread::{{closure}}
             at ./compiler/rustc_interface/src/util.rs:130:24
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-dev running on x86_64-apple-darwin

query stack during panic:
#0 [normalize_mir_const_after_erasing_regions] normalizing `std::process::ExitCode::SUCCESS`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
warning: 1 warning emitted
