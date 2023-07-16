plain
........................................................................................ 1672/13149
.........................i.....ii....................................................... 1760/13149
........................................................................................ 1848/13149
...............................................................................i........ 1936/13149
..................................................................................F..... 2024/13149
......F.......................................................F......................... 2112/13149
..............F......................................................................... 2200/13149
........................................................................................ 2376/13149
........................................................................................ 2376/13149
......................................................F.....................FF.FF.F..... 2464/13149
.F....................................................F.....F.........F...F...F......... 2552/13149
.................................................................F.F.................... 2640/13149
........................................................................................ 2816/13149
........................................................................................ 2904/13149
................................................i....................................... 2992/13149
..............i......................................................................... 3080/13149
---
........................................................................................ 4752/13149
........................................................................................ 4840/13149
........................................................................................ 4928/13149
........................................................................................ 5016/13149
.........................................................i..F........................... 5104/13149
........................................................................................ 5280/13149
........................................................................................ 5368/13149
........................................................................................ 5456/13149
........................................................................................ 5544/13149
---
---- [ui] src/test/ui/associated-consts/defaults-not-assumed-fail.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/defaults-not-assumed-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/defaults-not-assumed-fail/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/defaults/default-param-wf-concrete.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/default-param-wf-concrete.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/default-param-wf-concrete" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/default-param-wf-concrete/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/defaults/wfness.rs stdout ----
diff of stderr:


- error[E0080]: evaluation of constant value failed
-   --> $DIR/wfness.rs:1:33
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
- LL | struct Ooopsies<const N: u8 = { u8::MAX + 1 }>;
-    |                                 ^^^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
- 
7 error[E0277]: the trait bound `(): Trait<2_u8>` is not satisfied
8   --> $DIR/wfness.rs:6:47


25 LL | struct WhereClause<const N: u8 = 2> where (): Trait<N>;
26    |                                               ^^^^^^^^ required by this bound in `WhereClause`
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
29 
- Some errors have detailed explanations: E0080, E0277.
---
To only update this specific test, also pass `--test-args const-generics/defaults/wfness.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/wfness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/wfness" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/wfness/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): Trait<2_u8>` is not satisfied
   |
   |
LL | struct WhereClause<const N: u8 = 2> where (): Trait<N>;
   |                                               ^^^^^^^^ the trait `Trait<2_u8>` is not implemented for `()`
   |
   = help: the trait `Trait<3_u8>` is implemented for `()`

error[E0277]: the trait bound `(): Trait<1_u8>` is not satisfied
   |
   |
