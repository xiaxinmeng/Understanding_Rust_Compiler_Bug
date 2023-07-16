plain
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: internal compiler error: compiler/rustc_middle/src/ty/fast_reject.rs:240:31: unexpected impl_ty: [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f0a59cbd081 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8568fb9cf73ffe61
   1:     0x7f0a59d26b38 - core::fmt::write::h01a374123323c4c4
   1:     0x7f0a59d26b38 - core::fmt::write::h01a374123323c4c4
   2:     0x7f0a59cb1451 - std::io::Write::write_fmt::h9cf94f7ee0f74589
   3:     0x7f0a59cbce95 - std::sys_common::backtrace::print::hf66447743851f4fa
   4:     0x7f0a59cc0254 - std::panicking::default_hook::{{closure}}::hcce65c608141da85
   5:     0x7f0a59cc0008 - std::panicking::default_hook::hce2c3355d454fac7
   6:     0x7f0a5a76ab2c - rustc_driver_impl[51ae620fdc395faf]::install_ice_hook::{closure#0}
   7:     0x7f0a59cc0a4d - std::panicking::rust_panic_with_hook::h95c29c29d0102c3f
   8:     0x7f0a5d995c13 - std[4ccf602710e1e200]::panicking::begin_panic::<rustc_errors[f5492a8099896e81]::ExplicitBug>::{closure#0}
   9:     0x7f0a5d992e36 - std[4ccf602710e1e200]::sys_common::backtrace::__rust_end_short_backtrace::<std[4ccf602710e1e200]::panicking::begin_panic<rustc_errors[f5492a8099896e81]::ExplicitBug>::{closure#0}, !>
  10:     0x7f0a5a6ee126 - std[4ccf602710e1e200]::panicking::begin_panic::<rustc_errors[f5492a8099896e81]::ExplicitBug>
  11:     0x7f0a5d9ce3fa - <rustc_errors[f5492a8099896e81]::HandlerInner>::bug::<alloc[bc27d5164a9bfb01]::string::String>
  12:     0x7f0a5d9cddb4 - <rustc_errors[f5492a8099896e81]::Handler>::bug::<alloc[bc27d5164a9bfb01]::string::String>
  13:     0x7f0a5db6e837 - rustc_middle[515e59ca886bf23d]::util::bug::opt_span_bug_fmt::<rustc_span[22ba373f1707d53d]::span_encoding::Span>::{closure#0}
  14:     0x7f0a5db6d31c - rustc_middle[515e59ca886bf23d]::ty::context::tls::with_opt::<rustc_middle[515e59ca886bf23d]::util::bug::opt_span_bug_fmt<rustc_span[22ba373f1707d53d]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f0a5db6d2a4 - rustc_middle[515e59ca886bf23d]::ty::context::tls::with_context_opt::<rustc_middle[515e59ca886bf23d]::ty::context::tls::with_opt<rustc_middle[515e59ca886bf23d]::util::bug::opt_span_bug_fmt<rustc_span[22ba373f1707d53d]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f0a5db6e769 - rustc_middle[515e59ca886bf23d]::util::bug::opt_span_bug_fmt::<rustc_span[22ba373f1707d53d]::span_encoding::Span>
  17:     0x7f0a5a7019b5 - rustc_middle[515e59ca886bf23d]::util::bug::bug_fmt
  18:     0x7f0a5d9bfd3a - <rustc_middle[515e59ca886bf23d]::ty::fast_reject::DeepRejectCtxt>::types_may_unify
  19:     0x7f0a5d9bf8dc - <rustc_middle[515e59ca886bf23d]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  20:     0x7f0a5d9bf8dc - <rustc_middle[515e59ca886bf23d]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  21:     0x7f0a5d672a5c - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::assemble_candidates_from_caller_bounds
  22:     0x7f0a5d6717a9 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::assemble_candidates
  23:     0x7f0a5d667f05 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  24:     0x7f0a5d39e16a - <rustc_query_system[ca02a96f8717c136]::dep_graph::graph::DepGraph<rustc_middle[515e59ca886bf23d]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt, <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_middle[515e59ca886bf23d]::traits::select::SelectionCandidate>, rustc_middle[515e59ca886bf23d]::traits::SelectionError>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_middle[515e59ca886bf23d]::traits::select::SelectionCandidate>, rustc_middle[515e59ca886bf23d]::traits::SelectionError>>
  25:     0x7f0a5d684798 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::candidate_from_obligation
  26:     0x7f0a5d667b8c - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::select_from_obligation
  27:     0x7f0a5d6839f3 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::select::SelectionContext>::select
  28:     0x7f0a5d50024b - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  29:     0x7f0a5d4fe684 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor as rustc_data_structures[226bab0e9740ba66]::obligation_forest::ObligationProcessor>::process_obligation
  30:     0x7f0a5d5e9265 - <rustc_data_structures[226bab0e9740ba66]::obligation_forest::ObligationForest<rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillProcessor>
  31:     0x7f0a5d4f7684 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::fulfill::FulfillmentContext as rustc_infer[5ac755d9669a99aa]::traits::engine::TraitEngine>::select_where_possible
  32:     0x7f0a5c227976 - <rustc_infer[5ac755d9669a99aa]::infer::InferCtxt>::make_canonicalized_query_response::<rustc_middle[515e59ca886bf23d]::traits::query::NormalizationResult>
  33:     0x7f0a5c17ed14 - <rustc_trait_selection[dd94e78bf7210b6d]::traits::engine::ObligationCtxt>::make_canonicalized_query_response::<rustc_middle[515e59ca886bf23d]::traits::query::NormalizationResult>
  34:     0x7f0a5c233759 - <rustc_infer[5ac755d9669a99aa]::infer::InferCtxtBuilder as rustc_trait_selection[dd94e78bf7210b6d]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[515e59ca886bf23d]::ty::ParamEnvAnd<rustc_middle[515e59ca886bf23d]::ty::sty::AliasTy>, rustc_middle[515e59ca886bf23d]::traits::query::NormalizationResult, rustc_traits[776823d7fcad27ad]::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  35:     0x7f0a5c19e46e - rustc_traits[776823d7fcad27ad]::normalize_projection_ty::normalize_projection_ty
  36:     0x7f0a5c5a8024 - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::normalize_projection_ty, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  37:     0x7f0a5c8eb890 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::normalize_projection_ty, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  38:     0x7f0a5c78930b - rustc_query_impl[dccb39fc5ff2a194]::get_query::normalize_projection_ty
  39:     0x7f0a5d45b0d9 - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_middle[515e59ca886bf23d]::infer::canonical::Canonical<rustc_middle[515e59ca886bf23d]::ty::ParamEnvAnd<rustc_middle[515e59ca886bf23d]::ty::sty::AliasTy>>, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  40:     0x7f0a5d47fabe - <rustc_trait_selection[dd94e78bf7210b6d]::traits::query::normalize::QueryNormalizer as rustc_type_ir[76df902fb4b4283e]::fold::FallibleTypeFolder<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>>::try_fold_ty
  41:     0x7f0a5c12f66a - <rustc_infer[5ac755d9669a99aa]::infer::at::At as rustc_trait_selection[dd94e78bf7210b6d]::traits::query::normalize::QueryNormalizeExt>::query_normalize::<rustc_middle[515e59ca886bf23d]::ty::subst::GenericArg>
  42:     0x7f0a5c256f00 - <rustc_traits[776823d7fcad27ad]::normalize_erasing_regions::provide::{closure#0} as core[b17f3ca5674245f1]::ops::function::FnOnce<(rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt, rustc_middle[515e59ca886bf23d]::ty::ParamEnvAnd<rustc_middle[515e59ca886bf23d]::ty::subst::GenericArg>)>>::call_once
  43:     0x7f0a5c5ab70a - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[dccb39fc5ff2a194]::queries::try_normalize_generic_arg_after_erasing_regions, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>
  44:     0x7f0a5c918670 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::try_normalize_generic_arg_after_erasing_regions, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  45:     0x7f0a5c78a095 - rustc_query_impl[dccb39fc5ff2a194]::get_query::try_normalize_generic_arg_after_erasing_regions
  46:     0x7f0a5d9fb87c - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_middle[515e59ca886bf23d]::ty::ParamEnvAnd<rustc_middle[515e59ca886bf23d]::ty::subst::GenericArg>, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 8usize]>>>
  47:     0x7f0a5d9fb34d - <rustc_middle[515e59ca886bf23d]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  48:     0x7f0a5d9fb416 - <rustc_middle[515e59ca886bf23d]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_type_ir[76df902fb4b4283e]::fold::TypeFolder<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>>::fold_ty
  49:     0x7f0a5caf2841 - <&rustc_middle[515e59ca886bf23d]::ty::list::List<rustc_middle[515e59ca886bf23d]::ty::subst::GenericArg> as rustc_type_ir[76df902fb4b4283e]::fold::TypeFoldable<rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>>::try_fold_with::<rustc_middle[515e59ca886bf23d]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  50:     0x7f0a5cb061ef - <rustc_middle[515e59ca886bf23d]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[515e59ca886bf23d]::ty::sty::TraitRef>
  51:     0x7f0a5cb17cd5 - <&mut rustc_symbol_mangling[8ffc7b06c628e490]::v0::SymbolMangler as rustc_middle[515e59ca886bf23d]::ty::print::Printer>::print_impl_path
  52:     0x7f0a5cb172be - <&mut rustc_symbol_mangling[8ffc7b06c628e490]::v0::SymbolMangler as rustc_middle[515e59ca886bf23d]::ty::print::Printer>::print_def_path
  53:     0x7f0a5cb17609 - <&mut rustc_symbol_mangling[8ffc7b06c628e490]::v0::SymbolMangler as rustc_middle[515e59ca886bf23d]::ty::print::Printer>::print_def_path
  54:     0x7f0a5cb14f4f - rustc_symbol_mangling[8ffc7b06c628e490]::v0::mangle
  55:     0x7f0a5caeca89 - rustc_symbol_mangling[8ffc7b06c628e490]::symbol_name_provider
  56:     0x7f0a5c59cecd - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::symbol_name, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>
  57:     0x7f0a5c864fd4 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::symbol_name, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  58:     0x7f0a5c757bce - rustc_query_impl[dccb39fc5ff2a194]::get_query::symbol_name
  59:     0x7f0a5daa9092 - rustc_middle[515e59ca886bf23d]::ty::query::query_get_at::<rustc_query_system[ca02a96f8717c136]::query::caches::DefaultCache<rustc_middle[515e59ca886bf23d]::ty::instance::Instance, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 16usize]>>>
  60:     0x7f0a5da9302c - <rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem>::symbol_name
  61:     0x7f0a5b1e190c - <alloc[bc27d5164a9bfb01]::vec::Vec<(&rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem, rustc_middle[515e59ca886bf23d]::ty::SymbolName)> as alloc[bc27d5164a9bfb01]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem, rustc_middle[515e59ca886bf23d]::ty::SymbolName), core[b17f3ca5674245f1]::iter::adapters::map::Map<std[4ccf602710e1e200]::collections::hash::set::Iter<rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem>, rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::assert_symbols_are_distinct<std[4ccf602710e1e200]::collections::hash::set::Iter<rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  62:     0x7f0a5b21ed08 - rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::assert_symbols_are_distinct::<std[4ccf602710e1e200]::collections::hash::set::Iter<rustc_middle[515e59ca886bf23d]::mir::mono::MonoItem>>
  63:     0x7f0a5b1f9f76 - rustc_data_structures[226bab0e9740ba66]::sync::join::<rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::collect_and_partition_mono_items::{closure#0}::{closure#0}, rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::collect_and_partition_mono_items::{closure#0}::{closure#1}, &[rustc_middle[515e59ca886bf23d]::mir::mono::CodegenUnit], ()>
  64:     0x7f0a5b223863 - <rustc_session[d31b66e86b01534e]::session::Session>::time::<(&[rustc_middle[515e59ca886bf23d]::mir::mono::CodegenUnit], ()), rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::collect_and_partition_mono_items::{closure#0}>
  65:     0x7f0a5b21f0bf - rustc_monomorphize[64e8a49cfa2e84bc]::partitioning::collect_and_partition_mono_items
  66:     0x7f0a5c5aacee - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<rustc_query_system[ca02a96f8717c136]::query::plumbing::execute_job_non_incr<rustc_query_impl[dccb39fc5ff2a194]::queries::collect_and_partition_mono_items, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 24usize]>>::{closure#0}, rustc_middle[515e59ca886bf23d]::query::erase::Erased<[u8; 24usize]>>
  67:     0x7f0a5c90fb54 - rustc_query_system[ca02a96f8717c136]::query::plumbing::try_execute_query::<rustc_query_impl[dccb39fc5ff2a194]::queries::collect_and_partition_mono_items, rustc_query_impl[dccb39fc5ff2a194]::plumbing::QueryCtxt>
  68:     0x7f0a5c786575 - rustc_query_impl[dccb39fc5ff2a194]::get_query::collect_and_partition_mono_items
  69:     0x7f0a5aa8a8b7 - rustc_codegen_ssa[b7b08cb3b33cf95f]::base::codegen_crate::<rustc_codegen_llvm[5490c4a33fea8c8d]::LlvmCodegenBackend>
  70:     0x7f0a5aa47729 - <rustc_codegen_llvm[5490c4a33fea8c8d]::LlvmCodegenBackend as rustc_codegen_ssa[b7b08cb3b33cf95f]::traits::backend::CodegenBackend>::codegen_crate
  71:     0x7f0a5a941e73 - <rustc_session[d31b66e86b01534e]::session::Session>::time::<alloc[bc27d5164a9bfb01]::boxed::Box<dyn core[b17f3ca5674245f1]::any::Any>, rustc_interface[3675127c915af918]::passes::start_codegen::{closure#0}>
  72:     0x7f0a5a885515 - rustc_interface[3675127c915af918]::passes::start_codegen
  73:     0x7f0a5a850a4c - <std[4ccf602710e1e200]::thread::local::LocalKey<core[b17f3ca5674245f1]::cell::Cell<*const ()>>>::with::<rustc_middle[515e59ca886bf23d]::ty::context::tls::enter_context<<rustc_middle[515e59ca886bf23d]::ty::context::GlobalCtxt>::enter<<rustc_interface[3675127c915af918]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<alloc[bc27d5164a9bfb01]::boxed::Box<dyn core[b17f3ca5674245f1]::any::Any>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<alloc[bc27d5164a9bfb01]::boxed::Box<dyn core[b17f3ca5674245f1]::any::Any>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}, core[b17f3ca5674245f1]::result::Result<alloc[bc27d5164a9bfb01]::boxed::Box<dyn core[b17f3ca5674245f1]::any::Any>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
  74:     0x7f0a5a8703e9 - <rustc_interface[3675127c915af918]::queries::Queries>::ongoing_codegen
  75:     0x7f0a5a7c5dac - <rustc_interface[3675127c915af918]::interface::Compiler>::enter::<rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}::{closure#2}, core[b17f3ca5674245f1]::result::Result<core[b17f3ca5674245f1]::option::Option<rustc_interface[3675127c915af918]::queries::Linker>, rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
  76:     0x7f0a5a78329f - rustc_span[22ba373f1707d53d]::set_source_map::<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  77:     0x7f0a5a79746f - std[4ccf602710e1e200]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>
  78:     0x7f0a5a7892ae - <<std[4ccf602710e1e200]::thread::Builder>::spawn_unchecked_<rustc_interface[3675127c915af918]::util::run_in_thread_pool_with_globals<rustc_interface[3675127c915af918]::interface::run_compiler<core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>, rustc_driver_impl[51ae620fdc395faf]::run_compiler::{closure#1}>::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b17f3ca5674245f1]::result::Result<(), rustc_span[22ba373f1707d53d]::ErrorGuaranteed>>::{closure#1} as core[b17f3ca5674245f1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7f0a59ccda4e - std::sys::unix::thread::Thread::new::thread_start::hea5ef66f6d4d54ac
  80:     0x7f0a59a6cb43 - <unknown>
  81:     0x7f0a59afea00 - <unknown>
  82:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (100f0c79a 2023-05-11) running on x86_64-unknown-linux-gnu


note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [normalize_projection_ty] normalizing `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::Try>::Residual`
#1 [try_normalize_generic_arg_after_erasing_regions] normalizing `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::Try>::Residual`
#2 [symbol_name] computing the symbol for `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::FromResidual>::from_residual`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `rustc_codegen_llvm` (lib)
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:09:07
cat: /tmp/toolstate/toolstates.json: No such file or directory
