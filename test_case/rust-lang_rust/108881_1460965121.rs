plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e83cf548fe2db927e33b135adbd2e291ec48d14d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 5456/14577
........................................................................................ 5544/14577
........................................................................................ 5632/14577
........................................................................................ 5720/14577
.......................i.........F..iiii.F.F............................................ 5808/14577
........................................................................................ 5984/14577
........................................................................................ 6072/14577
........................................................................................ 6072/14577
......FF.FFFF..F........................................................................ 6160/14577
........................................................................................ 6336/14577
........................................................................................ 6424/14577
........................................................................................ 6512/14577
...........................FFF.......................................................... 6600/14577
---
........................................................................................ 9416/14577
..................................................................................i..... 9504/14577
...................................i.................................................... 9592/14577
...................i.................................................................... 9680/14577
................F......................................F................................ 9768/14577
........................................................................................ 9944/14577
....................................................................i................... 10032/14577
........................................................................................ 10120/14577
........................................................................................ 10208/14577
........................................................................................ 10208/14577
........................................................................................ 10296/14577
........................................................................................ 10384/14577
........................................................................................ 10472/14577
........................................................................................ 10560/14577
........................................................................................ 10648/14577
...................................ii...............i.....iii........................... 10736/14577
........................................................................................ 10824/14577
...F.F.....................................................................F............ 10912/14577
........................................................................................ 11088/14577
........................................................................................ 11176/14577
........................................................................................ 11264/14577
........................................................................................ 11352/14577
........................................................................................ 11352/14577
........................................................................................ 11440/14577
................................iiiii...i....i..i....................................... 11528/14577
........................................................................................ 11616/14577
..........................................i............................................. 11704/14577
....................................................ii..iiiiii.i..iiiiiiiiiii.i......... 11792/14577
........................................................................................ 11880/14577
........................................................................................ 11968/14577
..............................................F......F.................................. 12056/14577
..............................F.....F................................................... 12144/14577
........................................................................................ 12320/14577
........................................................................................ 12320/14577
...............................F..................................................F.F.F. 12408/14577
........................................................................................ 12584/14577
........................................................................................ 12672/14577
..........................................FF............................................ 12760/14577
............................i...........i........i.....i......................i......... 12848/14577
---
........................................................................................ 13552/14577
...............................................................F....................i... 13640/14577
........................................................................................ 13728/14577
........................................................................................ 13816/14577
................F.............F......................................................... 13904/14577
.............................................F.......................................... 13992/14577
........................................................................................ 14168/14577
..F..................................................................................... 14256/14577
........................................................................................ 14344/14577
........................................................................................ 14432/14577
........................................................................................ 14432/14577
...............................iii..................................F................... 14520/14577
.........................................................
failures:

---- [ui] tests/ui/consts/issue-103790.rs stdout ----
diff of stderr:

