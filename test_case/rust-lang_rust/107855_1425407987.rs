plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ca7cab66e2f838703fe12775fbabb05754421ad8)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 12848/14421
........................................................................................ 12936/14421
........................................................................................ 13024/14421
........................................................................................ 13112/14421
................F..F.................................................................... 13200/14421
........................................................................................ 13376/14421
........................................................................................ 13464/14421
..........................i............................................................. 13552/14421
........................................................................................ 13640/14421
---
.....................................................iii................................ 14344/14421
.............................................................................
failures:

---- [ui] tests/ui/traits/new-solver/param-candidate-doesnt-shadow-project.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/param-candidate-doesnt-shadow-project.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/param-candidate-doesnt-shadow-project" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/param-candidate-doesnt-shadow-project/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'not implemented', compiler/rustc_trait_selection/src/solve/project_goals.rs:206:51
stack backtrace:
   0:     0x7fc89236e535 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcebd85217be2ba7d
   1:     0x7fc8923dcfc8 - core::fmt::write::h24979f3ab61e0a2f
   2:     0x7fc892360431 - std::io::Write::write_fmt::hca048a438a86e1c0
   3:     0x7fc89236e341 - std::sys_common::backtrace::print::hdd5ad85fd49db4cc
   4:     0x7fc892371724 - std::panicking::default_hook::{{closure}}::h370691b92e5ad937
   5:     0x7fc8923713ea - std::panicking::default_hook::headf9998e7d18ba8
   6:     0x7fc892df63a2 - rustc_driver_impl[6e30b8bbb77d9f03]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc892371e91 - std::panicking::rust_panic_with_hook::h5693f79f5373b64c
   8:     0x7fc892371bba - std::panicking::begin_panic_handler::{{closure}}::h0cc64bcba9387fff
   9:     0x7fc89236ea54 - std::sys_common::backtrace::__rust_end_short_backtrace::h2f8a00806142eddc
  10:     0x7fc8923718a2 - rust_begin_unwind
  11:     0x7fc892320fe3 - core::panicking::panic_fmt::h58f0cf077b0bf65e
  12:     0x7fc89232107d - core::panicking::panic::hc59e12152224a360
  13:     0x7fc895856223 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  14:     0x7fc895852e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  15:     0x7fc89585509c - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  16:     0x7fc895852e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  17:     0x7fc89585367f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_all
  18:     0x7fc895853968 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_all_and_make_canonical_response
  19:     0x7fc8958fe4e3 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt>::probe::<core[af6c8a382f66e887]::result::Result<rustc_middle[c5dab125a153c7f2]::infer::canonical::Canonical<rustc_trait_selection[9f90e8cf5bc13697]::solve::Response>, rustc_middle[c5dab125a153c7f2]::traits::query::NoSolution>, <rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate as rustc_trait_selection[9f90e8cf5bc13697]::solve::assembly::GoalKind>::consider_impl_candidate::{closure#1}>
  20:     0x7fc895869a2c - <rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate as rustc_trait_selection[9f90e8cf5bc13697]::solve::assembly::GoalKind>::consider_impl_candidate
  21:     0x7fc89591d533 - <rustc_middle[c5dab125a153c7f2]::ty::context::TyCtxt>::for_each_relevant_impl::<<rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::assemble_impl_candidates<rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate>::{closure#0}>
  22:     0x7fc89584fbd6 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::assemble_and_evaluate_candidates::<rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate>
  23:     0x7fc895854b4c - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  24:     0x7fc895852e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  25:     0x7fc895911191 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt as rustc_trait_selection[9f90e8cf5bc13697]::solve::InferCtxtEvalExt>::evaluate_root_goal
  26:     0x7fc89575362f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::fulfill::FulfillmentCtxt as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine>::select_where_possible
  27:     0x7fc895782d6e - <dyn rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngineExt>::select_all_or_error
  28:     0x7fc895754d98 - <rustc_trait_selection[9f90e8cf5bc13697]::traits::engine::ObligationCtxt>::select_all_or_error
  29:     0x7fc893792819 - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_well_formed
  30:     0x7fc894bc4d98 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  31:     0x7fc894c9bfe0 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  32:     0x7fc894962140 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_well_formed
  33:     0x7fc89373eb00 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  34:     0x7fc8936ae016 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  35:     0x7fc8937ab02d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  36:     0x7fc89379d8ea - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf
  37:     0x7fc894bc42e9 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  38:     0x7fc894c9bf10 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  39:     0x7fc894936700 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_mod_type_wf
  40:     0x7fc89373ec30 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  41:     0x7fc8936ae036 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7fc8937ab19d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7fc89367c8a3 - <rustc_session[ed0247fe66e7b3c9]::session::Session>::track_errors::<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}, ()>
  44:     0x7fc8936c5f23 - rustc_hir_analysis[1dc240b991e498ef]::check_crate
  45:     0x7fc892f60154 - rustc_interface[f2b282fe288b7d47]::passes::analysis
  46:     0x7fc894c1ddc2 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  47:     0x7fc894cd33e1 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  48:     0x7fc89490e4ea - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::analysis
  49:     0x7fc892e81fa4 - <rustc_interface[f2b282fe288b7d47]::passes::QueryContext>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  50:     0x7fc892e61f5a - <rustc_interface[f2b282fe288b7d47]::interface::Compiler>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}, core[af6c8a382f66e887]::result::Result<core[af6c8a382f66e887]::option::Option<rustc_interface[f2b282fe288b7d47]::queries::Linker>, rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  51:     0x7fc892e80b17 - rustc_span[ddd0838a71d9e227]::with_source_map::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  52:     0x7fc892e62b56 - <scoped_tls[14b94ea6ef6c14cc]::ScopedKey<rustc_span[ddd0838a71d9e227]::SessionGlobals>>::set::<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  53:     0x7fc892e08249 - std[5787a4af135f819]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  54:     0x7fc892e59c76 - std[5787a4af135f819]::panicking::try::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  55:     0x7fc892e02485 - <<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1} as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7fc89237ed5e - std::sys::unix::thread::Thread::new::thread_start::hd63dba83c84c0356
  57:     0x7fc892111b43 - <unknown>
  58:     0x7fc8921a3a00 - <unknown>
  59:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (587548b72 2023-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=next
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `<impl at fake-test-src-base/traits/new-solver/param-candidate-doesnt-shadow-project.rs:14:1: 14:18>` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
thread 'rustc' panicked at 'not implemented', compiler/rustc_trait_selection/src/solve/trait_goals.rs:535:52
stack backtrace:
stack backtrace:
   0:     0x7fc89236e535 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcebd85217be2ba7d
   1:     0x7fc8923dcfc8 - core::fmt::write::h24979f3ab61e0a2f
   2:     0x7fc892360431 - std::io::Write::write_fmt::hca048a438a86e1c0
   3:     0x7fc89236e341 - std::sys_common::backtrace::print::hdd5ad85fd49db4cc
   4:     0x7fc892371724 - std::panicking::default_hook::{{closure}}::h370691b92e5ad937
   5:     0x7fc8923713ea - std::panicking::default_hook::headf9998e7d18ba8
   6:     0x7fc892df63a2 - rustc_driver_impl[6e30b8bbb77d9f03]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc892371e91 - std::panicking::rust_panic_with_hook::h5693f79f5373b64c
   8:     0x7fc892371bba - std::panicking::begin_panic_handler::{{closure}}::h0cc64bcba9387fff
   9:     0x7fc89236ea54 - std::sys_common::backtrace::__rust_end_short_backtrace::h2f8a00806142eddc
  10:     0x7fc8923718a2 - rust_begin_unwind
  11:     0x7fc892320fe3 - core::panicking::panic_fmt::h58f0cf077b0bf65e
  12:     0x7fc89232107d - core::panicking::panic::hc59e12152224a360
  13:     0x7fc89585618b - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  14:     0x7fc895852e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  15:     0x7fc895911191 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt as rustc_trait_selection[9f90e8cf5bc13697]::solve::InferCtxtEvalExt>::evaluate_root_goal
  16:     0x7fc89575362f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::fulfill::FulfillmentCtxt as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine>::select_where_possible
  17:     0x7fc895782d6e - <dyn rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngineExt>::select_all_or_error
  18:     0x7fc895754d98 - <rustc_trait_selection[9f90e8cf5bc13697]::traits::engine::ObligationCtxt>::select_all_or_error
  19:     0x7fc89379b05f - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_item_fn
  20:     0x7fc893791062 - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_well_formed
  21:     0x7fc894bc4d98 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  22:     0x7fc894c9bfe0 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  23:     0x7fc894962140 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_well_formed
  24:     0x7fc89373eb00 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  25:     0x7fc8936ae016 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7fc8937ab02d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  27:     0x7fc89379d8ea - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf
  28:     0x7fc894bc42e9 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  29:     0x7fc894c9bf10 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  30:     0x7fc894936700 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_mod_type_wf
  31:     0x7fc89373ec30 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  32:     0x7fc8936ae036 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7fc8937ab19d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7fc89367c8a3 - <rustc_session[ed0247fe66e7b3c9]::session::Session>::track_errors::<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}, ()>
  35:     0x7fc8936c5f23 - rustc_hir_analysis[1dc240b991e498ef]::check_crate
  36:     0x7fc892f60154 - rustc_interface[f2b282fe288b7d47]::passes::analysis
  37:     0x7fc894c1ddc2 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  38:     0x7fc894cd33e1 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  39:     0x7fc89490e4ea - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::analysis
  40:     0x7fc892e81fa4 - <rustc_interface[f2b282fe288b7d47]::passes::QueryContext>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  41:     0x7fc892e61f5a - <rustc_interface[f2b282fe288b7d47]::interface::Compiler>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}, core[af6c8a382f66e887]::result::Result<core[af6c8a382f66e887]::option::Option<rustc_interface[f2b282fe288b7d47]::queries::Linker>, rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  42:     0x7fc892e80b17 - rustc_span[ddd0838a71d9e227]::with_source_map::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  43:     0x7fc892e62b56 - <scoped_tls[14b94ea6ef6c14cc]::ScopedKey<rustc_span[ddd0838a71d9e227]::SessionGlobals>>::set::<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  44:     0x7fc892e08249 - std[5787a4af135f819]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  45:     0x7fc892e59c76 - std[5787a4af135f819]::panicking::try::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fc892e02485 - <<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1} as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fc89237ed5e - std::sys::unix::thread::Thread::new::thread_start::hd63dba83c84c0356
  48:     0x7fc892111b43 - <unknown>
  49:     0x7fc8921a3a00 - <unknown>
  50:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (587548b72 2023-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=next
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `require_bar` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
thread 'rustc' panicked at 'not implemented', compiler/rustc_trait_selection/src/solve/trait_goals.rs:535:52
stack backtrace:
stack backtrace:
   0:     0x7fc89236e535 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcebd85217be2ba7d
   1:     0x7fc8923dcfc8 - core::fmt::write::h24979f3ab61e0a2f
   2:     0x7fc892360431 - std::io::Write::write_fmt::hca048a438a86e1c0
   3:     0x7fc89236e341 - std::sys_common::backtrace::print::hdd5ad85fd49db4cc
   4:     0x7fc892371724 - std::panicking::default_hook::{{closure}}::h370691b92e5ad937
   5:     0x7fc8923713ea - std::panicking::default_hook::headf9998e7d18ba8
   6:     0x7fc892df63a2 - rustc_driver_impl[6e30b8bbb77d9f03]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc892371e91 - std::panicking::rust_panic_with_hook::h5693f79f5373b64c
   8:     0x7fc892371bba - std::panicking::begin_panic_handler::{{closure}}::h0cc64bcba9387fff
   9:     0x7fc89236ea54 - std::sys_common::backtrace::__rust_end_short_backtrace::h2f8a00806142eddc
  10:     0x7fc8923718a2 - rust_begin_unwind
  11:     0x7fc892320fe3 - core::panicking::panic_fmt::h58f0cf077b0bf65e
  12:     0x7fc89232107d - core::panicking::panic::hc59e12152224a360
  13:     0x7fc89585618b - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  14:     0x7fc895852e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  15:     0x7fc895911191 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt as rustc_trait_selection[9f90e8cf5bc13697]::solve::InferCtxtEvalExt>::evaluate_root_goal
  16:     0x7fc89575362f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::fulfill::FulfillmentCtxt as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine>::select_where_possible
  17:     0x7fc895782d6e - <dyn rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngineExt>::select_all_or_error
  18:     0x7fc895754d98 - <rustc_trait_selection[9f90e8cf5bc13697]::traits::engine::ObligationCtxt>::select_all_or_error
  19:     0x7fc89379b05f - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_item_fn
  20:     0x7fc893791062 - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_well_formed
  21:     0x7fc894bc4d98 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  22:     0x7fc894c9bfe0 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_well_formed, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  23:     0x7fc894962140 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_well_formed
  24:     0x7fc89373eb00 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  25:     0x7fc8936ae016 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26:     0x7fc8937ab02d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir::ItemId], <rustc_middle[c5dab125a153c7f2]::hir::ModuleItems>::par_items<rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>
  27:     0x7fc89379d8ea - rustc_hir_analysis[1dc240b991e498ef]::check::wfcheck::check_mod_type_wf
  28:     0x7fc894bc42e9 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  29:     0x7fc894c9bf10 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::check_mod_type_wf, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  30:     0x7fc894936700 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::check_mod_type_wf
  31:     0x7fc89373ec30 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  32:     0x7fc8936ae036 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7fc8937ab19d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_hir[10ef51d240d9420f]::hir_id::OwnerId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  34:     0x7fc89367c8a3 - <rustc_session[ed0247fe66e7b3c9]::session::Session>::track_errors::<rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#5}, ()>
  35:     0x7fc8936c5f23 - rustc_hir_analysis[1dc240b991e498ef]::check_crate
  36:     0x7fc892f60154 - rustc_interface[f2b282fe288b7d47]::passes::analysis
  37:     0x7fc894c1ddc2 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  38:     0x7fc894cd33e1 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  39:     0x7fc89490e4ea - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::analysis
  40:     0x7fc892e81fa4 - <rustc_interface[f2b282fe288b7d47]::passes::QueryContext>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  41:     0x7fc892e61f5a - <rustc_interface[f2b282fe288b7d47]::interface::Compiler>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}, core[af6c8a382f66e887]::result::Result<core[af6c8a382f66e887]::option::Option<rustc_interface[f2b282fe288b7d47]::queries::Linker>, rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  42:     0x7fc892e80b17 - rustc_span[ddd0838a71d9e227]::with_source_map::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  43:     0x7fc892e62b56 - <scoped_tls[14b94ea6ef6c14cc]::ScopedKey<rustc_span[ddd0838a71d9e227]::SessionGlobals>>::set::<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  44:     0x7fc892e08249 - std[5787a4af135f819]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  45:     0x7fc892e59c76 - std[5787a4af135f819]::panicking::try::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7fc892e02485 - <<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1} as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fc89237ed5e - std::sys::unix::thread::Thread::new::thread_start::hd63dba83c84c0356
  48:     0x7fc892111b43 - <unknown>
  49:     0x7fc8921a3a00 - <unknown>
  50:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (587548b72 2023-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=next
query stack during panic:
query stack during panic:
#0 [check_well_formed] checking that `foo` is well-formed
#1 [check_mod_type_wf] checking that types are well-formed in top-level module
#2 [analysis] running analysis passes on this crate
------------------------------------------



---- [ui] tests/ui/traits/new-solver/two-projection-param-candidates-are-ambiguous.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/two-projection-param-candidates-are-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/two-projection-param-candidates-are-ambiguous" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/two-projection-param-candidates-are-ambiguous/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'not implemented', compiler/rustc_trait_selection/src/solve/project_goals.rs:206:51
stack backtrace:
   0:     0x7fdbcd093535 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcebd85217be2ba7d
   1:     0x7fdbcd101fc8 - core::fmt::write::h24979f3ab61e0a2f
   2:     0x7fdbcd085431 - std::io::Write::write_fmt::hca048a438a86e1c0
   3:     0x7fdbcd093341 - std::sys_common::backtrace::print::hdd5ad85fd49db4cc
   4:     0x7fdbcd096724 - std::panicking::default_hook::{{closure}}::h370691b92e5ad937
   5:     0x7fdbcd0963ea - std::panicking::default_hook::headf9998e7d18ba8
   6:     0x7fdbcdb1b3a2 - rustc_driver_impl[6e30b8bbb77d9f03]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fdbcd096e91 - std::panicking::rust_panic_with_hook::h5693f79f5373b64c
   8:     0x7fdbcd096bba - std::panicking::begin_panic_handler::{{closure}}::h0cc64bcba9387fff
   9:     0x7fdbcd093a54 - std::sys_common::backtrace::__rust_end_short_backtrace::h2f8a00806142eddc
  10:     0x7fdbcd0968a2 - rust_begin_unwind
  11:     0x7fdbcd045fe3 - core::panicking::panic_fmt::h58f0cf077b0bf65e
  12:     0x7fdbcd04607d - core::panicking::panic::hc59e12152224a360
  13:     0x7fdbd057b223 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  14:     0x7fdbd0577e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  15:     0x7fdbd057a09c - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  16:     0x7fdbd0577e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  17:     0x7fdbd057867f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_all
  18:     0x7fdbd0578968 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_all_and_make_canonical_response
  19:     0x7fdbd06234e3 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt>::probe::<core[af6c8a382f66e887]::result::Result<rustc_middle[c5dab125a153c7f2]::infer::canonical::Canonical<rustc_trait_selection[9f90e8cf5bc13697]::solve::Response>, rustc_middle[c5dab125a153c7f2]::traits::query::NoSolution>, <rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate as rustc_trait_selection[9f90e8cf5bc13697]::solve::assembly::GoalKind>::consider_impl_candidate::{closure#1}>
  20:     0x7fdbd058ea2c - <rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate as rustc_trait_selection[9f90e8cf5bc13697]::solve::assembly::GoalKind>::consider_impl_candidate
  21:     0x7fdbd0642533 - <rustc_middle[c5dab125a153c7f2]::ty::context::TyCtxt>::for_each_relevant_impl::<<rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::assemble_impl_candidates<rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate>::{closure#0}>
  22:     0x7fdbd0574bd6 - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::assemble_and_evaluate_candidates::<rustc_middle[c5dab125a153c7f2]::ty::TraitPredicate>
  23:     0x7fdbd0579b4c - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_canonical_goal
  24:     0x7fdbd0577e6f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::EvalCtxt>::evaluate_goal
  25:     0x7fdbd0636191 - <rustc_infer[b4682ed52b5ad081]::infer::InferCtxt as rustc_trait_selection[9f90e8cf5bc13697]::solve::InferCtxtEvalExt>::evaluate_root_goal
  26:     0x7fdbd047862f - <rustc_trait_selection[9f90e8cf5bc13697]::solve::fulfill::FulfillmentCtxt as rustc_infer[b4682ed52b5ad081]::traits::engine::TraitEngine>::select_where_possible
  27:     0x7fdbce078d25 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_argument_types
  28:     0x7fdbce044e73 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::confirm_builtin_call
  29:     0x7fdbce043c61 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_call
  30:     0x7fdbce0c529f - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_kind
  31:     0x7fdbce05dba1 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  32:     0x7fdbce0c40c2 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  33:     0x7fdbce080b27 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_stmt
  34:     0x7fdbce081097 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_block_with_expected
  35:     0x7fdbce0c55d1 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_kind
  36:     0x7fdbce05dba1 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:     0x7fdbce0c40c2 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  38:     0x7fdbce05fb09 - <rustc_hir_typeck[deebc37c5693f3ec]::fn_ctxt::FnCtxt>::check_return_expr
  39:     0x7fdbce244b6e - rustc_hir_typeck[deebc37c5693f3ec]::check::check_fn
  40:     0x7fdbce240356 - rustc_hir_typeck[deebc37c5693f3ec]::typeck
  41:     0x7fdbcf940ae3 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::typeck, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  42:     0x7fdbcf9f8150 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::typeck, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  43:     0x7fdbcf65d4a0 - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::typeck
  44:     0x7fdbce1baa42 - <core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_span[ddd0838a71d9e227]::def_id::LocalDefId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[deebc37c5693f3ec]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once
  45:     0x7fdbce1286c9 - std[5787a4af135f819]::panicking::try::<(), core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in<&[rustc_span[ddd0838a71d9e227]::def_id::LocalDefId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[deebc37c5693f3ec]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7fdbce1fc93d - rustc_data_structures[df6360e0f93dc2c7]::sync::par_for_each_in::<&[rustc_span[ddd0838a71d9e227]::def_id::LocalDefId], <rustc_middle[c5dab125a153c7f2]::hir::map::Map>::par_body_owners<rustc_hir_typeck[deebc37c5693f3ec]::typeck_item_bodies::{closure#0}>::{closure#0}>
  47:     0x7fdbce23d1ca - rustc_hir_typeck[deebc37c5693f3ec]::typeck_item_bodies
  48:     0x7fdbcf8f7e11 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::typeck_item_bodies, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  49:     0x7fdbcf9c84ef - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::typeck_item_bodies, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  50:     0x7fdbcf65cd3a - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::typeck_item_bodies
  51:     0x7fdbce39dc61 - <rustc_session[ed0247fe66e7b3c9]::session::Session>::time::<(), rustc_hir_analysis[1dc240b991e498ef]::check_crate::{closure#7}>
  52:     0x7fdbce3eafb3 - rustc_hir_analysis[1dc240b991e498ef]::check_crate
  53:     0x7fdbcdc85154 - rustc_interface[f2b282fe288b7d47]::passes::analysis
  54:     0x7fdbcf942dc2 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::try_execute_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt>
  55:     0x7fdbcf9f83e1 - rustc_query_system[9f1d70d45ae086cd]::query::plumbing::get_query::<rustc_query_impl[fe30d081a5fe2355]::queries::analysis, rustc_query_impl[fe30d081a5fe2355]::plumbing::QueryCtxt, rustc_middle[c5dab125a153c7f2]::dep_graph::dep_node::DepKind>
  56:     0x7fdbcf6334ea - <rustc_query_impl[fe30d081a5fe2355]::Queries as rustc_middle[c5dab125a153c7f2]::ty::query::QueryEngine>::analysis
  57:     0x7fdbcdba6fa4 - <rustc_interface[f2b282fe288b7d47]::passes::QueryContext>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  58:     0x7fdbcdb86f5a - <rustc_interface[f2b282fe288b7d47]::interface::Compiler>::enter::<rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}::{closure#2}, core[af6c8a382f66e887]::result::Result<core[af6c8a382f66e887]::option::Option<rustc_interface[f2b282fe288b7d47]::queries::Linker>, rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  59:     0x7fdbcdba5b17 - rustc_span[ddd0838a71d9e227]::with_source_map::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  60:     0x7fdbcdb87b56 - <scoped_tls[14b94ea6ef6c14cc]::ScopedKey<rustc_span[ddd0838a71d9e227]::SessionGlobals>>::set::<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  61:     0x7fdbcdb2d249 - std[5787a4af135f819]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>
  62:     0x7fdbcdb7ec76 - std[5787a4af135f819]::panicking::try::<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, core[af6c8a382f66e887]::panic::unwind_safe::AssertUnwindSafe<<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  63:     0x7fdbcdb27485 - <<std[5787a4af135f819]::thread::Builder>::spawn_unchecked_<rustc_interface[f2b282fe288b7d47]::util::run_in_thread_pool_with_globals<rustc_interface[f2b282fe288b7d47]::interface::run_compiler<core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>, rustc_driver_impl[6e30b8bbb77d9f03]::run_compiler::{closure#1}>::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[af6c8a382f66e887]::result::Result<(), rustc_errors[599bb097f6d0ae09]::ErrorGuaranteed>>::{closure#1} as core[af6c8a382f66e887]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  64:     0x7fdbcd0a3d5e - std::sys::unix::thread::Thread::new::thread_start::hd63dba83c84c0356
  65:     0x7fdbcce36b43 - <unknown>
  66:     0x7fdbccec8a00 - <unknown>
  67:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (587548b72 2023-02-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trait-solver=next
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



