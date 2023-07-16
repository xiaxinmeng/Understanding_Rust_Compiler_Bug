plain
........................................................................................ 9240/13445
.................................................................i...................... 9328/13445
........................................................................................ 9416/13445
........................................................................................ 9504/13445
....................F..F................................................................ 9592/13445
........................................................................................ 9768/13445
........................................................................................ 9856/13445
............................................................................ii.......... 9944/13445
.....i.................................................................................. 10032/13445
---
failures:

---- [ui] src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-and-ref.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-and-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-and-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-and-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot borrow value as immutable because it is also borrowed as mutable
   |
   |
LL |         ref mut z @ &mut Some(ref a) => {
   |         |                     |
   |         |                     |
   |         |                     immutable borrow, by `a`, occurs here
   |         mutable borrow, by `z`, occurs here

thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f241e01f12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f241e087ca8 - core::fmt::write::h6b993355ae438c8d
   1:     0x7f241e087ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f241e00f9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f241e02211e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f241e021de7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f241e9b3244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f241e0228d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f241e0226f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f241e01f6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f241e0223c2 - rust_begin_unwind
  10:     0x7f241dfd2e43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f241e084431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f241e0843db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f241dfd2cb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f24216aecd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f24216ac262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f24216a47aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f24216a1e72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f24216b7cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f24216b68cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f24216fbd68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
  21:     0x7f24216f7a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f241f9c2920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f241fa17988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f241f987c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f241fa1491e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f241fa139cd - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_local
  27:     0x7f241f9f80d5 - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_expr::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f241fa1167c - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_expr
  29:     0x7f241fa1156f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  30:     0x7f242052d80c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  31:     0x7f24205c819a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  32:     0x7f2420437149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  33:     0x7f241eb4cfa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  34:     0x7f241eacea49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  35:     0x7f241eabaffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  36:     0x7f241eb4f46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  37:     0x7f241eaced19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  38:     0x7f241eb4214e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  39:     0x7f241eb08b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  40:     0x7f242053e874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  41:     0x7f2420620834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  42:     0x7f242041359d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  43:     0x7f241ea11cab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f241e9b4ca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  45:     0x7f241e9a1591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  46:     0x7f241e9b7801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  47:     0x7f241e99e402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f241ea164ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  49:     0x7f241e9a009e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7f241ea198d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f241e02f645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  52:     0x7f241ddcbb43 - <unknown>
  53:     0x7f241de5da00 - <unknown>
  54:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |     fn f1(ref a @ ref mut b: U) {}
   |           -----^^^---------
   |           |       |
   |           |       mutable borrow, by `b`, occurs here
   |           immutable borrow, by `a`, occurs here

error: cannot borrow value as immutable because it is also borrowed as mutable
   |
   |
LL |     fn f2(ref mut a @ ref b: U) {}
   |           ---------^^^-----
   |           |           |
   |           |           immutable borrow, by `b`, occurs here
   |           mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |     fn f3(ref a @ [ref b, ref mut mid @ .., ref c]: [U; 4]) {}
   |           |               |
   |           |               |
   |           |               mutable borrow, by `mid`, occurs here
   |           immutable borrow, by `a`, occurs here

error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |     fn f4_also_moved(ref a @ ref mut b @ c: U) {}
   |                      -----^^^-------------
   |                      |       |           also moved into `c` here
   |                      |       |           also moved into `c` here
   |                      |       mutable borrow, by `b`, occurs here
   |                      immutable borrow, by `a`, occurs here
error: cannot move out of value because it is borrowed
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-and-ref.rs:28:30
   |
   |
LL |     fn f4_also_moved(ref a @ ref mut b @ c: U) {}
   |                              ---------^^^-
   |                              |           value moved into `c` here
   |                              |           value moved into `c` here
   |                              value borrowed, by `b`, here
error: aborting due to 6 previous errors
------------------------------------------



---- [ui] src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f2164e9a12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f2164f02ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f2164e9d11e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f2164e9cde7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f216582e244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2164e9d8d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f2164e9d6f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f2164e9a6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f2164e9d3c2 - rust_begin_unwind
  10:     0x7f2164e4de43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f2164eff431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f2164eff3db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f2164e4dcb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f2168529cd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f2168527262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f216851f7aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f216851ce72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2168532cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f21685318cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f2168576d68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  21:     0x7f2168572a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f216683d920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f2166892988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f2166802c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f216688f91e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f216688e9cd - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_local
  27:     0x7f21668730d5 - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_expr::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f216688c67c - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_expr
  29:     0x7f216688c56f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  30:     0x7f21673a880c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  31:     0x7f216744319a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  32:     0x7f21672b2149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  33:     0x7f21659c7fa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  34:     0x7f2165949a49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  35:     0x7f2165935ffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  36:     0x7f21659ca46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  37:     0x7f2165949d19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  38:     0x7f21659bd14e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  39:     0x7f2165983b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  40:     0x7f21673b9874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  41:     0x7f216749b834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  42:     0x7f216728e59d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  43:     0x7f216588ccab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f216582fca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  45:     0x7f216581c591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  46:     0x7f2165832801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  47:     0x7f2165819402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f21658914ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  49:     0x7f216581b09e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7f21658948d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f2164eaa645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  52:     0x7f2164c46b43 - <unknown>
  53:     0x7f2164cd8a00 - <unknown>
  54:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f2164e9a12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f2164f02ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f2164e9d11e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f2164e9cde7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f216582e244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2164e9d8d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f2164e9d6f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f2164e9a6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f2164e9d3c2 - rust_begin_unwind
  10:     0x7f2164e4de43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f2164eff431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f2164eff3db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f2164e4dcb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f2168529cd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f2168527262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f216851f7aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f216851ce72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2168532cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f21685318cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f2168576d68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
  21:     0x7f2168572a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f216683d920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f2166892988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f2166802c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f216688f91e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f216688ea09 - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_param
  27:     0x7f2166872cca - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_body::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f216688c56f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  29:     0x7f21673a880c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  30:     0x7f216744319a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  31:     0x7f21672b2149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  32:     0x7f21659c7fa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  33:     0x7f2165949a49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  34:     0x7f2165935ffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  35:     0x7f21659ca46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  36:     0x7f2165949d19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  37:     0x7f21659bd14e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  38:     0x7f2165983b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  39:     0x7f21673b9874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  40:     0x7f216749b834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  41:     0x7f216728e59d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  42:     0x7f216588ccab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  43:     0x7f216582fca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f216581c591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f2165832801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  46:     0x7f2165819402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  47:     0x7f21658914ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f216581b09e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f21658948d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f2164eaa645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  51:     0x7f2164c46b43 - <unknown>
  52:     0x7f2164cd8a00 - <unknown>
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main::f1`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f2164e9a12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f2164f02ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f2164e9d11e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f2164e9cde7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f216582e244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2164e9d8d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f2164e9d6f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f2164e9a6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f2164e9d3c2 - rust_begin_unwind
  10:     0x7f2164e4de43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f2164eff431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f2164eff3db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f2164e4dcb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f2168529cd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f2168527262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f216851f7aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f216851ce72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2168532cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f21685318cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f2168576d68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
  21:     0x7f2168572a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f216683d920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f2166892988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f2166802c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f216688f91e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f216688ea09 - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_param
  27:     0x7f2166872cca - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_body::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f216688c56f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  29:     0x7f21673a880c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  30:     0x7f216744319a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  31:     0x7f21672b2149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  32:     0x7f21659c7fa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  33:     0x7f2165949a49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  34:     0x7f2165935ffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  35:     0x7f21659ca46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  36:     0x7f2165949d19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  37:     0x7f21659bd14e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  38:     0x7f2165983b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  39:     0x7f21673b9874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  40:     0x7f216749b834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  41:     0x7f216728e59d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  42:     0x7f216588ccab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  43:     0x7f216582fca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f216581c591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f2165832801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  46:     0x7f2165819402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  47:     0x7f21658914ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f216581b09e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f21658948d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f2164eaa645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  51:     0x7f2164c46b43 - <unknown>
  52:     0x7f2164cd8a00 - <unknown>
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main::f2`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f2164e9a12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f2164f02ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f2164e9d11e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f2164e9cde7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f216582e244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2164e9d8d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f2164e9d6f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f2164e9a6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f2164e9d3c2 - rust_begin_unwind
  10:     0x7f2164e4de43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f2164eff431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f2164eff3db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f2164e4dcb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f2168529cd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f2168527262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f216851f7aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f216851ce72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2168532cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f21685318cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f2168576d68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
  21:     0x7f2168572a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f216683d920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f2166892988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f2166802c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f216688f91e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f216688ea09 - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_param
  27:     0x7f2166872cca - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_body::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f216688c56f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  29:     0x7f21673a880c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  30:     0x7f216744319a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  31:     0x7f21672b2149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  32:     0x7f21659c7fa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  33:     0x7f2165949a49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  34:     0x7f2165935ffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  35:     0x7f21659ca46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  36:     0x7f2165949d19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  37:     0x7f21659bd14e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  38:     0x7f2165983b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  39:     0x7f21673b9874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  40:     0x7f216749b834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  41:     0x7f216728e59d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  42:     0x7f216588ccab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  43:     0x7f216582fca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f216581c591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f2165832801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  46:     0x7f2165819402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  47:     0x7f21658914ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f216581b09e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f21658948d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f2164eaa645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  51:     0x7f2164c46b43 - <unknown>
  52:     0x7f2164cd8a00 - <unknown>
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main::f3`
#1 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f2164e9a12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4eaa03586c8667d0
   1:     0x7f2164f02ca8 - core::fmt::write::h6b993355ae438c8d
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   2:     0x7f2164e8a9a1 - std::io::Write::write_fmt::h607407eb215ac18e
   3:     0x7f2164e9d11e - std::panicking::default_hook::{{closure}}::h9d5285c9ce308aa7
   4:     0x7f2164e9cde7 - std::panicking::default_hook::h84d90009a46dab40
   5:     0x7f216582e244 - rustc_driver[62b3453a7b595d3a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2164e9d8d1 - std::panicking::rust_panic_with_hook::hf0cccd5143172861
   7:     0x7f2164e9d6f7 - std::panicking::begin_panic_handler::{{closure}}::h3b69d001ae7ab6ef
   8:     0x7f2164e9a6a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4b5c57b7cc25360a
   9:     0x7f2164e9d3c2 - rust_begin_unwind
  10:     0x7f2164e4de43 - core::panicking::panic_fmt::h3fd7ec4cff0128a3
  11:     0x7f2164eff431 - core::panicking::panic_display::h676213f82882f289
  12:     0x7f2164eff3db - core::panicking::panic_str::h3d7ee398f0567093
  13:     0x7f2164e4dcb6 - core::option::expect_failed::h6741d41554cc152b
  14:     0x7f2168529cd5 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::translation::Translate>::translate_message
  15:     0x7f2168527262 - <rustc_errors[8d6350808890498a]::emitter::FileWithAnnotatedLines>::collect_annotations
  16:     0x7f216851f7aa - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f216851ce72 - <rustc_errors[8d6350808890498a]::emitter::EmitterWriter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  18:     0x7f2168532cd6 - <rustc_errors[8d6350808890498a]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f21685318cc - <rustc_errors[8d6350808890498a]::json::JsonEmitter as rustc_errors[8d6350808890498a]::emitter::Emitter>::emit_diagnostic
  20:     0x7f2168576d68 - <rustc_errors[8d6350808890498a]::HandlerInner>::emit_diagnostic
  21:     0x7f2168572a16 - <rustc_errors[8d6350808890498a]::ErrorGuaranteed as rustc_errors[8d6350808890498a]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  22:     0x7f216683d920 - <rustc_session[ce1cb988059bba4]::parse::ParseSess>::emit_err::<rustc_mir_build[d1643207f5745e1d]::errors::MultipleMutBorrows>
  23:     0x7f2166892988 - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_borrow_conflicts_in_at_patterns
  24:     0x7f2166802c32 - <rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_::<<rustc_hir[ec1d876b6ee76e77]::hir::Pat>::walk_always<<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_patterns::{closure#0}>::{closure#0}>
  25:     0x7f216688f91e - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>::check_irrefutable
  26:     0x7f216688ea09 - <rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor as rustc_hir[ec1d876b6ee76e77]::intravisit::Visitor>::visit_param
  27:     0x7f2166872cca - rustc_hir[ec1d876b6ee76e77]::intravisit::walk_body::<rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::MatchVisitor>
  28:     0x7f216688c56f - rustc_mir_build[d1643207f5745e1d]::thir::pattern::check_match::check_match
  29:     0x7f21673a880c - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<rustc_span[e88a759b2d82f03c]::def_id::DefId, ()>>
  30:     0x7f216744319a - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::check_match, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  31:     0x7f21672b2149 - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::check_match
  32:     0x7f21659c7fa5 - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  33:     0x7f2165949a49 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>, ()>
  34:     0x7f2165935ffd - rustc_data_structures[b2c29740c02fcaa5]::sync::par_for_each_in::<&[rustc_span[e88a759b2d82f03c]::def_id::LocalDefId], <rustc_middle[4cd9e8e974cdc535]::hir::map::Map>::par_body_owners<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}::{closure#0}::{closure#0}>::{closure#0}>
  35:     0x7f21659ca46b - <core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}> as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once
  36:     0x7f2165949d19 - std[e26ad457c5682ce8]::panic::catch_unwind::<core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}::{closure#0}>, ()>
  37:     0x7f21659bd14e - <rustc_session[ce1cb988059bba4]::session::Session>::time::<(), rustc_interface[5da0733b8d8ea88c]::passes::analysis::{closure#1}>
  38:     0x7f2165983b34 - rustc_interface[5da0733b8d8ea88c]::passes::analysis
  39:     0x7f21673b9874 - rustc_query_system[4c110a2738c2c821]::query::plumbing::try_execute_query::<rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt, rustc_query_system[4c110a2738c2c821]::query::caches::DefaultCache<(), core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>>
  40:     0x7f216749b834 - rustc_query_system[4c110a2738c2c821]::query::plumbing::get_query::<rustc_query_impl[cfa988bd7b633c4d]::queries::analysis, rustc_query_impl[cfa988bd7b633c4d]::plumbing::QueryCtxt>
  41:     0x7f216728e59d - <rustc_query_impl[cfa988bd7b633c4d]::Queries as rustc_middle[4cd9e8e974cdc535]::ty::query::QueryEngine>::analysis
  42:     0x7f216588ccab - <rustc_interface[5da0733b8d8ea88c]::passes::QueryContext>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  43:     0x7f216582fca8 - <rustc_interface[5da0733b8d8ea88c]::interface::Compiler>::enter::<rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}::{closure#2}, core[78b138fc6d9c8abf]::result::Result<core[78b138fc6d9c8abf]::option::Option<rustc_interface[5da0733b8d8ea88c]::queries::Linker>, rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  44:     0x7f216581c591 - rustc_span[e88a759b2d82f03c]::with_source_map::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f2165832801 - rustc_interface[5da0733b8d8ea88c]::interface::create_compiler_and_run::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>
  46:     0x7f2165819402 - <scoped_tls[451dea8974631d7f]::ScopedKey<rustc_span[e88a759b2d82f03c]::SessionGlobals>>::set::<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  47:     0x7f21658914ef - std[e26ad457c5682ce8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>
  48:     0x7f216581b09e - std[e26ad457c5682ce8]::panicking::try::<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, core[78b138fc6d9c8abf]::panic::unwind_safe::AssertUnwindSafe<<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f21658948d0 - <<std[e26ad457c5682ce8]::thread::Builder>::spawn_unchecked_<rustc_interface[5da0733b8d8ea88c]::util::run_in_thread_pool_with_globals<rustc_interface[5da0733b8d8ea88c]::interface::run_compiler<core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>, rustc_driver[62b3453a7b595d3a]::run_compiler::{closure#1}>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#0}, core[78b138fc6d9c8abf]::result::Result<(), rustc_errors[8d6350808890498a]::ErrorGuaranteed>>::{closure#1} as core[78b138fc6d9c8abf]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f2164eaa645 - std::sys::unix::thread::Thread::new::thread_start::h15ea0dea5d4af016
  51:     0x7f2164c46b43 - <unknown>
  52:     0x7f2164cd8a00 - <unknown>
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (6d4340914 2022-09-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
