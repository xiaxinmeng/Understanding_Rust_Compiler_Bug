plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [ui] tests/ui/issues/issue-106755.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-106755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-106755" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-106755/auxiliary" "-Ztranslate-lang=en_US"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: failed while formatting fluent string `trait_selection_negative_positive_conflict`: 
', compiler/rustc_errors/src/emitter.rs:1404:84
stack backtrace:
stack backtrace:
   0:     0x7f33195ed085 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfbeccf7ebe4c0d88
   1:     0x7f3319656938 - core::fmt::write::h4a7ff26f31818e82
   2:     0x7f33195df081 - std::io::Write::write_fmt::hed27fc53ff51fd92
   3:     0x7f33195ece91 - std::sys_common::backtrace::print::hd63f99539af3aab5
   4:     0x7f33195f0094 - std::panicking::default_hook::{{closure}}::h06bd5219fde9ec64
   5:     0x7f33195efd7a - std::panicking::default_hook::h7e1a69a51ab3909e
   6:     0x7f331a0d04a5 - rustc_driver_impl[4c499b68a135fea7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f33195f07b1 - std::panicking::rust_panic_with_hook::h7f5cf5ec3776d45a
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   8:     0x7f33195f0529 - std::panicking::begin_panic_handler::{{closure}}::h19ac8d965f46746f
   9:     0x7f33195ed556 - std::sys_common::backtrace::__rust_end_short_backtrace::h2aee877693b03c01
  10:     0x7f33195f0207 - rust_begin_unwind
  11:     0x7f33195a3303 - core::panicking::panic_fmt::h96c6a9e390ffa3b8
  12:     0x7f33195a3803 - core::result::unwrap_failed::h18ed95134df3035f
  13:     0x7f331d0cc057 - <rustc_errors[ca46ee07aa386081]::emitter::EmitterWriter>::emit_message_default::{closure#0}
  14:     0x7f331d0c5ca1 - <rustc_errors[ca46ee07aa386081]::emitter::EmitterWriter>::emit_message_default
  15:     0x7f331d0baa69 - <rustc_errors[ca46ee07aa386081]::emitter::EmitterWriter as rustc_errors[ca46ee07aa386081]::emitter::Emitter>::emit_diagnostic
  16:     0x7f331d0b12f5 - <rustc_errors[ca46ee07aa386081]::json::Diagnostic>::from_errors_diagnostic
  17:     0x7f331d0afdac - <rustc_errors[ca46ee07aa386081]::json::JsonEmitter as rustc_errors[ca46ee07aa386081]::emitter::Emitter>::emit_diagnostic
  18:     0x7f331d0a163b - <rustc_errors[ca46ee07aa386081]::HandlerInner>::emit_diagnostic::{closure#2}
  19:     0x7f331a19272a - rustc_interface[4f92da131eccc5dd]::callbacks::track_diagnostic
  20:     0x7f331d0a0d4a - <rustc_errors[ca46ee07aa386081]::HandlerInner>::emit_diagnostic
  21:     0x7f331d09b1a6 - <rustc_errors[ca46ee07aa386081]::ErrorGuaranteed as rustc_errors[ca46ee07aa386081]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f331c9ea4d8 - rustc_trait_selection[cb4d47436cc8480]::traits::specialize::report_negative_positive_conflict
  23:     0x7f331a03a962 - rustc_trait_selection[cb4d47436cc8480]::traits::specialize::report_overlap_conflict
  24:     0x7f331c9ea2b4 - rustc_trait_selection[cb4d47436cc8480]::traits::specialize::specialization_graph_provider
  25:     0x7f331bdd4c51 - rustc_query_system[b9941edda870110f]::query::plumbing::try_execute_query::<rustc_query_impl[96deca06f8516bcc]::queries::specialization_graph_of, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt>
  26:     0x7f331bea0be3 - rustc_query_system[b9941edda870110f]::query::plumbing::get_query::<rustc_query_impl[96deca06f8516bcc]::queries::specialization_graph_of, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt, rustc_middle[c476d66c62946628]::dep_graph::dep_node::DepKind>
  27:     0x7f331bcbfec5 - <rustc_query_impl[96deca06f8516bcc]::Queries as rustc_middle[c476d66c62946628]::ty::query::QueryEngine>::specialization_graph_of
  28:     0x7f331a84f1f9 - rustc_hir_analysis[2bf96db5ee7eac1d]::coherence::coherent_trait
  29:     0x7f331bd6f1e2 - rustc_query_system[b9941edda870110f]::query::plumbing::try_execute_query::<rustc_query_impl[96deca06f8516bcc]::queries::coherent_trait, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt>
  30:     0x7f331be6cccf - rustc_query_system[b9941edda870110f]::query::plumbing::get_query::<rustc_query_impl[96deca06f8516bcc]::queries::coherent_trait, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt, rustc_middle[c476d66c62946628]::dep_graph::dep_node::DepKind>
  31:     0x7f331bca85a5 - <rustc_query_impl[96deca06f8516bcc]::Queries as rustc_middle[c476d66c62946628]::ty::query::QueryEngine>::coherent_trait
  32:     0x7f331a93e65c - rustc_hir_analysis[2bf96db5ee7eac1d]::check_crate
  33:     0x7f331a1a0fd8 - rustc_interface[4f92da131eccc5dd]::passes::analysis
  34:     0x7f331bdf7d69 - rustc_query_system[b9941edda870110f]::query::plumbing::try_execute_query::<rustc_query_impl[96deca06f8516bcc]::queries::analysis, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt>
  35:     0x7f331bebed71 - rustc_query_system[b9941edda870110f]::query::plumbing::get_query::<rustc_query_impl[96deca06f8516bcc]::queries::analysis, rustc_query_impl[96deca06f8516bcc]::plumbing::QueryCtxt, rustc_middle[c476d66c62946628]::dep_graph::dep_node::DepKind>
  36:     0x7f331bc7b1ba - <rustc_query_impl[96deca06f8516bcc]::Queries as rustc_middle[c476d66c62946628]::ty::query::QueryEngine>::analysis
  37:     0x7f331a0d3750 - <rustc_middle[c476d66c62946628]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>
  38:     0x7f331a11c768 - <rustc_interface[4f92da131eccc5dd]::interface::Compiler>::enter::<rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}::{closure#2}, core[cf5a563bdb414e9a]::result::Result<core[cf5a563bdb414e9a]::option::Option<rustc_interface[4f92da131eccc5dd]::queries::Linker>, rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>
  39:     0x7f331a0d17af - rustc_span[74a5cba763f936d6]::with_source_map::<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_interface[4f92da131eccc5dd]::interface::run_compiler<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7f331a0e3cfb - <scoped_tls[efbe636d56d09a0f]::ScopedKey<rustc_span[74a5cba763f936d6]::SessionGlobals>>::set::<rustc_interface[4f92da131eccc5dd]::interface::run_compiler<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}>::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>
  41:     0x7f331a0dfd49 - std[9fc7fd8e5b5780a6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4f92da131eccc5dd]::util::run_in_thread_pool_with_globals<rustc_interface[4f92da131eccc5dd]::interface::run_compiler<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}>::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>
  42:     0x7f331a11e646 - std[9fc7fd8e5b5780a6]::panicking::try::<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, core[cf5a563bdb414e9a]::panic::unwind_safe::AssertUnwindSafe<<std[9fc7fd8e5b5780a6]::thread::Builder>::spawn_unchecked_<rustc_interface[4f92da131eccc5dd]::util::run_in_thread_pool_with_globals<rustc_interface[4f92da131eccc5dd]::interface::run_compiler<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}>::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7f331a0e0e25 - <<std[9fc7fd8e5b5780a6]::thread::Builder>::spawn_unchecked_<rustc_interface[4f92da131eccc5dd]::util::run_in_thread_pool_with_globals<rustc_interface[4f92da131eccc5dd]::interface::run_compiler<core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>, rustc_driver_impl[4c499b68a135fea7]::run_compiler::{closure#1}>::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cf5a563bdb414e9a]::result::Result<(), rustc_errors[ca46ee07aa386081]::ErrorGuaranteed>>::{closure#1} as core[cf5a563bdb414e9a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f33195fca2e - std::sys::unix::thread::Thread::new::thread_start::h8e2b8fc845c1d9e1
  45:     0x7f3319393b43 - <unknown>
  46:     0x7f3319425a00 - <unknown>
  47:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (ab570dc2f 2023-02-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z translate-lang=en_US
query stack during panic:
query stack during panic:
#0 [specialization_graph_of] building specialization graph of trait `core::marker::Send`
#1 [coherent_trait] coherence checking all impls of trait `core::marker::Send`
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed but no error emitted; use `DelayDm` for lints or `with_no_trimmed_paths` for debugging
   |
   = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
              1: rustc_middle::ty::print::pretty::trimmed_def_paths
              2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
              3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
              4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
              5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              6: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::default_print_def_path
              7: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
              8: <rustc_middle::ty::print::pretty::TraitRefPrintOnlyTraitPath as core::fmt::Display>::fmt
              9: <rustc_trait_selection::errors::NegativePositiveConflict as rustc_errors::diagnostic_builder::IntoDiagnostic>::into_diagnostic
             10: rustc_trait_selection::traits::specialize::report_negative_positive_conflict
             11: rustc_trait_selection::traits::specialize::report_overlap_conflict
             12: rustc_trait_selection::traits::specialize::specialization_graph_provider
             13: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::specialization_graph_of, rustc_query_impl::plumbing::QueryCtxt>
             14: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::specialization_graph_of, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::specialization_graph_of
             17: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::coherent_trait, rustc_query_impl::plumbing::QueryCtxt>
             18: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::coherent_trait, rustc_query_impl::plumbing::QueryCtxt, rustc_middle::dep_graph::dep_node::DepKind>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait
             20: rustc_hir_analysis::check_crate
---
             25: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             26: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
             27: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             28: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             29: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
             30: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             31: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             33: <unknown>
             34: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (ab570dc2f 2023-02-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z translate-lang=en_US
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
