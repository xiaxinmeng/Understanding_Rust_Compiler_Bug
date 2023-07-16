plain
   Compiling rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
   Compiling md-5 v0.10.0
   Compiling sha-1 v0.10.0
   Compiling adler v0.2.3
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Unknown(T)', compiler/rustc_lint/src/builtin.rs:2508:65
stack backtrace:
   0:     0x7f9373160f52 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbef12973c5507d53
   1:     0x7f93731c87c8 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f9373151631 - std::io::Write::write_fmt::h0fc5efafbc8ce341
   3:     0x7f9373160d15 - std::sys_common::backtrace::print::h7f5fe1a3e38a3041
   4:     0x7f93731640f7 - std::panicking::default_hook::{{closure}}::h62c63d72dd478c99
   5:     0x7f9373163e55 - std::panicking::default_hook::h61d2d5a20d5abab2
   6:     0x7f9373ad1534 - rustc_driver[220d861fe5665444]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f9373164a03 - std::panicking::rust_panic_with_hook::h6fb0b7b5144c30a1
   8:     0x7f9373164737 - std::panicking::begin_panic_handler::{{closure}}::ha34ec5ba80bbe079
   9:     0x7f93731614fc - std::sys_common::backtrace::__rust_end_short_backtrace::h94c9e486fd929163
  10:     0x7f9373164402 - rust_begin_unwind
  11:     0x7f93731167b3 - core::panicking::panic_fmt::h3a5b5d72039ab650
  12:     0x7f9373116ac3 - core::result::unwrap_failed::h119639a553f91aa2
  13:     0x7f9376039192 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::variant_find_init_error
  14:     0x7f93760397f0 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::ty_find_init_error
  15:     0x7f9376038800 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::variant_find_init_error
  16:     0x7f93760397f0 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::ty_find_init_error
  17:     0x7f9376039b5c - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::ty_find_init_error
  18:     0x7f9376038800 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::variant_find_init_error
  19:     0x7f93760397f0 - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr::ty_find_init_error
  20:     0x7f93760384ec - <rustc_lint[6e712a90dbb48168]::builtin::InvalidValue as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr
  21:     0x7f937604b18e - <rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass as rustc_lint[6e712a90dbb48168]::passes::LateLintPass>::check_expr
  22:     0x7f937608c5e9 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_expr
  23:     0x7f937608c5f4 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_expr
  24:     0x7f937608c5f4 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_expr
  25:     0x7f937608b513 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_nested_body
  26:     0x7f937608d301 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_fn
  27:     0x7f9376070f0b - rustc_hir[bc81097ff660bad4]::intravisit::walk_impl_item::<rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass>>
  28:     0x7f937608a225 - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_nested_impl_item
  29:     0x7f937607aa2c - rustc_hir[bc81097ff660bad4]::intravisit::walk_item::<rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass>>
  30:     0x7f9376089b6a - <rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass> as rustc_hir[bc81097ff660bad4]::intravisit::Visitor>::visit_nested_item
  31:     0x7f9376076bcc - rustc_hir[bc81097ff660bad4]::intravisit::walk_mod::<rustc_lint[6e712a90dbb48168]::late::LateContextAndPass<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass>>
  32:     0x7f937608f5cc - rustc_lint[6e712a90dbb48168]::late::late_lint_mod::<rustc_lint[6e712a90dbb48168]::BuiltinCombinedModuleLateLintPass>
  33:     0x7f9376045dbd - rustc_lint[6e712a90dbb48168]::lint_mod
  34:     0x7f93756ccf06 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<rustc_span[316458062c9a7e81]::def_id::LocalDefId, ()>>
  35:     0x7f93757f4dac - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::lint_mod, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  36:     0x7f937539a204 - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::lint_mod
  37:     0x7f9373c8daf4 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d503008fd0bcf4b6]::sync::par_for_each_in<&[rustc_hir[bc81097ff660bad4]::hir_id::OwnerId], <rustc_middle[df7c89aebf5ff77c]::hir::map::Map>::par_for_each_module<rustc_lint[6e712a90dbb48168]::late::check_crate<rustc_lint[6e712a90dbb48168]::BuiltinCombinedLateLintPass, rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  38:     0x7f9373bf297b - rustc_data_structures[d503008fd0bcf4b6]::sync::par_for_each_in::<&[rustc_hir[bc81097ff660bad4]::hir_id::OwnerId], <rustc_middle[df7c89aebf5ff77c]::hir::map::Map>::par_for_each_module<rustc_lint[6e712a90dbb48168]::late::check_crate<rustc_lint[6e712a90dbb48168]::BuiltinCombinedLateLintPass, rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7f9373bfa82c - <rustc_session[3283e5b6d6e5bbce]::session::Session>::time::<(), rustc_lint[6e712a90dbb48168]::late::check_crate<rustc_lint[6e712a90dbb48168]::BuiltinCombinedLateLintPass, rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  40:     0x7f9373bfa980 - <rustc_session[3283e5b6d6e5bbce]::session::Session>::time::<(), rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  41:     0x7f9373bf2355 - std[b4d7b7572d6e9bb7]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  42:     0x7f9373c8f193 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  43:     0x7f9373bf2476 - std[b4d7b7572d6e9bb7]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}::{closure#1}>, ()>
  44:     0x7f9373bfc705 - <rustc_session[3283e5b6d6e5bbce]::session::Session>::time::<(), rustc_interface[155c1abf899b1ba8]::passes::analysis::{closure#5}>
  45:     0x7f9373c2705c - rustc_interface[155c1abf899b1ba8]::passes::analysis
  46:     0x7f93757132c0 - rustc_query_system[8252885e881cb6ca]::query::plumbing::try_execute_query::<rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt, rustc_query_system[8252885e881cb6ca]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>>
  47:     0x7f93757f48d0 - rustc_query_system[8252885e881cb6ca]::query::plumbing::get_query::<rustc_query_impl[30976f78eb3c7de9]::queries::analysis, rustc_query_impl[30976f78eb3c7de9]::plumbing::QueryCtxt>
  48:     0x7f937537c9ba - <rustc_query_impl[30976f78eb3c7de9]::Queries as rustc_middle[df7c89aebf5ff77c]::ty::query::QueryEngine>::analysis
  49:     0x7f9373b2ba09 - <rustc_interface[155c1abf899b1ba8]::passes::QueryContext>::enter::<rustc_driver[220d861fe5665444]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  50:     0x7f9373b3df0b - <rustc_interface[155c1abf899b1ba8]::interface::Compiler>::enter::<rustc_driver[220d861fe5665444]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[155c1abf899b1ba8]::queries::Linker>, rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  51:     0x7f9373ad2c02 - rustc_span[316458062c9a7e81]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  52:     0x7f9373b31d1c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[316458062c9a7e81]::SessionGlobals>>::set::<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  53:     0x7f9373af0bda - std[b4d7b7572d6e9bb7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[155c1abf899b1ba8]::util::run_in_thread_pool_with_globals<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  54:     0x7f9373b36a26 - std[b4d7b7572d6e9bb7]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[b4d7b7572d6e9bb7]::thread::Builder>::spawn_unchecked_<rustc_interface[155c1abf899b1ba8]::util::run_in_thread_pool_with_globals<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>
  55:     0x7f9373ae2aa9 - <<std[b4d7b7572d6e9bb7]::thread::Builder>::spawn_unchecked_<rustc_interface[155c1abf899b1ba8]::util::run_in_thread_pool_with_globals<rustc_interface[155c1abf899b1ba8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>, rustc_driver[220d861fe5665444]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[319628db51427ac0]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f937317166e - std::sys::unix::thread::Thread::new::thread_start::h05afa8442b34c479
  57:     0x7f9372f0cb43 - <unknown>
  58:     0x7f9372f9ea00 - <unknown>
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (270cc7692 2022-11-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [lint_mod] linting module `flavors::list`
#1 [analysis] running analysis passes on this crate
error: could not compile `crossbeam-channel`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:44
