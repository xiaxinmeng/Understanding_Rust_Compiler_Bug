plain
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:44:17: expected wide pointer extra data (e.g. slice length or trait object vtable)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1313:9
stack backtrace:
   0:     0x7f2a47f0fc32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   0:     0x7f2a47f0fc32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha35fb4208844cb61
   1:     0x7f2a47f77638 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f2a47f00171 - std::io::Write::write_fmt::hf3faa85fa7d28190
   3:     0x7f2a47f12f76 - std::panicking::default_hook::{{closure}}::h243e0a014f6b15da
   4:     0x7f2a47f12b6d - std::panicking::default_hook::hdf681f01978f1e20
   5:     0x7f2a48a30ac1 - rustc_driver[65272f936875c6c1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2a47f13910 - std::panicking::rust_panic_with_hook::h1c127668bc0f49d8
   7:     0x7f2a4b2f2043 - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>::{closure#0}
   8:     0x7f2a4b2eff36 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_end_short_backtrace::<std[38ff3720b7fd637]::panicking::begin_panic<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2a48957f3f - std[38ff3720b7fd637]::panicking::begin_panic::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>
  10:     0x7f2a4b211cd6 - std[38ff3720b7fd637]::panic::panic_any::<rustc_errors[bdd66cd3e42ec997]::ExplicitBug>
  11:     0x7f2a4b20abc6 - <rustc_errors[bdd66cd3e42ec997]::HandlerInner>::bug::<&alloc[24f448636cd10183]::string::String>
  12:     0x7f2a4b20a890 - <rustc_errors[bdd66cd3e42ec997]::Handler>::bug::<&alloc[24f448636cd10183]::string::String>
  13:     0x7f2a4b305cce - rustc_middle[e995fb8dc5766eea]::ty::context::tls::with_opt::<rustc_middle[e995fb8dc5766eea]::util::bug::opt_span_bug_fmt<rustc_span[7ff2d2023f32a35]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f2a4b3093c9 - rustc_middle[e995fb8dc5766eea]::util::bug::opt_span_bug_fmt::<rustc_span[7ff2d2023f32a35]::span_encoding::Span>
  15:     0x7f2a4895fd8e - rustc_middle[e995fb8dc5766eea]::util::bug::bug_fmt
  16:     0x7f2a49fbe3d4 - <rustc_const_eval[3a2bef57e4276d55]::interpret::place::MemPlaceMeta>::unwrap_meta
  17:     0x7f2a49eaa671 - <rustc_const_eval[3a2bef57e4276d55]::interpret::eval_context::InterpCx<rustc_const_eval[3a2bef57e4276d55]::const_eval::machine::CompileTimeInterpreter>>::size_and_align_of
  18:     0x7f2a49fc5619 - rustc_const_eval[3a2bef57e4276d55]::const_eval::valtrees::fill_place_recursively
  19:     0x7f2a49fc3523 - rustc_const_eval[3a2bef57e4276d55]::const_eval::valtrees::valtree_to_const_value
  20:     0x7f2a49f0bd8b - <rustc_const_eval[3a2bef57e4276d55]::provide::{closure#2} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt, (rustc_middle[e995fb8dc5766eea]::ty::Ty, rustc_middle[e995fb8dc5766eea]::ty::consts::valtree::ValTree))>>::call_once
  21:     0x7f2a4a1d0778 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::valtree_to_const_val, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  22:     0x7f2a4a53d4fb - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::valtree_to_const_val
  23:     0x7f2a49f6def7 - rustc_const_eval[3a2bef57e4276d55]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f2a4a1e4fe6 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::eval_to_const_value_raw, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  25:     0x7f2a4a53d465 - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f2a49f6cdd6 - rustc_const_eval[3a2bef57e4276d55]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f2a4a1e4fe6 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::eval_to_const_value_raw, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  28:     0x7f2a4a53d465 - <rustc_query_impl[757f9843cb5f89aa]::Queries as rustc_middle[e995fb8dc5766eea]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7f2a4b176f2e - <rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt>::const_eval_global_id
  30:     0x7f2a4b1767ac - <rustc_middle[e995fb8dc5766eea]::ty::context::TyCtxt>::const_eval_poly
  31:     0x7f2a4ad18ec7 - <rustc_lint[bf8d83b15cf4719b]::builtin::UnusedBrokenConst as rustc_lint[bf8d83b15cf4719b]::passes::LateLintPass>::check_item
  32:     0x7f2a4ace19d4 - <rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass as rustc_lint[bf8d83b15cf4719b]::passes::LateLintPass>::check_item
  33:     0x7f2a48bb35c5 - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_nested_item
  34:     0x7f2a48bbb03c - rustc_hir[53f761ba5c0273ac]::intravisit::walk_mod::<rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>>
  35:     0x7f2a48bb1b9c - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_mod
  36:     0x7f2a48bb35d0 - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_nested_item
  37:     0x7f2a48bbb03c - rustc_hir[53f761ba5c0273ac]::intravisit::walk_mod::<rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>>
  38:     0x7f2a48bb1b9c - <rustc_lint[bf8d83b15cf4719b]::late::LateContextAndPass<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass> as rustc_hir[53f761ba5c0273ac]::intravisit::Visitor>::visit_mod
  39:     0x7f2a48bac6e9 - rustc_lint[bf8d83b15cf4719b]::late::late_lint_crate::<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass>
  40:     0x7f2a48b46c82 - rustc_data_structures[3ce3321229418a76]::sync::join::<rustc_lint[bf8d83b15cf4719b]::late::check_crate<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass, rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[bf8d83b15cf4719b]::late::check_crate<rustc_lint[bf8d83b15cf4719b]::BuiltinCombinedLateLintPass, rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  41:     0x7f2a48b4058a - <rustc_session[485e60f1359c6d5a]::session::Session>::time::<(), rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}::{closure#0}>
  42:     0x7f2a48b54935 - std[38ff3720b7fd637]::panicking::try::<(), core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}::{closure#2}>>
  43:     0x7f2a48b07392 - <core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}> as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once
  44:     0x7f2a48b54a26 - std[38ff3720b7fd637]::panicking::try::<(), core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}::{closure#0}>>
  45:     0x7f2a48b42e22 - <rustc_session[485e60f1359c6d5a]::session::Session>::time::<(), rustc_interface[ef49b6c417f36e1c]::passes::analysis::{closure#5}>
  46:     0x7f2a48b3d82c - rustc_interface[ef49b6c417f36e1c]::passes::analysis
  47:     0x7f2a4a11d7bc - rustc_query_system[989e46076e3dfb40]::query::plumbing::try_execute_query::<rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt, rustc_query_system[989e46076e3dfb40]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>>
  48:     0x7f2a4a211be9 - rustc_query_system[989e46076e3dfb40]::query::plumbing::get_query::<rustc_query_impl[757f9843cb5f89aa]::queries::analysis, rustc_query_impl[757f9843cb5f89aa]::plumbing::QueryCtxt>
  49:     0x7f2a48a224fa - <rustc_interface[ef49b6c417f36e1c]::passes::QueryContext>::enter::<rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  50:     0x7f2a489c5bf6 - <rustc_interface[ef49b6c417f36e1c]::interface::Compiler>::enter::<rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[ef49b6c417f36e1c]::queries::Linker>, rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  51:     0x7f2a489a76f6 - rustc_span[7ff2d2023f32a35]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7f2a489d9107 - rustc_interface[ef49b6c417f36e1c]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>
  53:     0x7f2a489dc452 - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[7ff2d2023f32a35]::SessionGlobals>>::set::<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  54:     0x7f2a48a24479 - std[38ff3720b7fd637]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>
  55:     0x7f2a489de6fe - std[38ff3720b7fd637]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  56:     0x7f2a48a1f510 - <<std[38ff3720b7fd637]::thread::Builder>::spawn_unchecked_<rustc_interface[ef49b6c417f36e1c]::util::run_in_thread_pool_with_globals<rustc_interface[ef49b6c417f36e1c]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>, rustc_driver[65272f936875c6c1]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[bdd66cd3e42ec997]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f2a47f20313 - std::sys::unix::thread::Thread::new::thread_start::h38902d511e7013ce
  58:     0x7f2a42470609 - start_thread
  59:     0x7f2a47d83163 - clone
  60:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (dd125f11d 2022-04-24) running on x86_64-unknown-linux-gnu

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
Build completed unsuccessfully in 0:09:45
