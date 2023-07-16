plain
........................................................................................ 10384/13441
........................................................................................ 10472/13441
........................................................................................ 10560/13441
........................................................................................ 10648/13441
.................iiiiiF..i....i.i....................................................... 10736/13441
....................................................................................iiii 10912/13441
ii.i..i.iiiiii.......................................................................... 11000/13441
........................................................................................ 11088/13441
........................................................................................ 11176/13441
---
failures:

---- [ui] src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs#disallowed stdout ----

error in revision `disallowed`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/irrefutable-lets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "disallowed" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/irrefutable-lets.disallowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/irrefutable-lets.disallowed/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
stack backtrace:
   0:     0x7fa254592e5c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2c1e712af9cf0d91
   1:     0x7fa2545fb8a8 - core::fmt::write::hff139303b3f10574
   2:     0x7fa254583901 - std::io::Write::write_fmt::h480c2e010b0b2928
   3:     0x7fa254595e4e - std::panicking::default_hook::{{closure}}::h06ab8d6f4a00ca9c
   4:     0x7fa254595b17 - std::panicking::default_hook::hf402eefa442b9145
   5:     0x7fa254f1e7b4 - rustc_driver[c23444e27ad84b6d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa254596601 - std::panicking::rust_panic_with_hook::h6beb59117729c24b
   7:     0x7fa254596427 - std::panicking::begin_panic_handler::{{closure}}::h77b8ad8a47a1db19
   8:     0x7fa2545933d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h89eab5a9f5f10a2d
   9:     0x7fa2545960f2 - rust_begin_unwind
  10:     0x7fa254546e43 - core::panicking::panic_fmt::h02e51f3221f88c9f
  11:     0x7fa2545f7f31 - core::panicking::panic_display::hd4b117d4a34e960b
  12:     0x7fa2545f7edb - core::panicking::panic_str::h2b63f9253ada071c
  13:     0x7fa254546cb6 - core::option::expect_failed::hc7eda4061e7ec866
  14:     0x7fa257c20872 - <rustc_errors[9faf910c9ff59b10]::emitter::EmitterWriter as rustc_errors[9faf910c9ff59b10]::translation::Translate>::translate_message
  15:     0x7fa257c1dbb2 - <rustc_errors[9faf910c9ff59b10]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7fa257c160da - <rustc_errors[9faf910c9ff59b10]::emitter::EmitterWriter>::emit_message_default
  17:     0x7fa257c13726 - <rustc_errors[9faf910c9ff59b10]::emitter::EmitterWriter as rustc_errors[9faf910c9ff59b10]::emitter::Emitter>::emit_diagnostic
  18:     0x7fa257c2bef2 - <rustc_errors[9faf910c9ff59b10]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7fa257c2a90c - <rustc_errors[9faf910c9ff59b10]::json::JsonEmitter as rustc_errors[9faf910c9ff59b10]::emitter::Emitter>::emit_diagnostic
  20:     0x7fa257c70448 - <rustc_errors[9faf910c9ff59b10]::HandlerInner>::emit_diagnostic
  21:     0x7fa257c6ed41 - <rustc_errors[9faf910c9ff59b10]::Handler>::emit_diagnostic
  22:     0x7fa257c7710d - <() as rustc_errors[9faf910c9ff59b10]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  23:     0x7fa255f10de2 - <rustc_mir_build[a555ccc92a82ced9]::errors::LeadingIrrefutableLetPatterns as rustc_errors[9faf910c9ff59b10]::diagnostic::DecorateLint<()>>::decorate_lint
  24:     0x7fa255f2fc74 - <<rustc_middle[e199242e5ace7c20]::ty::context::TyCtxt>::emit_spanned_lint<rustc_span[fdee0c38aeff0616]::span_encoding::Span, rustc_mir_build[a555ccc92a82ced9]::errors::LeadingIrrefutableLetPatterns>::{closure#0} as core[aaec84792fe89f7d]::ops::function::FnOnce<(rustc_errors[9faf910c9ff59b10]::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
  25:     0x7fa257aba684 - <alloc[daac8f40102988e2]::boxed::Box<dyn for<'a> core[aaec84792fe89f7d]::ops::function::FnOnce<(rustc_errors[9faf910c9ff59b10]::diagnostic_builder::LintDiagnosticBuilder<'a, ()>,), Output = ()>> as core[aaec84792fe89f7d]::ops::function::FnOnce<(rustc_errors[9faf910c9ff59b10]::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once
  26:     0x7fa257a53b60 - rustc_middle[e199242e5ace7c20]::lint::struct_lint_level::struct_lint_level_impl
  27:     0x7fa255f2abe6 - rustc_middle[e199242e5ace7c20]::lint::struct_lint_level::<<rustc_middle[e199242e5ace7c20]::ty::context::TyCtxt>::emit_spanned_lint<rustc_span[fdee0c38aeff0616]::span_encoding::Span, rustc_mir_build[a555ccc92a82ced9]::errors::LeadingIrrefutableLetPatterns>::{closure#0}>
  28:     0x7fa255efc34f - <rustc_middle[e199242e5ace7c20]::ty::context::TyCtxt>::emit_spanned_lint::<rustc_span[fdee0c38aeff0616]::span_encoding::Span, rustc_mir_build[a555ccc92a82ced9]::errors::LeadingIrrefutableLetPatterns>
  29:     0x7fa255f8ca0b - <rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor>::check_let_reachability
  30:     0x7fa255f8a46c - <rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor as rustc_hir[b4bae62d7449fd36]::intravisit::Visitor>::visit_expr
  31:     0x7fa255f8a01c - <rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor as rustc_hir[b4bae62d7449fd36]::intravisit::Visitor>::visit_expr
  32:     0x7fa255f70bc4 - rustc_hir[b4bae62d7449fd36]::intravisit::walk_expr::<rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor>
  33:     0x7fa255f8a01c - <rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor as rustc_hir[b4bae62d7449fd36]::intravisit::Visitor>::visit_expr
  34:     0x7fa255f70d2a - rustc_hir[b4bae62d7449fd36]::intravisit::walk_expr::<rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor>
  35:     0x7fa255f8a01c - <rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::MatchVisitor as rustc_hir[b4bae62d7449fd36]::intravisit::Visitor>::visit_expr
  36:     0x7fa255f89f0f - rustc_mir_build[a555ccc92a82ced9]::thir::pattern::check_match::check_match
  37:     0x7fa256a9a49c - rustc_query_system[a52cec32b353129b]::query::plumbing::try_execute_query::<rustc_query_impl[324139c53859cfe4]::plumbing::QueryCtxt, rustc_query_system[a52cec32b353129b]::query::caches::DefaultCache<rustc_span[fdee0c38aeff0616]::def_id::DefId, ()>>
  38:     0x7fa256b3676a - rustc_query_system[a52cec32b353129b]::query::plumbing::get_query::<rustc_query_impl[324139c53859cfe4]::queries::check_match, rustc_query_impl[324139c53859cfe4]::plumbing::QueryCtxt>
  39:     0x7fa2569a2949 - <rustc_query_impl[324139c53859cfe4]::Queries as rustc_middle[e199242e5ace7c20]::ty::query::QueryEngine>::check_match
  40:     0x7fa2550b96c5 - <core[aaec84792fe89f7d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ff278ada8eeeadfb]::sync::par_for_each_in<&[rustc_span[fdee0c38aeff0616]::def_id::LocalDefId], <rustc_middle[e199242e5ace7c20]::hir::map::Map>::par_body_owners<rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[aaec84792fe89f7d]::ops::function::FnOnce<()>>::call_once
  41:     0x7fa25503a339 - std[2c1e4648a57b8568]::panic::catch_unwind::<core[aaec84792fe89f7d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[ff278ada8eeeadfb]::sync::par_for_each_in<&[rustc_span[fdee0c38aeff0616]::def_id::LocalDefId], <rustc_middle[e199242e5ace7c20]::hir::map::Map>::par_body_owners<rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  42:     0x7fa255026f8d - rustc_data_structures[ff278ada8eeeadfb]::sync::par_for_each_in::<&[rustc_span[fdee0c38aeff0616]::def_id::LocalDefId], <rustc_middle[e199242e5ace7c20]::hir::map::Map>::par_body_owners<rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7fa2550bbb8b - <core[aaec84792fe89f7d]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}::{closure#0}> as core[aaec84792fe89f7d]::ops::function::FnOnce<()>>::call_once
  44:     0x7fa25503a609 - std[2c1e4648a57b8568]::panic::catch_unwind::<core[aaec84792fe89f7d]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}::{closure#0}>, ()>
  45:     0x7fa2550aeb6e - <rustc_session[df7b83c1796cd269]::session::Session>::time::<(), rustc_interface[4181091f1c29890f]::passes::analysis::{closure#1}>
  46:     0x7fa255074c44 - rustc_interface[4181091f1c29890f]::passes::analysis
  47:     0x7fa256aab6e4 - rustc_query_system[a52cec32b353129b]::query::plumbing::try_execute_query::<rustc_query_impl[324139c53859cfe4]::plumbing::QueryCtxt, rustc_query_system[a52cec32b353129b]::query::caches::DefaultCache<(), core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>>
  48:     0x7fa256b8f4f4 - rustc_query_system[a52cec32b353129b]::query::plumbing::get_query::<rustc_query_impl[324139c53859cfe4]::queries::analysis, rustc_query_impl[324139c53859cfe4]::plumbing::QueryCtxt>
  49:     0x7fa25697dcfe - <rustc_query_impl[324139c53859cfe4]::Queries as rustc_middle[e199242e5ace7c20]::ty::query::QueryEngine>::analysis
  50:     0x7fa254f89f3d - <rustc_interface[4181091f1c29890f]::passes::QueryContext>::enter::<rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>
  51:     0x7fa254f26876 - <rustc_interface[4181091f1c29890f]::interface::Compiler>::enter::<rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}::{closure#2}, core[aaec84792fe89f7d]::result::Result<core[aaec84792fe89f7d]::option::Option<rustc_interface[4181091f1c29890f]::queries::Linker>, rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>
  52:     0x7fa254f09045 - rustc_span[fdee0c38aeff0616]::with_source_map::<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_interface[4181091f1c29890f]::interface::create_compiler_and_run<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>::{closure#1}>
  53:     0x7fa254f28a31 - rustc_interface[4181091f1c29890f]::interface::create_compiler_and_run::<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>
  54:     0x7fa254f0aec2 - <scoped_tls[55b2635c66646703]::ScopedKey<rustc_span[fdee0c38aeff0616]::SessionGlobals>>::set::<rustc_interface[4181091f1c29890f]::interface::run_compiler<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>
  55:     0x7fa254f80c99 - std[2c1e4648a57b8568]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4181091f1c29890f]::util::run_in_thread_pool_with_globals<rustc_interface[4181091f1c29890f]::interface::run_compiler<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>
  56:     0x7fa254f0b26e - std[2c1e4648a57b8568]::panic::catch_unwind::<core[aaec84792fe89f7d]::panic::unwind_safe::AssertUnwindSafe<<std[2c1e4648a57b8568]::thread::Builder>::spawn_unchecked_<rustc_interface[4181091f1c29890f]::util::run_in_thread_pool_with_globals<rustc_interface[4181091f1c29890f]::interface::run_compiler<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>
  57:     0x7fa254f852d0 - <<std[2c1e4648a57b8568]::thread::Builder>::spawn_unchecked_<rustc_interface[4181091f1c29890f]::util::run_in_thread_pool_with_globals<rustc_interface[4181091f1c29890f]::interface::run_compiler<core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>, rustc_driver[c23444e27ad84b6d]::run_compiler::{closure#1}>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>::{closure#0}, core[aaec84792fe89f7d]::result::Result<(), rustc_errors[9faf910c9ff59b10]::ErrorGuaranteed>>::{closure#1} as core[aaec84792fe89f7d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7fa2545a3315 - std::sys::unix::thread::Thread::new::thread_start::hbb33ecdf04acff45
  59:     0x7fa254340b43 - <unknown>
  60:     0x7fa2543d2a00 - <unknown>
  61:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (41b1ea286 2022-08-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
------------------------------------------



