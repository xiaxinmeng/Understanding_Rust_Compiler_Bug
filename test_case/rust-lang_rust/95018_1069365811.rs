
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /rustc/52b34550aca5f7dd7e152f773e3ab786acb86f6f/compiler/rustc_middle/src/ty/sty.rs:1089:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/52b34550aca5f7dd7e152f773e3ab786acb86f6f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/52b34550aca5f7dd7e152f773e3ab786acb86f6f/library/core/src/panicking.rs:143:14
   2: core::panicking::panic
             at /rustc/52b34550aca5f7dd7e152f773e3ab786acb86f6f/library/core/src/panicking.rs:48:5
   3: rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions
   4: <rustc_infer::infer::InferCtxtBuilder>::enter::<bool, rustc_ty_utils::common_traits::is_item_raw::{closure#0}>
   5: rustc_ty_utils::common_traits::is_item_raw
   6: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, bool>
   7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, bool>>
   8: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::is_sized_raw, rustc_query_impl::plumbing::QueryCtxt>
   9: <rustc_middle::ty::Ty>::is_sized
  10: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  11: rustc_middle::ty::layout::layout_of
  12: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  15: <alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core::result::Result<core::convert::Infallible, rustc_middle::ty::layout::LayoutError>>>>::from_iter
  16: <alloc::vec::Vec<alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>>> as alloc::vec::spec_from_iter::SpecFromIter<alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>>, core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core::result::Result<core::convert::Infallible, rustc_middle::ty::layout::LayoutError>>>>::from_iter
  17: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  18: rustc_middle::ty::layout::layout_of
  19: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>
  20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  22: <alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core::result::Result<core::convert::Infallible, rustc_middle::ty::layout::LayoutError>>>>::from_iter
  23: <alloc::vec::Vec<alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>>> as alloc::vec::spec_from_iter::SpecFromIter<alloc::vec::Vec<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>>, core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core::result::Result<core::convert::Infallible, rustc_middle::ty::layout::LayoutError>>>>::from_iter
  24: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>>::layout_of_uncached
  25: rustc_middle::ty::layout::layout_of
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Ty>, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>
  27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
  28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
  29: rustc_codegen_ssa::debuginfo::type_names::push_debuginfo_type_name
  30: rustc_codegen_ssa::debuginfo::type_names::push_debuginfo_type_name
  31: rustc_codegen_ssa::debuginfo::type_names::push_generic_params_internal
  32: rustc_codegen_ssa::debuginfo::type_names::push_debuginfo_type_name
  33: rustc_codegen_ssa::debuginfo::type_names::compute_debuginfo_type_name
  34: rustc_codegen_llvm::debuginfo::metadata::type_di_node
  35: <rustc_codegen_llvm::context::CodegenCx as rustc_codegen_ssa::traits::debuginfo::DebugInfoMethods>::dbg_scope_fn
  36: rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder>
  37: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  38: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
  39: rustc_codegen_llvm::base::compile_codegen_unit
  40: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  41: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  42: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  43: <rustc_interface::queries::Queries>::ongoing_codegen
  44: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  45: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  46: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  47: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (52b34550a 2022-03-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [is_sized_raw] computing whether `dyn alloc::string::ToString` is `Sized`
#1 [layout_of] computing layout of `*const dyn alloc::string::ToString`
#2 [layout_of] computing layout of `core::ptr::unique::Unique<dyn alloc::string::ToString>`
#3 [layout_of] computing layout of `alloc::boxed::Box<dyn alloc::string::ToString>`
end of query stack
error: could not compile `broken`
