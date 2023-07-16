plain
........................................................................................  5544/14764
........................................................................................  5632/14764
........................................................................................  5720/14764
........................................................................................  5808/14764
....F......F......................i..........iiii.......................................  5896/14764
........................................................................................  6072/14764
........................................................................................  6160/14764
........................................................................................  6248/14764
........................................................................................  6336/14764
---
........................................................................................ 11880/14764
.........ii.ii.iiii.i..i.iiiiiiiiiii.................................................... 11968/14764
........................................................................................ 12056/14764
........................................................................................ 12144/14764
..........................F.....F.............................F......................... 12232/14764
........................................................................................ 12408/14764
........................................................................................ 12496/14764
........................................................................................ 12584/14764
........................................................................................ 12672/14764
---
diff of stderr:

2   --> $DIR/issue-91762.rs:24:15
3    |
4 LL |         ret = <Self::Base as Functor>::fmap(arg);
-    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `fmap`
+    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `U` declared on the associated function `fmap`
7 help: consider specifying the generic arguments
8    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-91762/issue-91762.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-91762.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generic-associated-types/bugs/issue-91762.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-91762" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-91762/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/generic-associated-types/bugs/issue-91762.rs:24:15
   |
   |
LL |         ret = <Self::Base as Functor>::fmap(arg);
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `U` declared on the associated function `fmap`
help: consider specifying the generic arguments
   |
   |
LL |         ret = <Self::Base as Functor>::fmap::<T, U>(arg);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---
normalized stderr:
error[E0282]: type annotations needed
  --> $DIR/issue-109905.rs:13:5
   |
LL |     <() as Trait<'static, _>>::m(());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the trait `Trait`
error[E0282]: type annotations needed
  --> $DIR/issue-109905.rs:20:5
   |
   |
LL |     Trait::<'static, _>::m(());
   |     ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the trait `Trait`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.

