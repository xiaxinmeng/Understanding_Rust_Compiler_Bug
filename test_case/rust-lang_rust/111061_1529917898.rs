plain
[RUSTC-TIMING] rustc_middle test:false 80.985
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
[RUSTC-TIMING] rustc_infer test:false 52.953
   Compiling rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:1327 ~ rustc_resolve[6a67]::late::diagnostics::{impl#2}::suggest_adding_generic_parameter)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((*_57)[-1 of 1].3: std::vec::Vec<rustc_ast::GenericBound>).0: smallvec::alloc::raw_vec::RawVec<rustc_ast::GenericBound>) in debuginfo for "self"
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
               16: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
               17: rustc_metadata::rmeta::encoder::encode_metadata
               18: rustc_metadata::fs::encode_and_write_metadata
               19: rustc_interface::passes::start_codegen
               20: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               21: <rustc_interface::queries::Queries>::ongoing_codegen
               23: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               25: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               26: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               27: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
               28: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
               29: std::sys::unix::thread::Thread::new::thread_start
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
               30: start_thread
               31: clone
               31: clone
             

error: internal compiler error: broken MIR in Item(DefId(0:1327 ~ rustc_resolve[6a67]::late::diagnostics::{impl#2}::suggest_adding_generic_parameter)) (after phase change to runtime-optimized) at bb0[0]:
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
               16: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
               17: rustc_metadata::rmeta::encoder::encode_metadata
               18: rustc_metadata::fs::encode_and_write_metadata
               19: rustc_interface::passes::start_codegen
               20: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               21: <rustc_interface::queries::Queries>::ongoing_codegen
               23: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               25: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               26: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               27: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
               28: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
               29: std::sys::unix::thread::Thread::new::thread_start
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
               30: start_thread
               31: clone
               31: clone
             

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (21a5d8922 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z dylib-lto -C lto=thin -C embed-bitcode=yes -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
[RUSTC-TIMING] rustc_resolve test:false 50.838
error: could not compile `rustc_resolve` (lib)
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_privacy test:false 8.528
[RUSTC-TIMING] rustc_codegen_llvm test:false 51.884
[RUSTC-TIMING] rustc_lint test:false 35.161
error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[dcf7]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_689)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).0: rustc_span::def_id::DefIndex) in debuginfo for "other"
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
               18: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
               19: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               20: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               21: rustc_monomorphize::collector::collect_crate_mono_items
               22: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               24: rustc_query_impl::get_query::collect_and_partition_mono_items
               25: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               26: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               27: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               28: rustc_interface::passes::start_codegen
               29: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               30: <rustc_interface::queries::Queries>::ongoing_codegen
               32: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
               33: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               33: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               34: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               35: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               36: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
               37: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
               38: std::sys::unix::thread::Thread::new::thread_start
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
               39: start_thread
               40: clone
               40: clone
             

error: internal compiler error: broken MIR in Item(DefId(0:5189 ~ rustc_trait_selection[dcf7]::traits::error_reporting::{impl#2}::maybe_report_ambiguity)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_689)[-2 of 2].2: rustc_hir::def::Res) as Def).1: rustc_span::def_id::DefId).1: rustc_span::def_id::CrateNum) in debuginfo for "other"
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
               18: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
               19: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               20: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               21: rustc_monomorphize::collector::collect_crate_mono_items
               22: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
               24: rustc_query_impl::get_query::collect_and_partition_mono_items
               25: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
               26: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
               27: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
               28: rustc_interface::passes::start_codegen
               29: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               30: <rustc_interface::queries::Queries>::ongoing_codegen
               32: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
               33: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               33: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               34: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               35: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               36: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
               37: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
               38: std::sys::unix::thread::Thread::new::thread_start
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
               39: start_thread
               40: clone
               40: clone
             

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (21a5d8922 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z dylib-lto -C lto=thin -C embed-bitcode=yes -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
[RUSTC-TIMING] rustc_trait_selection test:false 67.658
error: could not compile `rustc_trait_selection` (lib)
[RUSTC-TIMING] rustc_hir_analysis test:false 61.174
[RUSTC-TIMING] rustc_traits test:false 42.911
[RUSTC-TIMING] rustc_const_eval test:false 49.701
error: internal compiler error: broken MIR in Item(DefId(0:158 ~ rustc_hir_typeck[6cef]::callee::{impl#0}::suggest_call_as_method)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (*_4)[1:][0 of 1] in debuginfo for "first"
    |
    |
500 | /         if let [callee_expr, rest @ ..] = arg_exprs {
501 | |             let callee_ty = self.typeck_results.borrow().expr_ty_adjusted_opt(callee_expr)?;
502 | |
503 | |             // First, do a probe with `IsSuggestion(true)` to avoid emitting
582 | |             }
583 | |         }
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
              16: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
              17: rustc_metadata::rmeta::encoder::encode_metadata
              18: rustc_metadata::fs::encode_and_write_metadata
              19: rustc_interface::passes::start_codegen
              20: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
              21: <rustc_interface::queries::Queries>::ongoing_codegen
              23: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
              24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
              24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
              25: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
              26: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
              27: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                         at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
              28: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
              29: std::sys::unix::thread::Thread::new::thread_start
                         at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
              30: start_thread
              31: clone
              31: clone
            

error: internal compiler error: broken MIR in Item(DefId(0:1642 ~ rustc_hir_typeck[6cef]::fn_ctxt::suggestions::{impl#0}::suggest_associated_const)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_114)[-1 of 1].0: rustc_span::symbol::Ident).0: rustc_span::Symbol).0: rustc_span::symbol::SymbolIndex) in debuginfo for "other"
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
               16: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
               17: rustc_metadata::rmeta::encoder::encode_metadata
               18: rustc_metadata::fs::encode_and_write_metadata
               19: rustc_interface::passes::start_codegen
               20: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
               21: <rustc_interface::queries::Queries>::ongoing_codegen
               23: rustc_span::set_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               24: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               25: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               26: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               27: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/alloc/src/boxed.rs:1985:9
               28: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
               29: std::sys::unix::thread::Thread::new::thread_start
                          at /rustc/21a5d892227859d0e862af7df261cb38e8db5abc/library/std/src/sys/unix/thread.rs:108:17
               30: start_thread
               31: clone
               31: clone
             

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (21a5d8922 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z dylib-lto -C lto=thin -C embed-bitcode=yes -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
---
Total duration:                          16m 29s
------------------------------------------------
root INFO: Free disk space: 503.24 GiB out of total 581.32 GiB (13.43% used)
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
