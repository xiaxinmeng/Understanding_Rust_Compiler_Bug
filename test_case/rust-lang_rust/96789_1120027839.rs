plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'attempt to subtract with overflow', /checkout/compiler/rustc_target/src/abi/mod.rs:380:21
stack backtrace:
   0:     0x7fb20dc9dc52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7fb20dd05618 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fb20dc8e051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7fb20dca0f96 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7fb20dca0b8d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7fb20e7ee471 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb20dca1930 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7fb20dca1709 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7fb20dc9e1f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7fb20dca1439 - rust_begin_unwind
  10:     0x7fb20dc550b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7fb20dc54f7d - core::panicking::panic::h6966bf306ecf1686
  12:     0x7fb210f82eee - <rustc_middle[8d4dc3708b593ac1]::ty::consts::int::ConstInt as core[10878fb91fc84a80]::fmt::Debug>::fmt
  13:     0x7fb20dd05582 - core::fmt::write::h42234c3e51154f4c
  14:     0x7fb211111c3f - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const_value
  15:     0x7fb21110fbe6 - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const
  16:     0x7fb21112c6fc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::default_print_def_path
  17:     0x7fb2111134dc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::print_def_path
  18:     0x7fb21108de1d - <rustc_middle[8d4dc3708b593ac1]::ty::instance::Instance as core[10878fb91fc84a80]::fmt::Display>::fmt
  19:     0x7fb20fd6c8c1 - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_allocation_raw_provider
  20:     0x7fb20ffa5748 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_allocation_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  21:     0x7fb210337f91 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_allocation_raw
  22:     0x7fb20fd6aa0b - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_const_value_raw_provider
  23:     0x7fb20ffac834 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_const_value_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  24:     0x7fb210338561 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_const_value_raw
  25:     0x7fb20fd6a2df - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_const_value_raw_provider
  26:     0x7fb20ffac834 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_const_value_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  27:     0x7fb210338561 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_const_value_raw
  28:     0x7fb210fc6f8e - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::const_eval_global_id
  29:     0x7fb210fee00a - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::const_eval_resolve
  30:     0x7fb210c3b7d9 - <rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::QueryNormalizer as rustc_middle[8d4dc3708b593ac1]::ty::fold::FallibleTypeFolder>::try_fold_const
  31:     0x7fb210c3b9a7 - <rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::QueryNormalizer as rustc_middle[8d4dc3708b593ac1]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  32:     0x7fb20faeedd2 - <rustc_infer[26ac34c435530b6]::infer::at::At as rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::AtExt>::normalize::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  33:     0x7fb20fafc74c - <rustc_infer[26ac34c435530b6]::infer::InferCtxtBuilder>::enter::<core[10878fb91fc84a80]::result::Result<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind, rustc_middle[8d4dc3708b593ac1]::traits::query::NoSolution>, rustc_traits[638df15811e57f20]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>::{closure#0}>
  34:     0x7fb20fbe1d3e - <rustc_traits[638df15811e57f20]::normalize_erasing_regions::provide::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_middle[8d4dc3708b593ac1]::ty::ParamEnvAnd<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>)>>::call_once
  35:     0x7fb20ffca31a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  36:     0x7fb21036645f - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  37:     0x7fb20ec8d14c - <rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind as rustc_middle[8d4dc3708b593ac1]::ty::fold::TypeFoldable>::fold_with::<rustc_middle[8d4dc3708b593ac1]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  38:     0x7fb20ec7cbed - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  39:     0x7fb20ec7d9e1 - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  40:     0x7fb20ec6ed81 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_constant
  41:     0x7fb20ec70642 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_operand
  42:     0x7fb20ec6e782 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_rvalue
  43:     0x7fb20ec7334d - rustc_monomorphize[872f0885fec0fe91]::collector::collect_neighbours
  44:     0x7fb20ec6b612 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  45:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  46:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  47:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  48:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  49:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  50:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  51:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  52:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  53:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  54:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  55:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  56:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  57:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  58:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  59:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  60:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  61:     0x7fb20ec891b2 - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_monomorphize[872f0885fec0fe91]::collector::collect_crate_mono_items::{closure#1}>
  62:     0x7fb20ec68138 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_crate_mono_items
  63:     0x7fb20ec90b2f - rustc_monomorphize[872f0885fec0fe91]::partitioning::collect_and_partition_mono_items
  64:     0x7fb20ff0a7e5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), (&std[eba90a372f7a1edd]::collections::hash::set::HashSet<rustc_span[5c736203a6ab5594]::def_id::DefId, core[10878fb91fc84a80]::hash::BuildHasherDefault<rustc_hash[a38279527717d343]::FxHasher>>, &[rustc_middle[8d4dc3708b593ac1]::mir::mono::CodegenUnit])>>
  65:     0x7fb20ffc40ea - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::collect_and_partition_mono_items, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  66:     0x7fb210363319 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::collect_and_partition_mono_items
  67:     0x7fb20ea5b3cd - <rustc_codegen_llvm[e1ed357d67eb2340]::LlvmCodegenBackend as rustc_codegen_ssa[6c8b68572c710060]::traits::backend::CodegenBackend>::codegen_crate
  68:     0x7fb20e909c08 - <rustc_session[2a929b385c5bc398]::session::Session>::time::<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_interface[fc3bf7b819dbb0d8]::passes::start_codegen::{closure#0}>
  69:     0x7fb20e8fdd78 - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<<rustc_interface[fc3bf7b819dbb0d8]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  70:     0x7fb20e8e08ae - <rustc_interface[fc3bf7b819dbb0d8]::queries::Queries>::ongoing_codegen
  71:     0x7fb20e783f78 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  72:     0x7fb20e766016 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  73:     0x7fb20e78520f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  74:     0x7fb20e7d9e49 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  75:     0x7fb20e798801 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  76:     0x7fb20e7dc0c2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  77:     0x7fb20dcae333 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  78:     0x7fb2081fe609 - start_thread
  79:     0x7fb20db11163 - clone
  80:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d82701a3 2022-05-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'attempt to subtract with overflow', /checkout/compiler/rustc_target/src/abi/mod.rs:380:21
thread 'rustc' panicked at 'attempt to subtract with overflow', /checkout/compiler/rustc_target/src/abi/mod.rs:380:21
stack backtrace:
   0:     0x7fb20dc9dc52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7fb20dd05618 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fb20dc8e051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7fb20dca0f96 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7fb20dca0b8d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7fb20e7ee471 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb20dca1930 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7fb20dca1709 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7fb20dc9e1f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7fb20dca1439 - rust_begin_unwind
  10:     0x7fb20dc550b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7fb20dc54f7d - core::panicking::panic::h6966bf306ecf1686
  12:     0x7fb210f82eee - <rustc_middle[8d4dc3708b593ac1]::ty::consts::int::ConstInt as core[10878fb91fc84a80]::fmt::Debug>::fmt
  13:     0x7fb20dd05582 - core::fmt::write::h42234c3e51154f4c
  14:     0x7fb211111c3f - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const_value
  15:     0x7fb21110fbe6 - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const
  16:     0x7fb21112c6fc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::default_print_def_path
  17:     0x7fb2111134dc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::print_def_path
  18:     0x7fb21110fcb8 - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const
  19:     0x7fb2111449e7 - rustc_middle[8d4dc3708b593ac1]::mir::pretty_print_const
  20:     0x7fb20dd05618 - core::fmt::write::h42234c3e51154f4c
  21:     0x7fb20dcf54c9 - alloc::fmt::format::h1f2e8e1f2bb5eeae
  22:     0x7fb2100f00e0 - <rustc_query_impl[b8302d1c2e1c865b]::queries::try_normalize_mir_const_after_erasing_regions as rustc_query_system[4cf300c5f65a2dfc]::query::config::QueryDescription<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>>::describe
  23:     0x7fb2102e39d6 - rustc_query_impl[b8302d1c2e1c865b]::make_query::try_normalize_mir_const_after_erasing_regions
  24:     0x7fb20fe5aaf2 - <rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::QueryState<rustc_middle[8d4dc3708b593ac1]::ty::ParamEnvAnd<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>>>::try_collect_active_jobs::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  25:     0x7fb210312822 - <rustc_query_impl[b8302d1c2e1c865b]::Queries>::try_collect_active_jobs
  26:     0x7fb2102bebed - <rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>::try_print_query_stack
  27:     0x7fb20e975f94 - rustc_interface[fc3bf7b819dbb0d8]::interface::try_print_query_stack
  28:     0x7fb20e7ef008 - rustc_driver[87b42345fa270eee]::report_ice
  29:     0x7fb20dca1930 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
  30:     0x7fb20dca1709 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
  31:     0x7fb20dc9e1f4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
  32:     0x7fb20dca1439 - rust_begin_unwind
  33:     0x7fb20dc550b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  34:     0x7fb20dc54f7d - core::panicking::panic::h6966bf306ecf1686
  35:     0x7fb210f82eee - <rustc_middle[8d4dc3708b593ac1]::ty::consts::int::ConstInt as core[10878fb91fc84a80]::fmt::Debug>::fmt
  36:     0x7fb20dd05582 - core::fmt::write::h42234c3e51154f4c
  37:     0x7fb211111c3f - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const_value
  38:     0x7fb21110fbe6 - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::PrettyPrinter>::pretty_print_const
  39:     0x7fb21112c6fc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::default_print_def_path
  40:     0x7fb2111134dc - <rustc_middle[8d4dc3708b593ac1]::ty::print::pretty::FmtPrinter as rustc_middle[8d4dc3708b593ac1]::ty::print::Printer>::print_def_path
  41:     0x7fb21108de1d - <rustc_middle[8d4dc3708b593ac1]::ty::instance::Instance as core[10878fb91fc84a80]::fmt::Display>::fmt
  42:     0x7fb20fd6c8c1 - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_allocation_raw_provider
  43:     0x7fb20ffa5748 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_allocation_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  44:     0x7fb210337f91 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_allocation_raw
  45:     0x7fb20fd6aa0b - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_const_value_raw_provider
  46:     0x7fb20ffac834 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_const_value_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  47:     0x7fb210338561 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_const_value_raw
  48:     0x7fb20fd6a2df - rustc_const_eval[463eeed51a022010]::const_eval::eval_queries::eval_to_const_value_raw_provider
  49:     0x7fb20ffac834 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::eval_to_const_value_raw, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  50:     0x7fb210338561 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::eval_to_const_value_raw
  51:     0x7fb210fc6f8e - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::const_eval_global_id
  52:     0x7fb210fee00a - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::const_eval_resolve
  53:     0x7fb210c3b7d9 - <rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::QueryNormalizer as rustc_middle[8d4dc3708b593ac1]::ty::fold::FallibleTypeFolder>::try_fold_const
  54:     0x7fb210c3b9a7 - <rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::QueryNormalizer as rustc_middle[8d4dc3708b593ac1]::ty::fold::FallibleTypeFolder>::try_fold_mir_const
  55:     0x7fb20faeedd2 - <rustc_infer[26ac34c435530b6]::infer::at::At as rustc_trait_selection[617f9a3a34fa734c]::traits::query::normalize::AtExt>::normalize::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  56:     0x7fb20fafc74c - <rustc_infer[26ac34c435530b6]::infer::InferCtxtBuilder>::enter::<core[10878fb91fc84a80]::result::Result<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind, rustc_middle[8d4dc3708b593ac1]::traits::query::NoSolution>, rustc_traits[638df15811e57f20]::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>::{closure#0}>
  57:     0x7fb20fbe1d3e - <rustc_traits[638df15811e57f20]::normalize_erasing_regions::provide::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_middle[8d4dc3708b593ac1]::ty::ParamEnvAnd<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>)>>::call_once
  58:     0x7fb20ffca31a - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::try_normalize_mir_const_after_erasing_regions, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  59:     0x7fb21036645f - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::try_normalize_mir_const_after_erasing_regions
  60:     0x7fb20ec8d14c - <rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind as rustc_middle[8d4dc3708b593ac1]::ty::fold::TypeFoldable>::fold_with::<rustc_middle[8d4dc3708b593ac1]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>
  61:     0x7fb20ec7cbed - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  62:     0x7fb20ec7d9e1 - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::<rustc_middle[8d4dc3708b593ac1]::mir::ConstantKind>
  63:     0x7fb20ec6ed81 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_constant
  64:     0x7fb20ec70642 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_operand
  65:     0x7fb20ec6e782 - <rustc_monomorphize[872f0885fec0fe91]::collector::MirNeighborCollector as rustc_middle[8d4dc3708b593ac1]::mir::visit::Visitor>::visit_rvalue
  66:     0x7fb20ec7334d - rustc_monomorphize[872f0885fec0fe91]::collector::collect_neighbours
  67:     0x7fb20ec6b612 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  68:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  69:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  70:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  71:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  72:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  73:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  74:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  75:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  76:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  77:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  78:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  79:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  80:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  81:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  82:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  83:     0x7fb20ec6b8d8 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_items_rec
  84:     0x7fb20ec891b2 - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_monomorphize[872f0885fec0fe91]::collector::collect_crate_mono_items::{closure#1}>
  85:     0x7fb20ec68138 - rustc_monomorphize[872f0885fec0fe91]::collector::collect_crate_mono_items
  86:     0x7fb20ec90b2f - rustc_monomorphize[872f0885fec0fe91]::partitioning::collect_and_partition_mono_items
  87:     0x7fb20ff0a7e5 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), (&std[eba90a372f7a1edd]::collections::hash::set::HashSet<rustc_span[5c736203a6ab5594]::def_id::DefId, core[10878fb91fc84a80]::hash::BuildHasherDefault<rustc_hash[a38279527717d343]::FxHasher>>, &[rustc_middle[8d4dc3708b593ac1]::mir::mono::CodegenUnit])>>
  88:     0x7fb20ffc40ea - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::collect_and_partition_mono_items, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  89:     0x7fb210363319 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::collect_and_partition_mono_items
  90:     0x7fb20ea5b3cd - <rustc_codegen_llvm[e1ed357d67eb2340]::LlvmCodegenBackend as rustc_codegen_ssa[6c8b68572c710060]::traits::backend::CodegenBackend>::codegen_crate
  91:     0x7fb20e909c08 - <rustc_session[2a929b385c5bc398]::session::Session>::time::<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_interface[fc3bf7b819dbb0d8]::passes::start_codegen::{closure#0}>
  92:     0x7fb20e8fdd78 - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<<rustc_interface[fc3bf7b819dbb0d8]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[10878fb91fc84a80]::result::Result<alloc[24f448636cd10183]::boxed::Box<dyn core[10878fb91fc84a80]::any::Any>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  93:     0x7fb20e8e08ae - <rustc_interface[fc3bf7b819dbb0d8]::queries::Queries>::ongoing_codegen
  94:     0x7fb20e783f78 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  95:     0x7fb20e766016 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  96:     0x7fb20e78520f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  97:     0x7fb20e7d9e49 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  98:     0x7fb20e798801 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  99:     0x7fb20e7dc0c2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 100:     0x7fb20dcae333 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
 101:     0x7fb2081fe609 - start_thread
 102:     0x7fb20db11163 - clone
 103:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (4d82701a3 2022-05-06) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C metadata=a2a7040fb9f918eb -C extra-filename=-a2a7040fb9f918eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zunstable-options -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:03:58
