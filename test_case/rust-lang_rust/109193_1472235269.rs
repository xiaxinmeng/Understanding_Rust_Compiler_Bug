plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 440/14654
........................................................................................ 528/14654
........................................................................................ 616/14654
........................................................................................ 704/14654
............F.........F..FF..FF.....FF.....FF.F..FF....FFF.F.....F...F.................. 792/14654
..........................................................................i............. 968/14654
........................................................................................ 1056/14654
...i.................................................................................... 1144/14654
........................................................................................ 1232/14654
---
........................................................................................ 5016/14654
..........................................i............................................. 5104/14654
........................................................................................ 5192/14654
........................................................................................ 5280/14654
.......................................................................................F 5368/14654
.FF......F...F..F..F..F....F..F.F......F.F.F...........................F................ 5456/14654
........................................................................................ 5632/14654
........................................................................................ 5720/14654
...........................................................................i..........ii 5808/14654
ii........................................................................i............. 5896/14654
---
failures:

---- [ui] tests/ui/async-await/in-trait/async-associated-types.rs#next stdout ----

error in revision `next`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-associated-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-associated-types.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-associated-types.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7f984f03aaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7f984f0a7cc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7f984f02f1d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7f984f03a901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7f984f03a901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7f984f03dad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7f984f03d7ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7f984fb42555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f984f03e1f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7f984f03df69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7f984f03afc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7f984f03dc47 - rust_begin_unwind
  11:     0x7f984eff42c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7f984eff464c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7f984f8bff59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7f984ffebafa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7f984ffae52b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7f984ffeb0f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7f98519cebad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7f9851724bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7f985049c172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7f985048fa43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7f98518b674f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7f9851736029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7f98503bdf56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7f98504810ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7f985049b5ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7f98518b5588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7f9851701ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7f98503bdfb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f98504814fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7f985039ad8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7f98503dd2b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7f984fc01820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7f98519ba25f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7f98516d0663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7f984fb44a93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7f984fb897cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7f984fb43818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f984fb7e0e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7f984fb5c706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7f984fb299e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f984fb522e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f984f04a4de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7f984ede4b43 - <unknown>
  44:     0x7f984ee76a00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-example-desugared-boxed.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-example-desugared-boxed.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-example-desugared-boxed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-boxed.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-boxed.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7fb69b88faf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7fb69b8fccc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7fb69b8841d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7fb69b88f901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7fb69b88f901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7fb69b892ad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7fb69b8927ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7fb69c397555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb69b8931f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7fb69b892f69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7fb69b88ffc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7fb69b892c47 - rust_begin_unwind
  11:     0x7fb69b8492c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7fb69b84964c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7fb69c114f59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7fb69c840afa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7fb69c80352b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7fb69c8400f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7fb69e223bad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7fb69df79bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7fb69ccf1172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7fb69cce4a43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7fb69e10b74f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7fb69df8b029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7fb69cc12f56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fb69ccd60ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7fb69ccf05ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7fb69e10a588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7fb69df56ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7fb69cc12fb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7fb69ccd64fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7fb69cbefd8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7fb69cc322b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7fb69c456820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7fb69e20f25f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7fb69df25663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7fb69c399a93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7fb69c3de7cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7fb69c398818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fb69c3d30e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7fb69c3b1706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7fb69c37e9e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7fb69c3a72e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7fb69b89f4de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7fb69b639b43 - <unknown>
  44:     0x7fb69b6cba00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-example-desugared-extra.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-example-desugared-extra.rs#next stdout ----

error in revision `next`: test compilation failed although it shouldn't!
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-example-desugared-extra.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-extra.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-extra.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7fe731e10af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7fe731e7dcc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7fe731e051d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7fe731e10901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7fe731e10901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7fe731e13ad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7fe731e137ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7fe732918555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe731e141f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7fe731e13f69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7fe731e10fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7fe731e13c47 - rust_begin_unwind
  11:     0x7fe731dca2c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7fe731dca64c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7fe732695f59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7fe732dc1afa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7fe732d8452b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7fe732dc10f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7fe7347a4bad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7fe7344fabc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7fe733272172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7fe733265a43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7fe73468c74f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7fe73450c029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7fe733193f56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fe7332570ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7fe7332715ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7fe73468b588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7fe7344d7ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7fe733193fb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7fe7332574fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7fe733170d8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7fe7331b32b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7fe7329d7820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7fe73479025f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7fe7344a6663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7fe73291aa93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7fe73295f7cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7fe732919818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fe7329540e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7fe732932706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7fe7328ff9e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7fe7329282e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7fe731e204de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7fe731bbab43 - <unknown>
  44:     0x7fe731c4ca00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-example-desugared-in-trait.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-example-desugared-in-trait.rs#next stdout ----

