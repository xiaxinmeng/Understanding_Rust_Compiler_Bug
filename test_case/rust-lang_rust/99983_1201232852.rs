plain
....i................................................................................... 12496/13264
........................................................................................ 12584/13264
........................................................................................ 12672/13264
........................................................................................ 12760/13264
.......................................................................F..F...F.....FF.. 12848/13264
.............................F.FF..F.................................................... 12936/13264
........................................................................................ 13112/13264
.............................................iii........................................ 13200/13264
................................................................
failures:
failures:

---- [ui] src/test/ui/associated-type-bounds/union-bounds.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/union-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/union-bounds/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/union-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`Scalar` layout for non-primitive non-enum type Un4<()>', compiler/rustc_middle/src/ty/layout_sanity_check.rs:128:29
stack backtrace:
   0:     0x7fa03e8e1f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7fa03e948b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7fa03e8d2611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7fa03e8e4efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7fa03e8e4bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7fa03f2949d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa03e8e56b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7fa03e8e54d7 - std::panicking::begin_panic_handler::{{closure}}::h22accebf3aebfd42
   8:     0x7fa03e8e24c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd4c78465b1ef79f6
   9:     0x7fa03e8e51a2 - rust_begin_unwind
  10:     0x7fa03e894e13 - core::panicking::panic_fmt::hf49174a28eec326a
  11:     0x7fa041e2e27b - rustc_middle[214852cbba63d873]::ty::layout_sanity_check::sanity_check_layout
  12:     0x7fa041d83b21 - rustc_middle[214852cbba63d873]::ty::context::tls::with_context::<rustc_middle[214852cbba63d873]::ty::context::tls::with_related_context<rustc_middle[214852cbba63d873]::ty::layout::layout_of::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}
  13:     0x7fa041d8907b - rustc_middle[214852cbba63d873]::ty::layout::layout_of
  14:     0x7fa040fadc3d - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::layout_of, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  15:     0x7fa040aad213 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::layout_of
  16:     0x7fa03f7c8e4b - <rustc_mir_transform[d463174f352cef7e]::const_prop::CanConstProp>::check
  17:     0x7fa03f910891 - <rustc_mir_transform[d463174f352cef7e]::const_prop_lint::ConstProp as rustc_mir_transform[d463174f352cef7e]::pass_manager::MirLint>::run_lint
  18:     0x7fa03f7afe87 - rustc_mir_transform[d463174f352cef7e]::pass_manager::run_passes
  19:     0x7fa03f86e290 - rustc_mir_transform[d463174f352cef7e]::run_post_borrowck_cleanup_passes
  20:     0x7fa03f86dd16 - rustc_mir_transform[d463174f352cef7e]::mir_drops_elaborated_and_const_checked
  21:     0x7fa040e7a8c4 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_middle[214852cbba63d873]::ty::WithOptConstParam<rustc_span[1b831bd687513747]::def_id::LocalDefId>, &rustc_data_structures[7d0acec95599a8c0]::steal::Steal<rustc_middle[214852cbba63d873]::mir::Body>>>
  22:     0x7fa040fa75d3 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  23:     0x7fa040a809ea - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24:     0x7fa03f86e8f7 - rustc_mir_transform[d463174f352cef7e]::optimized_mir
  25:     0x7fa040eae1a0 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_span[1b831bd687513747]::def_id::DefId, &rustc_middle[214852cbba63d873]::mir::Body>>
  26:     0x7fa040f5be04 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::optimized_mir, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  27:     0x7fa040a82539 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::optimized_mir
  28:     0x7fa041e04c09 - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::instance_mir
  29:     0x7fa03f6e231a - rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_neighbours
  30:     0x7fa03f6dd3aa - rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_items_rec
  31:     0x7fa03f6e955d - <core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<alloc[4d9b0937cf17da9c]::vec::Vec<rustc_middle[214852cbba63d873]::mir::mono::MonoItem>, rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once
  32:     0x7fa03f721a55 - std[4d71c66624f49415]::panicking::try::<(), core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<alloc[4d9b0937cf17da9c]::vec::Vec<rustc_middle[214852cbba63d873]::mir::mono::MonoItem>, rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7fa03f6fc4be - rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in::<alloc[4d9b0937cf17da9c]::vec::Vec<rustc_middle[214852cbba63d873]::mir::mono::MonoItem>, rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  34:     0x7fa03f7211fb - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_crate_mono_items::{closure#1}>
  35:     0x7fa03f6d9cf7 - rustc_monomorphize[5a68faae1dbc5abf]::collector::collect_crate_mono_items
  36:     0x7fa03f6eb0ca - rustc_monomorphize[5a68faae1dbc5abf]::partitioning::collect_and_partition_mono_items
  37:     0x7fa040ed0332 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), (&std[4d71c66624f49415]::collections::hash::set::HashSet<rustc_span[1b831bd687513747]::def_id::DefId, core[fe278b4bcdbf1451]::hash::BuildHasherDefault<rustc_hash[7f4a04b56627b841]::FxHasher>>, &[rustc_middle[214852cbba63d873]::mir::mono::CodegenUnit])>>
  38:     0x7fa040fa29fa - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::collect_and_partition_mono_items, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  39:     0x7fa040ac67c9 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::collect_and_partition_mono_items
  40:     0x7fa03f4e44ad - rustc_codegen_ssa[3e1d1793645a26a8]::base::codegen_crate::<rustc_codegen_llvm[c567fd6ab8beaca9]::LlvmCodegenBackend>
  41:     0x7fa03f59b30d - <rustc_codegen_llvm[c567fd6ab8beaca9]::LlvmCodegenBackend as rustc_codegen_ssa[3e1d1793645a26a8]::traits::backend::CodegenBackend>::codegen_crate
  42:     0x7fa03f408abb - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<alloc[4d9b0937cf17da9c]::boxed::Box<dyn core[fe278b4bcdbf1451]::any::Any>, rustc_interface[438e1e4391a77fb4]::passes::start_codegen::{closure#0}>
  43:     0x7fa03f3ceccc - <rustc_interface[438e1e4391a77fb4]::passes::QueryContext>::enter::<<rustc_interface[438e1e4391a77fb4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[fe278b4bcdbf1451]::result::Result<alloc[4d9b0937cf17da9c]::boxed::Box<dyn core[fe278b4bcdbf1451]::any::Any>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  44:     0x7fa03f3b3eed - <rustc_interface[438e1e4391a77fb4]::queries::Queries>::ongoing_codegen
  45:     0x7fa03f29665f - <rustc_interface[438e1e4391a77fb4]::interface::Compiler>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}, core[fe278b4bcdbf1451]::result::Result<core[fe278b4bcdbf1451]::option::Option<rustc_interface[438e1e4391a77fb4]::queries::Linker>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  46:     0x7fa03f2ac8e8 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  47:     0x7fa03f29908a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  48:     0x7fa03f27fd32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  49:     0x7fa03f288319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  50:     0x7fa03f2ef3ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7fa03f2ba802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7fa03e8f0bd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  53:     0x7fa038e3b609 - start_thread
  54:     0x7fa03e74e133 - clone
  55:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [layout_of] computing layout of `Un4<()>`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#2 [optimized_mir] optimizing MIR for `main`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
              3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
              4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
              5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
              7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              8: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
              9: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             11: <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get
             12: std::panicking::rust_panic_with_hook
             13: std::panicking::begin_panic_handler::{{closure}}
             14: std::sys_common::backtrace::__rust_end_short_backtrace
             14: std::sys_common::backtrace::__rust_end_short_backtrace
             15: rust_begin_unwind
             16: core::panicking::panic_fmt
             17: rustc_middle::ty::layout_sanity_check::sanity_check_layout
             18: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<rustc_middle::ty::layout::layout_of::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}
             20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             22: <rustc_mir_transform::const_prop::CanConstProp>::check
             23: <rustc_mir_transform::const_prop_lint::ConstProp as rustc_mir_transform::pass_manager::MirLint>::run_lint
             24: rustc_mir_transform::pass_manager::run_passes
             25: rustc_mir_transform::run_post_borrowck_cleanup_passes
             26: rustc_mir_transform::mir_drops_elaborated_and_const_checked
             27: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
             28: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
             29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
             31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
             32: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
             33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
             34: <rustc_middle::ty::context::TyCtxt>::instance_mir
             34: <rustc_middle::ty::context::TyCtxt>::instance_mir
             35: rustc_monomorphize::collector::collect_neighbours
             36: rustc_monomorphize::collector::collect_items_rec
             37: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             38: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
             39: rustc_data_structures::sync::par_for_each_in::<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
             40: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
             41: rustc_monomorphize::collector::collect_crate_mono_items
             42: rustc_monomorphize::partitioning::collect_and_partition_mono_items
             43: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
             44: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
             45: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
             46: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
             47: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             48: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
             49: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
             50: <rustc_interface::queries::Queries>::ongoing_codegen
             52: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             53: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
             54: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             54: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             55: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             56: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             57: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             59: start_thread
             60: clone
           


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7fa03e8e1f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7fa03e948b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7fa03e8d2611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7fa03e8e4efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7fa03e8e4bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7fa03f2949d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa03e8e56b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7fa041fd4ca3 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}
   8:     0x7fa041fd4866 - std[4d71c66624f49415]::sys_common::backtrace::__rust_end_short_backtrace::<std[4d71c66624f49415]::panicking::begin_panic<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}, !>
   9:     0x7fa03f2553d6 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  10:     0x7fa042018b26 - std[4d71c66624f49415]::panic::panic_any::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  11:     0x7fa04201d8cc - <rustc_errors[db88e37312a15f7c]::HandlerInner as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  12:     0x7fa03f2b6872 - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_session[dc4c1eeffe07f685]::parse::ParseSess>
  13:     0x7fa03f2bd028 - <alloc[4d9b0937cf17da9c]::rc::Rc<rustc_session[dc4c1eeffe07f685]::session::Session> as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  14:     0x7fa03f2b5eac - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_interface[438e1e4391a77fb4]::interface::Compiler>
  15:     0x7fa03f2acd75 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fa03f29908a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  17:     0x7fa03f27fd32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  18:     0x7fa03f288319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  19:     0x7fa03f2ef3ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7fa03f2ba802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7fa03e8f0bd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  22:     0x7fa038e3b609 - start_thread
  23:     0x7fa03e74e133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/min_const_generics/invalid-patterns.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:29:21
   |
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:31:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:38:21
   |
   |
