plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:1327 ~ rustc_resolve[6da0]::late::diagnostics::{impl#2}::suggest_adding_generic_parameter)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((((*_57)[-1 of 1].5: rustc_ast::GenericParamKind) as Const).2: std::option::Option<rustc_ast::AnonConst>) as Some).0: rustc_ast::AnonConst) in debuginfo for "x"
     |
     |
2118 |           let (ident, span) = match path {
2119 | |             [segment]
2119 | |             [segment]
2120 | |                 if !segment.has_generic_args
2121 | |                     && segment.ident.name != kw::SelfUpper
2126 | |             _ => return None,
2127 | |         };
     | |_________^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_var_debug_info
                4: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                6: rustc_mir_transform::optimized_mir
                7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
                8: rustc_query_impl::get_query::optimized_mir
                8: rustc_query_impl::get_query::optimized_mir
                9: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               10: rustc_mir_transform::deduce_param_attrs::deduced_param_attrs
               11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::deduced_param_attrs, rustc_query_impl::plumbing::QueryCtxt>
               12: rustc_query_impl::get_query::deduced_param_attrs
               13: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 16]>>>
               14: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
               15: rustc_metadata::rmeta::encoder::encode_metadata_impl
               16: rustc_metadata::rmeta::encoder::encode_metadata
               17: rustc_metadata::fs::encode_and_write_metadata
               18: rustc_interface::passes::start_codegen
               19: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               20: <rustc_interface::queries::Queries>::ongoing_codegen
               21: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               22: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               23: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               25: <unknown>
               26: <unknown>
             


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (eda876c56 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc_resolve` (lib)
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[5668]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_713)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).0: rustc_span::def_id::DefIndex) in debuginfo for "other"
     |
     |
2046 |       #[instrument(skip(self), level = "debug")]
     |
    ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-attributes-0.1.22/src/lib.rs:552:1
     |
552  | / pub fn instrument(
552  | / pub fn instrument(
553  | |     args: proc_macro::TokenStream,
554  | |     item: proc_macro::TokenStream,
555  | | ) -> proc_macro::TokenStream {
     | |____________________________- in this expansion of `#[instrument]`
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_var_debug_info
                4: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                6: rustc_mir_transform::optimized_mir
                7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
                8: rustc_query_impl::get_query::optimized_mir
                8: rustc_query_impl::get_query::optimized_mir
                9: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               11: rustc_monomorphize::collector::collect_neighbours
               12: rustc_monomorphize::collector::collect_items_rec
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: rustc_monomorphize::collector::collect_items_rec
               17: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               18: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               19: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               20: rustc_monomorphize::collector::collect_crate_mono_items
               21: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               23: rustc_query_impl::get_query::collect_and_partition_mono_items
               24: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               25: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               26: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               27: rustc_interface::passes::start_codegen
               28: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               29: <rustc_interface::queries::Queries>::ongoing_codegen
               30: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               31: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               32: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               34: <unknown>
               35: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[5668]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_713)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).1: rustc_span::def_id::CrateNum) in debuginfo for "other"
     |
     |
2046 |       #[instrument(skip(self), level = "debug")]
     |
    ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-attributes-0.1.22/src/lib.rs:552:1
     |
552  | / pub fn instrument(
552  | / pub fn instrument(
553  | |     args: proc_macro::TokenStream,
554  | |     item: proc_macro::TokenStream,
555  | | ) -> proc_macro::TokenStream {
     | |____________________________- in this expansion of `#[instrument]`
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_var_debug_info
                4: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                6: rustc_mir_transform::optimized_mir
                7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
                8: rustc_query_impl::get_query::optimized_mir
                8: rustc_query_impl::get_query::optimized_mir
                9: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               11: rustc_monomorphize::collector::collect_neighbours
               12: rustc_monomorphize::collector::collect_items_rec
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: rustc_monomorphize::collector::collect_items_rec
               17: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               18: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               19: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               20: rustc_monomorphize::collector::collect_crate_mono_items
               21: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               23: rustc_query_impl::get_query::collect_and_partition_mono_items
               24: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               25: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               26: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               27: rustc_interface::passes::start_codegen
               28: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               29: <rustc_interface::queries::Queries>::ongoing_codegen
               30: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               31: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               32: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               34: <unknown>
               35: <unknown>
             


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (eda876c56 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