---
To only update this specific test, also pass `--test-args inference/need_type_info/issue-109905.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/inference/need_type_info/issue-109905.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/issue-109905" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/issue-109905/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/inference/need_type_info/issue-109905.rs:13:5
   |
   |
LL |     <() as Trait<'static, _>>::m(());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the trait `Trait`
error[E0282]: type annotations needed
  --> fake-test-src-base/inference/need_type_info/issue-109905.rs:20:5
   |
   |
LL |     Trait::<'static, _>::m(());
   |     ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the trait `Trait`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/inference/question-mark-type-infer.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/inference/question-mark-type-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/question-mark-type-infer" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/question-mark-type-infer/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
   0:     0x7f6a1aec0385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7f6a1af2ea68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7f6a1aeb45d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7f6a1aec0191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   3:     0x7f6a1aec0191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7f6a1aec3654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7f6a1aec3320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7f6a1b9ec765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f6a1aec3db1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7f6a1aec3b29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7f6a1aec0856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7f6a1aec37c7 - rust_begin_unwind
  11:     0x7f6a1ae78453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7f6a1ae785c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7f6a1e6baea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7f6a1e37e7ab - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  15:     0x7f6a1e37d63c - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  16:     0x7f6a1e36c25e - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  17:     0x7f6a1bf3ee84 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::report_ambiguity_errors
  18:     0x7f6a1c0bb573 - rustc_hir_typeck[18e04f1942980580]::typeck
  19:     0x7f6a1d719c1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  20:     0x7f6a1d916bfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  21:     0x7f6a1d545e19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  22:     0x7f6a1c112cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  23:     0x7f6a1bff7084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  24:     0x7f6a1c0b8853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  25:     0x7f6a1d712724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  26:     0x7f6a1d8b3078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  27:     0x7f6a1d545573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  28:     0x7f6a1c1811d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  29:     0x7f6a1c2fbfd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  30:     0x7f6a1baabcf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  31:     0x7f6a1d719e14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  32:     0x7f6a1d918a80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  33:     0x7f6a1d5126f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  34:     0x7f6a1b9f64f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  35:     0x7f6a1b9f58c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  36:     0x7f6a1ba23cc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  37:     0x7f6a1b9eda28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f6a1b9fc640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  39:     0x7f6a1ba377d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f6a1b9f2727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f6a1aed065e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  42:     0x7f6a1ac67b43 - <unknown>
  43:     0x7f6a1acf9a00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `g`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/issues/issue-69455.rs stdout ----
---- [ui] tests/ui/issues/issue-69455.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-69455.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69455/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
   0:     0x7fa97cfb4385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7fa97d022a68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7fa97cfa85d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7fa97cfb4191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   3:     0x7fa97cfb4191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7fa97cfb7654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7fa97cfb7320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7fa97dae0765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa97cfb7db1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7fa97cfb7b29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7fa97cfb4856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7fa97cfb77c7 - rust_begin_unwind
  11:     0x7fa97cf6c453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7fa97cf6c5c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7fa9807aeea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7fa980472264 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  15:     0x7fa98047163c - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  16:     0x7fa98046025e - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  17:     0x7fa97e032e84 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::report_ambiguity_errors
  18:     0x7fa97e1af573 - rustc_hir_typeck[18e04f1942980580]::typeck
  19:     0x7fa97f80dc1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  20:     0x7fa97fa0abfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  21:     0x7fa97f639e19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  22:     0x7fa97e206cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  23:     0x7fa97e0eb084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  24:     0x7fa97e1ac853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  25:     0x7fa97f806724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  26:     0x7fa97f9a7078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  27:     0x7fa97f639573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  28:     0x7fa97e2751d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  29:     0x7fa97e3effd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  30:     0x7fa97db9fcf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  31:     0x7fa97f80de14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  32:     0x7fa97fa0ca80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  33:     0x7fa97f6066f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  34:     0x7fa97daea4f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  35:     0x7fa97dae98c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  36:     0x7fa97db17cc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  37:     0x7fa97dae1a28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fa97daf0640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  39:     0x7fa97db2b7d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7fa97dae6727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fa97cfc465e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  42:     0x7fa97cd5bb43 - <unknown>
  43:     0x7fa97cdeda00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/span/issue-42234-unknown-receiver-type.rs#full stdout ----
---- [ui] tests/ui/span/issue-42234-unknown-receiver-type.rs#full stdout ----

error in revision `full`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/span/issue-42234-unknown-receiver-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type.full/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/span/issue-42234-unknown-receiver-type.rs:9:24
   |
   |
LL |     let x: Option<_> = None; //~ ERROR type annotations needed
   |                        ^^^^ cannot infer type of the type parameter `T` declared on the enum `Option`
LL |     x.unwrap().method_that_could_exist_on_some_type();
   |     ---------- type must be known at this point
help: consider specifying the generic argument
   |
   |
LL |     let x: Option<_> = None::<T>; //~ ERROR type annotations needed

thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
stack backtrace:
   0:     0x7f003ba41385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   0:     0x7f003ba41385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7f003baafa68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7f003ba355d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7f003ba41191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7f003ba44654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7f003ba44320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7f003c56d765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f003ba44db1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7f003ba44b29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7f003ba41856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7f003ba447c7 - rust_begin_unwind
  11:     0x7f003b9f9453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7f003b9f95c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7f003f23bea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7f003ca60998 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::structurally_resolved_type
  15:     0x7f003cab2ba6 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_kind
  16:     0x7f003ca4a88a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  17:     0x7f003cab1bd2 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  18:     0x7f003ca6d7e4 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_block_with_expected
  19:     0x7f003cab26ba - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f003ca4a88a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f003cab1bd2 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f003ca4c73a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7f003cc17055 - rustc_hir_typeck[18e04f1942980580]::check::check_fn
  24:     0x7f003cc3be1e - rustc_hir_typeck[18e04f1942980580]::typeck
  25:     0x7f003e29ac1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  26:     0x7f003e497bfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  27:     0x7f003e0c6e19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  28:     0x7f003cc93cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f003cb78084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  30:     0x7f003cc39853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  31:     0x7f003e293724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  32:     0x7f003e434078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  33:     0x7f003e0c6573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  34:     0x7f003cd021d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  35:     0x7f003ce7cfd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  36:     0x7f003c62ccf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  37:     0x7f003e29ae14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  38:     0x7f003e499a80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  39:     0x7f003e0936f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  40:     0x7f003c5774f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  41:     0x7f003c5768c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  42:     0x7f003c5a4cc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  43:     0x7f003c56ea28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  44:     0x7f003c57d640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  45:     0x7f003c5b87d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7f003c573727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f003ba5165e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  48:     0x7f003b7e8b43 - <unknown>
  49:     0x7f003b87aa00 - <unknown>
  50:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `courier_to_des_moines_and_points_west`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/span/issue-42234-unknown-receiver-type.rs#generic_arg stdout ----

error in revision `generic_arg`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/span/issue-42234-unknown-receiver-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "generic_arg" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type.generic_arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type.generic_arg/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/span/issue-42234-unknown-receiver-type.rs:9:24
   |
   |
LL |     let x: Option<_> = None; //~ ERROR type annotations needed
   |                        ^^^^ cannot infer type of the type parameter `T` declared on the enum `Option`
LL |     x.unwrap().method_that_could_exist_on_some_type();
   |     ---------- type must be known at this point
help: consider specifying the generic argument
   |
   |
LL |     let x: Option<_> = None::<T>; //~ ERROR type annotations needed

thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
stack backtrace:
   0:     0x7f51e52c2385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   0:     0x7f51e52c2385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7f51e5330a68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7f51e52b65d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7f51e52c2191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7f51e52c5654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7f51e52c5320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7f51e5dee765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f51e52c5db1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7f51e52c5b29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7f51e52c2856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7f51e52c57c7 - rust_begin_unwind
  11:     0x7f51e527a453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7f51e527a5c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7f51e8abcea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7f51e62e1998 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::structurally_resolved_type
  15:     0x7f51e6333ba6 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_kind
  16:     0x7f51e62cb88a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  17:     0x7f51e6332bd2 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  18:     0x7f51e62ee7e4 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_block_with_expected
  19:     0x7f51e63336ba - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_kind
  20:     0x7f51e62cb88a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  21:     0x7f51e6332bd2 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  22:     0x7f51e62cd73a - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_return_expr
  23:     0x7f51e6498055 - rustc_hir_typeck[18e04f1942980580]::check::check_fn
  24:     0x7f51e64bce1e - rustc_hir_typeck[18e04f1942980580]::typeck
  25:     0x7f51e7b1bc1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  26:     0x7f51e7d18bfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  27:     0x7f51e7947e19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  28:     0x7f51e6514cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7f51e63f9084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  30:     0x7f51e64ba853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  31:     0x7f51e7b14724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  32:     0x7f51e7cb5078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  33:     0x7f51e7947573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  34:     0x7f51e65831d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  35:     0x7f51e66fdfd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  36:     0x7f51e5eadcf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  37:     0x7f51e7b1be14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  38:     0x7f51e7d1aa80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  39:     0x7f51e79146f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  40:     0x7f51e5df84f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  41:     0x7f51e5df78c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  42:     0x7f51e5e25cc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  43:     0x7f51e5defa28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  44:     0x7f51e5dfe640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  45:     0x7f51e5e397d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7f51e5df4727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f51e52d265e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  48:     0x7f51e5069b43 - <unknown>
  49:     0x7f51e50fba00 - <unknown>
  50:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `courier_to_des_moines_and_points_west`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/span/type-annotations-needed-expr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/span/type-annotations-needed-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/type-annotations-needed-expr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/type-annotations-needed-expr/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
   0:     0x7f5a659c3385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7f5a65a31a68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7f5a659b75d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7f5a659c3191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   3:     0x7f5a659c3191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7f5a659c6654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7f5a659c6320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7f5a664ef765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5a659c6db1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7f5a659c6b29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7f5a659c3856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7f5a659c67c7 - rust_begin_unwind
  11:     0x7f5a6597b453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7f5a6597b5c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7f5a691bdea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7f5a669e2998 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::structurally_resolved_type
  15:     0x7f5a66b976ed - <rustc_hir_typeck[18e04f1942980580]::cast::CastCheck>::check
  16:     0x7f5a669e51fe - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::check_casts
  17:     0x7f5a66bbe233 - rustc_hir_typeck[18e04f1942980580]::typeck
  18:     0x7f5a6821cc1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  19:     0x7f5a68419bfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  20:     0x7f5a68048e19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  21:     0x7f5a66c15cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  22:     0x7f5a66afa084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  23:     0x7f5a66bbb853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  24:     0x7f5a68215724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  25:     0x7f5a683b6078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  26:     0x7f5a68048573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  27:     0x7f5a66c841d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  28:     0x7f5a66dfefd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  29:     0x7f5a665aecf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  30:     0x7f5a6821ce14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  31:     0x7f5a6841ba80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  32:     0x7f5a680156f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  33:     0x7f5a664f94f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  34:     0x7f5a664f88c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  35:     0x7f5a66526cc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  36:     0x7f5a664f0a28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f5a664ff640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  38:     0x7f5a6653a7d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  39:     0x7f5a664f5727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f5a659d365e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  41:     0x7f5a6576ab43 - <unknown>
  42:     0x7f5a657fca00 - <unknown>
  43:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/type-inference/sort_by_key.rs stdout ----
---- [ui] tests/ui/type-inference/sort_by_key.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-inference/sort_by_key.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:471:33
   0:     0x7f2fb7407385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha26301c97dd3c7b6
   1:     0x7f2fb7475a68 - core::fmt::write::hd4405a484f8d2b0e
   2:     0x7f2fb73fb5d1 - std::io::Write::write_fmt::h79b643c728183e75
   3:     0x7f2fb7407191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   3:     0x7f2fb7407191 - std::sys_common::backtrace::print::h75d6d69e76e84575
   4:     0x7f2fb740a654 - std::panicking::default_hook::{{closure}}::h907590d64c320cbe
   5:     0x7f2fb740a320 - std::panicking::default_hook::h5cc23f5b772b0040
   6:     0x7f2fb7f33765 - rustc_driver_impl[926040d66ce48da0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2fb740adb1 - std::panicking::rust_panic_with_hook::hd6a5cc7eda97ab46
   8:     0x7f2fb740ab29 - std::panicking::begin_panic_handler::{{closure}}::h0127bf63174592f6
   9:     0x7f2fb7407856 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d4b8c0c6c0951eb
  10:     0x7f2fb740a7c7 - rust_begin_unwind
  11:     0x7f2fb73bf453 - core::panicking::panic_fmt::h7a76edfb98f8c829
  12:     0x7f2fb73bf5c2 - core::panicking::panic_bounds_check::h7a894eab6ba0950e
  13:     0x7f2fbac01ea7 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt>::emit_inference_failure_err
  14:     0x7f2fba8c6067 - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  15:     0x7f2fba8c463c - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
  16:     0x7f2fba8b325e - <rustc_infer[d449ec949a8b55e4]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[736046bec9636255]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors
  17:     0x7f2fb8485e84 - <rustc_hir_typeck[18e04f1942980580]::fn_ctxt::FnCtxt>::report_ambiguity_errors
  18:     0x7f2fb8602573 - rustc_hir_typeck[18e04f1942980580]::typeck
  19:     0x7f2fb9c60c1a - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#1}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>::{closure#0}, &rustc_middle[5118e2e6888e6285]::ty::typeck_results::TypeckResults>
  20:     0x7f2fb9e5dbfd - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  21:     0x7f2fb9a8ce19 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck
  22:     0x7f2fb8659cb7 - std[f47b6d85fab4e19a]::panicking::try::<(), core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  23:     0x7f2fb853e084 - rustc_data_structures[b4e5888c4b434a3f]::sync::par_for_each_in::<&[rustc_span[435d7097f2ebeacd]::def_id::LocalDefId], <rustc_middle[5118e2e6888e6285]::hir::map::Map>::par_body_owners<rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies::{closure#0}>::{closure#0}>
  24:     0x7f2fb85ff853 - rustc_hir_typeck[18e04f1942980580]::typeck_item_bodies
  25:     0x7f2fb9c59724 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, ()>::{closure#0}, ()>
  26:     0x7f2fb9dfa078 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::typeck_item_bodies, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  27:     0x7f2fb9a8c573 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::typeck_item_bodies
  28:     0x7f2fb86c81d7 - <rustc_session[2d9fb6fb1c63ec00]::session::Session>::time::<(), rustc_hir_analysis[797593d4ae6501d2]::check_crate::{closure#7}>
  29:     0x7f2fb8842fd3 - rustc_hir_analysis[797593d4ae6501d2]::check_crate
  30:     0x7f2fb7ff2cf0 - rustc_interface[d5b4b2df5e2dbc25]::passes::analysis
  31:     0x7f2fb9c60e14 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<rustc_query_system[8ae825ea6696e5c7]::query::plumbing::execute_job_non_incr<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  32:     0x7f2fb9e5fa80 - rustc_query_system[8ae825ea6696e5c7]::query::plumbing::try_execute_query::<rustc_query_impl[e82ab1bbd2ac74f6]::queries::analysis, rustc_query_impl[e82ab1bbd2ac74f6]::plumbing::QueryCtxt>
  33:     0x7f2fb9a596f3 - <rustc_query_impl[e82ab1bbd2ac74f6]::Queries as rustc_middle[5118e2e6888e6285]::ty::query::QueryEngine>::analysis
  34:     0x7f2fb7f3d4f4 - <std[f47b6d85fab4e19a]::thread::local::LocalKey<core[cd45d6bd2f93f64f]::cell::Cell<*const ()>>>::with::<rustc_middle[5118e2e6888e6285]::ty::context::tls::enter_context<<rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>::enter<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  35:     0x7f2fb7f3c8c1 - <rustc_interface[d5b4b2df5e2dbc25]::queries::QueryResult<&rustc_middle[5118e2e6888e6285]::ty::context::GlobalCtxt>>::enter::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  36:     0x7f2fb7f6acc1 - <rustc_interface[d5b4b2df5e2dbc25]::interface::Compiler>::enter::<rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}::{closure#2}, core[cd45d6bd2f93f64f]::result::Result<core[cd45d6bd2f93f64f]::option::Option<rustc_interface[d5b4b2df5e2dbc25]::queries::Linker>, rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  37:     0x7f2fb7f34a28 - rustc_span[435d7097f2ebeacd]::set_source_map::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f2fb7f43640 - std[f47b6d85fab4e19a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>
  39:     0x7f2fb7f7e7d8 - std[f47b6d85fab4e19a]::panicking::try::<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, core[cd45d6bd2f93f64f]::panic::unwind_safe::AssertUnwindSafe<<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f2fb7f39727 - <<std[f47b6d85fab4e19a]::thread::Builder>::spawn_unchecked_<rustc_interface[d5b4b2df5e2dbc25]::util::run_in_thread_pool_with_globals<rustc_interface[d5b4b2df5e2dbc25]::interface::run_compiler<core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>, rustc_driver_impl[926040d66ce48da0]::run_compiler::{closure#1}>::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[cd45d6bd2f93f64f]::result::Result<(), rustc_span[435d7097f2ebeacd]::ErrorGuaranteed>>::{closure#1} as core[cd45d6bd2f93f64f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f2fb741765e - std::sys::unix::thread::Thread::new::thread_start::h66595e25b8dc623a
  42:     0x7f2fb71aeb43 - <unknown>
  43:     0x7f2fb7240a00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (25847272b 2023-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