error in revision `next`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-example-desugared-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-in-trait.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-in-trait.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7f95dfdcdaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7f95dfe3acc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7f95dfdc21d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7f95dfdcd901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7f95dfdcd901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7f95dfdd0ad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7f95dfdd07ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7f95e08d5555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f95dfdd11f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7f95dfdd0f69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7f95dfdcdfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7f95dfdd0c47 - rust_begin_unwind
  11:     0x7f95dfd872c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7f95dfd8764c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7f95e0652f59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7f95e0d7eafa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7f95e0d4152b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7f95e0d7e0f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7f95e2761bad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7f95e24b7bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7f95e122f172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7f95e1222a43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7f95e264974f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7f95e24c9029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7f95e1150f56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7f95e12140ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7f95e122e5ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7f95e2648588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7f95e2494ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7f95e1150fb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f95e12144fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7f95e112dd8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7f95e11702b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7f95e0994820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7f95e274d25f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7f95e2463663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7f95e08d7a93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7f95e091c7cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7f95e08d6818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f95e09110e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7f95e08ef706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7f95e08bc9e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f95e08e52e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f95dfddd4de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7f95dfb77b43 - <unknown>
  44:     0x7f95dfc09a00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-example-desugared.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-example-desugared.rs#next stdout ----

error in revision `next`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-example-desugared.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7f06b711caf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7f06b7189cc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7f06b71111d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7f06b711c901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7f06b711c901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7f06b711fad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7f06b711f7ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7f06b7c24555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f06b71201f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7f06b711ff69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7f06b711cfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7f06b711fc47 - rust_begin_unwind
  11:     0x7f06b70d62c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7f06b70d664c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7f06b79a1f59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7f06b80cdafa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7f06b809052b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7f06b80cd0f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7f06b9ab0bad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7f06b9806bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7f06b857e172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7f06b8571a43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7f06b999874f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7f06b9818029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7f06b849ff56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7f06b85630ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7f06b857d5ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7f06b9997588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7f06b97e3ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7f06b849ffb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f06b85634fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7f06b847cd8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7f06b84bf2b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7f06b7ce3820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7f06b9a9c25f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7f06b97b2663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7f06b7c26a93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7f06b7c6b7cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7f06b7c25818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f06b7c600e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7f06b7c3e706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7f06b7c0b9e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f06b7c342e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f06b712c4de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7f06b6ec6b43 - <unknown>
  44:     0x7f06b6f58a00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-example-desugared-manual.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-example-desugared-manual.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-example-desugared-manual.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-manual.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-example-desugared-manual.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7fc89ae5eaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7fc89aecbcc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7fc89ae531d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7fc89ae5e901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7fc89ae5e901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7fc89ae61ad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7fc89ae617ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7fc89b966555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc89ae621f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7fc89ae61f69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7fc89ae5efc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7fc89ae61c47 - rust_begin_unwind
  11:     0x7fc89ae182c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7fc89ae1864c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7fc89b6e3f59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7fc89be0fafa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7fc89bdd252b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7fc89be0f0f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7fc89d7f2bad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7fc89d548bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7fc89c2c0172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7fc89c2b3a43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7fc89d6da74f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7fc89d55a029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7fc89c1e1f56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fc89c2a50ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7fc89c2bf5ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7fc89d6d9588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7fc89d525ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7fc89c1e1fb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7fc89c2a54fd - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  30:     0x7fc89c1bed8f - <rustc_session[a4ebdab9eb09e67]::session::Session>::track_errors::<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}, ()>
  31:     0x7fc89c2012b3 - rustc_hir_analysis[79f806c9d37913a5]::check_crate
  32:     0x7fc89ba25820 - rustc_interface[91dc61729b253c88]::passes::analysis
  33:     0x7fc89d7de25f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::analysis, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  34:     0x7fc89d4f4663 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::analysis
  35:     0x7fc89b968a93 - <rustc_middle[252e67918677dbe9]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  36:     0x7fc89b9ad7cd - <rustc_interface[91dc61729b253c88]::interface::Compiler>::enter::<rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}::{closure#2}, core[9e03d261c2ba5097]::result::Result<core[9e03d261c2ba5097]::option::Option<rustc_interface[91dc61729b253c88]::queries::Linker>, rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  37:     0x7fc89b967818 - rustc_span[c187fc62240f9843]::with_source_map::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fc89b9a20e7 - <scoped_tls[6880822cd1553859]::ScopedKey<rustc_span[c187fc62240f9843]::SessionGlobals>>::set::<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  39:     0x7fc89b980706 - std[8b383dae0e31b9a0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>
  40:     0x7fc89b94d9e6 - std[8b383dae0e31b9a0]::panicking::try::<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7fc89b9762e5 - <<std[8b383dae0e31b9a0]::thread::Builder>::spawn_unchecked_<rustc_interface[91dc61729b253c88]::util::run_in_thread_pool_with_globals<rustc_interface[91dc61729b253c88]::interface::run_compiler<core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>, rustc_driver_impl[ac5107511d4a04e5]::run_compiler::{closure#1}>::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9e03d261c2ba5097]::result::Result<(), rustc_span[c187fc62240f9843]::ErrorGuaranteed>>::{closure#1} as core[9e03d261c2ba5097]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7fc89ae6e4de - std::sys::unix::thread::Thread::new::thread_start::h1b01a25bbf2de1e8
  43:     0x7fc89ac08b43 - <unknown>
  44:     0x7fc89ac9aa00 - <unknown>
  45:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (c8637e127 2023-03-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [param_env] computing normalized predicates of `MyTrait::foo`
#1 [check_well_formed] checking that `MyTrait::foo` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in top-level module
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/async-generics.rs#next stdout ----
---- [ui] tests/ui/async-await/in-trait/async-generics.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/async-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-generics.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-generics.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left matches right)`
  left: `(Opaque, AssocTy)`,
 right: `(ty::Opaque, DefKind::OpaqueTy) | (ty::Projection, DefKind::AssocTy) |
(ty::Opaque | ty::Projection, DefKind::ImplTraitPlaceholder)`', /checkout/compiler/rustc_middle/src/ty/context.rs:2053:9
   0:     0x7fea115b9af5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hef80340d9054dc4e
   1:     0x7fea11626cc8 - core::fmt::write::hfea731a26ffae99e
   2:     0x7fea115ae1d1 - std::io::Write::write_fmt::h0e2aae86a47890c8
   3:     0x7fea115b9901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   3:     0x7fea115b9901 - std::sys_common::backtrace::print::h54265f5ab5b4e45e
   4:     0x7fea115bcad4 - std::panicking::default_hook::{{closure}}::h5c337ab39301a393
   5:     0x7fea115bc7ba - std::panicking::default_hook::h0556d344ccc7e285
   6:     0x7fea120c1555 - rustc_driver_impl[ac5107511d4a04e5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fea115bd1f1 - std::panicking::rust_panic_with_hook::h0b47f41a79672a20
   8:     0x7fea115bcf69 - std::panicking::begin_panic_handler::{{closure}}::h4dc7917cede905d5
   9:     0x7fea115b9fc6 - std::sys_common::backtrace::__rust_end_short_backtrace::h167f045bf825509d
  10:     0x7fea115bcc47 - rust_begin_unwind
  11:     0x7fea115732c3 - core::panicking::panic_fmt::h6072f65c56c4a57d
  12:     0x7fea1157364c - core::panicking::assert_failed_inner::h362eb40a55c80f35
  13:     0x7fea11e3ef59 - core[9e03d261c2ba5097]::panicking::assert_matches_failed::<(rustc_type_ir[9b3d07b6b7fd9317]::sty::AliasKind, rustc_hir[555958620a3cdc33]::def::DefKind)>
  14:     0x7fea1256aafa - <rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitor<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_ty
  15:     0x7fea1252d52b - <rustc_middle[252e67918677dbe9]::ty::sty::FnSig as rustc_type_ir[9b3d07b6b7fd9317]::visit::TypeVisitable<rustc_middle[252e67918677dbe9]::ty::context::TyCtxt>>::visit_with::<rustc_ty_utils[4159105716f74e06]::ty::ImplTraitInTraitFinder>
  16:     0x7fea1256a0f0 - rustc_ty_utils[4159105716f74e06]::ty::param_env
  17:     0x7fea13f4dbad - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::param_env, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  18:     0x7fea13ca3bc3 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::param_env
  19:     0x7fea12a1b172 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_associated_item
  20:     0x7fea12a0ea43 - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_well_formed
  21:     0x7fea13e3574f - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_well_formed, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  22:     0x7fea13cb5029 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_well_formed
  23:     0x7fea1293cf56 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>::{closure#0}::{closure#0}>>
  24:     0x7fea12a000ad - rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in::<&[rustc_hir[555958620a3cdc33]::hir::TraitItemId], <rustc_middle[252e67918677dbe9]::hir::ModuleItems>::par_trait_items<rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf::{closure#2}>::{closure#0}>
  25:     0x7fea12a1a5ee - rustc_hir_analysis[79f806c9d37913a5]::check::wfcheck::check_mod_type_wf
  26:     0x7fea13e34588 - rustc_query_system[145bf2923e362f7d]::query::plumbing::try_execute_query::<rustc_query_impl[f84167ef151ec43c]::queries::check_mod_type_wf, rustc_query_impl[f84167ef151ec43c]::plumbing::QueryCtxt>
  27:     0x7fea13c80ab9 - <rustc_query_impl[f84167ef151ec43c]::Queries as rustc_middle[252e67918677dbe9]::ty::query::QueryEngine>::check_mod_type_wf
  28:     0x7fea1293cfb6 - std[8b383dae0e31b9a0]::panicking::try::<(), core[9e03d261c2ba5097]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8f38858d9fef1e71]::sync::par_for_each_in<&[rustc_hir[555958620a3cdc33]::hir_id::OwnerId], <rustc_middle[252e67918677dbe9]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[79f806c9d37913a5]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
