plain
failures:

---- [ui] src/test/ui/issues/issue-50618.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50618.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50618" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50618/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0560]: struct `Point` has no field named `nonexistent`
   |
LL |             nonexistent: 0,
   |             ^^^^^^^^^^^ `Point` does not have this field
   |
   |
   = note: available fields are: `x`, `y`

thread 'rustc' panicked at 'no index for a field', compiler/rustc_middle/src/ty/mod.rs:2232:9
stack backtrace:
   0:     0x7fde539570a5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8f4cb549da4837ea
   1:     0x7fde539c6ac8 - core::fmt::write::h92776e9ed8b4ea26
   2:     0x7fde53948e51 - std::io::Write::write_fmt::hcc028295c0b5e344
   3:     0x7fde53956eb1 - std::sys_common::backtrace::print::h41e42754b357339e
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   4:     0x7fde5395a214 - std::panicking::default_hook::{{closure}}::hfa50b73faaca5838
   5:     0x7fde53959eda - std::panicking::default_hook::hb59833b1578d1a74
   6:     0x7fde54399cb4 - rustc_driver[65dc365e0bd74bc2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fde5395a984 - std::panicking::rust_panic_with_hook::h4ba6b41fa38158ce
   8:     0x7fde5395a6e9 - std::panicking::begin_panic_handler::{{closure}}::hd3ab979dc9f98bf1
   9:     0x7fde539575c4 - std::sys_common::backtrace::__rust_end_short_backtrace::h5e8bbb00c3b78650
  10:     0x7fde5395a392 - rust_begin_unwind
  11:     0x7fde5390aff3 - core::panicking::panic_fmt::h194777d705dad74a
  12:     0x7fde539c3481 - core::panicking::panic_display::hc4aa186cb14f10b4
  13:     0x7fde539c342b - core::panicking::panic_str::h76af6c086aaa8cd8
  14:     0x7fde5390afb6 - core::option::expect_failed::h451ddbf5fa366830
  15:     0x7fde571e5303 - <rustc_middle[7d3e9cf1620ecd01]::ty::context::TyCtxt>::field_index
  16:     0x7fde54a7ffe7 - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::walk_expr
  17:     0x7fde54a7dc2c - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::consume_expr
  18:     0x7fde54a810aa - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::walk_block
  19:     0x7fde54a7ec91 - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::walk_expr
  20:     0x7fde54a7dc2c - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::consume_expr
  21:     0x7fde54a87644 - <rustc_hir_typeck[2de54f253a82f082]::expr_use_visitor::ExprUseVisitor>::consume_body
  22:     0x7fde5490337b - <rustc_hir_typeck[2de54f253a82f082]::fn_ctxt::FnCtxt>::analyze_closure
  23:     0x7fde54a84ada - <rustc_hir_typeck[2de54f253a82f082]::upvar::InferBorrowKindVisitor as rustc_hir[fe6b9d6ae7a0ce6d]::intravisit::Visitor>::visit_expr
  24:     0x7fde549626d3 - rustc_hir[fe6b9d6ae7a0ce6d]::intravisit::walk_local::<rustc_hir_typeck[2de54f253a82f082]::upvar::InferBorrowKindVisitor>
  25:     0x7fde5496ce74 - rustc_hir[fe6b9d6ae7a0ce6d]::intravisit::walk_expr::<rustc_hir_typeck[2de54f253a82f082]::upvar::InferBorrowKindVisitor>
  26:     0x7fde548d4157 - <rustc_hir_typeck[2de54f253a82f082]::fn_ctxt::FnCtxt>::closure_analyze
  27:     0x7fde549b56ce - rustc_hir_typeck[2de54f253a82f082]::typeck
  28:     0x7fde561b63b2 - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::try_execute_query::<rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt, rustc_query_system[1f862fc4e866aaf5]::query::caches::VecCache<rustc_span[ba710e9d572a5d50]::def_id::LocalDefId, &rustc_middle[7d3e9cf1620ecd01]::ty::context::TypeckResults>>
  29:     0x7fde562d1546 - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::get_query::<rustc_query_impl[c75774240ce7bcdd]::queries::typeck, rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt>
  30:     0x7fde55dcb280 - <rustc_query_impl[c75774240ce7bcdd]::Queries as rustc_middle[7d3e9cf1620ecd01]::ty::query::QueryEngine>::typeck
  31:     0x7fde549a49a1 - <core[20eb9dc6125dfe84]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[bc0439399c6f98a]::sync::par_for_each_in<&[rustc_span[ba710e9d572a5d50]::def_id::LocalDefId], <rustc_middle[7d3e9cf1620ecd01]::hir::map::Map>::par_body_owners<rustc_hir_typeck[2de54f253a82f082]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[20eb9dc6125dfe84]::ops::function::FnOnce<()>>::call_once
  32:     0x7fde54a720e6 - std[48761c830f407122]::panicking::try::<(), core[20eb9dc6125dfe84]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[bc0439399c6f98a]::sync::par_for_each_in<&[rustc_span[ba710e9d572a5d50]::def_id::LocalDefId], <rustc_middle[7d3e9cf1620ecd01]::hir::map::Map>::par_body_owners<rustc_hir_typeck[2de54f253a82f082]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7fde5494b1ed - rustc_data_structures[bc0439399c6f98a]::sync::par_for_each_in::<&[rustc_span[ba710e9d572a5d50]::def_id::LocalDefId], <rustc_middle[7d3e9cf1620ecd01]::hir::map::Map>::par_body_owners<rustc_hir_typeck[2de54f253a82f082]::typeck_item_bodies::{closure#0}>::{closure#0}>
  34:     0x7fde549b27cd - rustc_hir_typeck[2de54f253a82f082]::typeck_item_bodies
  35:     0x7fde5619246f - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::try_execute_query::<rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt, rustc_query_system[1f862fc4e866aaf5]::query::caches::DefaultCache<(), ()>>
  36:     0x7fde56288a14 - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::get_query::<rustc_query_impl[c75774240ce7bcdd]::queries::typeck_item_bodies, rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt>
  37:     0x7fde55dcab1a - <rustc_query_impl[c75774240ce7bcdd]::Queries as rustc_middle[7d3e9cf1620ecd01]::ty::query::QueryEngine>::typeck_item_bodies
  38:     0x7fde54b8ca6b - <rustc_session[e23dc16ea33337bc]::session::Session>::time::<(), rustc_hir_analysis[3154becdf3d00fec]::check_crate::{closure#7}>
  39:     0x7fde54d1b4d3 - rustc_hir_analysis[3154becdf3d00fec]::check_crate
  40:     0x7fde544b41f1 - rustc_interface[5d839891f4c49d08]::passes::analysis
  41:     0x7fde56181940 - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::try_execute_query::<rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt, rustc_query_system[1f862fc4e866aaf5]::query::caches::DefaultCache<(), core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>>
  42:     0x7fde562d1924 - rustc_query_system[1f862fc4e866aaf5]::query::plumbing::get_query::<rustc_query_impl[c75774240ce7bcdd]::queries::analysis, rustc_query_impl[c75774240ce7bcdd]::plumbing::QueryCtxt>
  43:     0x7fde55da21aa - <rustc_query_impl[c75774240ce7bcdd]::Queries as rustc_middle[7d3e9cf1620ecd01]::ty::query::QueryEngine>::analysis
  44:     0x7fde543f527c - <rustc_interface[5d839891f4c49d08]::passes::QueryContext>::enter::<rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>
  45:     0x7fde543ff5bf - <rustc_interface[5d839891f4c49d08]::interface::Compiler>::enter::<rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}::{closure#2}, core[20eb9dc6125dfe84]::result::Result<core[20eb9dc6125dfe84]::option::Option<rustc_interface[5d839891f4c49d08]::queries::Linker>, rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>
  46:     0x7fde5439b416 - rustc_span[ba710e9d572a5d50]::with_source_map::<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_interface[5d839891f4c49d08]::interface::run_compiler<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7fde5440034a - <scoped_tls[6be8471199862fb2]::ScopedKey<rustc_span[ba710e9d572a5d50]::SessionGlobals>>::set::<rustc_interface[5d839891f4c49d08]::interface::run_compiler<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}>::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>
  48:     0x7fde543b97bf - std[48761c830f407122]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5d839891f4c49d08]::util::run_in_thread_pool_with_globals<rustc_interface[5d839891f4c49d08]::interface::run_compiler<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}>::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>
  49:     0x7fde54419746 - std[48761c830f407122]::panicking::try::<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, core[20eb9dc6125dfe84]::panic::unwind_safe::AssertUnwindSafe<<std[48761c830f407122]::thread::Builder>::spawn_unchecked_<rustc_interface[5d839891f4c49d08]::util::run_in_thread_pool_with_globals<rustc_interface[5d839891f4c49d08]::interface::run_compiler<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}>::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  50:     0x7fde543abc35 - <<std[48761c830f407122]::thread::Builder>::spawn_unchecked_<rustc_interface[5d839891f4c49d08]::util::run_in_thread_pool_with_globals<rustc_interface[5d839891f4c49d08]::interface::run_compiler<core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>, rustc_driver[65dc365e0bd74bc2]::run_compiler::{closure#1}>::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20eb9dc6125dfe84]::result::Result<(), rustc_errors[568abf16d5e73a5a]::ErrorGuaranteed>>::{closure#1} as core[20eb9dc6125dfe84]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  51:     0x7fde53966f9e - std::sys::unix::thread::Thread::new::thread_start::hbbc6f755fb26dba5
  52:     0x7fde536fcb43 - <unknown>
  53:     0x7fde5378ea00 - <unknown>
  54:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (7e7ef5b5a 2022-12-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0560`.
------------------------------------------
