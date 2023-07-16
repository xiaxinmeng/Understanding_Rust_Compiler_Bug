
 Documenting rustdoc-error v0.1.0 (/home/ltdk/rustdoc-error)
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(generic_const_exprs)]
  |            ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<C as Const>, [])` during codegen

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1212:27
stack backtrace:
   0: rust_begin_unwind
             at /rustc/22e491ac7ed454d34669151a8b6464cb643c9b41/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/22e491ac7ed454d34669151a8b6464cb643c9b41/library/core/src/panicking.rs:110:14
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   3: <rustc_errors::HandlerInner>::emit_diagnostic
   4: <rustc_errors::HandlerInner>::emit_diag_at_span::<rustc_span::span_encoding::Span>
   5: <rustc_errors::HandlerInner>::span_bug::<rustc_span::span_encoding::Span>
   6: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span>
   7: <rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>, rustc_trait_selection::traits::codegen::codegen_fulfill_obligation::{closure#0}>
   8: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   9: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(rustc_middle::ty::ParamEnv, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::TraitRef>), core::result::Result<rustc_middle::traits::ImplSource<()>, rustc_errors::ErrorReported>>>
  10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
  11: rustc_ty_utils::instance::inner_resolve_instance
  12: rustc_ty_utils::instance::resolve_instance
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::resolve_instance, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  15: <rustc_middle::ty::instance::Instance>::resolve_opt_const_arg
  16: <rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  17: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_const
  18: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  19: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::<rustc_middle::mir::ConstantKind>
  20: <rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<rustc_middle::mir::ConstantKind, rustc_middle::traits::query::NoSolution>, rustc_traits::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle::mir::ConstantKind>::{closure#0}>
  21: <rustc_traits::normalize_erasing_regions::provide::{closure#1} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::ConstantKind>)>>::call_once
  22: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl::plumbing::QueryCtxt>
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  24: <rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  25: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::subst_from_current_frame_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  26: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  27: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
  28: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
  30: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, core::result::Result<rustc_middle::mir::interpret::value::ConstValue, rustc_middle::mir::interpret::error::ErrorHandled>>>
  32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  33: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  34: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>, core::result::Result<rustc_middle::mir::interpret::value::ConstValue, rustc_middle::mir::interpret::error::ErrorHandled>>>
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  36: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  37: <rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  38: <&rustc_middle::ty::TyS as rustdoc::clean::Clean<rustdoc::clean::types::Type>>::clean
  39: <rustdoc::core::DocContext>::with_param_env::<rustdoc::clean::types::Item, <rustc_hir::hir::ImplItem as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean::{closure#0}>
  40: <alloc::vec::Vec<rustdoc::clean::types::Item> as alloc::vec::spec_from_iter::SpecFromIter<rustdoc::clean::types::Item, core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_hir::hir::ImplItemRef>, rustdoc::clean::inline::build_impl<core::option::Option<rustc_span::def_id::DefId>>::{closure#0}>, rustdoc::clean::inline::build_impl<core::option::Option<rustc_span::def_id::DefId>>::{closure#1}>, rustdoc::clean::inline::build_impl<core::option::Option<rustc_span::def_id::DefId>>::{closure#2}>>>::from_iter
  41: rustdoc::clean::inline::build_impl::<core::option::Option<rustc_span::def_id::DefId>>
  42: <rustdoc::core::DocContext>::with_all_trait_impls::<rustdoc::passes::collect_trait_impls::collect_trait_impls::{closure#8}>
  43: rustdoc::passes::collect_trait_impls::collect_trait_impls
  44: <rustc_session::session::Session>::time::<rustdoc::clean::types::Crate, rustdoc::core::run_global_ctxt::{closure#8}>
  45: rustdoc::core::run_global_ctxt
  46: <rustc_session::session::Session>::time::<(rustdoc::clean::types::Crate, rustdoc::config::RenderOptions, rustdoc::formats::cache::Cache), rustdoc::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  47: <rustc_interface::interface::Compiler>::enter::<rustdoc::main_options::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
  48: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>::{closure#1}>
  49: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustdoc::main_options::{closure#0}>
  50: rustdoc::main_options
  51: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustdoc::main_args::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'

warning: `rustdoc-error` (lib doc) generated 1 warning
error: could not document `rustdoc-error`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type lib --crate-name rustdoc_error src/lib.rs -o /home/ltdk/rustdoc-error/target/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat -C metadata=6613bea700fb11b3 -L dependency=/home/ltdk/rustdoc-error/target/debug/deps -Ztreat-err-as-bug --crate-version 0.1.0` (exit status: 1)
