plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:658 ~ rustc_errors[0764]::emitter::{impl#7}::emit_suggestion_default)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((*_235)[0 of 1].1: std::string::String).0: std::vec::Vec<u8>) in debuginfo for "self"
     |
     |
1769 |         let Some(ref sm) = self.sm else {
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
               12: rustc_monomorphize::collector::collect_neighbours
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               17: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               18: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               19: rustc_monomorphize::collector::collect_crate_mono_items
               20: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               21: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>
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
             


error: internal compiler error: broken MIR in Item(DefId(0:658 ~ rustc_errors[0764]::emitter::{impl#7}::emit_suggestion_default)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_235)[0 of 1].1: std::string::String).0: std::vec::Vec<u8>).0: alloc::raw_vec::RawVec<u8>) in debuginfo for "self"
     |
     |
1769 |         let Some(ref sm) = self.sm else {
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
               12: rustc_monomorphize::collector::collect_neighbours
               13: rustc_monomorphize::collector::collect_items_rec
               14: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               15: rustc_monomorphize::collector::collect_items_rec
               16: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
               17: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
               18: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
               19: rustc_monomorphize::collector::collect_crate_mono_items
               20: rustc_monomorphize::partitioning::collect_and_partition_mono_items
               21: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 24]>>
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

note: rustc 1.71.0-nightly (28bde12d0 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc_errors` (lib)
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_53)[0 of 1] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_53)[0 of 1] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>).0: alloc::raw_vec::RawVec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_53)[0 of 2] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_53)[0 of 2] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>).0: alloc::raw_vec::RawVec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_53)[0 of 3] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_53)[0 of 3] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>).0: alloc::raw_vec::RawVec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place ((((*_53)[2 of 3] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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
             


error: internal compiler error: broken MIR in Item(DefId(0:632 ~ rustc_parse[78f7]::parser::expr::{impl#2}::parse_expr_tuple_field_access_float)) (after phase change to runtime-optimized) at bb0[0]:
                                illegal place (((((*_53)[2 of 3] as IdentLike).0: std::string::String).0: std::vec::Vec<u8>).0: alloc::raw_vec::RawVec<u8>) in debuginfo for "self"
     |
     |
1076 |         let float_str = float.as_str();
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

note: rustc 1.71.0-nightly (28bde12d0 2023-05-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