LL |   get_flag::<false, { unsafe { char_raw.character } }>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░


thread 'rustc' panicked at '`Scalar` layout for non-primitive non-enum type BoolRaw', compiler/rustc_middle/src/ty/layout_sanity_check.rs:128:29
   0:     0x7f305bbc6f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   0:     0x7f305bbc6f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7f305bc2db48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7f305bbb7611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7f305bbc9efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7f305bbc9bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7f305c5799d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f305bbca6b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7f305bbca4d7 - std::panicking::begin_panic_handler::{{closure}}::h22accebf3aebfd42
   8:     0x7f305bbc74c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd4c78465b1ef79f6
   9:     0x7f305bbca1a2 - rust_begin_unwind
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  10:     0x7f305bb79e13 - core::panicking::panic_fmt::hf49174a28eec326a
  11:     0x7f305f11327b - rustc_middle[214852cbba63d873]::ty::layout_sanity_check::sanity_check_layout
  12:     0x7f305f068b21 - rustc_middle[214852cbba63d873]::ty::context::tls::with_context::<rustc_middle[214852cbba63d873]::ty::context::tls::with_related_context<rustc_middle[214852cbba63d873]::ty::layout::layout_of::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}
  13:     0x7f305f06e07b - rustc_middle[214852cbba63d873]::ty::layout::layout_of
  14:     0x7f305e292c3d - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::layout_of, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  15:     0x7f305dd92213 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::layout_of
  16:     0x7f305dc6c8f1 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  17:     0x7f305e265d2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  18:     0x7f305dd7f73c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  19:     0x7f305db3c800 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
  20:     0x7f305db56e92 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  21:     0x7f305dc6c720 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  22:     0x7f305e265d2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  23:     0x7f305dd7f73c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  24:     0x7f305dc6beae - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7f305e265d2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  26:     0x7f305dd7f73c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7f305db98464 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_to_valtree
  28:     0x7f305dbdbdf0 - <rustc_const_eval[689b71e0e096c593]::provide::{closure#0} as core[fe278b4bcdbf1451]::ops::function::FnOnce<(rustc_middle[214852cbba63d873]::ty::context::TyCtxt, rustc_middle[214852cbba63d873]::ty::ParamEnvAnd<rustc_middle[214852cbba63d873]::mir::interpret::GlobalId>)>>::call_once
  29:     0x7f305e245662 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_valtree, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  30:     0x7f305dd8029c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_valtree
  31:     0x7f305f0dee4b - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  32:     0x7f305f0dd6fa - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  33:     0x7f305ed9908c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_const
  34:     0x7f305ec7b2e7 - <rustc_middle[214852cbba63d873]::ty::subst::GenericArg as rustc_middle[214852cbba63d873]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>
  35:     0x7f305ed2fd03 - <&rustc_middle[214852cbba63d873]::ty::list::List<rustc_middle[214852cbba63d873]::ty::subst::GenericArg> as rustc_middle[214852cbba63d873]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>
  36:     0x7f305ed14fbc - <rustc_middle[214852cbba63d873]::ty::Ty as rustc_middle[214852cbba63d873]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>
  37:     0x7f305ed8e156 - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_ty
  38:     0x7f305d21192c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[214852cbba63d873]::ty::Ty>
  39:     0x7f305d218a0e - rustc_trait_selection[926dc4e3345f25d1]::traits::project::normalize::<rustc_middle[214852cbba63d873]::ty::Ty>
  40:     0x7f305d072e49 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxt as rustc_trait_selection[926dc4e3345f25d1]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  41:     0x7f305d1dc23f - <rustc_typeck[15e221f5279097fa]::check::inherited::Inherited>::normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  42:     0x7f305ce6a7f9 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::instantiate_type_scheme::<rustc_middle[214852cbba63d873]::ty::Ty>
  43:     0x7f305ceca253 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::instantiate_value_path
  44:     0x7f305ce59a19 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_path
  45:     0x7f305ce58bf3 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7f305ce3fdc9 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_call
  47:     0x7f305cebc919 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_kind
  48:     0x7f305ce58c50 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  49:     0x7f305cebb6c9 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  50:     0x7f305ce76dbd - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_stmt
  51:     0x7f305ce77374 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  52:     0x7f305cebcbb2 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_kind
  53:     0x7f305ce58c50 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  54:     0x7f305cebb6c9 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  55:     0x7f305ce5a479 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::check_return_expr
  56:     0x7f305d0db304 - rustc_typeck[15e221f5279097fa]::check::check::check_fn
  57:     0x7f305d04e9e0 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxtBuilder>::enter::<&rustc_middle[214852cbba63d873]::ty::context::TypeckResults, <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>::{closure#0}>
  58:     0x7f305d1dad4e - <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>
  59:     0x7f305cfa82d3 - rustc_typeck[15e221f5279097fa]::check::typeck
  60:     0x7f305e16fccd - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_span[1b831bd687513747]::def_id::LocalDefId, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>>
  61:     0x7f305e2915d7 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  62:     0x7f305dd7b094 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck
  63:     0x7f305d10c575 - <core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once
  64:     0x7f305cf8e1f6 - std[4d71c66624f49415]::panicking::try::<(), core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  65:     0x7f305d14c541 - rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in::<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  66:     0x7f305d0e2763 - <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners::<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>
  67:     0x7f305cfaceed - rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies
  68:     0x7f305e1b60ef - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), ()>>
  69:     0x7f305e257065 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck_item_bodies, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  70:     0x7f305dd7ab3e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck_item_bodies
  71:     0x7f305cfd4ade - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_typeck[15e221f5279097fa]::check_crate::{closure#7}>
  72:     0x7f305d1e3142 - rustc_typeck[15e221f5279097fa]::check_crate
  73:     0x7f305c6b48f1 - rustc_interface[438e1e4391a77fb4]::passes::analysis
  74:     0x7f305e1aae70 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>>
  75:     0x7f305e2919b2 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::analysis, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  76:     0x7f305dd5e75e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::analysis
  77:     0x7f305c5d2b7e - <rustc_interface[438e1e4391a77fb4]::passes::QueryContext>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  78:     0x7f305c57b5fd - <rustc_interface[438e1e4391a77fb4]::interface::Compiler>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}, core[fe278b4bcdbf1451]::result::Result<core[fe278b4bcdbf1451]::option::Option<rustc_interface[438e1e4391a77fb4]::queries::Linker>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  79:     0x7f305c5918e8 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  80:     0x7f305c57e08a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  81:     0x7f305c564d32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  82:     0x7f305c56d319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  83:     0x7f305c5d43ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  84:     0x7f305c59f802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  85:     0x7f305bbd5bd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  86:     0x7f3056120609 - start_thread
  87:     0x7f305ba33133 - clone
  88:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `BoolRaw`
#0 [layout_of] computing layout of `BoolRaw`
#1 [eval_to_allocation_raw] const-evaluating + checking `bool_raw`
#2 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#8}`
#3 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#8}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [typeck] type-checking `main`
#6 [typeck_item_bodies] type-checking all item bodies
#7 [analysis] running analysis passes on this crate
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0080, E0308.
For more information about an error, try `rustc --explain E0080`.
For more information about an error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/consts/dangling-alloc-id-ice.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/dangling-alloc-id-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`Scalar` layout for non-primitive non-enum type Foo', compiler/rustc_middle/src/ty/layout_sanity_check.rs:128:29
   0:     0x7f0ac3722f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   0:     0x7f0ac3722f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7f0ac3789b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7f0ac3713611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7f0ac3725efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7f0ac3725bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7f0ac40d59d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0ac37266b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7f0ac37264d7 - std::panicking::begin_panic_handler::{{closure}}::h22accebf3aebfd42
   8:     0x7f0ac37234c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd4c78465b1ef79f6
   9:     0x7f0ac37261a2 - rust_begin_unwind
  10:     0x7f0ac36d5e13 - core::panicking::panic_fmt::hf49174a28eec326a
  11:     0x7f0ac6c6f27b - rustc_middle[214852cbba63d873]::ty::layout_sanity_check::sanity_check_layout
  12:     0x7f0ac6bc4b21 - rustc_middle[214852cbba63d873]::ty::context::tls::with_context::<rustc_middle[214852cbba63d873]::ty::context::tls::with_related_context<rustc_middle[214852cbba63d873]::ty::layout::layout_of::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}
  13:     0x7f0ac6bca07b - rustc_middle[214852cbba63d873]::ty::layout::layout_of
  14:     0x7f0ac5deec3d - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::layout_of, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  15:     0x7f0ac58ee213 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::layout_of
  16:     0x7f0ac5679096 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter> as rustc_middle[214852cbba63d873]::ty::layout::LayoutOf>::spanned_layout_of
  17:     0x7f0ac56820b4 - rustc_const_eval[689b71e0e096c593]::interpret::eval_context::from_known_layout::<<rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::layout_of_local::{closure#1}>
  18:     0x7f0ac56b8716 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::eval_place
  19:     0x7f0ac56a2e3b - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::run
  20:     0x7f0ac57c8739 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  21:     0x7f0ac5dc1d2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  22:     0x7f0ac58db73c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  23:     0x7f0ac57c666c - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f0ac5dc944e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_const_value_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  25:     0x7f0ac58dbcec - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f0ac57c62c0 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f0ac5dc944e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_const_value_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  28:     0x7f0ac58dbcec - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7f0ac6bf61b9 - <rustc_middle[214852cbba63d873]::ty::query::TyCtxtEnsure>::const_eval_poly
  30:     0x7f0ac66a5728 - <rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedModuleLateLintPass as rustc_lint[7bddc8fc3fc10e19]::passes::LateLintPass>::check_item
  31:     0x7f0ac667a66f - <rustc_lint[7bddc8fc3fc10e19]::late::LateContextAndPass<rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedModuleLateLintPass> as rustc_hir[7124ed6efcb39246]::intravisit::Visitor>::visit_nested_item
  32:     0x7f0ac66748dc - rustc_hir[7124ed6efcb39246]::intravisit::walk_mod::<rustc_lint[7bddc8fc3fc10e19]::late::LateContextAndPass<rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedModuleLateLintPass>>
  33:     0x7f0ac66804b0 - rustc_lint[7bddc8fc3fc10e19]::late::late_lint_mod::<rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedModuleLateLintPass>
  34:     0x7f0ac66a12ed - rustc_lint[7bddc8fc3fc10e19]::lint_mod
  35:     0x7f0ac5ccf280 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_span[1b831bd687513747]::def_id::LocalDefId, ()>>
  36:     0x7f0ac5dedf04 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::lint_mod, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  37:     0x7f0ac58d1aa4 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::lint_mod
  38:     0x7f0ac4210daa - <rustc_middle[214852cbba63d873]::hir::map::Map>::for_each_module::<rustc_lint[7bddc8fc3fc10e19]::late::check_crate<rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedLateLintPass, rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  39:     0x7f0ac424bc32 - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_lint[7bddc8fc3fc10e19]::late::check_crate<rustc_lint[7bddc8fc3fc10e19]::BuiltinCombinedLateLintPass, rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  40:     0x7f0ac424bd20 - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  41:     0x7f0ac424e9c5 - std[4d71c66624f49415]::panic::catch_unwind::<core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  42:     0x7f0ac41dba25 - <core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}::{closure#1}> as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once
  43:     0x7f0ac424d4ea - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_interface[438e1e4391a77fb4]::passes::analysis::{closure#5}>
  44:     0x7f0ac421098c - rustc_interface[438e1e4391a77fb4]::passes::analysis
  45:     0x7f0ac5d06e70 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>>
  46:     0x7f0ac5ded9b2 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::analysis, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  47:     0x7f0ac58ba75e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::analysis
  48:     0x7f0ac412eb7e - <rustc_interface[438e1e4391a77fb4]::passes::QueryContext>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  49:     0x7f0ac40d75fd - <rustc_interface[438e1e4391a77fb4]::interface::Compiler>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}, core[fe278b4bcdbf1451]::result::Result<core[fe278b4bcdbf1451]::option::Option<rustc_interface[438e1e4391a77fb4]::queries::Linker>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  50:     0x7f0ac40ed8e8 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  51:     0x7f0ac40da08a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  52:     0x7f0ac40c0d32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  53:     0x7f0ac40c9319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  54:     0x7f0ac41303ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7f0ac40fb802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f0ac3731bd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  57:     0x7f0abdc7c609 - start_thread
  58:     0x7f0ac358f133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `Foo`
#0 [layout_of] computing layout of `Foo`
#1 [eval_to_allocation_raw] const-evaluating + checking `FOO`
#2 [eval_to_const_value_raw] simplifying constant for the type system `FOO`
#3 [eval_to_const_value_raw] simplifying constant for the type system `FOO`
#4 [lint_mod] linting top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
              3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
              4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
              5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
              7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              8: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
              9: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             11: <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get
             12: std::panicking::rust_panic_with_hook
             13: std::panicking::begin_panic_handler::{{closure}}
             14: std::sys_common::backtrace::__rust_end_short_backtrace
             14: std::sys_common::backtrace::__rust_end_short_backtrace
             15: rust_begin_unwind
             16: core::panicking::panic_fmt
             17: rustc_middle::ty::layout_sanity_check::sanity_check_layout
             18: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<rustc_middle::ty::layout::layout_of::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}
             20: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             22: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter> as rustc_middle::ty::layout::LayoutOf>::spanned_layout_of
             23: rustc_const_eval::interpret::eval_context::from_known_layout::<<rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::layout_of_local::{closure#1}>
             24: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::eval_place
             25: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::run
             26: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             29: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
             30: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
             31: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
             32: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
             33: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
             34: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
             35: <rustc_middle::ty::query::TyCtxtEnsure>::const_eval_poly
             36: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc_lint::passes::LateLintPass>::check_item
             37: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
             38: rustc_hir::intravisit::walk_mod::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass>>
             39: rustc_lint::late::late_lint_mod::<rustc_lint::BuiltinCombinedModuleLateLintPass>
             41: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
             42: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::lint_mod, rustc_query_impl::plumbing::QueryCtxt>
             43: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::lint_mod
---
             27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
             28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
             29: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id
             30: <rustc_middle::ty::context::TyCtxt>::const_eval_instance
             31: <rustc_mir_build::thir::pattern::PatCtxt>::lower_path
             32: <rustc_mir_build::thir::pattern::PatCtxt>::lower_pattern
             33: <rustc_mir_build::thir::pattern::check_match::MatchVisitor>::lower_pattern
             34: <core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_hir::hir::Arm>, <rustc_mir_build::thir::pattern::check_match::MatchVisitor>::check_match::{closure#0}> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<rustc_mir_build::thir::pattern::usefulness::MatchArm, <alloc::vec::Vec<rustc_mir_build::thir::pattern::usefulness::MatchArm> as alloc::vec::spec_extend::SpecExtend<rustc_mir_build::thir::pattern::usefulness::MatchArm, core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_hir::hir::Arm>, <rustc_mir_build::thir::pattern::check_match::MatchVisitor>::check_match::{closure#0}>>>::spec_extend::{closure#0}>::{closure#0}>
             35: <alloc::vec::Vec<rustc_mir_build::thir::pattern::usefulness::MatchArm> as alloc::vec::spec_from_iter::SpecFromIter<rustc_mir_build::thir::pattern::usefulness::MatchArm, core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_hir::hir::Arm>, <rustc_mir_build::thir::pattern::check_match::MatchVisitor>::check_match::{closure#0}>>>::from_iter
             36: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             37: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
             38: rustc_mir_build::thir::pattern::check_match::check_match
             39: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, ()>>
             40: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_match, rustc_query_impl::plumbing::QueryCtxt>
             41: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_match
             42: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             43: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
             44: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
             45: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
             46: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}::{closure#0}::{closure#0}>
             47: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#1}>
             49: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
             50: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             51: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             52: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             52: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             53: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             54: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             55: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
             56: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             57: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             58: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             59: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             61: start_thread
             62: clone
           


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7f91ee753f4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7f91ee7bab48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7f91ee744611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7f91ee756efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7f91ee756bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7f91ef1069d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f91ee7576b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7f91f1e46ca3 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}
   8:     0x7f91f1e46866 - std[4d71c66624f49415]::sys_common::backtrace::__rust_end_short_backtrace::<std[4d71c66624f49415]::panicking::begin_panic<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f91ef0c73d6 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  10:     0x7f91f1e8ab26 - std[4d71c66624f49415]::panic::panic_any::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  11:     0x7f91f1e8f8cc - <rustc_errors[db88e37312a15f7c]::HandlerInner as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  12:     0x7f91ef128872 - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_session[dc4c1eeffe07f685]::parse::ParseSess>
  13:     0x7f91ef12f028 - <alloc[4d9b0937cf17da9c]::rc::Rc<rustc_session[dc4c1eeffe07f685]::session::Session> as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  14:     0x7f91ef127eac - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_interface[438e1e4391a77fb4]::interface::Compiler>
  15:     0x7f91ef11ed75 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f91ef10b08a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  17:     0x7f91ef0f1d32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  18:     0x7f91ef0fa319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  19:     0x7f91ef1613ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f91ef12c802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f91ee762bd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  22:     0x7f91e8cad609 - start_thread
  23:     0x7f91ee5c0133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/union/union-const-eval.rs#mirunsafeck stdout ----

error in revision `mirunsafeck`: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-const-eval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-const-eval.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-const-eval.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`Scalar` layout for non-primitive non-enum type U', compiler/rustc_middle/src/ty/layout_sanity_check.rs:128:29
   0:     0x7f1b30e1cf4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   0:     0x7f1b30e1cf4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7f1b30e83b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7f1b30e0d611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7f1b30e1fefe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7f1b30e1fbc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7f1b317cf9d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b30e206b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7f1b30e204d7 - std::panicking::begin_panic_handler::{{closure}}::h22accebf3aebfd42
   8:     0x7f1b30e1d4c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd4c78465b1ef79f6
   9:     0x7f1b30e201a2 - rust_begin_unwind
  10:     0x7f1b30dcfe13 - core::panicking::panic_fmt::hf49174a28eec326a
  11:     0x7f1b3436927b - rustc_middle[214852cbba63d873]::ty::layout_sanity_check::sanity_check_layout
  12:     0x7f1b342beb21 - rustc_middle[214852cbba63d873]::ty::context::tls::with_context::<rustc_middle[214852cbba63d873]::ty::context::tls::with_related_context<rustc_middle[214852cbba63d873]::ty::layout::layout_of::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}
  13:     0x7f1b342c407b - rustc_middle[214852cbba63d873]::ty::layout::layout_of
  14:     0x7f1b334e8c3d - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::layout_of, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  15:     0x7f1b32fe8213 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::layout_of
  16:     0x7f1b32ec28f1 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  17:     0x7f1b334bbd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  18:     0x7f1b32fd573c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  19:     0x7f1b32d92800 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
  20:     0x7f1b32dace92 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  21:     0x7f1b32ec2720 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  22:     0x7f1b334bbd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  23:     0x7f1b32fd573c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  24:     0x7f1b32ec1eae - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7f1b334bbd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  26:     0x7f1b32fd573c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7f1b32dee464 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_to_valtree
  28:     0x7f1b32e31df0 - <rustc_const_eval[689b71e0e096c593]::provide::{closure#0} as core[fe278b4bcdbf1451]::ops::function::FnOnce<(rustc_middle[214852cbba63d873]::ty::context::TyCtxt, rustc_middle[214852cbba63d873]::ty::ParamEnvAnd<rustc_middle[214852cbba63d873]::mir::interpret::GlobalId>)>>::call_once
  29:     0x7f1b3349b662 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_valtree, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  30:     0x7f1b32fd629c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_valtree
  31:     0x7f1b34334e4b - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  32:     0x7f1b343336fa - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  33:     0x7f1b33fef08c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_const
  34:     0x7f1b33f6af29 - <rustc_middle[214852cbba63d873]::ty::Ty as rustc_middle[214852cbba63d873]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>
  35:     0x7f1b33fe4156 - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_ty
  36:     0x7f1b3246792c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[214852cbba63d873]::ty::Ty>
  37:     0x7f1b3246ea0e - rustc_trait_selection[926dc4e3345f25d1]::traits::project::normalize::<rustc_middle[214852cbba63d873]::ty::Ty>
  38:     0x7f1b322c8e49 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxt as rustc_trait_selection[926dc4e3345f25d1]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  39:     0x7f1b3243223f - <rustc_typeck[15e221f5279097fa]::check::inherited::Inherited>::normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  40:     0x7f1b32293cda - <dyn rustc_typeck[15e221f5279097fa]::astconv::AstConv>::ast_ty_to_ty_inner
  41:     0x7f1b320c1293 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::to_ty
  42:     0x7f1b32210fcd - <rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor>::declare
  43:     0x7f1b3221163a - <rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor as rustc_hir[7124ed6efcb39246]::intravisit::Visitor>::visit_local
  44:     0x7f1b3245ed74 - rustc_hir[7124ed6efcb39246]::intravisit::walk_expr::<rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor>
  45:     0x7f1b32330bda - rustc_typeck[15e221f5279097fa]::check::check::check_fn
  46:     0x7f1b322a49e0 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxtBuilder>::enter::<&rustc_middle[214852cbba63d873]::ty::context::TypeckResults, <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>::{closure#0}>
  47:     0x7f1b32430d4e - <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>
  48:     0x7f1b321fe2d3 - rustc_typeck[15e221f5279097fa]::check::typeck
  49:     0x7f1b333c5ccd - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_span[1b831bd687513747]::def_id::LocalDefId, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>>
  50:     0x7f1b334e75d7 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  51:     0x7f1b32fd1094 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck
  52:     0x7f1b32362575 - <core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once
  53:     0x7f1b321e41f6 - std[4d71c66624f49415]::panicking::try::<(), core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  54:     0x7f1b323a2541 - rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in::<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  55:     0x7f1b32338763 - <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners::<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>
  56:     0x7f1b32202eed - rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies
  57:     0x7f1b3340c0ef - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), ()>>
  58:     0x7f1b334ad065 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck_item_bodies, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  59:     0x7f1b32fd0b3e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck_item_bodies
  60:     0x7f1b3222aade - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_typeck[15e221f5279097fa]::check_crate::{closure#7}>
  61:     0x7f1b32439142 - rustc_typeck[15e221f5279097fa]::check_crate
  62:     0x7f1b3190a8f1 - rustc_interface[438e1e4391a77fb4]::passes::analysis
  63:     0x7f1b33400e70 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>>
  64:     0x7f1b334e79b2 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::analysis, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  65:     0x7f1b32fb475e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::analysis
  66:     0x7f1b31828b7e - <rustc_interface[438e1e4391a77fb4]::passes::QueryContext>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  67:     0x7f1b317d15fd - <rustc_interface[438e1e4391a77fb4]::interface::Compiler>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}, core[fe278b4bcdbf1451]::result::Result<core[fe278b4bcdbf1451]::option::Option<rustc_interface[438e1e4391a77fb4]::queries::Linker>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  68:     0x7f1b317e78e8 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f1b317d408a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  70:     0x7f1b317bad32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  71:     0x7f1b317c3319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  72:     0x7f1b3182a3ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  73:     0x7f1b317f5802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:     0x7f1b30e2bbd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  75:     0x7f1b2b376609 - start_thread
  76:     0x7f1b30c89133 - clone
  77:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [layout_of] computing layout of `U`
#0 [layout_of] computing layout of `U`
#1 [eval_to_allocation_raw] const-evaluating + checking `C`
#2 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#3 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [typeck] type-checking `main`
#6 [typeck_item_bodies] type-checking all item bodies
#7 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
              3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
              4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
              5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
              7: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
              9: <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get
             10: std::panicking::rust_panic_with_hook
             11: std::panicking::begin_panic_handler::{{closure}}
             12: std::sys_common::backtrace::__rust_end_short_backtrace
             12: std::sys_common::backtrace::__rust_end_short_backtrace
             13: rust_begin_unwind
             14: core::panicking::panic_fmt
             15: rustc_middle::ty::layout_sanity_check::sanity_check_layout
             16: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<rustc_middle::ty::layout::layout_of::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}
             18: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             20: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             23: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
             24: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
             25: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             28: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             29: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             30: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             31: rustc_const_eval::const_eval::eval_to_valtree
             32: <rustc_const_eval::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>)>>::call_once
             33: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_valtree, rustc_query_impl::plumbing::QueryCtxt>
             34: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_valtree
             35: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id_for_typeck
             36: <rustc_middle::ty::context::TyCtxt>::const_eval_resolve_for_typeck
             37: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
             38: <rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
             39: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
             40: <rustc_trait_selection::traits::project::AssocTypeNormalizer>::fold::<rustc_middle::ty::Ty>
             41: rustc_trait_selection::traits::project::normalize::<rustc_middle::ty::Ty>
             42: <rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle::ty::Ty>
             43: <rustc_typeck::check::inherited::Inherited>::normalize_associated_types_in::<rustc_middle::ty::Ty>
             44: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner
             45: <rustc_typeck::check::fn_ctxt::FnCtxt>::to_ty
             46: <rustc_typeck::check::gather_locals::GatherLocalsVisitor>::declare
             47: <rustc_typeck::check::gather_locals::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_local
             48: rustc_hir::intravisit::walk_expr::<rustc_typeck::check::gather_locals::GatherLocalsVisitor>
             49: rustc_typeck::check::check::check_fn
             50: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
             51: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>
             52: rustc_typeck::check::typeck
             53: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
             54: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             55: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
             56: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             57: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             58: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>
             59: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
             60: rustc_typeck::check::typeck_item_bodies
             61: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
             62: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
             63: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies
             64: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
             65: rustc_typeck::check_crate
             67: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
             68: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             69: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             70: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             70: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             71: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             72: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             73: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
             74: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             75: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             76: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             77: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             79: start_thread
             80: clone
           


thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7f1b30e1cf4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7f1b30e83b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7f1b30e0d611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7f1b30e1fefe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7f1b30e1fbc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7f1b317cf9d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b30e206b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7f1b3450fca3 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}
   8:     0x7f1b3450f866 - std[4d71c66624f49415]::sys_common::backtrace::__rust_end_short_backtrace::<std[4d71c66624f49415]::panicking::begin_panic<rustc_errors[db88e37312a15f7c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f1b317903d6 - std[4d71c66624f49415]::panicking::begin_panic::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  10:     0x7f1b34553b26 - std[4d71c66624f49415]::panic::panic_any::<rustc_errors[db88e37312a15f7c]::ExplicitBug>
  11:     0x7f1b345588cc - <rustc_errors[db88e37312a15f7c]::HandlerInner as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  12:     0x7f1b317f1872 - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_session[dc4c1eeffe07f685]::parse::ParseSess>
  13:     0x7f1b317f8028 - <alloc[4d9b0937cf17da9c]::rc::Rc<rustc_session[dc4c1eeffe07f685]::session::Session> as core[fe278b4bcdbf1451]::ops::drop::Drop>::drop
  14:     0x7f1b317f0eac - core[fe278b4bcdbf1451]::ptr::drop_in_place::<rustc_interface[438e1e4391a77fb4]::interface::Compiler>
  15:     0x7f1b317e7d75 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f1b317d408a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  17:     0x7f1b317bad32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  18:     0x7f1b317c3319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  19:     0x7f1b3182a3ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f1b317f5802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f1b30e2bbd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  22:     0x7f1b2b376609 - start_thread
  23:     0x7f1b30c89133 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/union/union-const-eval.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-const-eval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-const-eval.thirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-const-eval.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at '`Scalar` layout for non-primitive non-enum type U', compiler/rustc_middle/src/ty/layout_sanity_check.rs:128:29
stack backtrace:
   0:     0x7ffb48baef4c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a9becd45de804fe
   1:     0x7ffb48c15b48 - core::fmt::write::h549ed923b1c2e9b3
   2:     0x7ffb48b9f611 - std::io::Write::write_fmt::h9a17bebc5385c0b5
   3:     0x7ffb48bb1efe - std::panicking::default_hook::{{closure}}::h4f876e9bb4cf1273
   4:     0x7ffb48bb1bc6 - std::panicking::default_hook::hd2a8bdd840b1c4b6
   5:     0x7ffb495619d4 - rustc_driver[db94148c310eac51]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ffb48bb26b2 - std::panicking::rust_panic_with_hook::h34f01e36870f145f
   7:     0x7ffb48bb24d7 - std::panicking::begin_panic_handler::{{closure}}::h22accebf3aebfd42
   8:     0x7ffb48baf4c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd4c78465b1ef79f6
   9:     0x7ffb48bb21a2 - rust_begin_unwind
  10:     0x7ffb48b61e13 - core::panicking::panic_fmt::hf49174a28eec326a
  11:     0x7ffb4c0fb27b - rustc_middle[214852cbba63d873]::ty::layout_sanity_check::sanity_check_layout
  12:     0x7ffb4c050b21 - rustc_middle[214852cbba63d873]::ty::context::tls::with_context::<rustc_middle[214852cbba63d873]::ty::context::tls::with_related_context<rustc_middle[214852cbba63d873]::ty::layout::layout_of::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<rustc_target[15764d7a0cdccb69]::abi::TyAndLayout<rustc_middle[214852cbba63d873]::ty::Ty>, rustc_middle[214852cbba63d873]::ty::layout::LayoutError>>::{closure#0}
  13:     0x7ffb4c05607b - rustc_middle[214852cbba63d873]::ty::layout::layout_of
  14:     0x7ffb4b27ac3d - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::layout_of, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  15:     0x7ffb4ad7a213 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::layout_of
  16:     0x7ffb4ac548f1 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  17:     0x7ffb4b24dd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  18:     0x7ffb4ad6773c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  19:     0x7ffb4ab24800 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
  20:     0x7ffb4ab3ee92 - <rustc_const_eval[689b71e0e096c593]::interpret::eval_context::InterpCx<rustc_const_eval[689b71e0e096c593]::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
  21:     0x7ffb4ac54720 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  22:     0x7ffb4b24dd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  23:     0x7ffb4ad6773c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  24:     0x7ffb4ac53eae - rustc_const_eval[689b71e0e096c593]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7ffb4b24dd2e - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_allocation_raw, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  26:     0x7ffb4ad6773c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7ffb4ab80464 - rustc_const_eval[689b71e0e096c593]::const_eval::eval_to_valtree
  28:     0x7ffb4abc3df0 - <rustc_const_eval[689b71e0e096c593]::provide::{closure#0} as core[fe278b4bcdbf1451]::ops::function::FnOnce<(rustc_middle[214852cbba63d873]::ty::context::TyCtxt, rustc_middle[214852cbba63d873]::ty::ParamEnvAnd<rustc_middle[214852cbba63d873]::mir::interpret::GlobalId>)>>::call_once
  29:     0x7ffb4b22d662 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::eval_to_valtree, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  30:     0x7ffb4ad6829c - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::eval_to_valtree
  31:     0x7ffb4c0c6e4b - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_global_id_for_typeck
  32:     0x7ffb4c0c56fa - <rustc_middle[214852cbba63d873]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  33:     0x7ffb4bd8108c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_const
  34:     0x7ffb4bcfcf29 - <rustc_middle[214852cbba63d873]::ty::Ty as rustc_middle[214852cbba63d873]::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>
  35:     0x7ffb4bd76156 - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer as rustc_middle[214852cbba63d873]::ty::fold::TypeFolder>::fold_ty
  36:     0x7ffb4a1f992c - <rustc_trait_selection[926dc4e3345f25d1]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[214852cbba63d873]::ty::Ty>
  37:     0x7ffb4a200a0e - rustc_trait_selection[926dc4e3345f25d1]::traits::project::normalize::<rustc_middle[214852cbba63d873]::ty::Ty>
  38:     0x7ffb4a05ae49 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxt as rustc_trait_selection[926dc4e3345f25d1]::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  39:     0x7ffb4a1c423f - <rustc_typeck[15e221f5279097fa]::check::inherited::Inherited>::normalize_associated_types_in::<rustc_middle[214852cbba63d873]::ty::Ty>
  40:     0x7ffb4a025cda - <dyn rustc_typeck[15e221f5279097fa]::astconv::AstConv>::ast_ty_to_ty_inner
  41:     0x7ffb49e53293 - <rustc_typeck[15e221f5279097fa]::check::fn_ctxt::FnCtxt>::to_ty
  42:     0x7ffb49fa2fcd - <rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor>::declare
  43:     0x7ffb49fa363a - <rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor as rustc_hir[7124ed6efcb39246]::intravisit::Visitor>::visit_local
  44:     0x7ffb4a1f0d74 - rustc_hir[7124ed6efcb39246]::intravisit::walk_expr::<rustc_typeck[15e221f5279097fa]::check::gather_locals::GatherLocalsVisitor>
  45:     0x7ffb4a0c2bda - rustc_typeck[15e221f5279097fa]::check::check::check_fn
  46:     0x7ffb4a0369e0 - <rustc_infer[8d3ebd59fe4a0391]::infer::InferCtxtBuilder>::enter::<&rustc_middle[214852cbba63d873]::ty::context::TypeckResults, <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>::{closure#0}>
  47:     0x7ffb4a1c2d4e - <rustc_typeck[15e221f5279097fa]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[15e221f5279097fa]::check::typeck_with_fallback<rustc_typeck[15e221f5279097fa]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>
  48:     0x7ffb49f902d3 - rustc_typeck[15e221f5279097fa]::check::typeck
  49:     0x7ffb4b157ccd - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<rustc_span[1b831bd687513747]::def_id::LocalDefId, &rustc_middle[214852cbba63d873]::ty::context::TypeckResults>>
  50:     0x7ffb4b2795d7 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  51:     0x7ffb4ad63094 - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck
  52:     0x7ffb4a0f4575 - <core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once
  53:     0x7ffb49f761f6 - std[4d71c66624f49415]::panicking::try::<(), core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  54:     0x7ffb4a134541 - rustc_data_structures[7d0acec95599a8c0]::sync::par_for_each_in::<&[rustc_span[1b831bd687513747]::def_id::LocalDefId], <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  55:     0x7ffb4a0ca763 - <rustc_middle[214852cbba63d873]::hir::map::Map>::par_body_owners::<rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies::{closure#0}>
  56:     0x7ffb49f94eed - rustc_typeck[15e221f5279097fa]::check::typeck_item_bodies
  57:     0x7ffb4b19e0ef - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), ()>>
  58:     0x7ffb4b23f065 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::typeck_item_bodies, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  59:     0x7ffb4ad62b3e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::typeck_item_bodies
  60:     0x7ffb49fbcade - <rustc_session[dc4c1eeffe07f685]::session::Session>::time::<(), rustc_typeck[15e221f5279097fa]::check_crate::{closure#7}>
  61:     0x7ffb4a1cb142 - rustc_typeck[15e221f5279097fa]::check_crate
  62:     0x7ffb4969c8f1 - rustc_interface[438e1e4391a77fb4]::passes::analysis
  63:     0x7ffb4b192e70 - rustc_query_system[7abd010232c7edfa]::query::plumbing::try_execute_query::<rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt, rustc_query_system[7abd010232c7edfa]::query::caches::DefaultCache<(), core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>>
  64:     0x7ffb4b2799b2 - rustc_query_system[7abd010232c7edfa]::query::plumbing::get_query::<rustc_query_impl[f55738a50d14d4d]::queries::analysis, rustc_query_impl[f55738a50d14d4d]::plumbing::QueryCtxt>
  65:     0x7ffb4ad4675e - <rustc_query_impl[f55738a50d14d4d]::Queries as rustc_middle[214852cbba63d873]::ty::query::QueryEngine>::analysis
  66:     0x7ffb495bab7e - <rustc_interface[438e1e4391a77fb4]::passes::QueryContext>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  67:     0x7ffb495635fd - <rustc_interface[438e1e4391a77fb4]::interface::Compiler>::enter::<rustc_driver[db94148c310eac51]::run_compiler::{closure#1}::{closure#2}, core[fe278b4bcdbf1451]::result::Result<core[fe278b4bcdbf1451]::option::Option<rustc_interface[438e1e4391a77fb4]::queries::Linker>, rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  68:     0x7ffb495798e8 - rustc_span[1b831bd687513747]::with_source_map::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7ffb4956608a - rustc_interface[438e1e4391a77fb4]::interface::create_compiler_and_run::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>
  70:     0x7ffb4954cd32 - <scoped_tls[1a8efe81f34f4283]::ScopedKey<rustc_span[1b831bd687513747]::SessionGlobals>>::set::<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  71:     0x7ffb49555319 - std[4d71c66624f49415]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>
  72:     0x7ffb495bc3ae - std[4d71c66624f49415]::panicking::try::<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, core[fe278b4bcdbf1451]::panic::unwind_safe::AssertUnwindSafe<<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  73:     0x7ffb49587802 - <<std[4d71c66624f49415]::thread::Builder>::spawn_unchecked_<rustc_interface[438e1e4391a77fb4]::util::run_in_thread_pool_with_globals<rustc_interface[438e1e4391a77fb4]::interface::run_compiler<core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>, rustc_driver[db94148c310eac51]::run_compiler::{closure#1}>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#0}, core[fe278b4bcdbf1451]::result::Result<(), rustc_errors[db88e37312a15f7c]::ErrorGuaranteed>>::{closure#1} as core[fe278b4bcdbf1451]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:     0x7ffb48bbdbd5 - std::sys::unix::thread::Thread::new::thread_start::h7de90ab6920a2ef0
  75:     0x7ffb43108609 - start_thread
  76:     0x7ffb48a1b133 - clone
  77:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (1f7ed23e4 2022-08-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z thir-unsafeck
query stack during panic:
#0 [layout_of] computing layout of `U`
#0 [layout_of] computing layout of `U`
#1 [eval_to_allocation_raw] const-evaluating + checking `C`
#2 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#3 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#4 [eval_to_valtree] evaluating type-level constant
#5 [typeck] type-checking `main`
#6 [typeck_item_bodies] type-checking all item bodies
#7 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
   |
   = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
              3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
              4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
              5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
              7: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
              9: <std::panicking::begin_panic_handler::PanicPayload as core::panic::BoxMeUp>::get
             10: std::panicking::rust_panic_with_hook
             11: std::panicking::begin_panic_handler::{{closure}}
             12: std::sys_common::backtrace::__rust_end_short_backtrace
             12: std::sys_common::backtrace::__rust_end_short_backtrace
             13: rust_begin_unwind
             14: core::panicking::panic_fmt
             15: rustc_middle::ty::layout_sanity_check::sanity_check_layout
             16: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<rustc_middle::ty::layout::layout_of::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}, core::result::Result<rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty>, rustc_middle::ty::layout::LayoutError>>::{closure#0}
             18: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::layout_of, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_of
             20: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             23: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::mir_const_to_op
             24: <rustc_const_eval::interpret::eval_context::InterpCx<rustc_const_eval::const_eval::machine::CompileTimeInterpreter>>::push_stack_frame
             25: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             26: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             28: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
             29: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
             30: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
             31: rustc_const_eval::const_eval::eval_to_valtree
             32: <rustc_const_eval::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::mir::interpret::GlobalId>)>>::call_once
             33: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_valtree, rustc_query_impl::plumbing::QueryCtxt>
             34: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_valtree
             35: <rustc_middle::ty::context::TyCtxt>::const_eval_global_id_for_typeck
             36: <rustc_middle::ty::context::TyCtxt>::const_eval_resolve_for_typeck
             37: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
             38: <rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
             39: <rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
             40: <rustc_trait_selection::traits::project::AssocTypeNormalizer>::fold::<rustc_middle::ty::Ty>
             41: rustc_trait_selection::traits::project::normalize::<rustc_middle::ty::Ty>
             42: <rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle::ty::Ty>
             43: <rustc_typeck::check::inherited::Inherited>::normalize_associated_types_in::<rustc_middle::ty::Ty>
             44: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner
             45: <rustc_typeck::check::fn_ctxt::FnCtxt>::to_ty
             46: <rustc_typeck::check::gather_locals::GatherLocalsVisitor>::declare
             47: <rustc_typeck::check::gather_locals::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_local
             48: rustc_hir::intravisit::walk_expr::<rustc_typeck::check::gather_locals::GatherLocalsVisitor>
             49: rustc_typeck::check::check::check_fn
             50: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
             51: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>
             52: rustc_typeck::check::typeck
             53: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
             54: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             55: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
             56: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             57: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             58: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>
             59: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
             60: rustc_typeck::check::typeck_item_bodies
             61: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
             62: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
             63: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies
             64: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
             65: rustc_typeck::check_crate
             67: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
             68: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             69: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             70: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             70: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