LL | fn foo() -> DependentDefaultWfness {
   |             ^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait<1_u8>` is not implemented for `()`
   |
   = help: the trait `Trait<3_u8>` is implemented for `()`
note: required by a bound in `WhereClause`
   |
   |
LL | struct WhereClause<const N: u8 = 2> where (): Trait<N>;
   |                                               ^^^^^^^^ required by this bound in `WhereClause`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/from-sig-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/from-sig-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/from-sig-fail/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to add with overflow', compiler/rustc_lint/src/unused.rs:265:86
stack backtrace:
   0:     0x7f4296d30b3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d7c9e1ba06bd87b
   1:     0x7f4296d95698 - core::fmt::write::he08feb8100b31028
   2:     0x7f4296d21101 - std::io::Write::write_fmt::h7c5efb64bd53d36f
   3:     0x7f4296d339be - std::panicking::default_hook::{{closure}}::h17938c02e0a90f31
   4:     0x7f4296d336a6 - std::panicking::default_hook::h49a428688d4c8dc1
   5:     0x7f42976e3424 - rustc_driver[712e67d57f3bc2a8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4296d34152 - std::panicking::rust_panic_with_hook::h2e70bef347f77b04
   7:     0x7f4296d33f49 - std::panicking::begin_panic_handler::{{closure}}::hf0811acaef7bfd21
   8:     0x7f4296d31054 - std::sys_common::backtrace::__rust_end_short_backtrace::ha05feedd065c6229
   9:     0x7f4296d33c62 - rust_begin_unwind
  10:     0x7f4296ce6fe3 - core::panicking::panic_fmt::h81f652e94f1b336b
  11:     0x7f4296ce6ead - core::panicking::panic::h29aa4ba515fe6fa2
  12:     0x7f4299c4ca19 - <rustc_lint[a182597129adb239]::unused::UnusedResults as rustc_lint[a182597129adb239]::passes::LateLintPass>::check_stmt::check_must_use_ty
  13:     0x7f4299c4b7f4 - <rustc_lint[a182597129adb239]::unused::UnusedResults as rustc_lint[a182597129adb239]::passes::LateLintPass>::check_stmt
  14:     0x7f4299ba1415 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_stmt
  15:     0x7f4299bfd8ec - rustc_hir[edb7b7db02352211]::intravisit::walk_expr::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  16:     0x7f4299ba0aed - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_expr
  17:     0x7f4299b9fab3 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_nested_body
  18:     0x7f4299ba1a6e - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_fn
  19:     0x7f4299bfe85a - rustc_hir[edb7b7db02352211]::intravisit::walk_item::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  20:     0x7f4299b9db69 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_nested_item
  21:     0x7f4299bfc50c - rustc_hir[edb7b7db02352211]::intravisit::walk_mod::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  22:     0x7f4299ba401b - rustc_lint[a182597129adb239]::late::late_lint_mod::<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>
  23:     0x7f4299c174ed - rustc_lint[a182597129adb239]::lint_mod
  24:     0x7f429925a819 - rustc_query_system[49057b4878f46471]::query::plumbing::try_execute_query::<rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt, rustc_query_system[49057b4878f46471]::query::caches::DefaultCache<rustc_span[b910d2ba22e12cb7]::def_id::LocalDefId, ()>>
  25:     0x7f4299372e54 - rustc_query_system[49057b4878f46471]::query::plumbing::get_query::<rustc_query_impl[a30d2cde1c4ddd28]::queries::lint_mod, rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt>
  26:     0x7f4298e6d764 - <rustc_query_impl[a30d2cde1c4ddd28]::Queries as rustc_middle[3d3513d363aa4038]::ty::query::QueryEngine>::lint_mod
  27:     0x7f42978688fa - <rustc_middle[3d3513d363aa4038]::hir::map::Map>::for_each_module::<rustc_lint[a182597129adb239]::late::check_crate<rustc_lint[a182597129adb239]::BuiltinCombinedLateLintPass, rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  28:     0x7f4297857982 - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_lint[a182597129adb239]::late::check_crate<rustc_lint[a182597129adb239]::BuiltinCombinedLateLintPass, rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  29:     0x7f4297857a30 - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  30:     0x7f429786dc75 - std[7ae61a3a5a014c38]::panicking::try::<(), core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  31:     0x7f429782513c - <core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}> as core[c92e07e48b67722f]::ops::function::FnOnce<()>>::call_once
  32:     0x7f429786dd96 - std[7ae61a3a5a014c38]::panicking::try::<(), core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}>>
  33:     0x7f429785957c - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}>
  34:     0x7f42977f62ec - rustc_interface[952d6184cd9a7d76]::passes::analysis
  35:     0x7f4299294354 - rustc_query_system[49057b4878f46471]::query::plumbing::try_execute_query::<rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt, rustc_query_system[49057b4878f46471]::query::caches::DefaultCache<(), core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>>
  36:     0x7f4299372902 - rustc_query_system[49057b4878f46471]::query::plumbing::get_query::<rustc_query_impl[a30d2cde1c4ddd28]::queries::analysis, rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt>
  37:     0x7f4298e5640e - <rustc_query_impl[a30d2cde1c4ddd28]::Queries as rustc_middle[3d3513d363aa4038]::ty::query::QueryEngine>::analysis
  38:     0x7f4297737c35 - <rustc_interface[952d6184cd9a7d76]::passes::QueryContext>::enter::<rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  39:     0x7f42976eabec - <rustc_interface[952d6184cd9a7d76]::interface::Compiler>::enter::<rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}::{closure#2}, core[c92e07e48b67722f]::result::Result<core[c92e07e48b67722f]::option::Option<rustc_interface[952d6184cd9a7d76]::queries::Linker>, rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  40:     0x7f42976d2648 - rustc_span[b910d2ba22e12cb7]::with_source_map::<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_interface[952d6184cd9a7d76]::interface::create_compiler_and_run<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#1}>
  41:     0x7f42976ec849 - rustc_interface[952d6184cd9a7d76]::interface::create_compiler_and_run::<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>
  42:     0x7f42976cca3f - <scoped_tls[7a83e9abeb5ff29]::ScopedKey<rustc_span[b910d2ba22e12cb7]::SessionGlobals>>::set::<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  43:     0x7f42976d66f9 - std[7ae61a3a5a014c38]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[952d6184cd9a7d76]::util::run_in_thread_pool_with_globals<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  44:     0x7f429773f119 - <<std[7ae61a3a5a014c38]::thread::Builder>::spawn_unchecked_<rustc_interface[952d6184cd9a7d76]::util::run_in_thread_pool_with_globals<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#1} as core[c92e07e48b67722f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f4296d3f033 - std::sys::unix::thread::Thread::new::thread_start::h3a67dd6d5420440e
  46:     0x7f429128d609 - start_thread
  47:     0x7f4296ba0133 - clone
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (659f2d0f1 2022-07-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [lint_mod] linting top-level module
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/simple_fail.rs stdout ----
---- [ui] src/test/ui/const-generics/generic_const_exprs/simple_fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/simple_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/simple_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/simple_fail/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'attempt to add with overflow', compiler/rustc_lint/src/unused.rs:265:86
stack backtrace:
   0:     0x7fd8a1123b3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7d7c9e1ba06bd87b
   1:     0x7fd8a1188698 - core::fmt::write::he08feb8100b31028
   2:     0x7fd8a1114101 - std::io::Write::write_fmt::h7c5efb64bd53d36f
   3:     0x7fd8a11269be - std::panicking::default_hook::{{closure}}::h17938c02e0a90f31
   4:     0x7fd8a11266a6 - std::panicking::default_hook::h49a428688d4c8dc1
   5:     0x7fd8a1ad6424 - rustc_driver[712e67d57f3bc2a8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd8a1127152 - std::panicking::rust_panic_with_hook::h2e70bef347f77b04
   7:     0x7fd8a1126f49 - std::panicking::begin_panic_handler::{{closure}}::hf0811acaef7bfd21
   8:     0x7fd8a1124054 - std::sys_common::backtrace::__rust_end_short_backtrace::ha05feedd065c6229
   9:     0x7fd8a1126c62 - rust_begin_unwind
  10:     0x7fd8a10d9fe3 - core::panicking::panic_fmt::h81f652e94f1b336b
  11:     0x7fd8a10d9ead - core::panicking::panic::h29aa4ba515fe6fa2
  12:     0x7fd8a403fa19 - <rustc_lint[a182597129adb239]::unused::UnusedResults as rustc_lint[a182597129adb239]::passes::LateLintPass>::check_stmt::check_must_use_ty
  13:     0x7fd8a403e7f4 - <rustc_lint[a182597129adb239]::unused::UnusedResults as rustc_lint[a182597129adb239]::passes::LateLintPass>::check_stmt
  14:     0x7fd8a3f94415 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_stmt
  15:     0x7fd8a3ff08ec - rustc_hir[edb7b7db02352211]::intravisit::walk_expr::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  16:     0x7fd8a3f93aed - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_expr
  17:     0x7fd8a3f92ab3 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_nested_body
  18:     0x7fd8a3f94a6e - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_fn
  19:     0x7fd8a3ff185a - rustc_hir[edb7b7db02352211]::intravisit::walk_item::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  20:     0x7fd8a3f90b69 - <rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass> as rustc_hir[edb7b7db02352211]::intravisit::Visitor>::visit_nested_item
  21:     0x7fd8a3fef50c - rustc_hir[edb7b7db02352211]::intravisit::walk_mod::<rustc_lint[a182597129adb239]::late::LateContextAndPass<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>>
  22:     0x7fd8a3f9701b - rustc_lint[a182597129adb239]::late::late_lint_mod::<rustc_lint[a182597129adb239]::BuiltinCombinedModuleLateLintPass>
  23:     0x7fd8a400a4ed - rustc_lint[a182597129adb239]::lint_mod
  24:     0x7fd8a364d819 - rustc_query_system[49057b4878f46471]::query::plumbing::try_execute_query::<rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt, rustc_query_system[49057b4878f46471]::query::caches::DefaultCache<rustc_span[b910d2ba22e12cb7]::def_id::LocalDefId, ()>>
  25:     0x7fd8a3765e54 - rustc_query_system[49057b4878f46471]::query::plumbing::get_query::<rustc_query_impl[a30d2cde1c4ddd28]::queries::lint_mod, rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt>
  26:     0x7fd8a3260764 - <rustc_query_impl[a30d2cde1c4ddd28]::Queries as rustc_middle[3d3513d363aa4038]::ty::query::QueryEngine>::lint_mod
  27:     0x7fd8a1c5b8fa - <rustc_middle[3d3513d363aa4038]::hir::map::Map>::for_each_module::<rustc_lint[a182597129adb239]::late::check_crate<rustc_lint[a182597129adb239]::BuiltinCombinedLateLintPass, rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  28:     0x7fd8a1c4a982 - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_lint[a182597129adb239]::late::check_crate<rustc_lint[a182597129adb239]::BuiltinCombinedLateLintPass, rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  29:     0x7fd8a1c4aa30 - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  30:     0x7fd8a1c60c75 - std[7ae61a3a5a014c38]::panicking::try::<(), core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  31:     0x7fd8a1c1813c - <core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}> as core[c92e07e48b67722f]::ops::function::FnOnce<()>>::call_once
  32:     0x7fd8a1c60d96 - std[7ae61a3a5a014c38]::panicking::try::<(), core[c92e07e48b67722f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}::{closure#1}>>
  33:     0x7fd8a1c4c57c - <rustc_session[6b00c1fac21832ac]::session::Session>::time::<(), rustc_interface[952d6184cd9a7d76]::passes::analysis::{closure#5}>
  34:     0x7fd8a1be92ec - rustc_interface[952d6184cd9a7d76]::passes::analysis
  35:     0x7fd8a3687354 - rustc_query_system[49057b4878f46471]::query::plumbing::try_execute_query::<rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt, rustc_query_system[49057b4878f46471]::query::caches::DefaultCache<(), core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>>
  36:     0x7fd8a3765902 - rustc_query_system[49057b4878f46471]::query::plumbing::get_query::<rustc_query_impl[a30d2cde1c4ddd28]::queries::analysis, rustc_query_impl[a30d2cde1c4ddd28]::plumbing::QueryCtxt>
  37:     0x7fd8a324940e - <rustc_query_impl[a30d2cde1c4ddd28]::Queries as rustc_middle[3d3513d363aa4038]::ty::query::QueryEngine>::analysis
  38:     0x7fd8a1b2ac35 - <rustc_interface[952d6184cd9a7d76]::passes::QueryContext>::enter::<rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  39:     0x7fd8a1addbec - <rustc_interface[952d6184cd9a7d76]::interface::Compiler>::enter::<rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}::{closure#2}, core[c92e07e48b67722f]::result::Result<core[c92e07e48b67722f]::option::Option<rustc_interface[952d6184cd9a7d76]::queries::Linker>, rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  40:     0x7fd8a1ac5648 - rustc_span[b910d2ba22e12cb7]::with_source_map::<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_interface[952d6184cd9a7d76]::interface::create_compiler_and_run<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#1}>
  41:     0x7fd8a1adf849 - rustc_interface[952d6184cd9a7d76]::interface::create_compiler_and_run::<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>
  42:     0x7fd8a1abfa3f - <scoped_tls[7a83e9abeb5ff29]::ScopedKey<rustc_span[b910d2ba22e12cb7]::SessionGlobals>>::set::<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  43:     0x7fd8a1ac96f9 - std[7ae61a3a5a014c38]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[952d6184cd9a7d76]::util::run_in_thread_pool_with_globals<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>
  44:     0x7fd8a1b32119 - <<std[7ae61a3a5a014c38]::thread::Builder>::spawn_unchecked_<rustc_interface[952d6184cd9a7d76]::util::run_in_thread_pool_with_globals<rustc_interface[952d6184cd9a7d76]::interface::run_compiler<core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>, rustc_driver[712e67d57f3bc2a8]::run_compiler::{closure#1}>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#0}, core[c92e07e48b67722f]::result::Result<(), rustc_errors[413f06be641ae388]::ErrorGuaranteed>>::{closure#1} as core[c92e07e48b67722f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7fd8a1132033 - std::sys::unix::thread::Thread::new::thread_start::h3a67dd6d5420440e
  46:     0x7fd89b680609 - start_thread
  47:     0x7fd8a0f93133 - clone
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (659f2d0f1 2022-07-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [lint_mod] linting top-level module
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/const-err-early.rs stdout ----
---- [ui] src/test/ui/consts/const-err-early.rs stdout ----
diff of stderr:

13    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
14 
15 error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:5:19
-    |
- LL | pub const B: u8 = 200u8 + 200u8;
-    | ---------------   ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:7:19
-    |
- LL | pub const C: u8 = 200u8 * 4;
-    | ---------------   ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:9:19
-    |
- LL | pub const D: u8 = 42u8 - (42u8 + 1);
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- 
- error: any use of this value will cause an error
43   --> $DIR/const-err-early.rs:11:19
44    |
45 LL | pub const E: u8 = [5u8][1];
48    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
49    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
50 
- error: aborting due to 5 previous errors
- error: aborting due to 5 previous errors
+ error: aborting due to 2 previous errors
52 
53 Future incompatibility report: Future breakage diagnostic:

56    |
56    |
57 LL | pub const A: i8 = -i8::MIN;
58    | ---------------   ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
- note: the lint level is defined here
-   --> $DIR/const-err-early.rs:1:9
-    |
- LL | #![deny(const_err)]
---
- Future breakage diagnostic:
- error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:5:19
-    |
- LL | pub const B: u8 = 200u8 + 200u8;
-    | ---------------   ^^^^^^^^^^^^^ attempt to compute `200_u8 + 200_u8`, which would overflow
- note: the lint level is defined here
-   --> $DIR/const-err-early.rs:1:9
-    |
- LL | #![deny(const_err)]
---
- Future breakage diagnostic:
- error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:7:19
-    |
- LL | pub const C: u8 = 200u8 * 4;
-    | ---------------   ^^^^^^^^^ attempt to compute `200_u8 * 4_u8`, which would overflow
- note: the lint level is defined here
-   --> $DIR/const-err-early.rs:1:9
-    |
- LL | #![deny(const_err)]
---
- Future breakage diagnostic:
- error: any use of this value will cause an error
-   --> $DIR/const-err-early.rs:9:19
-    |
- LL | pub const D: u8 = 42u8 - (42u8 + 1);
104    |
105 note: the lint level is defined here
106   --> $DIR/const-err-early.rs:1:9

---
To only update this specific test, also pass `--test-args consts/const-err-early.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err-early.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-early" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err-early/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-err-early.rs:3:19
   |
   |
LL | pub const A: i8 = -i8::MIN; //~ ERROR const_err
   | ---------------   ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-err-early.rs:1:9
   |
LL | #![deny(const_err)]
LL | #![deny(const_err)]
   |         ^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-err-early.rs:11:19
   |
LL | pub const E: u8 = [5u8][1]; //~ ERROR const_err
   | ---------------   ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

Future incompatibility report: Future breakage diagnostic:
  --> /checkout/src/test/ui/consts/const-err-early.rs:3:19
   |
   |
LL | pub const A: i8 = -i8::MIN; //~ ERROR const_err
   | ---------------   ^^^^^^^^ attempt to negate `i8::MIN`, which would overflow
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-err-early.rs:1:9
   |
LL | #![deny(const_err)]
---
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-err-early.rs:11:19
   |
LL | pub const E: u8 = [5u8][1]; //~ ERROR const_err
   | ---------------   ^^^^^^^^ index out of bounds: the length is 1 but the index is 1
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-err-early.rs:1:9
   |
LL | #![deny(const_err)]
---
---- [ui] src/test/ui/consts/const-eval/const-eval-overflow2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/const-eval-overflow2c.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow2c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2c" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2c/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/const-eval-overflow2b.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow2b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2b" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow2b/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/const-eval-overflow-3.rs stdout ----
diff of stderr:


- error[E0080]: evaluation of constant value failed
-   --> $DIR/const-eval-overflow-3.rs:18:11
+ error[E0308]: mismatched types
+   --> $DIR/const-eval-overflow-3.rs:18:7
3    |
4 LL |     = [0; (i8::MAX + 1) as usize];
-    |           ^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
+    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 128 elements, found one with 18446744073709551488 elements
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0080`.
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0308`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3/const-eval-overflow-3.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-3/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-3.rs:18:7
   |
   |
LL |     = [0; (i8::MAX + 1) as usize];
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 128 elements, found one with 18446744073709551488 elements
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
-   --> $DIR/const-eval-overflow-4.rs:11:13
+ error[E0308]: mismatched types
+   --> $DIR/const-eval-overflow-4.rs:13:7
3    |
- LL |     : [u32; (i8::MAX as i8 + 1i8) as usize]
-    |             ^^^^^^^^^^^^^^^^^^^^^ attempt to compute `i8::MAX + 1_i8`, which would overflow
+ LL |     = [0; (i8::MAX as usize) + 1];
+    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 18446744073709551488 elements, found one with 128 elements
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0080`.
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0308`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4/const-eval-overflow-4.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-4.rs:13:7
   |
   |
LL |     = [0; (i8::MAX as usize) + 1];
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 18446744073709551488 elements, found one with 128 elements
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/conditional_array_execution.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/conditional_array_execution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/conditional_array_execution/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/issue-50814.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/issue-43197.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-43197.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-43197/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/pub_const_err.rs stdout ----
diff of stderr:


- warning: any use of this value will cause an error
-   --> $DIR/pub_const_err.rs:6:20
-    |
- LL | pub const Z: u32 = 0 - 1;
-    |
- note: the lint level is defined here
-   --> $DIR/pub_const_err.rs:2:9
-    |
-    |
- LL | #![warn(const_err)]
-    |         ^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- warning: 1 warning emitted
- 
- Future incompatibility report: Future breakage diagnostic:
-   --> $DIR/pub_const_err.rs:6:20
-    |
-    |
- LL | pub const Z: u32 = 0 - 1;
-    |
- note: the lint level is defined here
-   --> $DIR/pub_const_err.rs:2:9
-    |
---
To only update this specific test, also pass `--test-args consts/const-eval/pub_const_err.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/pub_const_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/pub_const_err_bin.rs stdout ----
diff of stderr:


- warning: any use of this value will cause an error
-   --> $DIR/pub_const_err_bin.rs:4:20
-    |
- LL | pub const Z: u32 = 0 - 1;
-    |
- note: the lint level is defined here
-   --> $DIR/pub_const_err_bin.rs:2:9
-    |
-    |
- LL | #![warn(const_err)]
-    |         ^^^^^^^^^
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
- 
- warning: 1 warning emitted
- 
- Future incompatibility report: Future breakage diagnostic:
-   --> $DIR/pub_const_err_bin.rs:4:20
-    |
-    |
- LL | pub const Z: u32 = 0 - 1;
-    |
- note: the lint level is defined here
-   --> $DIR/pub_const_err_bin.rs:2:9
-    |
---
To only update this specific test, also pass `--test-args consts/const-eval/pub_const_err_bin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/pub_const_err_bin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err_bin" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/pub_const_err_bin/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-eval/shift_overflow.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/shift_overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/shift_overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/shift_overflow/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-len-underflow-separate-spans.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-len-underflow-separate-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-len-underflow-separate-spans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-len-underflow-separate-spans/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/consts/const-len-underflow-subspans.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-len-underflow-subspans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-len-underflow-subspans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-len-underflow-subspans/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/error-codes/E0080.rs stdout ----
diff of stderr:


1 error[E0080]: evaluation of constant value failed
-   --> $DIR/E0080.rs:2:9
-    |
- LL |     X = (1 << 500),
-    |         ^^^^^^^^^^ attempt to shift left by `500_i32`, which would overflow
- error[E0080]: evaluation of constant value failed
8   --> $DIR/E0080.rs:4:9
9    |
10 LL |     Y = (1 / 0)
10 LL |     Y = (1 / 0)

11    |         ^^^^^^^ attempt to divide `1_isize` by zero
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
14 
15 For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args error-codes/E0080.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0080.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0080" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0080/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0080.rs:4:9
   |
   |
LL |     Y = (1 / 0) //~ ERROR E0080
   |         ^^^^^^^ attempt to divide `1_isize` by zero
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---

12 LL |     foo::<i32>();
13    |     ^^^^^^^^^^^^
14 
- error[E0080]: evaluation of `bar::<0_usize>::{constant#0}` failed
-   --> $DIR/const-expr-generic-err.rs:9:13
-    |
- LL |     const { N - 1 }
-    |             ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
- 
- note: the above error was encountered while instantiating `fn bar::<0_usize>`
-   --> $DIR/const-expr-generic-err.rs:14:5
-    |
- LL |     bar::<0>();
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
28 
---
To only update this specific test, also pass `--test-args inline-const/const-expr-generic-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-expr-generic-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-generic-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-generic-err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `foo::<i32>::{constant#0}` failed
   |
   |
LL |     const { assert!(std::mem::size_of::<T>() == 0); } //~ ERROR E0080
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: std::mem::size_of::<T>() == 0', /checkout/src/test/ui/inline-const/const-expr-generic-err.rs:5:13
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)


note: the above error was encountered while instantiating `fn foo::<i32>`
   |
LL |     foo::<i32>();
   |     ^^^^^^^^^^^^

---
---- [ui] src/test/ui/treat-err-as-bug/err.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/associated-consts/defaults-not-assumed-fail.rs
