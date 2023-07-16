plain
failures:

---- [ui] src/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/issue-88577-check-fn-with-more-than-65535-arguments/auxiliary"
stdout: none
--- stderr -------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: function can not have more than 65535 arguments
   |
   |
LL |         fn _f($($t: ()),*) {} //~ ERROR function can not have more than 65535 arguments
...
...
LL | many_args!{[_]########## ######}
   |
   = note: this error originates in the macro `many_args` (in Nightly builds, run with -Z macro-backtrace for more info)


thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: TryFromIntError(())', compiler/rustc_typeck/src/check/wfcheck.rs:1500:41
stack backtrace:
   0:     0x7f6dd6c1fa0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb65e656366067b68
   1:     0x7f6dd6c881a8 - core::fmt::write::hd5bcb9ee6cb8c290
   2:     0x7f6dd6c10661 - std::io::Write::write_fmt::h3527162bdb44896d
   3:     0x7f6dd6c229fe - std::panicking::default_hook::{{closure}}::h85c962b84b65e527
   4:     0x7f6dd6c226c7 - std::panicking::default_hook::h071807a6d254f21f
   5:     0x7f6dd75b1ee4 - rustc_driver[c357436c66385061]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6dd6c231b1 - std::panicking::rust_panic_with_hook::h39be65ee3b7993a1
   7:     0x7f6dd6c22fd7 - std::panicking::begin_panic_handler::{{closure}}::hb8f6310de5993c5d
   8:     0x7f6dd6c1ff84 - std::sys_common::backtrace::__rust_end_short_backtrace::h2ef55768cf2e7826
   9:     0x7f6dd6c22ca2 - rust_begin_unwind
  10:     0x7f6dd6bd2e43 - core::panicking::panic_fmt::hc505f46822b66aba
  11:     0x7f6dd6bd2fe3 - core::result::unwrap_failed::hb8146cff8ef0acda
  12:     0x7f6dd818f3e1 - <&mut rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method::{closure#0} as core[b57c2d6458d4eee8]::ops::function::FnOnce<((usize, rustc_middle[4efba568853402db]::ty::Ty),)>>::call_once
  13:     0x7f6dd8057d46 - <smallvec[1d6ae32c044c3d66]::SmallVec<[rustc_middle[4efba568853402db]::ty::Ty; 8usize]> as core[b57c2d6458d4eee8]::iter::traits::collect::Extend<rustc_middle[4efba568853402db]::ty::Ty>>::extend::<core[b57c2d6458d4eee8]::iter::adapters::map::Map<core[b57c2d6458d4eee8]::iter::adapters::enumerate::Enumerate<core[b57c2d6458d4eee8]::iter::adapters::copied::Copied<core[b57c2d6458d4eee8]::slice::iter::Iter<rustc_middle[4efba568853402db]::ty::Ty>>>, rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method::{closure#0}>>
  14:     0x7f6dd80e4e16 - <rustc_middle[4efba568853402db]::ty::Ty as rustc_type_ir[63048a6fe64c439]::InternIteratorElement<rustc_middle[4efba568853402db]::ty::Ty, &rustc_middle[4efba568853402db]::ty::list::List<rustc_middle[4efba568853402db]::ty::Ty>>>::intern_with::<core[b57c2d6458d4eee8]::iter::adapters::map::Map<core[b57c2d6458d4eee8]::iter::adapters::enumerate::Enumerate<core[b57c2d6458d4eee8]::iter::adapters::copied::Copied<core[b57c2d6458d4eee8]::slice::iter::Iter<rustc_middle[4efba568853402db]::ty::Ty>>>, rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method::{closure#0}>, <rustc_middle[4efba568853402db]::ty::context::TyCtxt>::mk_type_list<core[b57c2d6458d4eee8]::iter::adapters::map::Map<core[b57c2d6458d4eee8]::iter::adapters::enumerate::Enumerate<core[b57c2d6458d4eee8]::iter::adapters::copied::Copied<core[b57c2d6458d4eee8]::slice::iter::Iter<rustc_middle[4efba568853402db]::ty::Ty>>>, rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method::{closure#0}>>::{closure#0}>
  15:     0x7f6dd8121542 - <rustc_middle[4efba568853402db]::ty::context::TyCtxt>::mk_type_list::<core[b57c2d6458d4eee8]::iter::adapters::map::Map<core[b57c2d6458d4eee8]::iter::adapters::enumerate::Enumerate<core[b57c2d6458d4eee8]::iter::adapters::copied::Copied<core[b57c2d6458d4eee8]::slice::iter::Iter<rustc_middle[4efba568853402db]::ty::Ty>>>, rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method::{closure#0}>>
  16:     0x7f6dd81b1401 - rustc_typeck[5ffe216723c39158]::check::wfcheck::check_fn_or_method
  17:     0x7f6dd808e295 - <rustc_infer[3bdaca9cffc71989]::infer::InferCtxtBuilder>::enter::<(), rustc_typeck[5ffe216723c39158]::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck[5ffe216723c39158]::check::wfcheck::check_item_fn::{closure#0}>::{closure#0}>
  18:     0x7f6dd819fee2 - rustc_typeck[5ffe216723c39158]::check::wfcheck::check_item_fn
  19:     0x7f6dd8196399 - rustc_typeck[5ffe216723c39158]::check::wfcheck::check_well_formed
  20:     0x7f6dd9162ff1 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::try_execute_query::<rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt, rustc_query_system[ef8e5840e3d01415]::query::caches::DefaultCache<rustc_span[d13a44178b5dad27]::def_id::LocalDefId, ()>>
  21:     0x7f6dd9239b24 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::get_query::<rustc_query_impl[e43c87356e4df1fe]::queries::check_well_formed, rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt>
  22:     0x7f6dd8db93b4 - <rustc_query_impl[e43c87356e4df1fe]::Queries as rustc_middle[4efba568853402db]::ty::query::QueryEngine>::check_well_formed
  23:     0x7f6dd80f0f55 - <core[b57c2d6458d4eee8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c885a98c441394d6]::sync::par_for_each_in<&[rustc_hir[7720361b1cf24856]::hir::ItemId], <rustc_middle[4efba568853402db]::hir::ModuleItems>::par_items<rustc_typeck[5ffe216723c39158]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[b57c2d6458d4eee8]::ops::function::FnOnce<()>>::call_once
  24:     0x7f6dd8009889 - std[b5620cb05a3981f2]::panicking::try::<(), core[b57c2d6458d4eee8]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c885a98c441394d6]::sync::par_for_each_in<&[rustc_hir[7720361b1cf24856]::hir::ItemId], <rustc_middle[4efba568853402db]::hir::ModuleItems>::par_items<rustc_typeck[5ffe216723c39158]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  25:     0x7f6dd81ffcad - rustc_data_structures[c885a98c441394d6]::sync::par_for_each_in::<&[rustc_hir[7720361b1cf24856]::hir::ItemId], <rustc_middle[4efba568853402db]::hir::ModuleItems>::par_items<rustc_typeck[5ffe216723c39158]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  26:     0x7f6dd81a2889 - rustc_typeck[5ffe216723c39158]::check::wfcheck::check_mod_type_wf
  27:     0x7f6dd9162ff1 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::try_execute_query::<rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt, rustc_query_system[ef8e5840e3d01415]::query::caches::DefaultCache<rustc_span[d13a44178b5dad27]::def_id::LocalDefId, ()>>
  28:     0x7f6dd92399f4 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::get_query::<rustc_query_impl[e43c87356e4df1fe]::queries::check_mod_type_wf, rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt>
  29:     0x7f6dd8d9a764 - <rustc_query_impl[e43c87356e4df1fe]::Queries as rustc_middle[4efba568853402db]::ty::query::QueryEngine>::check_mod_type_wf
  30:     0x7f6dd811ad4a - <rustc_middle[4efba568853402db]::hir::map::Map>::for_each_module::<rustc_typeck[5ffe216723c39158]::check_crate::{closure#5}::{closure#0}::{closure#0}>
  31:     0x7f6dd802b45b - <rustc_session[74ff2996db940cd9]::session::Session>::track_errors::<rustc_typeck[5ffe216723c39158]::check_crate::{closure#5}, ()>
  32:     0x7f6dd8228890 - rustc_typeck[5ffe216723c39158]::check_crate
  33:     0x7f6dd76eff31 - rustc_interface[3dec13644a6b9702]::passes::analysis
  34:     0x7f6dd919b4a5 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::try_execute_query::<rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt, rustc_query_system[ef8e5840e3d01415]::query::caches::DefaultCache<(), core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>>
  35:     0x7f6dd9277922 - rustc_query_system[ef8e5840e3d01415]::query::plumbing::get_query::<rustc_query_impl[e43c87356e4df1fe]::queries::analysis, rustc_query_impl[e43c87356e4df1fe]::plumbing::QueryCtxt>
  36:     0x7f6dd8d7f5ae - <rustc_query_impl[e43c87356e4df1fe]::Queries as rustc_middle[4efba568853402db]::ty::query::QueryEngine>::analysis
  37:     0x7f6dd7610f1b - <rustc_interface[3dec13644a6b9702]::passes::QueryContext>::enter::<rustc_driver[c357436c66385061]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>
  38:     0x7f6dd75b9404 - <rustc_interface[3dec13644a6b9702]::interface::Compiler>::enter::<rustc_driver[c357436c66385061]::run_compiler::{closure#1}::{closure#2}, core[b57c2d6458d4eee8]::result::Result<core[b57c2d6458d4eee8]::option::Option<rustc_interface[3dec13644a6b9702]::queries::Linker>, rustc_errors[b10182686a19108d]::ErrorGuaranteed>>
  39:     0x7f6dd759e471 - rustc_span[d13a44178b5dad27]::with_source_map::<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_interface[3dec13644a6b9702]::interface::create_compiler_and_run<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f6dd75ba7d1 - rustc_interface[3dec13644a6b9702]::interface::create_compiler_and_run::<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>
  41:     0x7f6dd7595f92 - <scoped_tls[3d43d7106533556a]::ScopedKey<rustc_span[d13a44178b5dad27]::SessionGlobals>>::set::<rustc_interface[3dec13644a6b9702]::interface::run_compiler<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>
  42:     0x7f6dd75a21e9 - std[b5620cb05a3981f2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3dec13644a6b9702]::util::run_in_thread_pool_with_globals<rustc_interface[3dec13644a6b9702]::interface::run_compiler<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>
  43:     0x7f6dd761c80e - std[b5620cb05a3981f2]::panicking::try::<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, core[b57c2d6458d4eee8]::panic::unwind_safe::AssertUnwindSafe<<std[b5620cb05a3981f2]::thread::Builder>::spawn_unchecked_<rustc_interface[3dec13644a6b9702]::util::run_in_thread_pool_with_globals<rustc_interface[3dec13644a6b9702]::interface::run_compiler<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  44:     0x7f6dd7617ff0 - <<std[b5620cb05a3981f2]::thread::Builder>::spawn_unchecked_<rustc_interface[3dec13644a6b9702]::util::run_in_thread_pool_with_globals<rustc_interface[3dec13644a6b9702]::interface::run_compiler<core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>, rustc_driver[c357436c66385061]::run_compiler::{closure#1}>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>::{closure#0}, core[b57c2d6458d4eee8]::result::Result<(), rustc_errors[b10182686a19108d]::ErrorGuaranteed>>::{closure#1} as core[b57c2d6458d4eee8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f6dd6c2fd45 - std::sys::unix::thread::Thread::new::thread_start::h5b1ac129307cac95
  46:     0x7f6dd69ccb43 - <unknown>
  47:     0x7f6dd6a5ea00 - <unknown>
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (3169a2b7f 2022-08-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `_f` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


