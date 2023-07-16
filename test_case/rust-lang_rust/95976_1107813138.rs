plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:44:17: expected wide pointer extra data (e.g. slice length or trait object vtable)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1313:9
stack backtrace:
   0:     0x7f1b1d395c32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f1b1d3fd638 - core::fmt::write::h42234c3e51154f4c
   1:     0x7f1b1d3fd638 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f1b1d386171 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f1b1d398f76 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f1b1d398b6d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f1b1deb6ac1 - rustc_driver[65272f936875c6c1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b1d399910 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f1b207781d3 - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>::{closure#0}
   8:     0x7f1b207760c6 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_end_short_backtrace::<std[38ff3720b7fd637]::panicking::begin_panic<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>::{closure#0}, !>
   9:     0x7f1b1ddddf3f - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>
  10:     0x7f1b20697e66 - std[38ff3720b7fd637]::panic::panic_any::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>
  11:     0x7f1b20690d56 - <rustc_errors[bdd66cd3e42ec997]::HandlerInner>::bug::<&alloc[24f448636cd10183]::string::String>
  12:     0x7f1b20690a20 - <rustc_errors[bdd66cd3e42ec997]::Handler>::bug::<&alloc[24f448636cd10183]::string::String>
  13:     0x7f1b2078be5e - rustc_middle[e995fb8dc5766eea]::ty::context::tls::with_opt::<rustc_middle[e995fb8dc5766eea]::util::bug::opt_span_bug_fmt<rustc_span[7ff2d2023f32a35]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f1b2078f559 - rustc_middle[e995fb8dc5766eea]::util::bug::opt_span_bug_fmt::<rustc_span[7ff2d2023f32a35]::span_encoding::Span>
  15:     0x7f1b1dde5d8e - rustc_middle[e995fb8dc5766eea]::util::bug::bug_fmt
  16:     0x7f1b1f4443f4 - <rustc_const_eval[3a2bef57e4276d55]::interpret::place::MemPlaceMeta>::unwrap_meta
  17:     0x7f1b1f330681 - <rustc_const_eval[3a2bef57e4276d55]::interpret::eval_context::InterpCx<rustc_const_eval[3a2bef57e4276d55]::const_eval::machine::CompileTimeInterpreter>>::size_and_align_of
  18:     0x7f1b1f360b26 - <rustc_const_eval[3a2bef57e4276d55]::interpret::eval_context::InterpCx<rustc_const_eval[3a2bef57e4276d55]::const_eval::machine::CompileTimeInterpreter>>::mplace_projection
  19:     0x7f1b1f44b38d - rustc_const_eval[3a2bef57e4276d55]::const_eval::valtrees::fill_place_recursively
  20:     0x7f1b1f449543 - rustc_const_eval[3a2bef57e4276d55]::const_eval::valtrees::valtree_to_const_value
  21:     0x7f1b1f391dab - <rustc_const_eval[3a2bef57e4276d55]::provide::{closure#2} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt, (rustc_middle[e995fb8dc5766eea]::ty::Ty, rustc_middle[e995fb8dc5766eea]::ty::consts::valtree::ValTree))>>::call_once
  22:     0x7f1b1f656918 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::valtree_to_const_val, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  23:     0x7f1b1f9c369b - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::valtree_to_const_val
  24:     0x7f1b1f3f3f17 - rustc_const_eval[3a2bef57e4276d55]::const_eval::eval_queries::eval_to_const_value_raw_provider
  25:     0x7f1b1f66b186 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::eval_to_const_value_raw, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  26:     0x7f1b1f9c3605 - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::eval_to_const_value_raw
  27:     0x7f1b1f3f2df6 - rustc_const_eval[3a2bef57e4276d55]::const_eval::eval_queries::eval_to_const_value_raw_provider
  28:     0x7f1b1f66b186 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::eval_to_const_value_raw, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  29:     0x7f1b1f9c3605 - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7f1b205fd0be - <rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt>::const_eval_global_id
  31:     0x7f1b205fc93c - <rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt>::const_eval_poly
  32:     0x7f1b2019f057 - <rustc_lint[bf8d83b15cf4719b]::builtin::UnusedBrokenConst as rustc_lint[bf8d83b15cf4719b]::passes::LateLintPass>::check_item
  33:     0x7f1b20167b64 - <rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass as rustc_lint[bf8d83b15cf4719b]::passes::LateLintPass>::check_item
  34:     0x7f1b1e0395c5 - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_nested_item
  35:     0x7f1b1e04103c - rustc_hir[53f761ba5c0273ac]::intravisit::walk_mod::<rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>>
  36:     0x7f1b1e037b9c - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_mod
  37:     0x7f1b1e0395d0 - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_nested_item
  38:     0x7f1b1e04103c - rustc_hir[53f761ba5c0273ac]::intravisit::walk_mod::<rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>>
  39:     0x7f1b1e037b9c - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_mod
  40:     0x7f1b1e0326e9 - rustc_lint[bf8d83b15cf4719b]::late::late_lint_crate::<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>
  41:     0x7f1b1dfccc82 - rustc_data_structures[3ce3321229418a76]::sync::join::<rustc_lint[bf8d83b15cf4719b]::late::check_crate<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass, rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[bf8d83b15cf4719b]::late::check_crate<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass, rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  42:     0x7f1b1dfc658a - <rustc_session[485e60f1359c6d5a]::session::Session>::time::<(), rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}>
  43:     0x7f1b1dfda935 - std[38ff3720b7fd637]::panicking::try::<(), core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}>>
  44:     0x7f1b1df8d392 - <core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}> as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once
  45:     0x7f1b1dfdaa26 - std[38ff3720b7fd637]::panicking::try::<(), core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}>>
  46:     0x7f1b1dfc8e22 - <rustc_session[485e60f1359c6d5a]::session::Session>::time::<(), rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}>
  47:     0x7f1b1dfc382c - rustc_interface[ef49b6c417f36e1c]::passes::analysis
  48:     0x7f1b1f5a395c - rustc_query_system[989e46076e3dfb40]::query::plumbing::try_execute_query::<rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt, rustc_query_system[989e46076e3dfb40]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>>
  49:     0x7f1b1f697d89 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::analysis, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  50:     0x7f1b1dea84fa - <rustc_interface[ef49b6c417f36e1c]::passes::QueryContext>::enter::<rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  51:     0x7f1b1de4bbf6 - <rustc_interface[ef49b6c417f36e1c]::interface::Compiler>::enter::<rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[ef49b6c417f36e1c]::queries::Linker>, rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  52:     0x7f1b1de2d6f6 - rustc_span[7ff2d2023f32a35]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#1}>
  53:     0x7f1b1de5f107 - rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>
  54:     0x7f1b1de62452 - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[7ff2d2023f32a35]::SessionGlobals>>::set::<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  55:     0x7f1b1deaa479 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  56:     0x7f1b1de646fe - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  57:     0x7f1b1dea5510 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f1b1d3a6313 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  59:     0x7f1b178f6609 - start_thread
  60:     0x7f1b1d209163 - clone
  61:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (5dc2033a7 2022-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [valtree_to_const_val] convert type-level constant value to mir constant value
#1 [eval_to_const_value_raw] simplifying constant for the type system `builder::EMPTY_C_STR`
#2 [eval_to_const_value_raw] simplifying constant for the type system `builder::EMPTY_C_STR`
#3 [analysis] running analysis passes on this crate
error: could not compile `rustc_codegen_llvm`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:55
