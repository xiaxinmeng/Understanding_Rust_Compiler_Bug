plain
   Compiling ansi_term v0.12.1
   Compiling adler v0.2.3
   Compiling unicode-script v0.5.5
   Compiling miniz_oxide v0.4.0
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Unknown(T)', compiler/rustc_lint/src/builtin.rs:2524:73
   0:     0x7f55935290c2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbef12973c5507d53
   0:     0x7f55935290c2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbef12973c5507d53
   1:     0x7f5593591058 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f55935198b1 - std::io::Write::write_fmt::h0fc5efafbc8ce341
   3:     0x7f5593528e85 - std::sys_common::backtrace::print::h7f5fe1a3e38a3041
   4:     0x7f559352c267 - std::panicking::default_hook::{{closure}}::h62c63d72dd478c99
   5:     0x7f559352bfca - std::panicking::default_hook::h61d2d5a20d5abab2
   6:     0x7f5593ea4656 - rustc_driver[aa2c3d0cc0fa7243]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f559352cb73 - std::panicking::rust_panic_with_hook::h6fb0b7b5144c30a1
   8:     0x7f559352c8a7 - std::panicking::begin_panic_handler::{{closure}}::ha34ec5ba80bbe079
   9:     0x7f559352966c - std::sys_common::backtrace::__rust_end_short_backtrace::h94c9e486fd929163
  10:     0x7f559352c572 - rust_begin_unwind
  11:     0x7f55934de4e3 - core::panicking::panic_fmt::h3a5b5d72039ab650
  12:     0x7f55934de7f3 - core::result::unwrap_failed::h119639a553f91aa2
  13:     0x7f5596afaee9 - <rustc_lint[c1685b9e95b96022]::builtin::InvalidValue as rustc_lint[c1685b9e95b96022]::passes::LateLintPass>::check_expr::ty_find_init_error
  14:     0x7f5596af9b14 - <rustc_lint[c1685b9e95b96022]::builtin::InvalidValue as rustc_lint[c1685b9e95b96022]::passes::LateLintPass>::check_expr
  15:     0x7f5596b32478 - <rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass as rustc_lint[c1685b9e95b96022]::passes::LateLintPass>::check_expr
  16:     0x7f5596b8acf9 - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_expr
  17:     0x7f5596b8ad04 - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_expr
  18:     0x7f5596b8ad04 - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_expr
  19:     0x7f5596b89c51 - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_nested_body
  20:     0x7f5596b8bb2d - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_fn
  21:     0x7f5596bc93a2 - rustc_hir[86677f9b4cf46361]::intravisit::walk_impl_item::<rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass>>
  22:     0x7f5596b888bd - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_nested_impl_item
  23:     0x7f5596bd337c - rustc_hir[86677f9b4cf46361]::intravisit::walk_item::<rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass>>
  24:     0x7f5596b880fd - <rustc_lint[c1685b9e95b96022]::late::LateContextAndPass<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass> as rustc_hir[86677f9b4cf46361]::intravisit::Visitor>::visit_nested_item
  25:     0x7f5596b8e1dd - rustc_lint[c1685b9e95b96022]::late::late_lint_mod::<rustc_lint[c1685b9e95b96022]::BuiltinCombinedModuleLateLintPass>
  26:     0x7f5596b2d1ed - rustc_lint[c1685b9e95b96022]::lint_mod
  27:     0x7f5595fd4e38 - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<rustc_span[6f04c596547a783a]::def_id::LocalDefId, ()>>
  28:     0x7f5596119a0c - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::lint_mod, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  29:     0x7f5595c14d54 - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::lint_mod
  30:     0x7f55940e6564 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c7b5c7cf6db03e91]::sync::par_for_each_in<&[rustc_hir[86677f9b4cf46361]::hir_id::OwnerId], <rustc_middle[f414e03c15862bd6]::hir::map::Map>::par_for_each_module<rustc_lint[c1685b9e95b96022]::late::check_crate<rustc_lint[c1685b9e95b96022]::BuiltinCombinedLateLintPass, rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  31:     0x7f559413445b - rustc_data_structures[c7b5c7cf6db03e91]::sync::par_for_each_in::<&[rustc_hir[86677f9b4cf46361]::hir_id::OwnerId], <rustc_middle[f414e03c15862bd6]::hir::map::Map>::par_for_each_module<rustc_lint[c1685b9e95b96022]::late::check_crate<rustc_lint[c1685b9e95b96022]::BuiltinCombinedLateLintPass, rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  32:     0x7f55940122d6 - <rustc_session[d359256240218516]::session::Session>::time::<(), rustc_lint[c1685b9e95b96022]::late::check_crate<rustc_lint[c1685b9e95b96022]::BuiltinCombinedLateLintPass, rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  33:     0x7f55940e7d49 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#5}::{closure#1}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  34:     0x7f55940143d5 - <rustc_session[d359256240218516]::session::Session>::time::<(), rustc_interface[f8ab63ad520f2673]::passes::analysis::{closure#5}>
  35:     0x7f5594062c23 - rustc_interface[f8ab63ad520f2673]::passes::analysis
  36:     0x7f5596053ef2 - rustc_query_system[818b688b820e3781]::query::plumbing::try_execute_query::<rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt, rustc_query_system[818b688b820e3781]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>>
  37:     0x7f5596119530 - rustc_query_system[818b688b820e3781]::query::plumbing::get_query::<rustc_query_impl[bf07a966c3412bd5]::queries::analysis, rustc_query_impl[bf07a966c3412bd5]::plumbing::QueryCtxt>
  38:     0x7f5595bf750a - <rustc_query_impl[bf07a966c3412bd5]::Queries as rustc_middle[f414e03c15862bd6]::ty::query::QueryEngine>::analysis
  39:     0x7f5593ec3b07 - <rustc_interface[f8ab63ad520f2673]::passes::QueryContext>::enter::<rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  40:     0x7f5593f3ef72 - <rustc_interface[f8ab63ad520f2673]::interface::Compiler>::enter::<rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[f8ab63ad520f2673]::queries::Linker>, rustc_errors[3464920041138443]::ErrorGuaranteed>>
  41:     0x7f5593ea61b2 - rustc_span[6f04c596547a783a]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  42:     0x7f5593f2d9b3 - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[6f04c596547a783a]::SessionGlobals>>::set::<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  43:     0x7f5593ed7ee0 - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f8ab63ad520f2673]::util::run_in_thread_pool_with_globals<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>
  44:     0x7f5593ebd809 - <<std[b4d7b7572d6e9bb7]::thread::Builder>::spawn_unchecked_<rustc_interface[f8ab63ad520f2673]::util::run_in_thread_pool_with_globals<rustc_interface[f8ab63ad520f2673]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>, rustc_driver[aa2c3d0cc0fa7243]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[3464920041138443]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f5593539d8e - std::sys::unix::thread::Thread::new::thread_start::h05afa8442b34c479
  46:     0x7f55932d3b43 - <unknown>
  47:     0x7f5593365a00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (054807c51 2022-10-30) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [lint_mod] linting module `flavors::list`
#1 [analysis] running analysis passes on this crate
error: could not compile `crossbeam-channel`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:55
cat: /tmp/toolstate/toolstates.json: No such file or directory
