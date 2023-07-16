plain
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[f88d]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_685)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).0: rustc_span::def_id::DefIndex) in debuginfo for "other"
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
                6: rustc_mir_transform::optimized_mir
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                9: rustc_query_impl::get_query::optimized_mir
                9: rustc_query_impl::get_query::optimized_mir
               10: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               12: rustc_monomorphize::collector::collect_neighbours
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: rustc_monomorphize::collector::collect_items_rec
               17: rustc_monomorphize::collector::collect_items_rec
               18: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               19: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               20: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               21: rustc_monomorphize::collector::collect_crate_mono_items
               22: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               23: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>
               24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               25: rustc_query_impl::get_query::collect_and_partition_mono_items
               26: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               27: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               28: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               29: rustc_interface::passes::start_codegen
               30: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               31: <rustc_interface::queries::Queries>::ongoing_codegen
               32: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               33: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               34: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               36: <unknown>
               37: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[f88d]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_685)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).1: rustc_span::def_id::CrateNum) in debuginfo for "other"
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
                6: rustc_mir_transform::optimized_mir
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                9: rustc_query_impl::get_query::optimized_mir
                9: rustc_query_impl::get_query::optimized_mir
               10: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               12: rustc_monomorphize::collector::collect_neighbours
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: rustc_monomorphize::collector::collect_items_rec
               17: rustc_monomorphize::collector::collect_items_rec
               18: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               19: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               20: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               21: rustc_monomorphize::collector::collect_crate_mono_items
               22: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               23: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>
               24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               25: rustc_query_impl::get_query::collect_and_partition_mono_items
               26: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               27: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               28: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               29: rustc_interface::passes::start_codegen
               30: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               31: <rustc_interface::queries::Queries>::ongoing_codegen
               32: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               33: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               34: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               36: <unknown>
               37: <unknown>
             


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (aad1d99e2 2023-05-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc_trait_selection` (lib)
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(DefId(0:1642 ~ rustc_hir_typeck[af29]::fn_ctxt::suggestions::{impl#0}::suggest_associated_const)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_112)[-1 of 1].0: rustc_span::symbol::Ident).0: rustc_span::Symbol).0: rustc_span::symbol::SymbolIndex) in debuginfo for "other"
     |
     |
1329 |         let Some((DefKind::AssocFn, old_def_id)) = self.typeck_results.borrow().type_dependent_def(expr.hir_id) else {
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_var_debug_info
                4: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                6: rustc_mir_transform::optimized_mir
                6: rustc_mir_transform::optimized_mir
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                9: rustc_query_impl::get_query::optimized_mir
                9: rustc_query_impl::get_query::optimized_mir
               10: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
               11: rustc_mir_transform::deduce_param_attrs::deduced_param_attrs
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::deduced_param_attrs, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 16]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 16]>>
               13: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::deduced_param_attrs, rustc_query_impl::plumbing::QueryCtxt>
               14: rustc_query_impl::get_query::deduced_param_attrs
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 16]>>>
               16: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
               17: rustc_metadata::rmeta::encoder::encode_metadata_impl
               18: rustc_metadata::rmeta::encoder::encode_metadata
               19: rustc_metadata::fs::encode_and_write_metadata
               20: rustc_interface::passes::start_codegen
               21: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               22: <rustc_interface::queries::Queries>::ongoing_codegen
               23: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               24: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               25: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               27: <unknown>
               28: <unknown>
             


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (aad1d99e2 2023-05-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
