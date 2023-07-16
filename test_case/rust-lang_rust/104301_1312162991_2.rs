
   Compiling build-std-test-ice v0.1.0 (/home/lukas/code/build-std-test-ice)
error[E0152]: duplicate lang item in crate `core` (which `rustc_std_workspace_core` depends on): `sized`.
  |
  = note: the lang item is first defined in crate `core` (which `build_std_test_ice` depends on)
  = note: first definition in `core` loaded from /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-3f68f86b034c02bd.rmeta
  = note: second definition in `core` loaded from /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-ca7d8634d68c826c.rmeta

error[E0463]: can't find crate for `test`

error[E0152]: duplicate lang item in crate `core` (which `rustc_std_workspace_core` depends on): `sized`.
  |
  = note: the lang item is first defined in crate `core` (which `build_std_test_ice` depends on)
  = note: first definition in `core` loaded from /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-3f68f86b034c02bd.rlib
  = note: second definition in `core` loaded from /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-ca7d8634d68c826c.rlib, /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-ca7d8634d68c826c.rmeta

error: `#[panic_handler]` function required, but not found

thread '<unnamed>' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
stack backtrace:
thread '<unnamed>' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:78:9
stack backtrace:
   0:     0x7f7d68766520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f7d68766520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f7d68766520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f7d68766520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f7d687c863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f7d68756795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   0:     0x7fc294f66520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc294f66520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   6:     0x7f7d687662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
   2:     0x7fc294f66520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   7:     0x7f7d687662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f7d687690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   3:     0x7fc294f66520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   9:     0x7f7d68768dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7f7d687698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7f7d68769667 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:579:13
  12:     0x7f7d687669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f7d68769382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7f7d687c5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7f7d687c5171 - core::panicking::panic_display::h4d285fa2a7b26f2a
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:139:5
  16:     0x7f7d687c511b - core::panicking::panic_str::h79efa26adb712718
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:123:5
  17:     0x7f7d687c4d96 - core::option::expect_failed::h8f4b55925d9edc3f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/option.rs:1879:5
   4:     0x7fc294fc863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fc294f56795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   6:     0x7fc294f662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc294f662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc294f690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   9:     0x7fc294f68dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7fc294f698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7fc294f69667 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:579:13
  12:     0x7fc294f669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fc294f69382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7fc294fc5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7fc294fc5171 - core::panicking::panic_display::h4d285fa2a7b26f2a
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:139:5
  16:     0x7fc294fc511b - core::panicking::panic_str::h79efa26adb712718
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:123:5
  17:     0x7fc294fc4d96 - core::option::expect_failed::h8f4b55925d9edc3f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/option.rs:1879:5
  18:     0x7f7d6af9860e - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter as rustc_errors[a1ce8dcef9ff7a7d]::translation::Translate>::translate_message
  19:     0x7f7d6af98112 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter>::msg_to_buffer
  20:     0x7f7d6afa3276 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter>::emit_message_default
  21:     0x7f7d6af8f8d3 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter as rustc_errors[a1ce8dcef9ff7a7d]::emitter::Emitter>::emit_diagnostic
  22:     0x7f7d6af8d2c2 - <rustc_errors[a1ce8dcef9ff7a7d]::json::Diagnostic>::from_errors_diagnostic
  23:     0x7f7d6af8c86c - <rustc_errors[a1ce8dcef9ff7a7d]::json::JsonEmitter as rustc_errors[a1ce8dcef9ff7a7d]::emitter::Emitter>::emit_diagnostic
  24:     0x7f7d69c3688a - <rustc_errors[a1ce8dcef9ff7a7d]::HandlerInner>::emit_diagnostic
  25:     0x7f7d6a74bf21 - <rustc_errors[a1ce8dcef9ff7a7d]::Handler>::emit_diagnostic
  26:     0x7f7d6a70934d - <rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed as rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  27:     0x7f7d6c0716dc - <rustc_session[47f761f7ecc78e0d]::parse::ParseSess>::emit_err::<rustc_passes[1a3c8d413335c20c]::errors::DuplicateDiagnosticItemInCrate>
  28:     0x7f7d6adb7156 - rustc_passes[1a3c8d413335c20c]::diagnostic_items::collect_item
  29:     0x7f7d6adb697d - rustc_passes[1a3c8d413335c20c]::diagnostic_items::all_diagnostic_items
  30:     0x7f7d6b184f48 - <rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind as rustc_query_system[731acdfd61cbbb95]::dep_graph::DepKind>::with_deps::<<rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>::{closure#0}, rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>
  31:     0x7f7d6b184a20 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>
  32:     0x7f7d6b183f56 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::ArenaCache<(), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>>
  33:     0x7f7d6b1838a7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::all_diagnostic_items, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  34:     0x7f7d6a4c2418 - <rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt>::get_diagnostic_item
  35:     0x7f7d6c4f3d7c - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::suggestions::TypeErrCtxtExt>::suggest_add_reference_to_arg
  36:     0x7f7d6c506a80 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  37:     0x7f7d6c50f568 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  38:     0x7f7d6c50385f - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  39:     0x7f7d69be0ef3 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_argument_types
  40:     0x7f7d69bac989 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_call
  41:     0x7f7d6a260f39 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  42:     0x7f7d6a2a33dc - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_block_with_expected
  43:     0x7f7d6a26146a - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  44:     0x7f7d6a09ef86 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_return_expr
  45:     0x7f7d6a096f4c - rustc_hir_typeck[6da16006badde0e1]::check::check_fn
  46:     0x7f7d6a082cdb - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  47:     0x7f7d6a0759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  48:     0x7f7d6a07f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  49:     0x7f7d6a0741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  50:     0x7f7d6b108008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  51:     0x7f7d6b107ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  52:     0x7f7d6afffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  53:     0x7f7d6affe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  54:     0x7f7d6affe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  55:     0x7f7d69dc7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  56:     0x7f7d69dc724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  57:     0x7f7d69dc6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  18:     0x7fc29779860e - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter as rustc_errors[a1ce8dcef9ff7a7d]::translation::Translate>::translate_message
  19:     0x7fc297798112 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter>::msg_to_buffer
  58:     0x7f7d6b1610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  20:     0x7fc2977a3276 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter>::emit_message_default
  21:     0x7fc29778f8d3 - <rustc_errors[a1ce8dcef9ff7a7d]::emitter::EmitterWriter as rustc_errors[a1ce8dcef9ff7a7d]::emitter::Emitter>::emit_diagnostic
  59:     0x7f7d6b160464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  22:     0x7fc29778d2c2 - <rustc_errors[a1ce8dcef9ff7a7d]::json::Diagnostic>::from_errors_diagnostic
  60:     0x7f7d6b15fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  23:     0x7fc29778c86c - <rustc_errors[a1ce8dcef9ff7a7d]::json::JsonEmitter as rustc_errors[a1ce8dcef9ff7a7d]::emitter::Emitter>::emit_diagnostic
  24:     0x7fc29643688a - <rustc_errors[a1ce8dcef9ff7a7d]::HandlerInner>::emit_diagnostic
  25:     0x7fc296f4bf21 - <rustc_errors[a1ce8dcef9ff7a7d]::Handler>::emit_diagnostic
  61:     0x7f7d6ac0fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  26:     0x7fc296f0934d - <rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed as rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  27:     0x7fc2988716dc - <rustc_session[47f761f7ecc78e0d]::parse::ParseSess>::emit_err::<rustc_passes[1a3c8d413335c20c]::errors::DuplicateDiagnosticItemInCrate>
  62:     0x7f7d6ac0cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  28:     0x7fc2975b7156 - rustc_passes[1a3c8d413335c20c]::diagnostic_items::collect_item
  29:     0x7fc2975b697d - rustc_passes[1a3c8d413335c20c]::diagnostic_items::all_diagnostic_items
  63:     0x7f7d6ac07c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  30:     0x7fc297984f48 - <rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind as rustc_query_system[731acdfd61cbbb95]::dep_graph::DepKind>::with_deps::<<rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>::{closure#0}, rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>
  64:     0x7f7d6ac0776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  31:     0x7fc297984a20 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>
  32:     0x7fc297983f56 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::ArenaCache<(), rustc_hir[b8f1839d7a3a65f1]::diagnostic_items::DiagnosticItems>>
  65:     0x7f7d6ac06d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  33:     0x7fc2979838a7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::all_diagnostic_items, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  34:     0x7fc296cc2418 - <rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt>::get_diagnostic_item
  35:     0x7fc298cf3d7c - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::suggestions::TypeErrCtxtExt>::suggest_add_reference_to_arg
  36:     0x7fc298d06a80 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  66:     0x7f7d6ac06a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7fc298d0f568 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  38:     0x7fc298d0385f - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  39:     0x7fc2963e0ef3 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_argument_types
  40:     0x7fc2963ac989 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_call
  41:     0x7fc296a60f39 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  42:     0x7fc296aa33dc - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_block_with_expected
  43:     0x7fc296a6146a - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  44:     0x7fc29689ef86 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_return_expr
  45:     0x7fc296896f4c - rustc_hir_typeck[6da16006badde0e1]::check::check_fn
  46:     0x7fc296882cdb - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  47:     0x7fc2968759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  48:     0x7fc29687f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  49:     0x7fc2968741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  50:     0x7fc297908008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  51:     0x7fc297907ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  52:     0x7fc2977ffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  53:     0x7fc2977fe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  54:     0x7fc2977fe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  55:     0x7fc2965c7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  56:     0x7fc2965c724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  57:     0x7fc2965c6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  58:     0x7fc2979610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  59:     0x7fc297960464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  60:     0x7fc29795fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  61:     0x7fc29740fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  62:     0x7fc29740cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  63:     0x7fc297407c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  64:     0x7fc29740776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  65:     0x7fc297406d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  66:     0x7fc297406a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  67:     0x7f7d6c698563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  68:     0x7f7d6c698563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  69:     0x7f7d6c698563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  70:     0x7f7d6850a8fd - <unknown>
  71:     0x7f7d6858ca60 - <unknown>
  72:                0x0 - <unknown>
error: internal compiler error: the following error was constructed but not emitted

error[E0277]: the size for values of type `()` cannot be known at compilation time
 --> src/lib.rs:6:21
  |
6 |     UnsafeCell::new(())
  |                     ^^

thread '<unnamed>' panicked at 'error was constructed but not emitted', compiler/rustc_errors/src/diagnostic_builder.rs:676:21
stack backtrace:
   0:     0x7f7d68766520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f7d68766520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f7d68766520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f7d68766520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f7d687c863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f7d68756795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   6:     0x7f7d687662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f7d687662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f7d687690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   9:     0x7f7d68768dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7f7d687698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7f7d68769621 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:577:13
  12:     0x7f7d687669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7f7d68769382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7f7d687c5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7f7d6a67c59b - <rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::DiagnosticBuilderInner as core[4e899ad432e99573]::ops::drop::Drop>::drop
  16:     0x7f7d6c4eea9d - core[4e899ad432e99573]::ptr::drop_in_place::<rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::DiagnosticBuilder<rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  17:     0x7f7d6c5093aa - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  18:     0x7f7d6c50f568 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  19:     0x7f7d6c50385f - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  20:     0x7f7d69be0ef3 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_argument_types
  21:     0x7f7d69bac989 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_call
  22:     0x7f7d6a260f39 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:     0x7f7d6a2a33dc - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f7d6a26146a - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7f7d6a09ef86 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7f7d6a096f4c - rustc_hir_typeck[6da16006badde0e1]::check::check_fn
  27:     0x7f7d6a082cdb - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  28:     0x7f7d6a0759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  29:     0x7f7d6a07f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  30:     0x7f7d6a0741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  31:     0x7f7d6b108008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  32:     0x7f7d6b107ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  33:     0x7f7d6afffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  34:     0x7f7d6affe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  35:     0x7f7d6affe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  36:     0x7f7d69dc7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  37:     0x7f7d69dc724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  38:     0x7f7d69dc6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  39:     0x7f7d6b1610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  40:     0x7f7d6b160464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  41:     0x7f7d6b15fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  42:     0x7f7d6ac0fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  43:     0x7f7d6ac0cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  44:     0x7f7d6ac07c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  45:     0x7f7d6ac0776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  67:     0x7fc298e98563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  46:     0x7f7d6ac06d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  68:     0x7fc298e98563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  69:     0x7fc298e98563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  47:     0x7f7d6ac06a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f7d6c698563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  70:     0x7fc294d0a8fd - <unknown>
  71:     0x7fc294d8ca60 - <unknown>
  49:     0x7f7d6c698563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
  72:                0x0 - <unknown>
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  50:     0x7f7d6c698563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  51:     0x7f7d6850a8fd - <unknown>
  52:     0x7f7d6858ca60 - <unknown>
  53:                0x0 - <unknown>
thread panicked while panicking. aborting.
thread '<unnamed>' panicked at 'error was constructed but not emitted', compiler/rustc_errors/src/diagnostic_builder.rs:676:21
stack backtrace:
   0:     0x7fc294f66520 - std::backtrace_rs::backtrace::libunwind::trace::h1c0259b0e54a0ec7
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc294f66520 - std::backtrace_rs::backtrace::trace_unsynchronized::hef156fa43f257fff
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc294f66520 - std::sys_common::backtrace::_print_fmt::h14c30be4df8a453b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc294f66520 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6bc9ef49ac207d73
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc294fc863e - core::fmt::write::h670e2d8b480677bf
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fc294f56795 - std::io::Write::write_fmt::h96cca416c9a25a8f
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/io/mod.rs:1682:15
   6:     0x7fc294f662e5 - std::sys_common::backtrace::_print::h209eac292799b1d3
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc294f662e5 - std::sys_common::backtrace::print::h02ded1b579fe176b
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc294f690bf - std::panicking::default_hook::{{closure}}::h8511971ca7cce5b1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:267:22
   9:     0x7fc294f68dfa - std::panicking::default_hook::h8bc8dfd96a84f923
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:286:9
  10:     0x7fc294f698cc - std::panicking::rust_panic_with_hook::h9f2bfce9b4c1819d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:688:13
  11:     0x7fc294f69621 - std::panicking::begin_panic_handler::{{closure}}::h167b5f05f9579bc1
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:577:13
  12:     0x7fc294f669cc - std::sys_common::backtrace::__rust_end_short_backtrace::h99451dfb597ab6ac
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys_common/backtrace.rs:137:18
  13:     0x7fc294f69382 - rust_begin_unwind
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/panicking.rs:575:5
  14:     0x7fc294fc5023 - core::panicking::panic_fmt::hf9d641a075644e68
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/core/src/panicking.rs:65:14
  15:     0x7fc296e7c59b - <rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::DiagnosticBuilderInner as core[4e899ad432e99573]::ops::drop::Drop>::drop
  16:     0x7fc298ceea9d - core[4e899ad432e99573]::ptr::drop_in_place::<rustc_errors[a1ce8dcef9ff7a7d]::diagnostic_builder::DiagnosticBuilder<rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  17:     0x7fc298d093aa - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error
  18:     0x7fc298d0f568 - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  19:     0x7fc298d0385f - <rustc_infer[183205aa2824036]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[1a866c97ed6ff1a8]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  20:     0x7fc2963e0ef3 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_argument_types
  21:     0x7fc2963ac989 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_call
  22:     0x7fc296a60f39 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  23:     0x7fc296aa33dc - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7fc296a6146a - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7fc29689ef86 - <rustc_hir_typeck[6da16006badde0e1]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7fc296896f4c - rustc_hir_typeck[6da16006badde0e1]::check::check_fn
  27:     0x7fc296882cdb - <rustc_hir_typeck[6da16006badde0e1]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6da16006badde0e1]::typeck_with_fallback<rustc_hir_typeck[6da16006badde0e1]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  28:     0x7fc2968759dd - rustc_hir_typeck[6da16006badde0e1]::typeck
  29:     0x7fc29687f513 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>
  30:     0x7fc2968741a4 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<rustc_span[2c66fb4e929445ca]::def_id::LocalDefId, &rustc_middle[ef15c0f09f0ff036]::ty::context::TypeckResults>>
  31:     0x7fc297908008 - rustc_data_structures[fcbea81f06a7da6b]::sync::par_for_each_in::<&[rustc_span[2c66fb4e929445ca]::def_id::LocalDefId], <rustc_middle[ef15c0f09f0ff036]::hir::map::Map>::par_body_owners<rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies::{closure#0}>::{closure#0}>
  32:     0x7fc297907ce3 - rustc_hir_typeck[6da16006badde0e1]::typeck_item_bodies
  33:     0x7fc2977ffa19 - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), ()>
  34:     0x7fc2977fe9bd - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), ()>>
  35:     0x7fc2977fe347 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::typeck_item_bodies, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  36:     0x7fc2965c7b0a - <rustc_session[47f761f7ecc78e0d]::session::Session>::time::<(), rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate::{closure#7}>
  37:     0x7fc2965c724b - rustc_hir_analysis[7cbe4a3e57699ef0]::check_crate
  38:     0x7fc2965c6e7b - rustc_interface[d2b1e7858cec3daa]::passes::analysis
  39:     0x7fc2979610ee - <rustc_query_system[731acdfd61cbbb95]::dep_graph::graph::DepGraph<rustc_middle[ef15c0f09f0ff036]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[ef15c0f09f0ff036]::ty::context::TyCtxt, (), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  40:     0x7fc297960464 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::try_execute_query::<rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt, rustc_query_system[731acdfd61cbbb95]::query::caches::DefaultCache<(), core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>>
  41:     0x7fc29795fee7 - rustc_query_system[731acdfd61cbbb95]::query::plumbing::get_query::<rustc_query_impl[5d8cc954b14a6b51]::queries::analysis, rustc_query_impl[5d8cc954b14a6b51]::plumbing::QueryCtxt>
  42:     0x7fc29740fb8e - <rustc_interface[d2b1e7858cec3daa]::passes::QueryContext>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  43:     0x7fc29740cc5f - <rustc_interface[d2b1e7858cec3daa]::interface::Compiler>::enter::<rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}::{closure#2}, core[4e899ad432e99573]::result::Result<core[4e899ad432e99573]::option::Option<rustc_interface[d2b1e7858cec3daa]::queries::Linker>, rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  44:     0x7fc297407c92 - rustc_span[2c66fb4e929445ca]::with_source_map::<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  45:     0x7fc29740776c - <scoped_tls[a162aae2f2befadc]::ScopedKey<rustc_span[2c66fb4e929445ca]::SessionGlobals>>::set::<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  46:     0x7fc297406d58 - std[60fbac34357a1008]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>
  47:     0x7fc297406a7c - <<std[60fbac34357a1008]::thread::Builder>::spawn_unchecked_<rustc_interface[d2b1e7858cec3daa]::util::run_in_thread_pool_with_globals<rustc_interface[d2b1e7858cec3daa]::interface::run_compiler<core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>, rustc_driver[766838ccb0cc5547]::run_compiler::{closure#1}>::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[4e899ad432e99573]::result::Result<(), rustc_errors[a1ce8dcef9ff7a7d]::ErrorGuaranteed>>::{closure#1} as core[4e899ad432e99573]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7fc298e98563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c5bc954debcee72
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  49:     0x7fc298e98563 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he1973107145a5c1d
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/alloc/src/boxed.rs:2000:9
  50:     0x7fc298e98563 - std::sys::unix::thread::Thread::new::thread_start::h3f40abd87f665b17
                               at /rustc/c1a859b25a95999ba276075bbd8e284ea675b41a/library/std/src/sys/unix/thread.rs:108:17
  51:     0x7fc294d0a8fd - <unknown>
  52:     0x7fc294d8ca60 - <unknown>
  53:                0x0 - <unknown>
thread panicked while panicking. aborting.
error: could not compile `build-std-test-ice` due to 2 previous errors

Caused by:
  process didn't exit successfully: `rustc --crate-name build_std_test_ice --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=b730d1e69ea6f03d -C extra-filename=-b730d1e69ea6f03d --out-dir /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps --target x86_64-unknown-none -C incremental=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/incremental -L dependency=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps -L dependency=/home/lukas/code/build-std-test-ice/target/debug/deps --extern 'noprelude:compiler_builtins=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcompiler_builtins-f99e1711e5f3d440.rmeta' --extern 'noprelude:core=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-3f68f86b034c02bd.rmeta' -Z unstable-options` (signal: 6, SIGABRT: process abort signal)
warning: build failed, waiting for other jobs to finish...
error: could not compile `build-std-test-ice` due to 4 previous errors

Caused by:
  process didn't exit successfully: `rustc --crate-name build_std_test_ice --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test -C metadata=843f1f74bc06120e -C extra-filename=-843f1f74bc06120e --out-dir /home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps --target x86_64-unknown-none -C incremental=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/incremental -L dependency=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps -L dependency=/home/lukas/code/build-std-test-ice/target/debug/deps --extern 'noprelude:compiler_builtins=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcompiler_builtins-f99e1711e5f3d440.rlib' --extern 'noprelude:core=/home/lukas/code/build-std-test-ice/target/x86_64-unknown-none/debug/deps/libcore-3f68f86b034c02bd.rlib' -Z unstable-options` (signal: 6, SIGABRT: process abort signal)