35 LL | struct S<const S: (), const S: S = { S }>;
37 
37 
- error[E0391]: cycle detected when computing type of `S`
-   --> $DIR/issue-103790.rs:4:1
-    |
- LL | struct S<const S: (), const S: S = { S }>;
-    |
-    |
- note: ...which requires computing type of `S::S`...
-   --> $DIR/issue-103790.rs:4:32
-    |
- LL | struct S<const S: (), const S: S = { S }>;
-    |                                ^
-    = note: ...which again requires computing type of `S`, completing the cycle
- note: cycle used when collecting item types in top-level module
-   --> $DIR/issue-103790.rs:1:1
-    |
- LL | / #![feature(generic_const_exprs)]
- LL | | #![allow(incomplete_features)]
- LL | |
- LL | | struct S<const S: (), const S: S = { S }>;
- LL | |
- LL | | fn main() {}
-    | |____________^
- 
---
To only update this specific test, also pass `--test-args consts/issue-103790.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/issue-103790.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-103790" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-103790/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0403]: the name `S` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/consts/issue-103790.rs:4:29
   |
LL | struct S<const S: (), const S: S = { S }>;
   |                -            ^ already used
   |                first use of `S`

error[E0107]: missing generics for struct `S`
  --> fake-test-src-base/consts/issue-103790.rs:4:32
  --> fake-test-src-base/consts/issue-103790.rs:4:32
   |
LL | struct S<const S: (), const S: S = { S }>;
   |                                ^ expected at least 1 generic argument
   |
note: struct defined here, with at least 1 generic parameter: `S`
  --> fake-test-src-base/consts/issue-103790.rs:4:8
   |
LL | struct S<const S: (), const S: S = { S }>;
   |        ^ -----------
help: add missing generic argument
   |
LL | struct S<const S: (), const S: S<S> = { S }>;


error[E0391]: cycle detected when computing type of `S::S`
  --> fake-test-src-base/consts/issue-103790.rs:4:32
   |
LL | struct S<const S: (), const S: S = { S }>;
   |
   |
   = note: ...which immediately requires computing type of `S::S` again
note: cycle used when computing type of `S`
  --> fake-test-src-base/consts/issue-103790.rs:4:1
   |
LL | struct S<const S: (), const S: S = { S }>;

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0391, E0403.
Some errors have detailed explanations: E0107, E0391, E0403.
For more information about an error, try `rustc --explain E0107`.
------------------------------------------


---- [ui] tests/ui/enum-discriminant/issue-72554.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/enum-discriminant/issue-72554.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/issue-72554" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/issue-72554/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7ff4e77c6a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7ff4e7833df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7ff4e77bb4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7ff4e77c6881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7ff4e77c9a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7ff4e77c976a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7ff4e82b3fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff4e77ca1a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7ff4e77c9eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7ff4e77c6f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7ff4e77c9bf7 - rust_begin_unwind
  11:     0x7ff4e77802c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7ff4e9d31466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7ff4e81581cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7ff4ea02c399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7ff4e9e438b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7ff4e87313fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7ff4ea0d269f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7ff4e9e441f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7ff4e87311d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7ff4e8730e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7ff4ea02c011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7ff4e9e438b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7ff4e8730ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7ff4ea02c011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7ff4e9e438b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7ff4e8bf2644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7ff4e8be9bea - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7ff4ea035316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7ff4e9ea7210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7ff4e8b28875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7ff4e8ba75b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7ff4e8c7038d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7ff4e8bf7e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7ff4ea033f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7ff4e9e70030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7ff4e8b289c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7ff4e8ba75d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7ff4e8c704fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7ff4e8a78759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7ff4e8a73de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7ff4e8376282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7ff4ea13cbdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7ff4e9e3c8a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7ff4e82befac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7ff4e82fd273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7ff4e829fdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7ff4e82c9c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7ff4e8311966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7ff4e82c2f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7ff4e77d647e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7ff4e7570b43 - <unknown>
  52:     0x7ff4e7602a00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `ElemDerived` is representable
#1 [representability] checking if `ElemDerived::A::0` is representable
#2 [representability] checking if `ElemDerived` is representable
#3 [check_well_formed] checking that `ElemDerived` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when checking if `ElemDerived` is representable
  --> fake-test-src-base/enum-discriminant/issue-72554.rs:4:1
   |
LL | pub enum ElemDerived {
   |
   |
note: ...which requires checking if `ElemDerived::A::0` is representable...
  --> fake-test-src-base/enum-discriminant/issue-72554.rs:6:7
   |
LL |     A(ElemDerived)
   |       ^^^^^^^^^^^
   = note: ...which requires checking if `ElemDerived` is representable...
   = note: ...which again requires checking if `ElemDerived` is representable, completing the cycle
note: cycle used when checking that `ElemDerived` is well-formed
  --> fake-test-src-base/enum-discriminant/issue-72554.rs:4:1
   |
LL | pub enum ElemDerived {
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<[u8; 1]>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
              6: rustc_ty_utils::representability::representability_adt_ty
              7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability_adt_ty, rustc_query_impl::plumbing::QueryCtxt>
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability_adt_ty
              9: rustc_ty_utils::representability::representability_ty
             10: rustc_ty_utils::representability::representability
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             13: rustc_ty_utils::representability::representability
             14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             16: rustc_hir_analysis::check::wfcheck::check_type_defn
             17: rustc_hir_analysis::check::wfcheck::check_well_formed
             18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_well_formed
             20: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             21: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             22: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
             23: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
             24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
             25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
             26: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             27: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             28: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
             29: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#5}, ()>
             30: rustc_hir_analysis::check_crate
             32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             34: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             36: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             37: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             38: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             39: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             41: <unknown>
             42: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/infinite/infinite-tag-type-recursion.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/infinite/infinite-tag-type-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-tag-type-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-tag-type-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f952928aa75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f95292f7df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f952927f4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f952928a881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f952928da84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f952928d76a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f9529d77fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f952928e1a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f952928deda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f952928af46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f952928dbf7 - rust_begin_unwind
  11:     0x7f95292442c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f952b7f5466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f9529c1c1cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f952baf0399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f952b9078b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f952a1f53fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f952bb9669f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f952b9081f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f952a1f51d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f952a1f4e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7f952baf0011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f952b9078b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7f952a1f4ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7f952baf0011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7f952b9078b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7f952a6b6644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7f952a6adbea - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7f952baf9316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f952b96b210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7f952a5ec875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7f952a66b5b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7f952a73438d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7f952a6bbe3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7f952baf7f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7f952b934030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7f952a5ec9c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7f952a66b5d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7f952a7344fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7f952a53c759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7f952a537de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7f9529e3a282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7f952bc00bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7f952b9008a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7f9529d82fac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7f9529dc1273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7f9529d63dff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7f9529d8dc43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7f9529dd5966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f9529d86f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f952929a47e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7f9529034b43 - <unknown>
  52:     0x7f95290c6a00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `MList` is representable
#1 [representability] checking if `MList::Cons::1` is representable
#2 [representability] checking if `MList` is representable
#3 [check_well_formed] checking that `MList` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when checking if `MList` is representable
  --> fake-test-src-base/infinite/infinite-tag-type-recursion.rs:1:1
   |
LL | enum MList { Cons(isize, MList), Nil }
   |
   |
note: ...which requires checking if `MList::Cons::1` is representable...
  --> fake-test-src-base/infinite/infinite-tag-type-recursion.rs:1:26
   |
LL | enum MList { Cons(isize, MList), Nil }
   |                          ^^^^^
   = note: ...which requires checking if `MList` is representable...
   = note: ...which again requires checking if `MList` is representable, completing the cycle
note: cycle used when checking that `MList` is well-formed
  --> fake-test-src-base/infinite/infinite-tag-type-recursion.rs:1:1
   |
LL | enum MList { Cons(isize, MList), Nil }
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<[u8; 1]>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
              6: rustc_ty_utils::representability::representability_adt_ty
              7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability_adt_ty, rustc_query_impl::plumbing::QueryCtxt>
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability_adt_ty
              9: rustc_ty_utils::representability::representability_ty
             10: rustc_ty_utils::representability::representability
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             13: rustc_ty_utils::representability::representability
             14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             16: rustc_hir_analysis::check::wfcheck::check_type_defn
             17: rustc_hir_analysis::check::wfcheck::check_well_formed
             18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_well_formed
             20: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             21: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             22: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
             23: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
             24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
             25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
             26: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             27: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             28: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
             29: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#5}, ()>
             30: rustc_hir_analysis::check_crate
             32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             34: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             36: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             37: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             38: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             39: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             41: <unknown>
             42: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/infinite/infinite-struct.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/infinite/infinite-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-struct/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7fe760e86a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7fe760ef3df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7fe760e7b4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7fe760e86881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7fe760e89a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7fe760e8976a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7fe761973fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe760e8a1a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7fe760e89eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7fe760e86f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7fe760e89bf7 - rust_begin_unwind
  11:     0x7fe760e402c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7fe7633f1466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7fe7618181cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7fe7636ec399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7fe761df13fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7fe76379269f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7fe7635041f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7fe761df11d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7fe761df0e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7fe7636ec011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7fe761df0ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7fe7636ec011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7fe7622b2644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7fe7622a9c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7fe7636f5316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7fe763567210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7fe7621e8875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7fe7622675b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7fe76233038d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7fe7622b7e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7fe7636f3f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7fe763530030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7fe7621e89c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7fe7622675d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7fe7623304fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7fe762138759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7fe762133de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7fe761a36282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7fe7637fcbdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7fe7634fc8a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7fe76197efac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7fe7619bd273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7fe76195fdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7fe761989c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7fe7619d1966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7fe761982f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7fe760e9647e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7fe760c30b43 - <unknown>
  52:     0x7fe760cc2a00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `Take` is representable
#1 [representability] checking if `Take::0` is representable
#2 [representability] checking if `Take` is representable
#3 [check_well_formed] checking that `Take` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7fe760e86a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7fe760ef3df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7fe760e7b4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7fe760e86881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7fe760e89a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7fe760e8976a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7fe761973fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe760e8a1a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7fe760e89eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7fe760e86f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7fe760e89bf7 - rust_begin_unwind
  11:     0x7fe760e402c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7fe7633f1466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7fe7618181cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7fe7636ec399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7fe761df13fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7fe76379269f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7fe7635041f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7fe761df11d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7fe761df1694 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  21:     0x7fe76379269f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7fe7635041f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  23:     0x7fe761df11d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  24:     0x7fe761df0e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  25:     0x7fe7636ec011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  26:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  27:     0x7fe761df0ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  28:     0x7fe7636ec011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7fe7635038b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  30:     0x7fe7622b2644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  31:     0x7fe7622a9c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  32:     0x7fe7636f5316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  33:     0x7fe763567210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  34:     0x7fe7621e8875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  35:     0x7fe7622675b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7fe76233038d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  37:     0x7fe7622b7e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  38:     0x7fe7636f3f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  39:     0x7fe763530030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  40:     0x7fe7621e89c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  41:     0x7fe7622675d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7fe7623304fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7fe762138759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  44:     0x7fe762133de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  45:     0x7fe761a36282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  46:     0x7fe7637fcbdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  47:     0x7fe7634fc8a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  48:     0x7fe76197efac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  49:     0x7fe7619bd273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  50:     0x7fe76195fdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7fe761989c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  52:     0x7fe7619d1966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
---

14    |
15    = note: you cannot use `Self` as a generic parameter because it is reserved for associated items
16 
- error[E0072]: recursive type `Foo` has infinite size
-   --> $DIR/keyword-self-as-type-param.rs:3:1
-    |
- LL | struct Foo<Self>(Self);
-    | ^^^^^^^^^^^^^^^^ ---- recursive without indirection
-    |
- help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
-    |
- LL | struct Foo<Self>(Box<Self>);
+ error: aborting due to 2 previous errors
27 
- error: aborting due to 3 previous errors
- 
---
To only update this specific test, also pass `--test-args keyword/keyword-self-as-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/keyword/keyword-self-as-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/keyword-self-as-type-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword/keyword-self-as-type-param/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected keyword `Self` in generic parameters
  --> fake-test-src-base/keyword/keyword-self-as-type-param.rs:3:12
   |
