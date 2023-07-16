plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'diverge_from called on block with terminator that cannot unwind.', compiler/rustc_mir_build/src/build/scope.rs:1030:9
stack backtrace:
   0:     0x7fcf332527d2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a88a4dc96b6b8f8
   1:     0x7fcf332b7468 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fcf33242ce1 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   3:     0x7fcf33255dd6 - std::panicking::default_hook::{{closure}}::hbe1bb29927e8c85b
   4:     0x7fcf332559d5 - std::panicking::default_hook::h30c665989e20cb24
   5:     0x7fcf33d64671 - rustc_driver[2b15ae7948b6e616]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcf33256741 - std::panicking::rust_panic_with_hook::hab898bc6064aa4ee
   7:     0x7fcf33256549 - std::panicking::begin_panic_handler::{{closure}}::h688434a74a3a0f4f
   8:     0x7fcf33252d74 - std::sys_common::backtrace::__rust_end_short_backtrace::hf938ae7adc39f6fa
   9:     0x7fcf33256279 - rust_begin_unwind
  10:     0x7fcf33208dc3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7fcf34c07139 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::as_rvalue
  12:     0x7fcf34c0a7d4 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest
  13:     0x7fcf34c0a03a - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::as_temp_inner
  14:     0x7fcf34c02ecc - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_as_place
  15:     0x7fcf34c042f3 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_as_place
  16:     0x7fcf34bfe24b - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::ast_block_stmts
  17:     0x7fcf34c20629 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::in_scope::<<rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::ast_block::{closure#2}::{closure#0}, ()>
  18:     0x7fcf34c0c9ca - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest
  19:     0x7fcf34c0a03a - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::as_temp_inner
  20:     0x7fcf34c02ecc - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_as_place
  21:     0x7fcf34c03804 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_as_place
  22:     0x7fcf34c02d0c - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::as_place
  23:     0x7fcf34c0ca04 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest
  24:     0x7fcf34c21714 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::in_scope::<<rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest::{closure#0}::{closure#0}, ()>
  25:     0x7fcf34c0baa9 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest
  26:     0x7fcf34c21714 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::in_scope::<<rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest::{closure#0}::{closure#0}, ()>
  27:     0x7fcf34c0baa9 - <rustc_mir_build[1e9d80c4f7d94376]::build::Builder>::expr_into_dest
  28:     0x7fcf34bfb04e - rustc_mir_build[1e9d80c4f7d94376]::build::construct_fn::<core[10878fb91fc84a80]::iter::adapters::chain::Chain<alloc[4b492a408420e30b]::vec::into_iter::IntoIter<rustc_mir_build[1e9d80c4f7d94376]::build::ArgInfo>, core[10878fb91fc84a80]::iter::adapters::map::Map<core[10878fb91fc84a80]::iter::adapters::enumerate::Enumerate<core[10878fb91fc84a80]::slice::iter::Iter<rustc_hir[83eb891e04ea7418]::hir::Param>>, rustc_mir_build[1e9d80c4f7d94376]::build::mir_build::{closure#1}::{closure#1}>>>
  29:     0x7fcf34c9faaa - <rustc_infer[e9909202d8948eff]::infer::InferCtxtBuilder>::enter::<rustc_middle[87143a0b6987fc74]::mir::Body, rustc_mir_build[1e9d80c4f7d94376]::build::mir_build::{closure#1}>
  30:     0x7fcf34bf930f - rustc_mir_build[1e9d80c4f7d94376]::build::mir_built
  31:     0x7fcf353b590c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_middle[87143a0b6987fc74]::ty::WithOptConstParam<rustc_span[e033c2886c1ea87]::def_id::LocalDefId>, &rustc_data_structures[e873bb7798a1662c]::steal::Steal<rustc_middle[87143a0b6987fc74]::mir::Body>>>
  32:     0x7fcf354f4b6e - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_built, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  33:     0x7fcf3580fc86 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::mir_built
  34:     0x7fcf3442848c - rustc_mir_transform[7b037954339a5e74]::check_unsafety::unsafety_check_result
  35:     0x7fcf34425119 - <rustc_mir_transform[7b037954339a5e74]::check_unsafety::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt, rustc_span[e033c2886c1ea87]::def_id::LocalDefId)>>::call_once
  36:     0x7fcf353c6bf4 - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_span[e033c2886c1ea87]::def_id::LocalDefId, &rustc_middle[87143a0b6987fc74]::mir::query::UnsafetyCheckResult>>
  37:     0x7fcf354b6f40 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::unsafety_check_result, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  38:     0x7fcf343ef783 - rustc_mir_transform[7b037954339a5e74]::mir_const
  39:     0x7fcf353b590c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_middle[87143a0b6987fc74]::ty::WithOptConstParam<rustc_span[e033c2886c1ea87]::def_id::LocalDefId>, &rustc_data_structures[e873bb7798a1662c]::steal::Steal<rustc_middle[87143a0b6987fc74]::mir::Body>>>
  40:     0x7fcf354f4f2b - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_const, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  41:     0x7fcf3580fcb6 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::mir_const
  42:     0x7fcf343f03fc - rustc_mir_transform[7b037954339a5e74]::mir_promoted
  43:     0x7fcf354857fc - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_promoted, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  44:     0x7fcf3580fe06 - <rustc_query_impl[62d43cdc40496630]::Queries as rustc_middle[87143a0b6987fc74]::ty::query::QueryEngine>::mir_promoted
  45:     0x7fcf34f3343a - rustc_borrowck[56119217130bda78]::mir_borrowck
  46:     0x7fcf34f00399 - <rustc_borrowck[56119217130bda78]::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[87143a0b6987fc74]::ty::context::TyCtxt, rustc_span[e033c2886c1ea87]::def_id::LocalDefId)>>::call_once
  47:     0x7fcf353c5e84 - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<rustc_span[e033c2886c1ea87]::def_id::LocalDefId, &rustc_middle[87143a0b6987fc74]::mir::query::BorrowCheckResult>>
  48:     0x7fcf35484a80 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::mir_borrowck, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  49:     0x7fcf33ecb9dd - <rustc_middle[87143a0b6987fc74]::hir::map::Map>::par_body_owners::<rustc_interface[1e8edbf255833c1]::passes::analysis::{closure#2}::{closure#0}>
  50:     0x7fcf33e6dd20 - <rustc_session[490bd8b11d3080dc]::session::Session>::time::<(), rustc_interface[1e8edbf255833c1]::passes::analysis::{closure#2}>
  51:     0x7fcf33e5e1cb - rustc_interface[1e8edbf255833c1]::passes::analysis
  52:     0x7fcf353ffb2c - rustc_query_system[6655655853c83e28]::query::plumbing::try_execute_query::<rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt, rustc_query_system[6655655853c83e28]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>>
  53:     0x7fcf354f1079 - rustc_query_system[6655655853c83e28]::query::plumbing::get_query::<rustc_query_impl[62d43cdc40496630]::queries::analysis, rustc_query_impl[62d43cdc40496630]::plumbing::QueryCtxt>
  54:     0x7fcf33d55f1a - <rustc_interface[1e8edbf255833c1]::passes::QueryContext>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  55:     0x7fcf33cf8e80 - <rustc_interface[1e8edbf255833c1]::interface::Compiler>::enter::<rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[1e8edbf255833c1]::queries::Linker>, rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  56:     0x7fcf33cdc486 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7fcf33d0be57 - rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>
  58:     0x7fcf33d1058f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  59:     0x7fcf33d57ed9 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  60:     0x7fcf33d126c1 - std[e4dc215d72d9f73d]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  61:     0x7fcf33d52f52 - <<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fcf33262df3 - std::sys::unix::thread::Thread::new::thread_start::h2f9ecc8966c8b525
  63:     0x7fcf2d7b3609 - start_thread
  64:     0x7fcf330c6163 - clone
  65:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (3a6d5285c 2022-04-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_built] building MIR for `sync::atomic::<impl at library/core/src/sync/atomic.rs:941:1: 1410:2>::from_mut`
#1 [unsafety_check_result] unsafety-checking `sync::atomic::<impl at library/core/src/sync/atomic.rs:941:1: 1410:2>::from_mut`
#2 [mir_const] processing MIR for `sync::atomic::<impl at library/core/src/sync/atomic.rs:941:1: 1410:2>::from_mut`
#3 [mir_promoted] processing `sync::atomic::<impl at library/core/src/sync/atomic.rs:941:1: 1410:2>::from_mut`
#4 [mir_borrowck] borrow-checking `sync::atomic::<impl at library/core/src/sync/atomic.rs:941:1: 1410:2>::from_mut`
#5 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:04:11
