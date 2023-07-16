plain
   Compiling miniz_oxide v0.6.2
   Compiling hashbrown v0.12.3
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.19.0
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (before pass LowerIntrinsics) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (after pass LowerIntrinsics) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (before pass RemovePlaceMention) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (after pass RemovePlaceMention) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (before pass SimplifyCfg-elaborate-drops) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (after pass SimplifyCfg-elaborate-drops) at bb0[0]:
                                Free regions in optimized runtime MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


error: internal compiler error: broken MIR in Item(DefId(0:579 ~ std[427f]::thread::{impl#13}::into_inner)) (after phase change to runtime-post-cleanup) at bb0[0]:
                                Free regions in optimized runtime-post-cleanup MIR
     |
1588 |         self.0.native
     |         ^^^^^^^^^^^^^
     |
     |
     = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
                1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, alloc::string::String>
                2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>
                3: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass
                5: rustc_mir_transform::run_analysis_to_runtime_passes
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                6: rustc_mir_transform::mir_drops_elaborated_and_const_checked
                7: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>::{closure#1}, rustc_middle::query::erase::Erased<[u8; 8]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
                8: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
                9: rustc_query_impl::get_query::mir_drops_elaborated_and_const_checked
               10: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#2}>
               11: rustc_interface::passes::analysis
               12: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<rustc_query_system::query::plumbing::execute_job_non_incr<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
               14: rustc_query_impl::get_query::analysis
               14: rustc_query_impl::get_query::analysis
               15: rustc_middle::ty::query::query_get_at::<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>>
               16: <std::thread::local::LocalKey<core::cell::Cell<*const ()>>>::with::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::ty::context::GlobalCtxt>::enter<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               17: <rustc_interface::queries::QueryResult<&rustc_middle::ty::context::GlobalCtxt>>::enter::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}>
               18: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
               19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
               20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
               22: <unknown>
               23: <unknown>
             


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (ca99fbe59 2023-05-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -Z validate-mir -Z mir-opt-level=3 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