LL | struct Foo<Self>(Self);
   |
   = note: you cannot use `Self` as a generic parameter because it is reserved for associated items


error: unexpected keyword `Self` in generic parameters
  --> fake-test-src-base/keyword/keyword-self-as-type-param.rs:7:11
   |
LL | trait Bar<Self> {}
   |
   = note: you cannot use `Self` as a generic parameter because it is reserved for associated items

error: aborting due to 2 previous errors
---

224 LL | | }
225    | |_^
226 
- error[E0391]: cycle detected when computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`
-   --> $DIR/fn-header-semantic-fail.rs:33:48
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |
-    |
- note: ...which requires borrow-checking `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}` is freeze...
-    = note: ...which requires evaluating trait selection obligation `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}: core::marker::Freeze`...
-    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/fn-header-semantic-fail.rs:5:1
- LL | / #![feature(const_extern_fn)]
- LL | |
- LL | | fn main() {
- LL | | fn main() {
- LL | |     async fn ff1() {} // OK.
- LL | |     }
- LL | | }
-    | |_^
- 
- 
- error[E0391]: cycle detected when computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`
-   --> $DIR/fn-header-semantic-fail.rs:45:48
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |
-    |
- note: ...which requires borrow-checking `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}` is freeze...
-    = note: ...which requires evaluating trait selection obligation `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}: core::marker::Freeze`...
-    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/fn-header-semantic-fail.rs:5:1
- LL | / #![feature(const_extern_fn)]
- LL | |
- LL | | fn main() {
- LL | | fn main() {
- LL | |     async fn ff1() {} // OK.
- LL | |     }
- LL | | }
-    | |_^
- 
---
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:12:5
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:19:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();


error: functions cannot be both `const` and `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:21:9
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:31:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}


error: functions cannot be both `const` and `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:33:9
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:45:9
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:51:18
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:52:19
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
LL |         unsafe fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:53:18
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:54:23
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:55:42
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |
help: remove the qualifiers
   |
   |
LL |         fn fe5(); //~ ERROR functions in `extern` blocks


