plain
   Compiling fluent-syntax v0.11.0
   Compiling tracing v0.1.35
   Compiling tracing-subscriber v0.3.3
   Compiling thorin-dwp v0.3.0
error: internal compiler error: no warnings or errors encountered even though `delayed_good_path_bugs` issued

error: internal compiler error: trimmed_def_paths constructed
  |
  = note: delayed at    0: <rustc_errors::Handler>::delay_good_path_bug::<&str>
             1: rustc_middle::ty::print::pretty::trimmed_def_paths
             2: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), std::collections::hash::map::HashMap<rustc_span::def_id::DefId, rustc_span::symbol::Symbol, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>
             3: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::trimmed_def_paths, rustc_query_impl::plumbing::QueryCtxt>
             4: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::trimmed_def_paths
             5: <rustc_middle::ty::print::pretty::FmtPrinter as rustc_middle::ty::print::Printer>::print_def_path
             7: <rustc_middle::ty::context::TyCtxt>::def_path_str
             7: <rustc_middle::ty::context::TyCtxt>::def_path_str
             8: <rustc_lint::traits::DropTraitConstraints as rustc_lint::passes::LateLintPass>::check_item
             9: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            10: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_stmt
            11: rustc_hir::intravisit::walk_expr::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass>>
            12: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_expr
            13: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_body
            14: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
            15: rustc_hir::intravisit::walk_mod::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass>>
            16: rustc_lint::late::late_lint_mod::<rustc_lint::BuiltinCombinedModuleLateLintPass>
            18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
            19: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::lint_mod, rustc_query_impl::plumbing::QueryCtxt>
            20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::lint_mod
            20: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::lint_mod
            21: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
            22: rustc_data_structures::sync::par_for_each_in::<&[rustc_span::def_id::LocalDefId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
            23: <rustc_session::session::Session>::time::<(), rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
            24: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}> as core::ops::function::FnOnce<()>>::call_once
            25: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
            26: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
            27: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}>, ()>
            28: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}>
            30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
            31: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
            32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
            33: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            33: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            34: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
            35: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
            36: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
            37: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            38: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
            39: std::panicking::try::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
            40: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            41: std::sys::unix::thread::Thread::new::thread_start
            42: <unknown>
            43: <unknown>

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1499:13
stack backtrace:
   0:     0x7f5984e98ecd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   0:     0x7f5984e98ecd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f5984efe9a8 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f5984e89fe1 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f5984e9beee - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f5984e9bbb7 - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f598584ab04 - rustc_driver[491383e5761cfd5f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5984e9c691 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f59885cc163 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}
   8:     0x7f59885c91c6 - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_end_short_backtrace::<std[8a3c335779a4ef7b]::panicking::begin_panic<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}, !>
   9:     0x7f598580a9b6 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  10:     0x7f59885bd666 - std[8a3c335779a4ef7b]::panic::panic_any::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  11:     0x7f59885c28ad - <rustc_errors[a9e596aa33e6456]::HandlerInner as core[c1e30f1bd259d119]::ops::drop::Drop>::drop
  12:     0x7f59858a6ae2 - core[c1e30f1bd259d119]::ptr::drop_in_place::<rustc_session[5e51afea9c8b884e]::parse::ParseSess>
  13:     0x7f59858aee15 - <alloc[45623a189840f9f9]::rc::Rc<rustc_session[5e51afea9c8b884e]::session::Session> as core[c1e30f1bd259d119]::ops::drop::Drop>::drop
  14:     0x7f598583b87c - core[c1e30f1bd259d119]::ptr::drop_in_place::<rustc_interface[62be15c45d0bd73b]::interface::Compiler>
  15:     0x7f5985839537 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[62be15c45d0bd73b]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f598584d98e - rustc_interface[62be15c45d0bd73b]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>
  17:     0x7f59858304f2 - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  18:     0x7f59858ab0bf - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  19:     0x7f598583abde - std[8a3c335779a4ef7b]::panicking::try::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7f59858ac8d0 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[62be15c45d0bd73b]::util::run_in_thread_pool_with_globals<rustc_interface[62be15c45d0bd73b]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[491383e5761cfd5f]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7f5984ea8665 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
  22:     0x7f5984c47b43 - <unknown>
  23:     0x7f5984cd9a00 - <unknown>
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (d6996aabb 2022-09-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
