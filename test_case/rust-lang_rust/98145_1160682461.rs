plain
error: internal compiler error: compiler/rustc_const_eval/src/interpret/eval_context.rs:203:17: The type checker should prevent reading from a never-written local

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f9989205d72 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f998926d9a8 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f99891f6051 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f9989209089 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f9989208d2a - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f9989cd5e34 - rustc_driver[d4c9122d320bec50]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f99892098ef - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f998c476903 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}
   8:     0x7f998c4759a6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}, !>
   9:     0x7f9989c59c76 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  10:     0x7f998c4352b6 - std[836a811975e52724]::panic::panic_any::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  11:     0x7f998c432f56 - <rustc_errors[81b66f48ab2827ec]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7f998c432c20 - <rustc_errors[81b66f48ab2827ec]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7f998c6457be - rustc_middle[7c2d6da264b3b0e3]::ty::context::tls::with_opt::<rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt<rustc_span[ec683a5befddaf22]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f998c645a99 - rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt::<rustc_span[ec683a5befddaf22]::span_encoding::Span>
  15:     0x7f9989c620c5 - rustc_middle[7c2d6da264b3b0e3]::util::bug::bug_fmt
  16:     0x7f998b1a3175 - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_const_eval[af402a68111e18b7]::const_eval::machine::CompileTimeInterpreter>>::access_local
  17:     0x7f998b1c2dcc - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_const_eval[af402a68111e18b7]::const_eval::machine::CompileTimeInterpreter>>::eval_place
  18:     0x7f998b1b1214 - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_const_eval[af402a68111e18b7]::const_eval::machine::CompileTimeInterpreter>>::eval_rvalue_into_place
  19:     0x7f998b1ad373 - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_const_eval[af402a68111e18b7]::const_eval::machine::CompileTimeInterpreter>>::run
  20:     0x7f998b1dd35a - rustc_const_eval[af402a68111e18b7]::const_eval::eval_queries::eval_to_allocation_raw_provider
  21:     0x7f998b810cb7 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::eval_to_allocation_raw, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  22:     0x7f998b3aea21 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::eval_to_allocation_raw
  23:     0x7f998b1dcaf5 - rustc_const_eval[af402a68111e18b7]::const_eval::eval_queries::eval_to_allocation_raw_provider
  24:     0x7f998b810cb7 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::eval_to_allocation_raw, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  25:     0x7f998b3aea21 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::eval_to_allocation_raw
  26:     0x7f998c5de3ab - <rustc_middle[7c2d6da264b3b0e3]::ty::query::TyCtxtEnsure>::eval_static_initializer
  27:     0x7f998c02f298 - <rustc_lint[ea1b66899186e3e4]::BuiltinCombinedModuleLateLintPass as rustc_lint[ea1b66899186e3e4]::passes::LateLintPass>::check_item
  28:     0x7f998c0b4aea - <rustc_lint[ea1b66899186e3e4]::late::LateContextAndPass<rustc_lint[ea1b66899186e3e4]::BuiltinCombinedModuleLateLintPass> as rustc_hir[aa1b53fbe977c493]::intravisit::Visitor>::visit_nested_item
  29:     0x7f998c0e4dac - rustc_hir[aa1b53fbe977c493]::intravisit::walk_mod::<rustc_lint[ea1b66899186e3e4]::late::LateContextAndPass<rustc_lint[ea1b66899186e3e4]::BuiltinCombinedModuleLateLintPass>>
  30:     0x7f998c0bb240 - rustc_lint[ea1b66899186e3e4]::late::late_lint_mod::<rustc_lint[ea1b66899186e3e4]::BuiltinCombinedModuleLateLintPass>
  31:     0x7f998c02adcd - rustc_lint[ea1b66899186e3e4]::lint_mod
  32:     0x7f998b72ea4a - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_span[ec683a5befddaf22]::def_id::LocalDefId, ()>>
  33:     0x7f998b83a944 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::lint_mod, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  34:     0x7f998b3a5384 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::lint_mod
  35:     0x7f9989e97416 - <rustc_middle[7c2d6da264b3b0e3]::hir::map::Map>::for_each_module::<rustc_lint[ea1b66899186e3e4]::late::check_crate<rustc_lint[ea1b66899186e3e4]::BuiltinCombinedLateLintPass, rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  36:     0x7f9989e1e690 - <rustc_session[898bbfa7f8b71f8d]::session::Session>::time::<(), rustc_lint[ea1b66899186e3e4]::late::check_crate<rustc_lint[ea1b66899186e3e4]::BuiltinCombinedLateLintPass, rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  37:     0x7f9989e1e85a - <rustc_session[898bbfa7f8b71f8d]::session::Session>::time::<(), rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  38:     0x7f9989e0b025 - std[836a811975e52724]::panic::catch_unwind::<core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  39:     0x7f9989e9ffa2 - <core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}> as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once
  40:     0x7f9989e0b146 - std[836a811975e52724]::panic::catch_unwind::<core[6d9550a4e960c99f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}::{closure#1}>, ()>
  41:     0x7f9989e211ce - <rustc_session[898bbfa7f8b71f8d]::session::Session>::time::<(), rustc_interface[867e122e1003978e]::passes::analysis::{closure#5}>
  42:     0x7f9989e08e1c - rustc_interface[867e122e1003978e]::passes::analysis
  43:     0x7f998b765edc - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<(), core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>>
  44:     0x7f998b83a3f2 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::analysis, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  45:     0x7f998b38e25e - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::analysis
  46:     0x7f9989d419ca - <rustc_interface[867e122e1003978e]::passes::QueryContext>::enter::<rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  47:     0x7f9989cf46ad - <rustc_interface[867e122e1003978e]::interface::Compiler>::enter::<rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[867e122e1003978e]::queries::Linker>, rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  48:     0x7f9989cd7466 - rustc_span[ec683a5befddaf22]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_interface[867e122e1003978e]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}>::{closure#1}>
  49:     0x7f9989cf592e - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ec683a5befddaf22]::SessionGlobals>>::set::<rustc_interface[867e122e1003978e]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  50:     0x7f9989d4cff9 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[867e122e1003978e]::util::run_in_thread_pool_with_globals<rustc_interface[867e122e1003978e]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  51:     0x7f9989d44e59 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[867e122e1003978e]::util::run_in_thread_pool_with_globals<rustc_interface[867e122e1003978e]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[d4c9122d320bec50]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f99892162b3 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  53:     0x7f9983766609 - start_thread
  54:     0x7f9989079133 - clone
  55:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (13ee4688d 2022-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `internal::DEFAULT_HASH_TYPES`
#1 [eval_to_allocation_raw] const-evaluating + checking `internal::DEFAULT_HASH_TYPES`
#2 [lint_mod] linting module `internal`
#3 [analysis] running analysis passes on this crate
error: could not compile `rustc_lint`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:17