error: functions cannot be both `const` and `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:55:9
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:17:9
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |               |
   |               |
   |               `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:29:9
   |
   |
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |               |
   |               |
   |               `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable

error[E0391]: cycle detected when computing type of `main::ff5::{opaque#0}`
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:12:44
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |
   |
note: ...which requires borrow-checking `main::ff5`...
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:12:5
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `main::ff5`...
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:12:5
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::ff5`...
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:12:5
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `main::ff5::{opaque#0}` is freeze...
   = note: ...which requires evaluating trait selection obligation `main::ff5::{opaque#0}: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::ff5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
  --> fake-test-src-base/parser/fn-header-semantic-fail.rs:5:1
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^

---

---- [ui] tests/ui/parser/issue-103748-ICE-wrong-braces.rs stdout ----
diff of stderr:

34 LL | struct Apple((Apple, Option<Banana ? Citron>));
36 
36 
- error[E0072]: recursive type `Apple` has infinite size
-   --> $DIR/issue-103748-ICE-wrong-braces.rs:3:1
-    |
- LL | struct Apple((Apple, Option(Banana ? Citron)));
-    | ^^^^^^^^^^^^  ----- recursive without indirection
-    |
- help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
-    |
- LL | struct Apple((Box<Apple>, Option(Banana ? Citron)));
+ error: aborting due to 4 previous errors
47 
- error: aborting due to 5 previous errors
- 
---
To only update this specific test, also pass `--test-args parser/issue-103748-ICE-wrong-braces.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issue-103748-ICE-wrong-braces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-103748-ICE-wrong-braces" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-103748-ICE-wrong-braces/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `?` in type
  --> fake-test-src-base/parser/issue-103748-ICE-wrong-braces.rs:3:36
   |
LL | struct Apple((Apple, Option(Banana ? Citron)));
   |                                    ^ `?` is only allowed on expressions, not types
help: if you meant to express that the type might not contain a value, use the `Option` wrapper type
   |
   |
LL | struct Apple((Apple, Option(Option<Banana > Citron)));


error: expected one of `)` or `,`, found `Citron`
  --> fake-test-src-base/parser/issue-103748-ICE-wrong-braces.rs:3:38
   |
LL | struct Apple((Apple, Option(Banana ? Citron)));
   |                                     -^^^^^^ expected one of `)` or `,`
   |                                     help: missing `,`


error[E0412]: cannot find type `Citron` in this scope
  --> fake-test-src-base/parser/issue-103748-ICE-wrong-braces.rs:3:38
   |
LL | struct Apple((Apple, Option(Banana ? Citron)));


error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
  --> fake-test-src-base/parser/issue-103748-ICE-wrong-braces.rs:3:22
   |
LL | struct Apple((Apple, Option(Banana ? Citron)));
   |                      ^^^^^^^^^^^^^^^^^^^^^^^ only `Fn` traits may use parentheses
help: use angle brackets instead
   |
   |
LL | struct Apple((Apple, Option<Banana ? Citron>));

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0214, E0412.
Some errors have detailed explanations: E0214, E0412.
For more information about an error, try `rustc --explain E0214`.
------------------------------------------


---- [ui] tests/ui/query-system/no-query-in-printing-during-query-descr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/query-system/no-query-in-printing-during-query-descr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/query-system/no-query-in-printing-during-query-descr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/query-system/no-query-in-printing-during-query-descr/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f1901f8fa75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f1901ffcdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f1901f844a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f1901f8f881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f1901f92a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f1901f9276a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f1902a7cfb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1901f931a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f1901f92eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f1901f8ff46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f1901f92bf7 - rust_begin_unwind
  11:     0x7f1901f492c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f19044fab86 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<rustc_middle[fd21469e3adb42a8]::ty::subst::EarlyBinder<rustc_middle[fd21469e3adb42a8]::ty::sty::Binder<rustc_middle[fd21469e3adb42a8]::ty::sty::FnSig>>> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f1902926150 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<rustc_middle[fd21469e3adb42a8]::ty::subst::EarlyBinder<rustc_middle[fd21469e3adb42a8]::ty::sty::Binder<rustc_middle[fd21469e3adb42a8]::ty::sty::FnSig>>>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f19048ff010 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::fn_sig, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f19046317c1 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::fn_sig
  16:     0x7f19057ca1eb - <rustc_middle[fd21469e3adb42a8]::ty::Ty>::fn_sig
  17:     0x7f1902f5d3ef - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_path
  18:     0x7f1902f5bfe3 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:     0x7f1902fc8122 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  20:     0x7f1902fcceac - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_kind
  21:     0x7f1902f5c04f - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  22:     0x7f1902fc8122 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  23:     0x7f1902f7fcc6 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f1902fc96d7 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7f1902f5c04f - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7f1902fc8122 - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7f1902f5e04a - <rustc_hir_typeck[6d15a1d604b68afb]::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7f19030ccdde - rustc_hir_typeck[6d15a1d604b68afb]::check::check_fn
  29:     0x7f19031590d8 - <rustc_hir_typeck[6d15a1d604b68afb]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d15a1d604b68afb]::typeck_with_fallback<rustc_hir_typeck[6d15a1d604b68afb]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[fd21469e3adb42a8]::ty::typeck_results::TypeckResults>
  30:     0x7f1903127f75 - rustc_hir_typeck[6d15a1d604b68afb]::typeck
  31:     0x7f190490112a - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::typeck, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  32:     0x7f190463b560 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::typeck
  33:     0x7f190333d67b - rustc_hir_analysis[7ce2480036b3af61]::collect::infer_return_ty_for_fn_sig
  34:     0x7f190334bcea - rustc_hir_analysis[7ce2480036b3af61]::collect::fn_sig
  35:     0x7f19048fdf55 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::fn_sig, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  36:     0x7f19046317c1 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::fn_sig
  37:     0x7f1903339c64 - rustc_hir_analysis[7ce2480036b3af61]::collect::convert_item
  38:     0x7f19033368da - <rustc_hir_analysis[7ce2480036b3af61]::collect::CollectItemTypesVisitor as rustc_hir[3bb29479b91ebc00]::intravisit::Visitor>::visit_item
  39:     0x7f1903366655 - <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[7ce2480036b3af61]::collect::CollectItemTypesVisitor>
  40:     0x7f1903335d7d - rustc_hir_analysis[7ce2480036b3af61]::collect::collect_mod_item_types
  41:     0x7f190486c3ed - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::collect_mod_item_types, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  42:     0x7f1904639970 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::collect_mod_item_types
  43:     0x7f1903365aee - <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::for_each_module::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  44:     0x7f19032409bc - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#0}, ()>
  45:     0x7f190323cd3b - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  46:     0x7f1902b3f282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  47:     0x7f1904905bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  48:     0x7f19046058a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  49:     0x7f1902a87fac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  50:     0x7f1902ac6273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  51:     0x7f1902a68dff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  52:     0x7f1902a92c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  53:     0x7f1902ada966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  54:     0x7f1902a8bf55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f1901f9f47e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  56:     0x7f1901d39b43 - <unknown>
  57:     0x7f1901dcba00 - <unknown>
  58:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `a`
#1 [fn_sig] computing function signature of `a`
#2 [collect_mod_item_types] collecting item types in top-level module
#3 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when computing function signature of `a`
  --> fake-test-src-base/query-system/no-query-in-printing-during-query-descr.rs:1:1
   |
LL | fn a() -> _ {
   |
   |
note: ...which requires type-checking `a`...
  --> fake-test-src-base/query-system/no-query-in-printing-during-query-descr.rs:1:1
   |
LL | fn a() -> _ {
   | ^^^^^^^^^^^
   = note: ...which again requires computing function signature of `a`, completing the cycle
note: cycle used when collecting item types in top-level module
  --> fake-test-src-base/query-system/no-query-in-printing-during-query-descr.rs:1:1
   |
LL | / fn a() -> _ {
LL | |     //~^ ERROR the placeholder `_` is not allowed within types on item signatures for return types
LL | |     &a
LL | | }
LL | |
LL | | fn main() {}
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<rustc_middle::ty::subst::EarlyBinder<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>>>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig
              6: <rustc_middle::ty::Ty>::fn_sig
              7: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_path
              8: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
              9: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation
             10: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_kind
             11: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
             12: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation
             13: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_block_with_expected
             14: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_kind
             15: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
             16: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_expr_with_expectation
             17: <rustc_hir_typeck::fn_ctxt::FnCtxt>::check_return_expr
             18: rustc_hir_typeck::check::check_fn
             19: <rustc_hir_typeck::inherited::InheritedBuilder>::enter::<rustc_hir_typeck::typeck_with_fallback<rustc_hir_typeck::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle::ty::typeck_results::TypeckResults>
             20: rustc_hir_typeck::typeck
             22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
             23: rustc_hir_analysis::collect::infer_return_ty_for_fn_sig
             24: rustc_hir_analysis::collect::fn_sig
             25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
             25: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
             26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig
             27: rustc_hir_analysis::collect::convert_item
             28: <rustc_hir_analysis::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
             29: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis::collect::CollectItemTypesVisitor>
             30: rustc_hir_analysis::collect::collect_mod_item_types
             31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
             32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
             33: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#0}::{closure#0}::{closure#0}>
             34: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#0}, ()>
             35: rustc_hir_analysis::check_crate
             37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             38: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             39: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             40: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             40: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             41: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             42: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             43: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             44: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             46: <unknown>
             47: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/query-system/fn-sig-cycle-arity.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/query-system/fn-sig-cycle-arity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/query-system/fn-sig-cycle-arity" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/query-system/fn-sig-cycle-arity/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f4761d55a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f4761dc2df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f4761d4a4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f4761d55881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f4761d58a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f4761d5876a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f4762842fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4761d591a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f4761d58eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f4761d55f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f4761d58bf7 - rust_begin_unwind
  11:     0x7f4761d0f2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f47642c0b86 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<rustc_middle[fd21469e3adb42a8]::ty::subst::EarlyBinder<rustc_middle[fd21469e3adb42a8]::ty::sty::Binder<rustc_middle[fd21469e3adb42a8]::ty::sty::FnSig>>> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f47626ec150 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<rustc_middle[fd21469e3adb42a8]::ty::subst::EarlyBinder<rustc_middle[fd21469e3adb42a8]::ty::sty::Binder<rustc_middle[fd21469e3adb42a8]::ty::sty::FnSig>>>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f47646c5010 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::fn_sig, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f47643f77c1 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::fn_sig
  16:     0x7f4762cd4806 - rustc_ty_utils[e1dd546375f78c05]::ty::param_env
  17:     0x7f47646e20ad - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::param_env, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f47644240e1 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::param_env
  19:     0x7f4762f1ebc4 - <rustc_hir_typeck[6d15a1d604b68afb]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[6d15a1d604b68afb]::typeck_with_fallback<rustc_hir_typeck[6d15a1d604b68afb]::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle[fd21469e3adb42a8]::ty::typeck_results::TypeckResults>
  20:     0x7f4762eedf75 - rustc_hir_typeck[6d15a1d604b68afb]::typeck
  21:     0x7f47646c712a - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::typeck, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f4764401560 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::typeck
  23:     0x7f476310367b - rustc_hir_analysis[7ce2480036b3af61]::collect::infer_return_ty_for_fn_sig
  24:     0x7f4763111cea - rustc_hir_analysis[7ce2480036b3af61]::collect::fn_sig
  25:     0x7f47646c3f55 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::fn_sig, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  26:     0x7f47643f77c1 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::fn_sig
  27:     0x7f47630fd093 - <rustc_hir_analysis[7ce2480036b3af61]::collect::CollectItemTypesVisitor as rustc_hir[3bb29479b91ebc00]::intravisit::Visitor>::visit_trait_item
  28:     0x7f476312c695 - <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[7ce2480036b3af61]::collect::CollectItemTypesVisitor>
  29:     0x7f47630fbd7d - rustc_hir_analysis[7ce2480036b3af61]::collect::collect_mod_item_types
  30:     0x7f47646323ed - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::collect_mod_item_types, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  31:     0x7f47643ff970 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::collect_mod_item_types
  32:     0x7f476312baee - <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::for_each_module::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  33:     0x7f47630069bc - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#0}, ()>
  34:     0x7f4763002d3b - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  35:     0x7f4762905282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  36:     0x7f47646cbbdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  37:     0x7f47643cb8a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  38:     0x7f476284dfac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  39:     0x7f476288c273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  40:     0x7f476282edff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  41:     0x7f4762858c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  42:     0x7f47628a0966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7f4762851f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f4761d6547e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  45:     0x7f4761affb43 - <unknown>
  46:     0x7f4761b91a00 - <unknown>
  47:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `Dancer::dance`
#1 [typeck] type-checking `Dancer::dance`
#2 [fn_sig] computing function signature of `Dancer::dance`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when computing function signature of `Dancer::dance`
  --> fake-test-src-base/query-system/fn-sig-cycle-arity.rs:2:5
   |
LL |     fn dance(&self) -> _ {
   |
   |
note: ...which requires type-checking `Dancer::dance`...
  --> fake-test-src-base/query-system/fn-sig-cycle-arity.rs:2:5
   |
LL |     fn dance(&self) -> _ {
   |     ^^^^^^^^^^^^^^^^^^^^
note: ...which requires computing normalized predicates of `Dancer::dance`...
  --> fake-test-src-base/query-system/fn-sig-cycle-arity.rs:2:5
   |
LL |     fn dance(&self) -> _ {
   |     ^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing function signature of `Dancer::dance`, completing the cycle
note: cycle used when collecting item types in top-level module
  --> fake-test-src-base/query-system/fn-sig-cycle-arity.rs:1:1
   |
LL | / trait Dancer {
LL | |     fn dance(&self) -> _ {
LL | |         //~^ ERROR the placeholder `_` is not allowed within types on item signatures for return types
LL | |         self.dance()
LL | |
LL | | fn main() {}
   | |____________^
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<rustc_middle::ty::subst::EarlyBinder<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>>>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig
              7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::param_env, rustc_query_impl::plumbing::QueryCtxt>
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::param_env
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::param_env
              9: <rustc_hir_typeck::inherited::InheritedBuilder>::enter::<rustc_hir_typeck::typeck_with_fallback<rustc_hir_typeck::typeck::{closure#0}>::{closure#0}::{closure#1}, &rustc_middle::ty::typeck_results::TypeckResults>
             10: rustc_hir_typeck::typeck
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
             13: rustc_hir_analysis::collect::infer_return_ty_for_fn_sig
             14: rustc_hir_analysis::collect::fn_sig
             15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
             15: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::fn_sig, rustc_query_impl::plumbing::QueryCtxt>
             16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::fn_sig
             17: <rustc_hir_analysis::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_trait_item
             18: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis::collect::CollectItemTypesVisitor>
             19: rustc_hir_analysis::collect::collect_mod_item_types
             20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
             21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
             22: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#0}::{closure#0}::{closure#0}>
             23: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#0}, ()>
             24: rustc_hir_analysis::check_crate
             26: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             27: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             28: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             29: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             29: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             30: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             31: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             32: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             33: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             35: <unknown>
             36: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/recursion/recursive-enum.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/recursion/recursive-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-enum/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7fc86dc6ca75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7fc86dcd9df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7fc86dc614a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7fc86dc6c881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7fc86dc6fa84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7fc86dc6f76a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7fc86e759fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc86dc701a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7fc86dc6feda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7fc86dc6cf46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7fc86dc6fbf7 - rust_begin_unwind
  11:     0x7fc86dc262c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7fc8701d7466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7fc86e5fe1cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7fc8704d2399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7fc8702e98b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7fc86ebd73fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7fc87057869f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7fc8702ea1f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7fc86ebd71d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7fc86ebd6e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7fc8704d2011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7fc8702e98b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7fc86ebd6ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7fc8704d2011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7fc8702e98b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7fc86f098644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7fc86f08fbea - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7fc8704db316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7fc87034d210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7fc86efce875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7fc86f04d5b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7fc86f11638d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7fc86f09de3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7fc8704d9f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7fc870316030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7fc86efce9c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7fc86f04d5d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7fc86f1164fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7fc86ef1e759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7fc86ef19de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7fc86e81c282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7fc8705e2bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7fc8702e28a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7fc86e764fac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7fc86e7a3273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7fc86e745dff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7fc86e76fc43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7fc86e7b7966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7fc86e768f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7fc86dc7c47e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7fc86da16b43 - <unknown>
  52:     0x7fc86daa8a00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `List<T>` is representable
#1 [representability] checking if `List::Cons::1` is representable
#2 [representability] checking if `List` is representable
#3 [check_well_formed] checking that `List` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when checking if `List` is representable
  --> fake-test-src-base/recursion/recursive-enum.rs:1:1
   |
LL | enum List<T> { Cons(T, List<T>), Nil }
   |
   |
note: ...which requires checking if `List::Cons::1` is representable...
  --> fake-test-src-base/recursion/recursive-enum.rs:1:24
   |
LL | enum List<T> { Cons(T, List<T>), Nil }
   |                        ^^^^^^^
   = note: ...which requires checking if `List<T>` is representable...
   = note: ...which again requires checking if `List` is representable, completing the cycle
note: cycle used when checking that `List` is well-formed
  --> fake-test-src-base/recursion/recursive-enum.rs:1:1
   |
LL | enum List<T> { Cons(T, List<T>), Nil }
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<[u8; 1]>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
              6: rustc_ty_utils::representability::representability_adt_ty
              7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability_adt_ty, rustc_query_impl::plumbing::QueryCtxt>
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability_adt_ty
              9: rustc_ty_utils::representability::representability_ty
             10: rustc_ty_utils::representability::representability
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             13: rustc_ty_utils::representability::representability
             14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             16: rustc_hir_analysis::check::wfcheck::check_type_defn
             17: rustc_hir_analysis::check::wfcheck::check_well_formed
             18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_well_formed
             20: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             21: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             22: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
             23: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
             24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
             25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
             26: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
---

24 LL | | }
25    | |_^
26 
- error[E0277]: the trait bound `U: From<T>` is not satisfied
-   --> $DIR/issue-53092-2.rs:9:5
- LL |     |x| x.into()
-    |     ^^^^^^^^^^^^ the trait `From<T>` is not implemented for `U`
-    |
- note: required by a bound in `make_bug`
- note: required by a bound in `make_bug`
-   --> $DIR/issue-53092-2.rs:8:19
-    |
- LL | fn make_bug<T, U: From<T>>() -> Bug<T, U> {
-    |                   ^^^^^^^ required by this bound in `make_bug`
- help: consider restricting type parameter `U`
-    |
- LL | type Bug<T, U: std::convert::From<T>> = impl Fn(T) -> U + Copy;
+ error: aborting due to previous error
42 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-53092-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/issue-53092-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-53092-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-53092-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Bug::{opaque#0}`
  --> fake-test-src-base/type-alias-impl-trait/issue-53092-2.rs:4:18
   |
LL | type Bug<T, U> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected
   |
   |
note: ...which requires type-checking `CONST_BUG`...
  --> fake-test-src-base/type-alias-impl-trait/issue-53092-2.rs:6:1
   |
LL | const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Bug<u8, ()>`...
   = note: ...which requires normalizing `Bug<u8, ()>`...
   = note: ...which again requires computing type of `Bug::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
  --> fake-test-src-base/type-alias-impl-trait/issue-53092-2.rs:1:1
LL | / #![feature(type_alias_impl_trait)]
LL | | #![allow(dead_code)]
LL | |
LL | |
LL | | type Bug<T, U> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected
LL | |     CONST_BUG(0);
LL | | }
   | |_^

---


---- [ui] tests/ui/type/type-recursive-box-shadowed.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/type-recursive-box-shadowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-recursive-box-shadowed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-recursive-box-shadowed/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f99cb069a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f99cb0d6df8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f99cb05e4a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f99cb069881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f99cb06ca84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f99cb06c76a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f99cbb56fb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f99cb06d1a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f99cb06ceda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f99cb069f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f99cb06cbf7 - rust_begin_unwind
  11:     0x7f99cb0232c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f99cd5d4466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f99cb9fb1cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f99cd8cf399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f99cd6e68b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f99cbfd43fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f99cd97569f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f99cd6e71f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f99cbfd41d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f99cbfd3e25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7f99cd8cf011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f99cd6e68b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7f99cbfd3ac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7f99cd8cf011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7f99cd6e68b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7f99cc495644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7f99cc48cc1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7f99cd8d8316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f99cd74a210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7f99cc3cb875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7f99cc44a5b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7f99cc51338d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7f99cc49ae3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7f99cd8d6f8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7f99cd713030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7f99cc3cb9c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7f99cc44a5d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7f99cc5134fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7f99cc31b759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7f99cc316de3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7f99cbc19282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7f99cd9dfbdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7f99cd6df8a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7f99cbb61fac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7f99cbba0273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7f99cbb42dff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7f99cbb6cc43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7f99cbbb4966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f99cbb65f55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f99cb07947e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7f99cae13b43 - <unknown>
  52:     0x7f99caea5a00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `Foo` is representable
#1 [representability] checking if `Foo::inner` is representable
#2 [representability] checking if `Foo` is representable
#3 [check_well_formed] checking that `Foo` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error[E0391]: cycle detected when checking if `Foo` is representable
  --> fake-test-src-base/type/type-recursive-box-shadowed.rs:7:1
LL | struct Foo {
   | ^^^^^^^^^^
   |
   |
note: ...which requires checking if `Foo::inner` is representable...
  --> fake-test-src-base/type/type-recursive-box-shadowed.rs:9:5
LL |     inner: Foo,
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^
   = note: ...which requires checking if `Foo` is representable...
   = note: ...which again requires checking if `Foo` is representable, completing the cycle
note: cycle used when checking that `Foo` is well-formed
  --> fake-test-src-base/type/type-recursive-box-shadowed.rs:7:1
LL | struct Foo {
   | ^^^^^^^^^^
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              1: <rustc_errors::Handler>::emit_diagnostic
              2: <rustc_span::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
              3: rustc_query_system::query::plumbing::mk_cycle::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::query::erase::Erased<[u8; 1]>, rustc_middle::dep_graph::dep_node::DepKind>
              4: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
              5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
              6: rustc_ty_utils::representability::representability_adt_ty
              7: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability_adt_ty, rustc_query_impl::plumbing::QueryCtxt>
              8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability_adt_ty
              9: rustc_ty_utils::representability::representability_ty
             10: rustc_ty_utils::representability::representability
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             13: rustc_ty_utils::representability::representability
             14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::representability, rustc_query_impl::plumbing::QueryCtxt>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::representability
             16: rustc_hir_analysis::check::wfcheck::check_type_defn
             17: rustc_hir_analysis::check::wfcheck::check_well_formed
             18: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt>
             19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_well_formed
             20: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             21: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             22: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ItemId], <rustc_middle::hir::ModuleItems>::par_items<rustc_hir_analysis::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
             23: rustc_hir_analysis::check::wfcheck::check_mod_type_wf
             24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
             25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_type_wf
             26: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
             27: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
             28: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_hir_analysis::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
             29: <rustc_session::session::Session>::track_errors::<rustc_hir_analysis::check_crate::{closure#5}, ()>
             30: rustc_hir_analysis::check_crate
             32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             34: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             35: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             36: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             37: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             38: std::panicking::try::<core::result::Result<(), rustc_span::ErrorGuaranteed>, core::panic::unwind_safe::AssertUnwindSafe<<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
             39: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             41: <unknown>
             42: <unknown>
           


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/type/type-recursive.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/type-recursive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-recursive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-recursive/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f57e97a2a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f57e980fdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f57e97974a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f57e97a2881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f57e97a5a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f57e97a576a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f57ea28ffb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57e97a61a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f57e97a5eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f57e97a2f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f57e97a5bf7 - rust_begin_unwind
  11:     0x7f57e975c2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f57ebd0d466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f57ea1341cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f57ec008399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f57ea70d3fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f57ea70ce25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  21:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  23:     0x7f57ea70cac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  24:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  25:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  26:     0x7f57eabce644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  27:     0x7f57eabc5c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  28:     0x7f57ec011316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f57ebe83210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  30:     0x7f57eab04875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  31:     0x7f57eab835b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7f57eac4c38d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  33:     0x7f57eabd3e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  34:     0x7f57ec00ff8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  35:     0x7f57ebe4c030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  36:     0x7f57eab049c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  37:     0x7f57eab835d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  38:     0x7f57eac4c4fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  39:     0x7f57eaa54759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  40:     0x7f57eaa4fde3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  41:     0x7f57ea352282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  42:     0x7f57ec118bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  43:     0x7f57ebe188a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  44:     0x7f57ea29afac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  45:     0x7f57ea2d9273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  46:     0x7f57ea27bdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7f57ea2a5c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  48:     0x7f57ea2ed966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f57ea29ef55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f57e97b247e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  51:     0x7f57e954cb43 - <unknown>
  52:     0x7f57e95dea00 - <unknown>
  53:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `T1` is representable
#1 [representability] checking if `T1::foolish` is representable
#2 [representability] checking if `T1` is representable
#3 [check_well_formed] checking that `T1` is well-formed
#4 [check_mod_type_wf] checking that types are well-formed in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f57e97a2a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f57e980fdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f57e97974a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f57e97a2881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f57e97a5a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f57e97a576a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f57ea28ffb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57e97a61a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f57e97a5eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f57e97a2f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f57e97a5bf7 - rust_begin_unwind
  11:     0x7f57e975c2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f57ebd0d466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f57ea1341cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f57ec008399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f57ea70d3fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f57ea70d694 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  21:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  23:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  24:     0x7f57ea70ce25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  25:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  26:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  27:     0x7f57ea70cac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  28:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  30:     0x7f57eabce644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  31:     0x7f57eabc5c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  32:     0x7f57ec011316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  33:     0x7f57ebe83210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  34:     0x7f57eab04875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  35:     0x7f57eab835b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f57eac4c38d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  37:     0x7f57eabd3e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  38:     0x7f57ec00ff8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  39:     0x7f57ebe4c030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  40:     0x7f57eab049c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  41:     0x7f57eab835d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7f57eac4c4fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7f57eaa54759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  44:     0x7f57eaa4fde3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  45:     0x7f57ea352282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  46:     0x7f57ec118bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  47:     0x7f57ebe188a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  48:     0x7f57ea29afac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  49:     0x7f57ea2d9273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  50:     0x7f57ea27bdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f57ea2a5c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  52:     0x7f57ea2ed966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:     0x7f57ea29ef55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7f57e97b247e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  55:     0x7f57e954cb43 - <unknown>
  56:     0x7f57e95dea00 - <unknown>
  57:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `T2` is representable
#1 [representability_adt_ty] checking if `core::option::Option<T2>` is representable
#2 [representability] checking if `T2::inner` is representable
#3 [representability] checking if `T2` is representable
#4 [check_well_formed] checking that `T2` is well-formed
#5 [check_mod_type_wf] checking that types are well-formed in top-level module
#6 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f57e97a2a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f57e980fdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f57e97974a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f57e97a2881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f57e97a5a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f57e97a576a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f57ea28ffb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57e97a61a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f57e97a5eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f57e97a2f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f57e97a5bf7 - rust_begin_unwind
  11:     0x7f57e975c2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f57ebd0d466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f57ea1341cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f57ec008399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f57ea70d3fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f57ea70d694 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  21:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  23:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  24:     0x7f57ea70ce25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  25:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  26:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  27:     0x7f57ea70cac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  28:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  30:     0x7f57eabce644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  31:     0x7f57eabc5c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  32:     0x7f57ec011316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  33:     0x7f57ebe83210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  34:     0x7f57eab04875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  35:     0x7f57eab835b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f57eac4c38d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  37:     0x7f57eabd3e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  38:     0x7f57ec00ff8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  39:     0x7f57ebe4c030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  40:     0x7f57eab049c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  41:     0x7f57eab835d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7f57eac4c4fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7f57eaa54759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  44:     0x7f57eaa4fde3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  45:     0x7f57ea352282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  46:     0x7f57ec118bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  47:     0x7f57ebe188a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  48:     0x7f57ea29afac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  49:     0x7f57ea2d9273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  50:     0x7f57ea27bdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f57ea2a5c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  52:     0x7f57ea2ed966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:     0x7f57ea29ef55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7f57e97b247e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  55:     0x7f57e954cb43 - <unknown>
  56:     0x7f57e95dea00 - <unknown>
  57:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `T3` is representable
#1 [representability_adt_ty] checking if `core::option::Option<T3>` is representable
#2 [representability] checking if `T3::inner` is representable
#3 [representability] checking if `T3` is representable
#4 [check_well_formed] checking that `T3` is well-formed
#5 [check_mod_type_wf] checking that types are well-formed in top-level module
#6 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f57e97a2a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f57e980fdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f57e97974a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f57e97a2881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f57e97a5a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f57e97a576a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f57ea28ffb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57e97a61a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f57e97a5eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f57e97a2f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f57e97a5bf7 - rust_begin_unwind
  11:     0x7f57e975c2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f57ebd0d466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f57ea1341cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f57ec008399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f57ea70d3fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f57ea70d694 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  21:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  23:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  24:     0x7f57ea70ce25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  25:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  26:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  27:     0x7f57ea70cac1 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
  28:     0x7f57ec008011 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  29:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  30:     0x7f57eabce644 - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_type_defn
  31:     0x7f57eabc5c1a - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_well_formed
  32:     0x7f57ec011316 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_well_formed, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  33:     0x7f57ebe83210 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_well_formed
  34:     0x7f57eab04875 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  35:     0x7f57eab835b6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f57eac4c38d - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir::ItemId], <rustc_middle[fd21469e3adb42a8]::hir::ModuleItems>::par_items<rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  37:     0x7f57eabd3e3f - rustc_hir_analysis[7ce2480036b3af61]::check::wfcheck::check_mod_type_wf
  38:     0x7f57ec00ff8d - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::check_mod_type_wf, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  39:     0x7f57ebe4c030 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::check_mod_type_wf
  40:     0x7f57eab049c5 - <core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once
  41:     0x7f57eab835d6 - std[c84aed779b5c947d]::panicking::try::<(), core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7f57eac4c4fd - rustc_data_structures[e9a3567b76577cfa]::sync::par_for_each_in::<&[rustc_hir[3bb29479b91ebc00]::hir_id::OwnerId], <rustc_middle[fd21469e3adb42a8]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7f57eaa54759 - <rustc_session[897b0873b9997223]::session::Session>::track_errors::<rustc_hir_analysis[7ce2480036b3af61]::check_crate::{closure#5}, ()>
  44:     0x7f57eaa4fde3 - rustc_hir_analysis[7ce2480036b3af61]::check_crate
  45:     0x7f57ea352282 - rustc_interface[eb790dfc14a37847]::passes::analysis
  46:     0x7f57ec118bdc - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::analysis, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  47:     0x7f57ebe188a8 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::analysis
  48:     0x7f57ea29afac - <rustc_middle[fd21469e3adb42a8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  49:     0x7f57ea2d9273 - <rustc_interface[eb790dfc14a37847]::interface::Compiler>::enter::<rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}::{closure#2}, core[781ce462709a9f9f]::result::Result<core[781ce462709a9f9f]::option::Option<rustc_interface[eb790dfc14a37847]::queries::Linker>, rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  50:     0x7f57ea27bdff - rustc_span[6423ed9b1513bcee]::with_source_map::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  51:     0x7f57ea2a5c43 - std[c84aed779b5c947d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>
  52:     0x7f57ea2ed966 - std[c84aed779b5c947d]::panicking::try::<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, core[781ce462709a9f9f]::panic::unwind_safe::AssertUnwindSafe<<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  53:     0x7f57ea29ef55 - <<std[c84aed779b5c947d]::thread::Builder>::spawn_unchecked_<rustc_interface[eb790dfc14a37847]::util::run_in_thread_pool_with_globals<rustc_interface[eb790dfc14a37847]::interface::run_compiler<core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>, rustc_driver_impl[8a261955774da82e]::run_compiler::{closure#1}>::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[781ce462709a9f9f]::result::Result<(), rustc_span[6423ed9b1513bcee]::ErrorGuaranteed>>::{closure#1} as core[781ce462709a9f9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7f57e97b247e - std::sys::unix::thread::Thread::new::thread_start::h0019900357eeb8c0
  55:     0x7f57e954cb43 - <unknown>
  56:     0x7f57e95dea00 - <unknown>
  57:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (4014e325b 2023-03-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [representability_adt_ty] checking if `T4` is representable
#1 [representability_adt_ty] checking if `core::option::Option<T4>` is representable
#2 [representability] checking if `T4::0` is representable
#3 [representability] checking if `T4` is representable
#4 [check_well_formed] checking that `T4` is well-formed
#5 [check_mod_type_wf] checking that types are well-formed in top-level module
#6 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'Value::from_cycle_error called without errors', /checkout/compiler/rustc_query_system/src/values.rs:13:9
stack backtrace:
   0:     0x7f57e97a2a75 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1d39860b5ec0bc1b
   1:     0x7f57e980fdf8 - core::fmt::write::h79df5e17c8307fc3
   2:     0x7f57e97974a1 - std::io::Write::write_fmt::hea10d0f17bab4ae8
   3:     0x7f57e97a2881 - std::sys_common::backtrace::print::h8ec895ccb5fb60fb
   4:     0x7f57e97a5a84 - std::panicking::default_hook::{{closure}}::h4928bac57a18221b
   5:     0x7f57e97a576a - std::panicking::default_hook::hda5a6c00a8c66085
   6:     0x7f57ea28ffb5 - rustc_driver_impl[8a261955774da82e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f57e97a61a1 - std::panicking::rust_panic_with_hook::hfe9a7563f7eb9e1b
   8:     0x7f57e97a5eda - std::panicking::begin_panic_handler::{{closure}}::h347a13beefac16bf
   9:     0x7f57e97a2f46 - std::sys_common::backtrace::__rust_end_short_backtrace::h401b15a6419e1541
  10:     0x7f57e97a5bf7 - rust_begin_unwind
  11:     0x7f57e975c2c3 - core::panicking::panic_fmt::h1d3ba4150837b3fa
  12:     0x7f57ebd0d466 - <rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]> as rustc_query_system[fc467f7d9cdfeb5c]::values::Value<rustc_middle[fd21469e3adb42a8]::ty::context::TyCtxt, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>>::from_cycle_error
  13:     0x7f57ea1341cd - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::mk_cycle::<rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt, rustc_middle[fd21469e3adb42a8]::query::erase::Erased<[u8; 1usize]>, rustc_middle[fd21469e3adb42a8]::dep_graph::dep_node::DepKind>
  14:     0x7f57ec008399 - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  15:     0x7f57ebe1f8b0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability
  16:     0x7f57ea70d3fc - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  17:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  18:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  19:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  20:     0x7f57ea70d694 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_adt_ty
  21:     0x7f57ec0ae69f - rustc_query_system[fc467f7d9cdfeb5c]::query::plumbing::try_execute_query::<rustc_query_impl[792a3605d0add3da]::queries::representability_adt_ty, rustc_query_impl[792a3605d0add3da]::plumbing::QueryCtxt>
  22:     0x7f57ebe201f0 - <rustc_query_impl[792a3605d0add3da]::Queries as rustc_middle[fd21469e3adb42a8]::ty::query::QueryEngine>::representability_adt_ty
  23:     0x7f57ea70d1d3 - rustc_ty_utils[e1dd546375f78c05]::representability::representability_ty
  24:     0x7f57ea70ce25 - rustc_ty_utils[e1dd546375f78c05]::representability::representability
