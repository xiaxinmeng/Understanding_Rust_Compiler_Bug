plain
Suite(test::src/test/incremental) not skipped for "bootstrap::test::Incremental" -- not in [src/tools/tidy]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 153 tests
..............F...................F....................................F............................ 100/153
.............F.........F...................F......F..
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----


error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: encountered incremental compilation error with module_children(a[d0bd])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for module_children(a[d0bd]): [ModChild { ident: function0#0, res: Def(Fn, DefId(20:3 ~ a[d0bd]::function0)), vis: Public, span: /checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs:9:1: 9:32 (#0) }, ModChild { ident: function1#0, res: Def(Fn, DefId(20:4 ~ a[d0bd]::function1)), vis: Public, span: /checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs:13:1: 13:25 (#0) }, ModChild { ident: std#0, res: Def(Mod, DefId(1:0 ~ std[3c40])), vis: Restricted(DefId(20:0 ~ a[d0bd])), span: no-location (#3) }]', compiler/rustc_query_system/src/query/plumbing.rs:657:9
stack backtrace:
   0:     0x7fb7afded2bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc18d4c7d014769a
   1:     0x7fb7afe5d11f - core::fmt::write::hb1ff246c8c00a9b0
   2:     0x7fb7afdda4c1 - std::io::Write::write_fmt::h32248f607bc51588
   3:     0x7fb7afded0db - std::sys_common::backtrace::print::hc6b32bc7abd05d9b
   4:     0x7fb7afdf1894 - std::panicking::default_hook::{{closure}}::h6a9d1e968b5dbc0c
   5:     0x7fb7afdf1459 - std::panicking::default_hook::h3cf294e0bf4ea041
   6:     0x7fb7b0829361 - rustc_driver[fed345b07abbbca1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb7afdf1f78 - std::panicking::rust_panic_with_hook::he3799b6fad3f8eb1
   8:     0x7fb7afdf1da7 - std::panicking::begin_panic_handler::{{closure}}::habe275bcbdd1ab22
   9:     0x7fb7afded7d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4630e0a3d791697c
  10:     0x7fb7afdf1a89 - rust_begin_unwind
  11:     0x7fb7afda5843 - core::panicking::panic_fmt::h2ed899c94647804e
  12:     0x7fb7b07df5ea - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich_cold
  13:     0x7fb7b1e888c1 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  14:     0x7fb7b1e4c9bf - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  15:     0x7fb7b1e11e68 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>>
  16:     0x7fb7b1ec2147 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::module_children, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  17:     0x7fb7b26d606f - <rustc_metadata[1ae07c2cc1b994f1]::rmeta::decoder::cstore_impl::provide::{closure#7} as core[cb47576bca1a9817]::ops::function::FnOnce<(rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, ())>>::call_once
  18:     0x7fb7b22600be - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>::{closure#1}, std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  19:     0x7fb7b1e5c451 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  20:     0x7fb7b1dc945d - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::ArenaCache<(), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>>
  21:     0x7fb7b1ed7623 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::visible_parent_map, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  22:     0x7fb7b2e1ff8d - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::PrettyPrinter>::try_print_visible_def_path_recur
  23:     0x7fb7b2e32f11 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  24:     0x7fb7b2e55df3 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::default_print_def_path
  25:     0x7fb7b2e32ed2 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  26:     0x7fb7b2e528ca - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::TraitRefPrintOnlyTraitPath as core[cb47576bca1a9817]::fmt::Display>::fmt
  27:     0x7fb7afe5d11f - core::fmt::write::hb1ff246c8c00a9b0
  28:     0x7fb7b2e5142a - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as core[cb47576bca1a9817]::fmt::Write>::write_fmt
  29:     0x7fb7b2f7365f - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as rustc_middle[245f60d6e1f8fa6a]::ty::print::Print<rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter>>>::print
  30:     0x7fb7b2f73f58 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Display>::fmt
  31:     0x7fb7b2f66f26 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Debug>::fmt
  32:     0x7fb7afe5d11f - core::fmt::write::hb1ff246c8c00a9b0
  33:     0x7fb7afe5dfe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  34:     0x7fb7b2e9e836 - <rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind as core[cb47576bca1a9817]::fmt::Debug>::fmt
  35:     0x7fb7afe59cd3 - core::fmt::builders::DebugTuple::field::ha6dcb5359088f338
  36:     0x7fb7b2f81bc4 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::Binder<rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  37:     0x7fb7afe5d11f - core::fmt::write::hb1ff246c8c00a9b0
  38:     0x7fb7afe5dfe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  39:     0x7fb7b2e9e346 - <rustc_middle[245f60d6e1f8fa6a]::ty::Predicate as core[cb47576bca1a9817]::fmt::Debug>::fmt
  40:     0x7fb7afe59a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  41:     0x7fb7b2059ae4 - <&rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  42:     0x7fb7afe59a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  43:     0x7fb7b20520d4 - <rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  44:     0x7fb7afe5d11f - core::fmt::write::hb1ff246c8c00a9b0
  45:     0x7fb7afe4d719 - alloc::fmt::format::h16f52e55b390e579
  46:     0x7fb7b2016a0b - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::dep_node::DepNode<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>>>
  47:     0x7fb7b1ed911f - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::evaluate_obligation, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  48:     0x7fb7b2008754 - <rustc_query_impl[fedb310caa113915]::Queries as rustc_middle[245f60d6e1f8fa6a]::ty::query::QueryEngine>::evaluate_obligation
  49:     0x7fb7b2ad9759 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  50:     0x7fb7b2ad997d - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  51:     0x7fb7b2b1d4cd - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  52:     0x7fb7b2b09b8b - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  53:     0x7fb7b2ba65de - <rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::ObligationForest<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor, rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::Outcome<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation, rustc_infer[d6b5c214026f6079]::traits::FulfillmentErrorCode>>
  54:     0x7fb7b2b08adb - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7fb7b2b08968 - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7fb7b118d316 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[b1b99abb46596271]::check_main_fn_ty::{closure#0}>
  57:     0x7fb7b1121e1c - rustc_typeck[b1b99abb46596271]::check_crate
  58:     0x7fb7b0a67881 - rustc_interface[e1a8b57a397b7e94]::passes::analysis
  59:     0x7fb7b2248864 - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<<rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  60:     0x7fb7b20d16f5 - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  61:     0x7fb7b1e247e3 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<(), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>>
  62:     0x7fb7b1f228e0 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::analysis, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  63:     0x7fb7b08c21a0 - <rustc_interface[e1a8b57a397b7e94]::passes::QueryContext>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  64:     0x7fb7b08aba26 - <rustc_interface[e1a8b57a397b7e94]::interface::Compiler>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}, core[cb47576bca1a9817]::result::Result<core[cb47576bca1a9817]::option::Option<rustc_interface[e1a8b57a397b7e94]::queries::Linker>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  65:     0x7fb7b08b9b3b - rustc_span[ad2be3227d5a21b1]::with_source_map::<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_interface[e1a8b57a397b7e94]::interface::create_compiler_and_run<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#1}>
  66:     0x7fb7b08a99b3 - <scoped_tls[1714625bafbf4de]::ScopedKey<rustc_span[ad2be3227d5a21b1]::SessionGlobals>>::set::<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  67:     0x7fb7b0856489 - std[3c40391f8aa09356]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  68:     0x7fb7b08bcace - std[3c40391f8aa09356]::panic::catch_unwind::<core[cb47576bca1a9817]::panic::unwind_safe::AssertUnwindSafe<<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1}::{closure#0}>, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  69:     0x7fb7b084b402 - <<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1} as core[cb47576bca1a9817]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  70:     0x7fb7afe00e43 - std::sys::unix::thread::Thread::new::thread_start::h90d4051769151081
  71:     0x7fb7aa16f609 - start_thread
  72:     0x7fb7afc66293 - clone
  73:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (ce03fdb13 2022-02-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [module_children] collecting child items of module `a`
#1 [visible_parent_map] calculating the visible parent map
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: encountered incremental compilation error with module_children(a[d0bd])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for module_children(a[d0bd]): [ModChild { ident: foo#0, res: Def(Fn, DefId(20:3 ~ a[d0bd]::foo)), vis: Public, span: /checkout/src/test/incremental/change_implementation_cross_crate/auxiliary/a.rs:13:1: 13:25 (#0) }, ModChild { ident: bar#0, res: Def(Fn, DefId(20:4 ~ a[d0bd]::bar)), vis: Public, span: /checkout/src/test/incremental/change_implementation_cross_crate/auxiliary/a.rs:17:1: 17:25 (#0) }, ModChild { ident: bar_impl#0, res: Def(Fn, DefId(20:5 ~ a[d0bd]::bar_impl)), vis: Restricted(DefId(20:0 ~ a[d0bd])), span: /checkout/src/test/incremental/change_implementation_cross_crate/auxiliary/a.rs:29:1: 29:26 (#0) }, ModChild { ident: std#0, res: Def(Mod, DefId(1:0 ~ std[3c40])), vis: Restricted(DefId(20:0 ~ a[d0bd])), span: no-location (#9) }]', compiler/rustc_query_system/src/query/plumbing.rs:657:9
stack backtrace:
   0:     0x7fc7439b62bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc18d4c7d014769a
   1:     0x7fc743a2611f - core::fmt::write::hb1ff246c8c00a9b0
   2:     0x7fc7439a34c1 - std::io::Write::write_fmt::h32248f607bc51588
   3:     0x7fc7439b60db - std::sys_common::backtrace::print::hc6b32bc7abd05d9b
   4:     0x7fc7439ba894 - std::panicking::default_hook::{{closure}}::h6a9d1e968b5dbc0c
   5:     0x7fc7439ba459 - std::panicking::default_hook::h3cf294e0bf4ea041
   6:     0x7fc7443f2361 - rustc_driver[fed345b07abbbca1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc7439baf78 - std::panicking::rust_panic_with_hook::he3799b6fad3f8eb1
   8:     0x7fc7439bada7 - std::panicking::begin_panic_handler::{{closure}}::habe275bcbdd1ab22
   9:     0x7fc7439b67d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4630e0a3d791697c
  10:     0x7fc7439baa89 - rust_begin_unwind
  11:     0x7fc74396e843 - core::panicking::panic_fmt::h2ed899c94647804e
  12:     0x7fc7443a85ea - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich_cold
  13:     0x7fc745a518c1 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  14:     0x7fc745a159bf - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  15:     0x7fc7459dae68 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>>
  16:     0x7fc745a8b147 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::module_children, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  17:     0x7fc74629f06f - <rustc_metadata[1ae07c2cc1b994f1]::rmeta::decoder::cstore_impl::provide::{closure#7} as core[cb47576bca1a9817]::ops::function::FnOnce<(rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, ())>>::call_once
  18:     0x7fc745e290be - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>::{closure#1}, std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  19:     0x7fc745a25451 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  20:     0x7fc74599245d - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::ArenaCache<(), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>>
  21:     0x7fc745aa0623 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::visible_parent_map, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  22:     0x7fc7469e8f8d - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::PrettyPrinter>::try_print_visible_def_path_recur
  23:     0x7fc7469fbf11 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  24:     0x7fc746a1edf3 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::default_print_def_path
  25:     0x7fc7469fbed2 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  26:     0x7fc746a1b8ca - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::TraitRefPrintOnlyTraitPath as core[cb47576bca1a9817]::fmt::Display>::fmt
  27:     0x7fc743a2611f - core::fmt::write::hb1ff246c8c00a9b0
  28:     0x7fc746a1a42a - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as core[cb47576bca1a9817]::fmt::Write>::write_fmt
  29:     0x7fc746b3c65f - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as rustc_middle[245f60d6e1f8fa6a]::ty::print::Print<rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter>>>::print
  30:     0x7fc746b3cf58 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Display>::fmt
  31:     0x7fc746b2ff26 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Debug>::fmt
  32:     0x7fc743a2611f - core::fmt::write::hb1ff246c8c00a9b0
  33:     0x7fc743a26fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  34:     0x7fc746a67836 - <rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind as core[cb47576bca1a9817]::fmt::Debug>::fmt
  35:     0x7fc743a22cd3 - core::fmt::builders::DebugTuple::field::ha6dcb5359088f338
  36:     0x7fc746b4abc4 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::Binder<rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  37:     0x7fc743a2611f - core::fmt::write::hb1ff246c8c00a9b0
  38:     0x7fc743a26fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  39:     0x7fc746a67346 - <rustc_middle[245f60d6e1f8fa6a]::ty::Predicate as core[cb47576bca1a9817]::fmt::Debug>::fmt
  40:     0x7fc743a22a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  41:     0x7fc745c22ae4 - <&rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  42:     0x7fc743a22a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  43:     0x7fc745c1b0d4 - <rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  44:     0x7fc743a2611f - core::fmt::write::hb1ff246c8c00a9b0
  45:     0x7fc743a16719 - alloc::fmt::format::h16f52e55b390e579
  46:     0x7fc745bdfa0b - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::dep_node::DepNode<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>>>
  47:     0x7fc745aa211f - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::evaluate_obligation, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  48:     0x7fc745bd1754 - <rustc_query_impl[fedb310caa113915]::Queries as rustc_middle[245f60d6e1f8fa6a]::ty::query::QueryEngine>::evaluate_obligation
  49:     0x7fc7466a2759 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  50:     0x7fc7466a297d - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  51:     0x7fc7466e64cd - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  52:     0x7fc7466d2b8b - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  53:     0x7fc74676f5de - <rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::ObligationForest<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor, rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::Outcome<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation, rustc_infer[d6b5c214026f6079]::traits::FulfillmentErrorCode>>
  54:     0x7fc7466d1adb - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7fc7466d1968 - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7fc744d56316 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[b1b99abb46596271]::check_main_fn_ty::{closure#0}>
  57:     0x7fc744ceae1c - rustc_typeck[b1b99abb46596271]::check_crate
  58:     0x7fc744630881 - rustc_interface[e1a8b57a397b7e94]::passes::analysis
  59:     0x7fc745e11864 - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<<rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  60:     0x7fc745c9a6f5 - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  61:     0x7fc7459ed7e3 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<(), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>>
  62:     0x7fc745aeb8e0 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::analysis, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  63:     0x7fc74448b1a0 - <rustc_interface[e1a8b57a397b7e94]::passes::QueryContext>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  64:     0x7fc744474a26 - <rustc_interface[e1a8b57a397b7e94]::interface::Compiler>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}, core[cb47576bca1a9817]::result::Result<core[cb47576bca1a9817]::option::Option<rustc_interface[e1a8b57a397b7e94]::queries::Linker>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  65:     0x7fc744482b3b - rustc_span[ad2be3227d5a21b1]::with_source_map::<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_interface[e1a8b57a397b7e94]::interface::create_compiler_and_run<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#1}>
  66:     0x7fc7444729b3 - <scoped_tls[1714625bafbf4de]::ScopedKey<rustc_span[ad2be3227d5a21b1]::SessionGlobals>>::set::<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  67:     0x7fc74441f489 - std[3c40391f8aa09356]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  68:     0x7fc744485ace - std[3c40391f8aa09356]::panic::catch_unwind::<core[cb47576bca1a9817]::panic::unwind_safe::AssertUnwindSafe<<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1}::{closure#0}>, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  69:     0x7fc744414402 - <<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1} as core[cb47576bca1a9817]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  70:     0x7fc7439c9e43 - std::sys::unix::thread::Thread::new::thread_start::h90d4051769151081
  71:     0x7fc73dd38609 - start_thread
  72:     0x7fc74382f293 - clone
  73:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (ce03fdb13 2022-02-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [module_children] collecting child items of module `a`
#1 [visible_parent_map] calculating the visible parent map
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/load_cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: encountered incremental compilation error with optimized_mir(cached_hygiene[77b8]::changed_fn)
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for optimized_mir(cached_hygiene[77b8]::changed_fn): Body { basic_blocks: [BasicBlockData { statements: [StorageLive(_1), StorageLive(_2), StorageLive(_3), StorageLive(_4), StorageLive(_5), _10 = const changed_fn::promoted[1], _5 = _10, _4 = _5, _3 = move _4 as &[&str] (Pointer(Unsize)), StorageDead(_4), StorageLive(_6), StorageLive(_7), StorageLive(_8), _9 = const changed_fn::promoted[0], _8 = _9, _7 = _8, _6 = move _7 as &[std::fmt::ArgumentV1] (Pointer(Unsize)), StorageDead(_7)], terminator: Some(Terminator { source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] }, kind: _2 = Arguments::new_v1(move _3, move _6) -> bb1 }), is_cleanup: false }, BasicBlockData { statements: [StorageDead(_6), StorageDead(_3)], terminator: Some(Terminator { source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:9: 99:62 (#74), scope: scope[0] }, kind: _1 = _print(move _2) -> bb2 }), is_cleanup: false }, BasicBlockData { statements: [StorageDead(_2), StorageDead(_8), StorageDead(_5), StorageDead(_1)], terminator: Some(Terminator { source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:24:2: 24:2 (#0), scope: scope[0] }, kind: return }), is_cleanup: false }], phase: Optimization, source: MirSource { instance: Item(WithOptConstParam { did: DefId(20:4 ~ cached_hygiene[77b8]::changed_fn), const_param_did: None }), promoted: None }, source_scopes: [SourceScopeData { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:18:1: 24:2 (#0), parent_scope: None, inlined: None, inlined_parent_scope: None, local_data: Clear }], generator: None, local_decls: [LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: (), user_ty: None, source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:18:21: 18:21 (#0), scope: scope[0] } }, LocalDecl { mutability: Not, local_info: None, internal: false, is_block_tail: None, ty: (), user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:9: 99:62 (#74), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: std::fmt::Arguments, user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[&str], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:12:18: 12:29 (#75), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[&str; 1], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:12:18: 12:29 (#75), scope: scope[0] } }, LocalDecl { mutability: Not, local_info: None, internal: false, is_block_tail: None, ty: &[&str; 1], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:12:18: 12:29 (#75), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[std::fmt::ArgumentV1], user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[std::fmt::ArgumentV1; 0], user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] } }, LocalDecl { mutability: Not, local_info: None, internal: false, is_block_tail: None, ty: &[std::fmt::ArgumentV1; 0], user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[std::fmt::ArgumentV1; 0], user_ty: None, source_info: SourceInfo { span: /checkout/library/std/src/macros.rs:99:28: 99:61 (#76), scope: scope[0] } }, LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: &[&str; 1], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:12:18: 12:29 (#75), scope: scope[0] } }], user_type_annotations: [], arg_count: 0, spread_arg: None, var_debug_info: [], span: /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:18:1: 24:2 (#0), required_consts: [], is_polymorphic: false, predecessor_cache: PredecessorCache { cache: OnceCell(Uninit) }, is_cyclic: GraphIsCyclicCache { cache: OnceCell(Uninit) }, tainted_by_errors: None }', compiler/rustc_query_system/src/query/plumbing.rs:657:9
stack backtrace:
   0:     0x7f0bccb392bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc18d4c7d014769a
   1:     0x7f0bccba911f - core::fmt::write::hb1ff246c8c00a9b0
   2:     0x7f0bccb264c1 - std::io::Write::write_fmt::h32248f607bc51588
   3:     0x7f0bccb390db - std::sys_common::backtrace::print::hc6b32bc7abd05d9b
   4:     0x7f0bccb3d894 - std::panicking::default_hook::{{closure}}::h6a9d1e968b5dbc0c
   5:     0x7f0bccb3d459 - std::panicking::default_hook::h3cf294e0bf4ea041
   6:     0x7f0bcd575361 - rustc_driver[fed345b07abbbca1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0bccb3df78 - std::panicking::rust_panic_with_hook::he3799b6fad3f8eb1
   8:     0x7f0bccb3dda7 - std::panicking::begin_panic_handler::{{closure}}::habe275bcbdd1ab22
   9:     0x7f0bccb397d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4630e0a3d791697c
  10:     0x7f0bccb3da89 - rust_begin_unwind
  11:     0x7f0bccaf1843 - core::panicking::panic_fmt::h2ed899c94647804e
  12:     0x7f0bcd52b5ea - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich_cold
  13:     0x7f0bcebd2461 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &rustc_middle[245f60d6e1f8fa6a]::mir::Body>
  14:     0x7f0bceb96ee1 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &rustc_middle[245f60d6e1f8fa6a]::mir::Body>
  15:     0x7f0bceb577a0 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<rustc_span[ad2be3227d5a21b1]::def_id::DefId, &rustc_middle[245f60d6e1f8fa6a]::mir::Body>>
  16:     0x7f0bcec0461c - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::optimized_mir, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  17:     0x7f0bcfc58cc5 - <rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt>::instance_mir
  18:     0x7f0bcd9c1c91 - rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_neighbours
  19:     0x7f0bcd9b8bde - rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_items_rec
  20:     0x7f0bcd9b8f06 - rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_items_rec
  21:     0x7f0bcd9b8f06 - rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_items_rec
  22:     0x7f0bcd9c8549 - <rustc_session[7b34d4a47beabd35]::session::Session>::time::<(), rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_crate_mono_items::{closure#1}>
  23:     0x7f0bcd9b752f - rustc_monomorphize[b7af7c9f3343bb03]::collector::collect_crate_mono_items
  24:     0x7f0bcd9f5def - rustc_monomorphize[b7af7c9f3343bb03]::partitioning::collect_and_partition_mono_items
  25:     0x7f0bcef95f0b - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<<rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), (&std[3c40391f8aa09356]::collections::hash::set::HashSet<rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>, &[rustc_middle[245f60d6e1f8fa6a]::mir::mono::CodegenUnit])>::{closure#0}, (&std[3c40391f8aa09356]::collections::hash::set::HashSet<rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>, &[rustc_middle[245f60d6e1f8fa6a]::mir::mono::CodegenUnit])>
  26:     0x7f0bcee2aa76 - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), (&std[3c40391f8aa09356]::collections::hash::set::HashSet<rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>, &[rustc_middle[245f60d6e1f8fa6a]::mir::mono::CodegenUnit])>
  27:     0x7f0bceb7b458 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<(), (&std[3c40391f8aa09356]::collections::hash::set::HashSet<rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>, &[rustc_middle[245f60d6e1f8fa6a]::mir::mono::CodegenUnit])>>
  28:     0x7f0bcec610a5 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::collect_and_partition_mono_items, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  29:     0x7f0bced5444e - <rustc_query_impl[fedb310caa113915]::Queries as rustc_middle[245f60d6e1f8fa6a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  30:     0x7f0bcd7f5172 - rustc_codegen_ssa[46ca48d06522ca49]::base::codegen_crate::<rustc_codegen_llvm[9260dd76babae8d0]::LlvmCodegenBackend>
  31:     0x7f0bcd8aca49 - <rustc_codegen_llvm[9260dd76babae8d0]::LlvmCodegenBackend as rustc_codegen_ssa[46ca48d06522ca49]::traits::backend::CodegenBackend>::codegen_crate
  32:     0x7f0bcd716381 - <rustc_session[7b34d4a47beabd35]::session::Session>::time::<alloc[7d29cb220a69be9c]::boxed::Box<dyn core[cb47576bca1a9817]::any::Any>, rustc_interface[e1a8b57a397b7e94]::passes::start_codegen::{closure#0}>
  33:     0x7f0bcd7b2766 - <rustc_interface[e1a8b57a397b7e94]::passes::QueryContext>::enter::<<rustc_interface[e1a8b57a397b7e94]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cb47576bca1a9817]::result::Result<alloc[7d29cb220a69be9c]::boxed::Box<dyn core[cb47576bca1a9817]::any::Any>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  34:     0x7f0bcd78ee7e - <rustc_interface[e1a8b57a397b7e94]::queries::Queries>::ongoing_codegen
  35:     0x7f0bcd5f7a88 - <rustc_interface[e1a8b57a397b7e94]::interface::Compiler>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}, core[cb47576bca1a9817]::result::Result<core[cb47576bca1a9817]::option::Option<rustc_interface[e1a8b57a397b7e94]::queries::Linker>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  36:     0x7f0bcd605b3b - rustc_span[ad2be3227d5a21b1]::with_source_map::<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_interface[e1a8b57a397b7e94]::interface::create_compiler_and_run<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7f0bcd5f59b3 - <scoped_tls[1714625bafbf4de]::ScopedKey<rustc_span[ad2be3227d5a21b1]::SessionGlobals>>::set::<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  38:     0x7f0bcd5a2489 - std[3c40391f8aa09356]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  39:     0x7f0bcd608ace - std[3c40391f8aa09356]::panic::catch_unwind::<core[cb47576bca1a9817]::panic::unwind_safe::AssertUnwindSafe<<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1}::{closure#0}>, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  40:     0x7f0bcd597402 - <<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1} as core[cb47576bca1a9817]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f0bccb4ce43 - std::sys::unix::thread::Thread::new::thread_start::h90d4051769151081
  42:     0x7f0bc6ebb609 - start_thread
  43:     0x7f0bcc9b2293 - clone
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (ce03fdb13 2022-02-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `cached_hygiene::changed_fn`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: encountered incremental compilation error with module_children(a[d0bd])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for module_children(a[d0bd]): [ModChild { ident: foo#0, res: Def(Fn, DefId(20:3 ~ a[d0bd]::foo)), vis: Public, span: /checkout/src/test/incremental/remove-private-item-cross-crate/auxiliary/a.rs:5:1: 5:25 (#0) }, ModChild { ident: std#0, res: Def(Mod, DefId(1:0 ~ std[3c40])), vis: Restricted(DefId(20:0 ~ a[d0bd])), span: no-location (#7) }]', compiler/rustc_query_system/src/query/plumbing.rs:657:9
stack backtrace:
   0:     0x7faaf22252bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc18d4c7d014769a
   1:     0x7faaf229511f - core::fmt::write::hb1ff246c8c00a9b0
   2:     0x7faaf22124c1 - std::io::Write::write_fmt::h32248f607bc51588
   3:     0x7faaf22250db - std::sys_common::backtrace::print::hc6b32bc7abd05d9b
   4:     0x7faaf2229894 - std::panicking::default_hook::{{closure}}::h6a9d1e968b5dbc0c
   5:     0x7faaf2229459 - std::panicking::default_hook::h3cf294e0bf4ea041
   6:     0x7faaf2c61361 - rustc_driver[fed345b07abbbca1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7faaf2229f78 - std::panicking::rust_panic_with_hook::he3799b6fad3f8eb1
   8:     0x7faaf2229da7 - std::panicking::begin_panic_handler::{{closure}}::habe275bcbdd1ab22
   9:     0x7faaf22257d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4630e0a3d791697c
  10:     0x7faaf2229a89 - rust_begin_unwind
  11:     0x7faaf21dd843 - core::panicking::panic_fmt::h2ed899c94647804e
  12:     0x7faaf2c175ea - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich_cold
  13:     0x7faaf42c08c1 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  14:     0x7faaf42849bf - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  15:     0x7faaf4249e68 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>>
  16:     0x7faaf42fa147 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::module_children, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  17:     0x7faaf4b0e06f - <rustc_metadata[1ae07c2cc1b994f1]::rmeta::decoder::cstore_impl::provide::{closure#7} as core[cb47576bca1a9817]::ops::function::FnOnce<(rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, ())>>::call_once
  18:     0x7faaf46980be - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>::{closure#1}, std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  19:     0x7faaf4294451 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  20:     0x7faaf420145d - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::ArenaCache<(), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>>
  21:     0x7faaf430f623 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::visible_parent_map, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  22:     0x7faaf5257f8d - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::PrettyPrinter>::try_print_visible_def_path_recur
  23:     0x7faaf526af11 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  24:     0x7faaf528ddf3 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::default_print_def_path
  25:     0x7faaf526aed2 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  26:     0x7faaf528a8ca - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::TraitRefPrintOnlyTraitPath as core[cb47576bca1a9817]::fmt::Display>::fmt
  27:     0x7faaf229511f - core::fmt::write::hb1ff246c8c00a9b0
  28:     0x7faaf528942a - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as core[cb47576bca1a9817]::fmt::Write>::write_fmt
  29:     0x7faaf53ab65f - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as rustc_middle[245f60d6e1f8fa6a]::ty::print::Print<rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter>>>::print
  30:     0x7faaf53abf58 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Display>::fmt
  31:     0x7faaf539ef26 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Debug>::fmt
  32:     0x7faaf229511f - core::fmt::write::hb1ff246c8c00a9b0
  33:     0x7faaf2295fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  34:     0x7faaf52d6836 - <rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind as core[cb47576bca1a9817]::fmt::Debug>::fmt
  35:     0x7faaf2291cd3 - core::fmt::builders::DebugTuple::field::ha6dcb5359088f338
  36:     0x7faaf53b9bc4 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::Binder<rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  37:     0x7faaf229511f - core::fmt::write::hb1ff246c8c00a9b0
  38:     0x7faaf2295fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  39:     0x7faaf52d6346 - <rustc_middle[245f60d6e1f8fa6a]::ty::Predicate as core[cb47576bca1a9817]::fmt::Debug>::fmt
  40:     0x7faaf2291a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  41:     0x7faaf4491ae4 - <&rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  42:     0x7faaf2291a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  43:     0x7faaf448a0d4 - <rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  44:     0x7faaf229511f - core::fmt::write::hb1ff246c8c00a9b0
  45:     0x7faaf2285719 - alloc::fmt::format::h16f52e55b390e579
  46:     0x7faaf444ea0b - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::dep_node::DepNode<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>>>
  47:     0x7faaf431111f - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::evaluate_obligation, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  48:     0x7faaf4440754 - <rustc_query_impl[fedb310caa113915]::Queries as rustc_middle[245f60d6e1f8fa6a]::ty::query::QueryEngine>::evaluate_obligation
  49:     0x7faaf4f11759 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  50:     0x7faaf4f1197d - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  51:     0x7faaf4f554cd - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  52:     0x7faaf4f41b8b - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  53:     0x7faaf4fde5de - <rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::ObligationForest<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor, rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::Outcome<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation, rustc_infer[d6b5c214026f6079]::traits::FulfillmentErrorCode>>
  54:     0x7faaf4f40adb - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7faaf4f40968 - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7faaf35c5316 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[b1b99abb46596271]::check_main_fn_ty::{closure#0}>
  57:     0x7faaf3559e1c - rustc_typeck[b1b99abb46596271]::check_crate
  58:     0x7faaf2e9f881 - rustc_interface[e1a8b57a397b7e94]::passes::analysis
  59:     0x7faaf4680864 - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<<rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  60:     0x7faaf45096f5 - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  61:     0x7faaf425c7e3 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<(), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>>
  62:     0x7faaf435a8e0 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::analysis, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  63:     0x7faaf2cfa1a0 - <rustc_interface[e1a8b57a397b7e94]::passes::QueryContext>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  64:     0x7faaf2ce3a26 - <rustc_interface[e1a8b57a397b7e94]::interface::Compiler>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}, core[cb47576bca1a9817]::result::Result<core[cb47576bca1a9817]::option::Option<rustc_interface[e1a8b57a397b7e94]::queries::Linker>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  65:     0x7faaf2cf1b3b - rustc_span[ad2be3227d5a21b1]::with_source_map::<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_interface[e1a8b57a397b7e94]::interface::create_compiler_and_run<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#1}>
  66:     0x7faaf2ce19b3 - <scoped_tls[1714625bafbf4de]::ScopedKey<rustc_span[ad2be3227d5a21b1]::SessionGlobals>>::set::<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  67:     0x7faaf2c8e489 - std[3c40391f8aa09356]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  68:     0x7faaf2cf4ace - std[3c40391f8aa09356]::panic::catch_unwind::<core[cb47576bca1a9817]::panic::unwind_safe::AssertUnwindSafe<<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1}::{closure#0}>, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  69:     0x7faaf2c83402 - <<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1} as core[cb47576bca1a9817]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  70:     0x7faaf2238e43 - std::sys::unix::thread::Thread::new::thread_start::h90d4051769151081
  71:     0x7faaec5a7609 - start_thread
  72:     0x7faaf209e293 - clone
  73:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (ce03fdb13 2022-02-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z query-dep-graph
query stack during panic:
query stack during panic:
#0 [module_children] collecting child items of module `a`
#1 [visible_parent_map] calculating the visible parent map
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/rlib_cross_crate/b.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/rlib_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: encountered incremental compilation error with module_children(a[d0bd])
   |
   = help: This is a known issue with the compiler. Run `cargo clean` to allow your project to compile
   = note: Please follow the instructions below to create a bug report with the provided information


thread 'rustc' panicked at 'Found unstable fingerprints for module_children(a[d0bd]): [ModChild { ident: X#0, res: Def(TyAlias, DefId(20:3 ~ a[d0bd]::X)), vis: Public, span: /checkout/src/test/incremental/rlib_cross_crate/auxiliary/a.rs:10:1: 10:18 (#0) }, ModChild { ident: Y#0, res: Def(TyAlias, DefId(20:4 ~ a[d0bd]::Y)), vis: Public, span: /checkout/src/test/incremental/rlib_cross_crate/auxiliary/a.rs:16:1: 16:19 (#0) }, ModChild { ident: std#0, res: Def(Mod, DefId(1:0 ~ std[3c40])), vis: Restricted(DefId(20:0 ~ a[d0bd])), span: no-location (#3) }]', compiler/rustc_query_system/src/query/plumbing.rs:657:9
stack backtrace:
   0:     0x7f78908e72bc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc18d4c7d014769a
   1:     0x7f789095711f - core::fmt::write::hb1ff246c8c00a9b0
   2:     0x7f78908d44c1 - std::io::Write::write_fmt::h32248f607bc51588
   3:     0x7f78908e70db - std::sys_common::backtrace::print::hc6b32bc7abd05d9b
   4:     0x7f78908eb894 - std::panicking::default_hook::{{closure}}::h6a9d1e968b5dbc0c
   5:     0x7f78908eb459 - std::panicking::default_hook::h3cf294e0bf4ea041
   6:     0x7f7891323361 - rustc_driver[fed345b07abbbca1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f78908ebf78 - std::panicking::rust_panic_with_hook::he3799b6fad3f8eb1
   8:     0x7f78908ebda7 - std::panicking::begin_panic_handler::{{closure}}::habe275bcbdd1ab22
   9:     0x7f78908e77d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h4630e0a3d791697c
  10:     0x7f78908eba89 - rust_begin_unwind
  11:     0x7f789089f843 - core::panicking::panic_fmt::h2ed899c94647804e
  12:     0x7f78912d95ea - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich_cold
  13:     0x7f78929828c1 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::incremental_verify_ich::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  14:     0x7f78929469bf - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>
  15:     0x7f789290be68 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<rustc_span[ad2be3227d5a21b1]::def_id::DefId, &[rustc_middle[245f60d6e1f8fa6a]::metadata::ModChild]>>
  16:     0x7f78929bc147 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::module_children, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  17:     0x7f78931d006f - <rustc_metadata[1ae07c2cc1b994f1]::rmeta::decoder::cstore_impl::provide::{closure#7} as core[cb47576bca1a9817]::ops::function::FnOnce<(rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, ())>>::call_once
  18:     0x7f7892d5a0be - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>::{closure#1}, std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  19:     0x7f7892956451 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, (), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>
  20:     0x7f78928c345d - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::ArenaCache<(), std[3c40391f8aa09356]::collections::hash::map::HashMap<rustc_span[ad2be3227d5a21b1]::def_id::DefId, rustc_span[ad2be3227d5a21b1]::def_id::DefId, core[cb47576bca1a9817]::hash::BuildHasherDefault<rustc_hash[ee90695a50132a87]::FxHasher>>>>
  21:     0x7f78929d1623 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::visible_parent_map, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  22:     0x7f7893919f8d - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::PrettyPrinter>::try_print_visible_def_path_recur
  23:     0x7f789392cf11 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  24:     0x7f789394fdf3 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::default_print_def_path
  25:     0x7f789392ced2 - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as rustc_middle[245f60d6e1f8fa6a]::ty::print::Printer>::print_def_path
  26:     0x7f789394c8ca - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::TraitRefPrintOnlyTraitPath as core[cb47576bca1a9817]::fmt::Display>::fmt
  27:     0x7f789095711f - core::fmt::write::hb1ff246c8c00a9b0
  28:     0x7f789394b42a - <rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter> as core[cb47576bca1a9817]::fmt::Write>::write_fmt
  29:     0x7f7893a6d65f - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as rustc_middle[245f60d6e1f8fa6a]::ty::print::Print<rustc_middle[245f60d6e1f8fa6a]::ty::print::pretty::FmtPrinter<&mut core[cb47576bca1a9817]::fmt::Formatter>>>::print
  30:     0x7f7893a6df58 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Display>::fmt
  31:     0x7f7893a60f26 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::TraitRef as core[cb47576bca1a9817]::fmt::Debug>::fmt
  32:     0x7f789095711f - core::fmt::write::hb1ff246c8c00a9b0
  33:     0x7f7890957fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  34:     0x7f7893998836 - <rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind as core[cb47576bca1a9817]::fmt::Debug>::fmt
  35:     0x7f7890953cd3 - core::fmt::builders::DebugTuple::field::ha6dcb5359088f338
  36:     0x7f7893a7bbc4 - <rustc_middle[245f60d6e1f8fa6a]::ty::sty::Binder<rustc_middle[245f60d6e1f8fa6a]::ty::PredicateKind> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  37:     0x7f789095711f - core::fmt::write::hb1ff246c8c00a9b0
  38:     0x7f7890957fe4 - <core::fmt::Formatter as core::fmt::Write>::write_fmt::h208955a58833cf54
  39:     0x7f7893998346 - <rustc_middle[245f60d6e1f8fa6a]::ty::Predicate as core[cb47576bca1a9817]::fmt::Debug>::fmt
  40:     0x7f7890953a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  41:     0x7f7892b53ae4 - <&rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  42:     0x7f7890953a3c - core::fmt::builders::DebugStruct::field::h87d66ef85751a62f
  43:     0x7f7892b4c0d4 - <rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>> as core[cb47576bca1a9817]::fmt::Debug>::fmt
  44:     0x7f789095711f - core::fmt::write::hb1ff246c8c00a9b0
  45:     0x7f7890947719 - alloc::fmt::format::h16f52e55b390e579
  46:     0x7f7892b10a0b - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::dep_node::DepNode<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::construct::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, rustc_middle[245f60d6e1f8fa6a]::infer::canonical::Canonical<rustc_middle[245f60d6e1f8fa6a]::ty::ParamEnvAnd<rustc_middle[245f60d6e1f8fa6a]::ty::Predicate>>>
  47:     0x7f78929d311f - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::evaluate_obligation, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  48:     0x7f7892b02754 - <rustc_query_impl[fedb310caa113915]::Queries as rustc_middle[245f60d6e1f8fa6a]::ty::query::QueryEngine>::evaluate_obligation
  49:     0x7f78935d3759 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  50:     0x7f78935d397d - <rustc_infer[d6b5c214026f6079]::infer::InferCtxt as rustc_trait_selection[c0d3f5f7c0875f30]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  51:     0x7f78936174cd - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  52:     0x7f7893603b8b - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor>::process_changed_obligations
  53:     0x7f78936a05de - <rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::ObligationForest<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillProcessor, rustc_data_structures[3fcd5223b39f2b09]::obligation_forest::Outcome<rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::PendingPredicateObligation, rustc_infer[d6b5c214026f6079]::traits::FulfillmentErrorCode>>
  54:     0x7f7893602adb - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_where_possible
  55:     0x7f7893602968 - <rustc_trait_selection[c0d3f5f7c0875f30]::traits::fulfill::FulfillmentContext as rustc_infer[d6b5c214026f6079]::traits::engine::TraitEngine>::select_all_or_error
  56:     0x7f7891c87316 - <rustc_infer[d6b5c214026f6079]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[b1b99abb46596271]::check_main_fn_ty::{closure#0}>
  57:     0x7f7891c1be1c - rustc_typeck[b1b99abb46596271]::check_crate
  58:     0x7f7891561881 - rustc_interface[e1a8b57a397b7e94]::passes::analysis
  59:     0x7f7892d42864 - <rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind as rustc_query_system[ab0b2543e70d13cc]::dep_graph::DepKind>::with_deps::<<rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  60:     0x7f7892bcb6f5 - <rustc_query_system[ab0b2543e70d13cc]::dep_graph::graph::DepGraph<rustc_middle[245f60d6e1f8fa6a]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[245f60d6e1f8fa6a]::ty::context::TyCtxt, (), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  61:     0x7f789291e7e3 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::try_execute_query::<rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt, rustc_query_system[ab0b2543e70d13cc]::query::caches::DefaultCache<(), core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>>
  62:     0x7f7892a1c8e0 - rustc_query_system[ab0b2543e70d13cc]::query::plumbing::get_query::<rustc_query_impl[fedb310caa113915]::queries::analysis, rustc_query_impl[fedb310caa113915]::plumbing::QueryCtxt>
  63:     0x7f78913bc1a0 - <rustc_interface[e1a8b57a397b7e94]::passes::QueryContext>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  64:     0x7f78913a5a26 - <rustc_interface[e1a8b57a397b7e94]::interface::Compiler>::enter::<rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}::{closure#2}, core[cb47576bca1a9817]::result::Result<core[cb47576bca1a9817]::option::Option<rustc_interface[e1a8b57a397b7e94]::queries::Linker>, rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  65:     0x7f78913b3b3b - rustc_span[ad2be3227d5a21b1]::with_source_map::<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_interface[e1a8b57a397b7e94]::interface::create_compiler_and_run<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#1}>
  66:     0x7f78913a39b3 - <scoped_tls[1714625bafbf4de]::ScopedKey<rustc_span[ad2be3227d5a21b1]::SessionGlobals>>::set::<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  67:     0x7f7891350489 - std[3c40391f8aa09356]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  68:     0x7f78913b6ace - std[3c40391f8aa09356]::panic::catch_unwind::<core[cb47576bca1a9817]::panic::unwind_safe::AssertUnwindSafe<<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1}::{closure#0}>, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>
  69:     0x7f7891345402 - <<std[3c40391f8aa09356]::thread::Builder>::spawn_unchecked_<rustc_interface[e1a8b57a397b7e94]::util::run_in_thread_pool_with_globals<rustc_interface[e1a8b57a397b7e94]::interface::run_compiler<core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>, rustc_driver[fed345b07abbbca1]::run_compiler::{closure#1}>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#0}, core[cb47576bca1a9817]::result::Result<(), rustc_errors[80188210a0e1f0c3]::ErrorReported>>::{closure#1} as core[cb47576bca1a9817]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
