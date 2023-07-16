
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `infer_but_needs_to_satisfy`: 
the fluent string has an argument `has_param_name` that was not found.
help: the arguments `has_lifetime`, `lifetime` and `spans_empty` are available
', compiler/rustc_errors/src/emitter.rs:1391:84
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/core/src/panicking.rs:64:14
   2: core::result::unwrap_failed
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/core/src/result.rs:1790:5
   3: <rustc_errors::emitter::EmitterWriter>::emit_message_default::{closure#0}
   4: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
   5: <rustc_errors::json::Diagnostic>::from_errors_diagnostic
   6: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
   7: <rustc_errors::HandlerInner>::emit_diagnostic::{closure#2}
   8: rustc_interface::callbacks::track_diagnostic
   9: <rustc_errors::HandlerInner>::emit_diagnostic
  10: <rustc_errors::Handler>::emit_diagnostic
  11: <rustc_errors::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  12: <rustc_infer::infer::error_reporting::nice_region_error::NiceRegionError>::try_report
  13: <rustc_infer::infer::error_reporting::TypeErrCtxt>::try_report_nice_region_error
  14: <rustc_infer::infer::error_reporting::TypeErrCtxt>::check_region_obligations_and_report_errors
  15: rustc_hir_analysis::check::wfcheck::check_item_fn
  16: rustc_hir_analysis::check::wfcheck::check_well_formed
  17: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_hir::hir_id::OwnerId, ()>
  18: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
  19: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  20: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
  21: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
  24: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  25: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#5}, ()>
  26: rustc_hir_analysis::check_crate
  27: rustc_interface::passes::analysis
  28: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  29: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  30: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  31: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  32: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  33: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=1 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_well_formed] checking that `rust_error::cursor_state` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in module `rust_error`
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::{closure#0}, std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
             3: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>
             4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             5: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             8: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             9: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
            10: <rustc_infer::infer::error_reporting::nice_region_error::NiceRegionError>::try_report
            11: <rustc_infer::infer::error_reporting::TypeErrCtxt>::try_report_nice_region_error
            12: <rustc_infer::infer::error_reporting::TypeErrCtxt>::check_region_obligations_and_report_errors
            13: rustc_hir_analysis::check::wfcheck::check_item_fn
            14: rustc_hir_analysis::check::wfcheck::check_well_formed
            15: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_hir::hir_id::OwnerId, ()>
            16: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
            17: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
            18: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
            19: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
            20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
            21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
            22: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
            23: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#5}, ()>
            24: rustc_hir_analysis::check_crate
            25: rustc_interface::passes::analysis
            26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            27: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            28: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            29: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#2}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            30: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            31: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            32: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            33: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            34: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
            35: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/alloc/src/boxed.rs:1988:9
            36: std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library/std/src/sys/unix/thread.rs:108:17
            37: start_thread
                       at ./nptl/./nptl/pthread_create.c:442:8
            38: clone3
                       at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
          

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=1 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `art_of_card_rust`

Caused by:
  process didn't exit successfully: `rustc --crate-name art_of_card_rust --edition=2021 art_of_card_rust/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=223 --crate-type bin --emit=dep-info,metadata -C opt-level=1 -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C metadata=13758a3270c01dfe -C extra-filename=-13758a3270c01dfe --out-dir /home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps -C incremental=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/incremental -L dependency=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps --extern bevy=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps/libbevy-e00b874e6dfd8862.rmeta --extern bevy_egui=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps/libbevy_egui-dde1b91242d49388.rmeta --extern bevy_reflect=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps/libbevy_reflect-9554aedb42a31c47.rmeta --extern rand=/home/giaco/Progetti/Personale/rust_bevy_projects/target/debug/deps/librand-17fc224bf3495686.rmeta -L native=/usr/lib/x86_64-linux-gnu -L native=/usr/lib/x86_64-linux-gnu` (signal: 6, SIGABRT: process abort signal)

