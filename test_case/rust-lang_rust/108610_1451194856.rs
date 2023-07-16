plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling parking_lot v0.11.2
   Compiling rand_core v0.6.4
   Compiling odht v0.3.1
   Compiling termize v0.1.1
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             3: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
             6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             8: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             9: <rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay as rustc_errors::diagnostic::IntoDiagnosticArg>::into_diagnostic_arg
            10: <rustc_errors::diagnostic::Diagnostic>::set_arg::<&str, rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay>
            11: <rustc_session::session::Session>::emit_warning::<rustc_privacy::errors::WarnInPublicInterfaceTraits>
            12: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit_def_id
            13: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            14: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            15: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit::<rustc_middle::ty::Ty>
            16: <rustc_privacy::PrivateItemsInPublicInterfacesChecker>::check_assoc_item
            17: rustc_privacy::check_private_in_public
            18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_private_in_public, rustc_query_impl::plumbing::QueryCtxt>
            19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_private_in_public
            20: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}::{closure#0}>>
            21: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
            22: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#6}>
            24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            26: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            27: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            27: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            28: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            29: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            30: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            32: <unknown>
            33: <unknown>
          


   Compiling lazy_static v1.4.0
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (38be4b25f 2023-03-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `tracing-core`
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             3: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             6: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             7: <rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay as rustc_errors::diagnostic::IntoDiagnosticArg>::into_diagnostic_arg
             8: <rustc_errors::diagnostic::Diagnostic>::set_arg::<&str, rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay>
             9: <rustc_session::session::Session>::emit_warning::<rustc_privacy::errors::WarnInPublicInterfaceTraits>
            10: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit_def_id
            11: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            12: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            13: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit::<rustc_middle::ty::Ty>
            14: <rustc_privacy::PrivateItemsInPublicInterfacesChecker>::check_assoc_item
            15: rustc_privacy::check_private_in_public
            16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_private_in_public, rustc_query_impl::plumbing::QueryCtxt>
            17: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_private_in_public
            18: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}::{closure#0}>>
            19: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
            20: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#6}>
            22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            24: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            25: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            25: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            26: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            27: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            28: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            30: <unknown>
            31: <unknown>
          


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (38be4b25f 2023-03-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `tempfile`
error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             3: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             4: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
             6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::pretty::PrettyPrinter>::pretty_print_type
             8: <rustc_middle::ty::Ty as core::fmt::Display>::fmt
             9: <rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay as rustc_errors::diagnostic::IntoDiagnosticArg>::into_diagnostic_arg
            10: <rustc_errors::diagnostic::Diagnostic>::set_arg::<&str, rustc_errors::diagnostic_impls::DiagnosticArgFromDisplay>
            11: <rustc_session::session::Session>::emit_warning::<rustc_privacy::errors::WarnInPublicInterfaceTraits>
            12: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit_def_id
            13: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            14: <rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
            15: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            16: <rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
            17: <rustc_privacy::SearchInterfaceForPrivateItemsVisitor as rustc_privacy::DefIdVisitor>::visit::<rustc_middle::ty::Ty>
            18: rustc_privacy::check_private_in_public
            19: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_private_in_public, rustc_query_impl::plumbing::QueryCtxt>
            20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_private_in_public
            21: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}::{closure#0}>>
            22: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#6}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
            23: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#6}>
            25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            27: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            28: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            28: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
            29: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            30: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            31: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            33: <unknown>
            34: <unknown>
          


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (38be4b25f 2023-03-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -Z binary-dep-depinfo -Z unstable-options -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
