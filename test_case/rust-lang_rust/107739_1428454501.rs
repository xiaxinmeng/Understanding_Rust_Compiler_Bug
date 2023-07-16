plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
---- [ui] tests/ui/traits/new-solver/provisional-result-done.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/provisional-result-done.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/provisional-result-done" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/provisional-result-done/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: search_graph.is_empty()', compiler/rustc_trait_selection/src/solve/mod.rs:178:9
   0:     0x7f9a724c8ec5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he5f05269633e7673
   0:     0x7f9a724c8ec5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he5f05269633e7673
   1:     0x7f9a72536678 - core::fmt::write::h98390bef7954ee80
   2:     0x7f9a724bb001 - std::io::Write::write_fmt::h814c0484086c7e87
   3:     0x7f9a724c8cd1 - std::sys_common::backtrace::print::hc492cecc6629e8e9
   4:     0x7f9a724cbf34 - std::panicking::default_hook::{{closure}}::h28c7b90fdfd3a2ab
   5:     0x7f9a724cbc1a - std::panicking::default_hook::h38406c2b54b041ce
   6:     0x7f9a72f63d42 - rustc_driver_impl[2babf7facf6d4c63]::DEFAULT_HOOK::{closure#0}::{closure#0}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   7:     0x7f9a724cc671 - std::panicking::rust_panic_with_hook::hcbacc183eaf79ba1
   8:     0x7f9a724cc3aa - std::panicking::begin_panic_handler::{{closure}}::h641e02e9a7de1537
   9:     0x7f9a724c93b4 - std::sys_common::backtrace::__rust_end_short_backtrace::had6a85b3c1758c8d
  10:     0x7f9a724cc0b2 - rust_begin_unwind
  11:     0x7f9a7247df93 - core::panicking::panic_fmt::h1b74fc78a0506ce9
  12:     0x7f9a7247e02d - core::panicking::panic::h63a41645673f64d6
  13:     0x7f9a75aa569f - <rustc_infer[7ea9774d1970527c]::infer::InferCtxt as rustc_trait_selection[5b4ebe70bd541ff4]::solve::InferCtxtEvalExt>::evaluate_root_goal
  14:     0x7f9a758d977f - <rustc_trait_selection[5b4ebe70bd541ff4]::solve::fulfill::FulfillmentCtxt as rustc_infer[7ea9774d1970527c]::traits::engine::TraitEngine>::select_where_possible
  15:     0x7f9a734b161b - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  16:     0x7f9a734b8809 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::structurally_resolved_type
  17:     0x7f9a73486fdb - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_call
  18:     0x7f9a7350ae9f - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_kind
  19:     0x7f9a734a196e - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  20:     0x7f9a73509cc2 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  21:     0x7f9a734c4967 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_stmt
  22:     0x7f9a734c4ed7 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_block_with_expected
  23:     0x7f9a7350b1d1 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_kind
  24:     0x7f9a734a196e - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  25:     0x7f9a73509cc2 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  26:     0x7f9a734a38d9 - <rustc_hir_typeck[a625bdcd453fe27c]::fn_ctxt::FnCtxt>::check_return_expr
  27:     0x7f9a7368c4ae - rustc_hir_typeck[a625bdcd453fe27c]::check::check_fn
  28:     0x7f9a73687c86 - rustc_hir_typeck[a625bdcd453fe27c]::typeck
  29:     0x7f9a74da90a2 - rustc_query_system[c6895dd135e2ef32]::query::plumbing::try_execute_query::<rustc_query_impl[5cd90b55140b0966]::queries::typeck, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt>
  30:     0x7f9a74e5e980 - rustc_query_system[c6895dd135e2ef32]::query::plumbing::get_query::<rustc_query_impl[5cd90b55140b0966]::queries::typeck, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt, rustc_middle[4ed786de52b63562]::dep_graph::dep_node::DepKind>
  31:     0x7f9a74a989d0 - <rustc_query_impl[5cd90b55140b0966]::Queries as rustc_middle[4ed786de52b63562]::ty::query::QueryEngine>::typeck
  32:     0x7f9a7356d6cc - std[fcc41a6e6445c782]::panicking::try::<(), core[6d65229bcb1ce383]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[1fcb6833aefb830f]::sync::par_for_each_in<&[rustc_span[67a3bbb0d3c53c4c]::def_id::LocalDefId], <rustc_middle[4ed786de52b63562]::hir::map::Map>::par_body_owners<rustc_hir_typeck[a625bdcd453fe27c]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7f9a7364233d - rustc_data_structures[1fcb6833aefb830f]::sync::par_for_each_in::<&[rustc_span[67a3bbb0d3c53c4c]::def_id::LocalDefId], <rustc_middle[4ed786de52b63562]::hir::map::Map>::par_body_owners<rustc_hir_typeck[a625bdcd453fe27c]::typeck_item_bodies::{closure#0}>::{closure#0}>
  34:     0x7f9a73684afa - rustc_hir_typeck[a625bdcd453fe27c]::typeck_item_bodies
  35:     0x7f9a74d61725 - rustc_query_system[c6895dd135e2ef32]::query::plumbing::try_execute_query::<rustc_query_impl[5cd90b55140b0966]::queries::typeck_item_bodies, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt>
  36:     0x7f9a74e2fa6f - rustc_query_system[c6895dd135e2ef32]::query::plumbing::get_query::<rustc_query_impl[5cd90b55140b0966]::queries::typeck_item_bodies, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt, rustc_middle[4ed786de52b63562]::dep_graph::dep_node::DepKind>
  37:     0x7f9a74a9826a - <rustc_query_impl[5cd90b55140b0966]::Queries as rustc_middle[4ed786de52b63562]::ty::query::QueryEngine>::typeck_item_bodies
  38:     0x7f9a73758464 - <rustc_session[a4f914c10c44f95]::session::Session>::time::<(), rustc_hir_analysis[d3199365215437de]::check_crate::{closure#7}>
  39:     0x7f9a73846be6 - rustc_hir_analysis[d3199365215437de]::check_crate
  40:     0x7f9a730ca873 - rustc_interface[8301120d6e04c263]::passes::analysis
  41:     0x7f9a74dab23c - rustc_query_system[c6895dd135e2ef32]::query::plumbing::try_execute_query::<rustc_query_impl[5cd90b55140b0966]::queries::analysis, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt>
  42:     0x7f9a74e5ec11 - rustc_query_system[c6895dd135e2ef32]::query::plumbing::get_query::<rustc_query_impl[5cd90b55140b0966]::queries::analysis, rustc_query_impl[5cd90b55140b0966]::plumbing::QueryCtxt, rustc_middle[4ed786de52b63562]::dep_graph::dep_node::DepKind>
  43:     0x7f9a74a6ea1a - <rustc_query_impl[5cd90b55140b0966]::Queries as rustc_middle[4ed786de52b63562]::ty::query::QueryEngine>::analysis
  44:     0x7f9a72ff0889 - <rustc_middle[4ed786de52b63562]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>
  45:     0x7f9a72fd0355 - <rustc_interface[8301120d6e04c263]::interface::Compiler>::enter::<rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}::{closure#2}, core[6d65229bcb1ce383]::result::Result<core[6d65229bcb1ce383]::option::Option<rustc_interface[8301120d6e04c263]::queries::Linker>, rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>
  46:     0x7f9a72fef367 - rustc_span[67a3bbb0d3c53c4c]::with_source_map::<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_interface[8301120d6e04c263]::interface::run_compiler<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7f9a72fd0df6 - <scoped_tls[388f67f448ebb926]::ScopedKey<rustc_span[67a3bbb0d3c53c4c]::SessionGlobals>>::set::<rustc_interface[8301120d6e04c263]::interface::run_compiler<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}>::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>
  48:     0x7f9a72f80779 - std[fcc41a6e6445c782]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8301120d6e04c263]::util::run_in_thread_pool_with_globals<rustc_interface[8301120d6e04c263]::interface::run_compiler<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}>::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>
  49:     0x7f9a72fc72b8 - std[fcc41a6e6445c782]::panicking::try::<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, core[6d65229bcb1ce383]::panic::unwind_safe::AssertUnwindSafe<<std[fcc41a6e6445c782]::thread::Builder>::spawn_unchecked_<rustc_interface[8301120d6e04c263]::util::run_in_thread_pool_with_globals<rustc_interface[8301120d6e04c263]::interface::run_compiler<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}>::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7f9a72f6f407 - <<std[fcc41a6e6445c782]::thread::Builder>::spawn_unchecked_<rustc_interface[8301120d6e04c263]::util::run_in_thread_pool_with_globals<rustc_interface[8301120d6e04c263]::interface::run_compiler<core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>, rustc_driver_impl[2babf7facf6d4c63]::run_compiler::{closure#1}>::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6d65229bcb1ce383]::result::Result<(), rustc_errors[74d0cc563583483b]::ErrorGuaranteed>>::{closure#1} as core[6d65229bcb1ce383]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7f9a724d948e - std::sys::unix::thread::Thread::new::thread_start::he00c9052a8f03621
  52:     0x7f9a7226eb43 - <unknown>
  53:     0x7f9a72300a00 - <unknown>
  54:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (371fb6bf0 2023-02-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=next
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



