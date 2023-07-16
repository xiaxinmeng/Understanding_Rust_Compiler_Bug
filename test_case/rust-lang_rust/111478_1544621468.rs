plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
[RUSTC-TIMING] rustc_incremental test:false 12.780
[RUSTC-TIMING] rustc_transmute test:false 7.174
error: internal compiler error: compiler/rustc_middle/src/ty/fast_reject.rs:240:31: unexpected impl_ty: [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/compiler/rustc_errors/src/lib.rs:1650:9
stack backtrace:
   0:     0x7f3b2c07c373 - std::backtrace_rs::backtrace::libunwind::trace::hfcc030a743eddb7b
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f3b2c07c373 - std::backtrace_rs::backtrace::trace_unsynchronized::h2a28d8a3a62380e7
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f3b2c07c373 - std::sys_common::backtrace::_print_fmt::hf52c8c8741f11a1b
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f3b2c07c373 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h00f4aa2bef2728c1
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f3b2c0dce8f - core::fmt::rt::Argument::fmt::hb007c02558a7e048
   5:     0x7f3b2c0dce8f - core::fmt::write::h889e7642f4b05be0
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/core/src/fmt/mod.rs:1094:21
   6:     0x7f3b2c06efa5 - std::io::Write::write_fmt::h58c68ec39c4961cd
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/io/mod.rs:1712:15
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/io/mod.rs:1712:15
   7:     0x7f3b2c07c185 - std::sys_common::backtrace::_print::h4f1592d0554d785d
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7f3b2c07c185 - std::sys_common::backtrace::print::hc53a72c79a120141
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7f3b2c07eefe - std::panicking::default_hook::{{closure}}::hff91d36be240d353
  10:     0x7f3b2c07eca5 - std::panicking::default_hook::hdfdda6545880dc86
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/panicking.rs:288:9
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/panicking.rs:288:9
  11:     0x7f3b2935ccbc - rustc_driver_impl[73c87b44fd5cc2bd]::install_ice_hook::{closure#0}
  12:     0x7f3b2c07f6f4 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc96d96de54d8e79e
  13:     0x7f3b2c07f6f4 - std::panicking::rust_panic_with_hook::h1cd2048e84fdeb5f
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/panicking.rs:695:13
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/panicking.rs:695:13
  14:     0x7f3b2b78f363 - std[655df35e4a40b28f]::panicking::begin_panic::<rustc_errors[67e98f4ee719b666]::ExplicitBug>::{closure#0}
  15:     0x7f3b2b78d0a6 - std[655df35e4a40b28f]::sys_common::backtrace::__rust_end_short_backtrace::<std[655df35e4a40b28f]::panicking::begin_panic<rustc_errors[67e98f4ee719b666]::ExplicitBug>::{closure#0}, !>
  16:     0x7f3b2b89c1f6 - std[655df35e4a40b28f]::panicking::begin_panic::<rustc_errors[67e98f4ee719b666]::ExplicitBug>
  17:     0x7f3b2b7a1d8a - <rustc_errors[67e98f4ee719b666]::HandlerInner>::bug::<alloc[30b2db67739fbdd3]::string::String>
  18:     0x7f3b2b7a1a0b - <rustc_errors[67e98f4ee719b666]::Handler>::bug::<alloc[30b2db67739fbdd3]::string::String>
  19:     0x7f3b2b8fc2a7 - rustc_middle[9c6cad357f4a2726]::util::bug::opt_span_bug_fmt::<rustc_span[1bf2f546da16ba0]::span_encoding::Span>::{closure#0}
  20:     0x7f3b2b8f866c - rustc_middle[9c6cad357f4a2726]::ty::context::tls::with_opt::<rustc_middle[9c6cad357f4a2726]::util::bug::opt_span_bug_fmt<rustc_span[1bf2f546da16ba0]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7f3b2b8f861e - rustc_middle[9c6cad357f4a2726]::ty::context::tls::with_context_opt::<rustc_middle[9c6cad357f4a2726]::ty::context::tls::with_opt<rustc_middle[9c6cad357f4a2726]::util::bug::opt_span_bug_fmt<rustc_span[1bf2f546da16ba0]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7f3b2b8fc1d9 - rustc_middle[9c6cad357f4a2726]::util::bug::opt_span_bug_fmt::<rustc_span[1bf2f546da16ba0]::span_encoding::Span>
  23:     0x7f3b2b8fc155 - rustc_middle[9c6cad357f4a2726]::util::bug::bug_fmt
  24:     0x7f3b2b75c3b1 - <rustc_middle[9c6cad357f4a2726]::ty::fast_reject::DeepRejectCtxt>::types_may_unify
  25:     0x7f3b2b75bf65 - <rustc_middle[9c6cad357f4a2726]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  26:     0x7f3b2b75bf65 - <rustc_middle[9c6cad357f4a2726]::ty::fast_reject::DeepRejectCtxt>::substs_refs_may_unify
  27:     0x7f3b2b419eb6 - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::assemble_candidates_from_caller_bounds
  28:     0x7f3b2b4195e0 - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::assemble_candidates
  29:     0x7f3b2b4147b0 - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  30:     0x7f3b2b3ca039 - <rustc_query_system[7c49f14a8fbd6785]::dep_graph::graph::DepGraph<rustc_middle[9c6cad357f4a2726]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt, <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<core[51e91c5fbe619c17]::option::Option<rustc_middle[9c6cad357f4a2726]::traits::select::SelectionCandidate>, rustc_middle[9c6cad357f4a2726]::traits::SelectionError>>::{closure#0}, core[51e91c5fbe619c17]::result::Result<core[51e91c5fbe619c17]::option::Option<rustc_middle[9c6cad357f4a2726]::traits::select::SelectionCandidate>, rustc_middle[9c6cad357f4a2726]::traits::SelectionError>>
  31:     0x7f3b2b42263f - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::candidate_from_obligation
  32:     0x7f3b2b4145f8 - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::select_from_obligation
  33:     0x7f3b2b422392 - <rustc_trait_selection[f47807a20da33a90]::traits::select::SelectionContext>::select
  34:     0x7f3b2b44c4c8 - <rustc_trait_selection[f47807a20da33a90]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  35:     0x7f3b2b44b42f - <rustc_trait_selection[f47807a20da33a90]::traits::fulfill::FulfillProcessor as rustc_data_structures[7f987559c70ff1c3]::obligation_forest::ObligationProcessor>::process_obligation
  36:     0x7f3b2b4e43c9 - <rustc_data_structures[7f987559c70ff1c3]::obligation_forest::ObligationForest<rustc_trait_selection[f47807a20da33a90]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[f47807a20da33a90]::traits::fulfill::FulfillProcessor>
  37:     0x7f3b2b4474b6 - <rustc_trait_selection[f47807a20da33a90]::traits::fulfill::FulfillmentContext as rustc_infer[2056ae862679510e]::traits::engine::TraitEngine>::select_where_possible
  38:     0x7f3b2a674ff3 - <rustc_infer[2056ae862679510e]::infer::InferCtxt>::make_canonicalized_query_response::<rustc_middle[9c6cad357f4a2726]::traits::query::NormalizationResult>
  39:     0x7f3b2a679a2d - <rustc_infer[2056ae862679510e]::infer::InferCtxtBuilder as rustc_trait_selection[f47807a20da33a90]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[9c6cad357f4a2726]::ty::ParamEnvAnd<rustc_middle[9c6cad357f4a2726]::ty::sty::AliasTy>, rustc_middle[9c6cad357f4a2726]::traits::query::NormalizationResult, rustc_traits[8a56acc5ada12deb]::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  40:     0x7f3b2a5f6c94 - rustc_traits[8a56acc5ada12deb]::normalize_projection_ty::normalize_projection_ty
  41:     0x7f3b2aa043e7 - rustc_query_system[7c49f14a8fbd6785]::query::plumbing::try_execute_query::<rustc_query_impl[4dd0e81501809c4f]::queries::normalize_projection_ty, rustc_query_impl[4dd0e81501809c4f]::plumbing::QueryCtxt>
  42:     0x7f3b2a7a8784 - rustc_query_impl[4dd0e81501809c4f]::get_query::normalize_projection_ty
  43:     0x7f3b2b57c939 - rustc_middle[9c6cad357f4a2726]::ty::query::query_get_at::<rustc_query_system[7c49f14a8fbd6785]::query::caches::DefaultCache<rustc_middle[9c6cad357f4a2726]::infer::canonical::Canonical<rustc_middle[9c6cad357f4a2726]::ty::ParamEnvAnd<rustc_middle[9c6cad357f4a2726]::ty::sty::AliasTy>>, rustc_middle[9c6cad357f4a2726]::query::erase::Erased<[u8; 8usize]>>>
  44:     0x7f3b2b59185c - <rustc_trait_selection[f47807a20da33a90]::traits::query::normalize::QueryNormalizer as rustc_type_ir[136c85a5720d5b22]::fold::FallibleTypeFolder<rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt>>::try_fold_ty
  45:     0x7f3b2a558617 - <rustc_infer[2056ae862679510e]::infer::at::At as rustc_trait_selection[f47807a20da33a90]::traits::query::normalize::QueryNormalizeExt>::query_normalize::<rustc_middle[9c6cad357f4a2726]::ty::subst::GenericArg>
  46:     0x7f3b2a5b104d - <rustc_traits[8a56acc5ada12deb]::normalize_erasing_regions::provide::{closure#0} as core[51e91c5fbe619c17]::ops::function::FnOnce<(rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt, rustc_middle[9c6cad357f4a2726]::ty::ParamEnvAnd<rustc_middle[9c6cad357f4a2726]::ty::subst::GenericArg>)>>::call_once
  47:     0x7f3b2aa248d8 - rustc_query_system[7c49f14a8fbd6785]::query::plumbing::try_execute_query::<rustc_query_impl[4dd0e81501809c4f]::queries::try_normalize_generic_arg_after_erasing_regions, rustc_query_impl[4dd0e81501809c4f]::plumbing::QueryCtxt>
  48:     0x7f3b2a7a8ba6 - rustc_query_impl[4dd0e81501809c4f]::get_query::try_normalize_generic_arg_after_erasing_regions
  49:     0x7f3b2b7e1a1a - rustc_middle[9c6cad357f4a2726]::ty::query::query_get_at::<rustc_query_system[7c49f14a8fbd6785]::query::caches::DefaultCache<rustc_middle[9c6cad357f4a2726]::ty::ParamEnvAnd<rustc_middle[9c6cad357f4a2726]::ty::subst::GenericArg>, rustc_middle[9c6cad357f4a2726]::query::erase::Erased<[u8; 8usize]>>>
  50:     0x7f3b2b7e1233 - <rustc_middle[9c6cad357f4a2726]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_type_ir[136c85a5720d5b22]::fold::TypeFolder<rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt>>::fold_ty
  51:     0x7f3b2acafc65 - <&rustc_middle[9c6cad357f4a2726]::ty::list::List<rustc_middle[9c6cad357f4a2726]::ty::subst::GenericArg> as rustc_type_ir[136c85a5720d5b22]::fold::TypeFoldable<rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt>>::try_fold_with::<rustc_middle[9c6cad357f4a2726]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  52:     0x7f3b2acc2aa6 - <rustc_middle[9c6cad357f4a2726]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[9c6cad357f4a2726]::ty::sty::TraitRef>
  53:     0x7f3b2acc56fc - <&mut rustc_symbol_mangling[cad3e42ab8d672bd]::v0::SymbolMangler as rustc_middle[9c6cad357f4a2726]::ty::print::Printer>::print_impl_path
  54:     0x7f3b2acc5126 - <&mut rustc_symbol_mangling[cad3e42ab8d672bd]::v0::SymbolMangler as rustc_middle[9c6cad357f4a2726]::ty::print::Printer>::print_def_path
  55:     0x7f3b2acc52eb - <&mut rustc_symbol_mangling[cad3e42ab8d672bd]::v0::SymbolMangler as rustc_middle[9c6cad357f4a2726]::ty::print::Printer>::print_def_path
  56:     0x7f3b2acc3b09 - rustc_symbol_mangling[cad3e42ab8d672bd]::v0::mangle
  57:     0x7f3b2acb4ce4 - rustc_symbol_mangling[cad3e42ab8d672bd]::symbol_name_provider
  58:     0x7f3b2a9a08cc - rustc_query_system[7c49f14a8fbd6785]::query::plumbing::try_execute_query::<rustc_query_impl[4dd0e81501809c4f]::queries::symbol_name, rustc_query_impl[4dd0e81501809c4f]::plumbing::QueryCtxt>
  59:     0x7f3b2a799bd5 - rustc_query_impl[4dd0e81501809c4f]::get_query::symbol_name
  60:     0x7f3b2b7e1c72 - rustc_middle[9c6cad357f4a2726]::ty::query::query_get_at::<rustc_query_system[7c49f14a8fbd6785]::query::caches::DefaultCache<rustc_middle[9c6cad357f4a2726]::ty::instance::Instance, rustc_middle[9c6cad357f4a2726]::query::erase::Erased<[u8; 16usize]>>>
  61:     0x7f3b2b7dced3 - <rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem>::symbol_name
  62:     0x7f3b29aa9c8e - <alloc[30b2db67739fbdd3]::vec::Vec<(&rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem, rustc_middle[9c6cad357f4a2726]::ty::SymbolName)> as alloc[30b2db67739fbdd3]::vec::spec_from_iter::SpecFromIter<(&rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem, rustc_middle[9c6cad357f4a2726]::ty::SymbolName), core[51e91c5fbe619c17]::iter::adapters::map::Map<std[655df35e4a40b28f]::collections::hash::set::Iter<rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem>, rustc_monomorphize[2c9524d22b0aed47]::partitioning::assert_symbols_are_distinct<std[655df35e4a40b28f]::collections::hash::set::Iter<rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem>>::{closure#0}>>>::from_iter
  63:     0x7f3b29abd328 - rustc_monomorphize[2c9524d22b0aed47]::partitioning::assert_symbols_are_distinct::<std[655df35e4a40b28f]::collections::hash::set::Iter<rustc_middle[9c6cad357f4a2726]::mir::mono::MonoItem>>
  64:     0x7f3b29ad2663 - rustc_data_structures[7f987559c70ff1c3]::sync::join::<rustc_monomorphize[2c9524d22b0aed47]::partitioning::collect_and_partition_mono_items::{closure#0}::{closure#0}, rustc_monomorphize[2c9524d22b0aed47]::partitioning::collect_and_partition_mono_items::{closure#0}::{closure#1}, &[rustc_middle[9c6cad357f4a2726]::mir::mono::CodegenUnit], ()>
  65:     0x7f3b29ad38b3 - <rustc_session[fb51f884770b4300]::session::Session>::time::<(&[rustc_middle[9c6cad357f4a2726]::mir::mono::CodegenUnit], ()), rustc_monomorphize[2c9524d22b0aed47]::partitioning::collect_and_partition_mono_items::{closure#0}>
  66:     0x7f3b29abd6ba - rustc_monomorphize[2c9524d22b0aed47]::partitioning::collect_and_partition_mono_items
  67:     0x7f3b2aa1e60f - rustc_query_system[7c49f14a8fbd6785]::query::plumbing::try_execute_query::<rustc_query_impl[4dd0e81501809c4f]::queries::collect_and_partition_mono_items, rustc_query_impl[4dd0e81501809c4f]::plumbing::QueryCtxt>
  68:     0x7f3b2a7a79e2 - rustc_query_impl[4dd0e81501809c4f]::get_query::collect_and_partition_mono_items
  69:     0x7f3b29505a31 - rustc_codegen_ssa[7fa8a61fe150c5a6]::base::codegen_crate::<rustc_codegen_llvm[60f7880173d7aea7]::LlvmCodegenBackend>
  70:     0x7f3b294ee375 - <rustc_codegen_llvm[60f7880173d7aea7]::LlvmCodegenBackend as rustc_codegen_ssa[7fa8a61fe150c5a6]::traits::backend::CodegenBackend>::codegen_crate
  71:     0x7f3b293df133 - <rustc_session[fb51f884770b4300]::session::Session>::time::<alloc[30b2db67739fbdd3]::boxed::Box<dyn core[51e91c5fbe619c17]::any::Any>, rustc_interface[fb41af4016d2bfb4]::passes::start_codegen::{closure#0}>
  72:     0x7f3b293a9d00 - rustc_interface[fb41af4016d2bfb4]::passes::start_codegen
  73:     0x7f3b2939206e - <std[655df35e4a40b28f]::thread::local::LocalKey<core[51e91c5fbe619c17]::cell::Cell<*const ()>>>::with::<rustc_middle[9c6cad357f4a2726]::ty::context::tls::enter_context<<rustc_middle[9c6cad357f4a2726]::ty::context::GlobalCtxt>::enter<<rustc_interface[fb41af4016d2bfb4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<alloc[30b2db67739fbdd3]::boxed::Box<dyn core[51e91c5fbe619c17]::any::Any>, rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>::{closure#0}, core[51e91c5fbe619c17]::result::Result<alloc[30b2db67739fbdd3]::boxed::Box<dyn core[51e91c5fbe619c17]::any::Any>, rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>::{closure#0}, core[51e91c5fbe619c17]::result::Result<alloc[30b2db67739fbdd3]::boxed::Box<dyn core[51e91c5fbe619c17]::any::Any>, rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>
  74:     0x7f3b293f6e18 - <rustc_interface[fb41af4016d2bfb4]::queries::Queries>::ongoing_codegen
  75:     0x7f3b292e8a9f - <rustc_interface[fb41af4016d2bfb4]::interface::Compiler>::enter::<rustc_driver_impl[73c87b44fd5cc2bd]::run_compiler::{closure#1}::{closure#2}, core[51e91c5fbe619c17]::result::Result<core[51e91c5fbe619c17]::option::Option<rustc_interface[fb41af4016d2bfb4]::queries::Linker>, rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>
  76:     0x7f3b29307dcf - rustc_span[1bf2f546da16ba0]::set_source_map::<core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>, rustc_interface[fb41af4016d2bfb4]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>, rustc_driver_impl[73c87b44fd5cc2bd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  77:     0x7f3b292e0128 - <scoped_tls[d59a433dba1d5125]::ScopedKey<rustc_span[1bf2f546da16ba0]::SessionGlobals>>::set::<rustc_interface[fb41af4016d2bfb4]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>, rustc_driver_impl[73c87b44fd5cc2bd]::run_compiler::{closure#1}>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>
  78:     0x7f3b29304d09 - std[655df35e4a40b28f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fb41af4016d2bfb4]::util::run_in_thread_pool_with_globals<rustc_interface[fb41af4016d2bfb4]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>, rustc_driver_impl[73c87b44fd5cc2bd]::run_compiler::{closure#1}>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>
  79:     0x7f3b292e630a - <<std[655df35e4a40b28f]::thread::Builder>::spawn_unchecked_<rustc_interface[fb41af4016d2bfb4]::util::run_in_thread_pool_with_globals<rustc_interface[fb41af4016d2bfb4]::interface::run_compiler<core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>, rustc_driver_impl[73c87b44fd5cc2bd]::run_compiler::{closure#1}>::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[51e91c5fbe619c17]::result::Result<(), rustc_span[1bf2f546da16ba0]::ErrorGuaranteed>>::{closure#1} as core[51e91c5fbe619c17]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  80:     0x7f3b2c089bd5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8c0161ed37b47442
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/alloc/src/boxed.rs:1985:9
  81:     0x7f3b2c089bd5 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h610ae07dd4da15ba
  82:     0x7f3b2c089bd5 - std::sys::unix::thread::Thread::new::thread_start::hd713bbad390c185f
                               at /rustc/f85313a28e253ffbd98fb5b39bff893b4132d7cf/library/std/src/sys/unix/thread.rs:108:17
  83:     0x7f3b27b85ea5 - start_thread
  84:     0x7f3b278aeb0d - clone
  84:     0x7f3b278aeb0d - clone
  85:                0x0 - <unknown>

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (f85313a28 2023-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z dylib-lto -C lto=thin -C embed-bitcode=yes -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [normalize_projection_ty] normalizing `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::Try>::Residual`
#1 [try_normalize_generic_arg_after_erasing_regions] normalizing `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::Try>::Residual`
#2 [symbol_name] computing the symbol for `<core::ops::control_flow::ControlFlow<core::iter::adapters::map::Map<smallvec::IntoIter<[&str; 2]>, [closure@compiler/rustc_codegen_llvm/src/llvm_util.rs:481:26: 481:34]>> as core::ops::try_trait::FromResidual>::from_residual`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] rustc_codegen_llvm test:false 6.369
error: could not compile `rustc_codegen_llvm` (lib)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_mir_dataflow test:false 21.263
---
Total duration:                           16m 3s
------------------------------------------------
root INFO: Free disk space: 503.65 GiB out of total 581.32 GiB (13.36% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 760, in execute_build_pipeline
    LLVM_PROFILE_DIR=str(pipeline.llvm_profile_dir_root() / "prof-%p")
  File "../src/ci/stage-build.py", line 571, in build_rustc
    cmd(arguments, env=env)
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/usr/bin/python3', '/checkout/x.py', 'build', '--target', 'x86_64-unknown-linux-gnu', '--host', 'x86_64-unknown-linux-gnu', '--stage', '2', 'library/std', '--llvm-profile-generate']' returned non-zero exit status 1.
