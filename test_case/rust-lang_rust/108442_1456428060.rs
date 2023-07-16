plain
   Compiling matches v0.1.8
   Compiling thin-slice v0.1.1
   Compiling smallvec v0.6.7
   Compiling servo_arc v0.1.1 (/checkout/obj/build/ct/servo/components/servo_arc)
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             3: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             6: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             7: core::fmt::write
             8: <rustc_const_eval::interpret::validity::ValidityVisitor<rustc_mir_transform::const_prop::ConstPropMachine>>::check_safe_pointer
             9: <rustc_const_eval::interpret::validity::ValidityVisitor<rustc_mir_transform::const_prop::ConstPropMachine>>::try_visit_primitive
            10: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_mir_transform::const_prop::ConstPropMachine>>::validate_operand_internal
            11: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
            12: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_basic_block_data
            14: rustc_mir_transform::pass_manager::run_passes_inner
            15: rustc_mir_transform::optimized_mir
            16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
            17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
            17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
            18: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root
            19: rustc_metadata::rmeta::encoder::encode_metadata_impl
            20: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
            21: rustc_metadata::rmeta::encoder::encode_metadata
            22: rustc_metadata::fs::encode_and_write_metadata
            23: rustc_interface::passes::start_codegen
            24: <rustc_middle::ty::context::GlobalCtxt>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_span::ErrorGuaranteed>>
            25: <rustc_interface::queries::Queries>::ongoing_codegen
            27: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            27: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            28: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            29: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            31: <unknown>
            32: <unknown>
          

---
warning: build failed, waiting for other jobs to finish...
thread 'main' panicked at 'tests failed for https://github.com/servo/servo', src/tools/cargotest/main.rs:124:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:18:10
make: *** [Makefile:44: check-aux] Error 1
