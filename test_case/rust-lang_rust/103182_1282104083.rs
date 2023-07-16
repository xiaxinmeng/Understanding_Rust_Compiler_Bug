plain
........................................................................................ 2728/13680
........................................................................................ 2816/13680
........................................................................................ 2904/13680
........................................................................................ 2992/13680
......................................................................F..F..........i... 3080/13680
......................F...........................i..........F......F................... 3168/13680
........................................................................................ 3344/13680
..........................iiiii......................................................... 3432/13680
........................................................................................ 3520/13680
........................................................................................ 3608/13680
---
........................................................................................ 12232/13680
........................................................................................ 12320/13680
........................................................................................ 12408/13680
........................................................................................ 12496/13680
............................................F........................................... 12584/13680
...............................................F........................................ 12672/13680
........................................F.......F....................................... 12760/13680
........................F......F.............i.......................................... 12848/13680
.............F.......................................................................... 12936/13680
........................................................................................ 13112/13680
........................................................................................ 13200/13680
........................................................................................ 13288/13680
........................................................................................ 13376/13680
---
---- [ui] src/test/ui/async-await/no-const-async.rs stdout ----
diff of stderr:

18    |
19 LL | pub const async fn x() {}
20    | ^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `x`...
+ note: ...which requires processing MIR for `x`...
23    |
23    |
24 LL | pub const async fn x() {}

31    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
32    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
33    = note: ...which again requires computing type of `x::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
36    |
36    |
37 LL | pub const async fn x() {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async/no-const-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/no-const-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-const-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async/auxiliary"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL | pub const async fn x() {}
   | ----^^^^^-^^^^^----------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this

error[E0391]: cycle detected when computing type of `x::{opaque#0}`
   |
   |
LL | pub const async fn x() {}
   |
   |
note: ...which requires borrow-checking `x`...
   |
   |
LL | pub const async fn x() {}
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `x`...
   |
   |
LL | pub const async fn x() {}
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `x`...
   |
   |
LL | pub const async fn x() {}
   | ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `x::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
   |
LL | pub const async fn x() {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/closures.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when building an abstract representation for test::{constant#0}
+ error[E0391]: cycle detected when building an abstract representation for `test::{constant#0}`
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
4 LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
14    |
14    |
15 LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
16    |                                   ^^^^^^^^^^^^^
-    = note: ...which again requires building an abstract representation for test::{constant#0}, completing the cycle
+    = note: ...which again requires building an abstract representation for `test::{constant#0}`, completing the cycle
18 note: cycle used when checking that `test` is well-formed
20    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/closures/closures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/closures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/closures/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when building an abstract representation for `test::{constant#0}`
   |
   |
LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
   |
   |
note: ...which requires building THIR for `test::{constant#0}`...
   |
   |
LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
   |                                   ^^^^^^^^^^^^^
note: ...which requires type-checking `test::{constant#0}`...
   |
   |
LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
   |                                   ^^^^^^^^^^^^^
   = note: ...which again requires building an abstract representation for `test::{constant#0}`, completing the cycle
note: cycle used when checking that `test` is well-formed
   |
   |
LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/const-eval-query-stack.rs stdout ----
diff of stderr:

8 #0 [eval_to_allocation_raw] const-evaluating + checking `X`
9 #1 [eval_to_const_value_raw] simplifying constant for the type system `X`
10 #2 [eval_to_const_value_raw] simplifying constant for the type system `X`
- #3 [lint_mod] linting top-level module
+ #3 [lint_mod] linting `top-level module`
12 #4 [analysis] running analysis passes on this crate
14 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/const-eval-query-stack.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-query-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:17:16
   |
   |
LL | const X: i32 = 1 / 0; //~ERROR constant
   |                ^^^^^ attempt to divide `1_i32` by zero

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1575:30
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   3: <rustc_errors::HandlerInner>::emit_diagnostic
   4: <rustc_errors::ErrorGuaranteed as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   5: <rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_error
   6: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
   7: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_allocation_raw, rustc_query_impl::plumbing::QueryCtxt>
   8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
   9: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  10: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  12: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::eval_to_const_value_raw, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  15: <rustc_middle::ty::query::TyCtxtEnsure>::const_eval_poly
  16: <rustc_lint::BuiltinCombinedModuleLateLintPass as rustc_lint::passes::LateLintPass>::check_item
  17: <rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_nested_item
  18: rustc_hir::intravisit::walk_mod::<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass>>
  19: rustc_lint::late::late_lint_mod::<rustc_lint::BuiltinCombinedModuleLateLintPass>
  21: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  22: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::lint_mod, rustc_query_impl::plumbing::QueryCtxt>
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::lint_mod
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::lint_mod
  24: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  25: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  26: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir_id::OwnerId], <rustc_middle::hir::map::Map>::par_for_each_module<rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  27: <rustc_session::session::Session>::time::<(), rustc_lint::late::check_crate<rustc_lint::BuiltinCombinedLateLintPass, rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  28: <rustc_session::session::Session>::time::<(), rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  29: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  30: <core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}> as core::ops::function::FnOnce<()>>::call_once
  31: std::panicking::try::<(), core::panic::unwind_safe::AssertUnwindSafe<rustc_interface::passes::analysis::{closure#5}::{closure#1}>>
  33: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  34: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  36: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (1a0af7e16 2022-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `X`
#1 [eval_to_const_value_raw] simplifying constant for the type system `X`
#2 [eval_to_const_value_raw] simplifying constant for the type system `X`
#3 [lint_mod] linting `top-level module`
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/consts/recursive-zst-static.rs#default stdout ----
---- [ui] src/test/ui/consts/recursive-zst-static.rs#default stdout ----
diff of stderr:

10 LL | static FOO: () = FOO;
11    |                  ^^^
12    = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
- note: cycle used when linting top-level module
+ note: cycle used when linting `top-level module`
15    |
15    |
16 LL | / static FOO: () = FOO;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.default/recursive-zst-static.default.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/recursive-zst-static.rs`

error in revision `default`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/recursive-zst-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |
   |
note: ...which requires const-evaluating + checking `FOO`...
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |                  ^^^
   = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
note: cycle used when linting `top-level module`
   |
   |
LL | / static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
LL | |
LL | | fn main() {
LL | |     FOO
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/consts/recursive-zst-static.rs#unleash stdout ----

10 LL | static FOO: () = FOO;
11    |                  ^^^
11    |                  ^^^
12    = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
- note: cycle used when linting top-level module
+ note: cycle used when linting `top-level module`
15    |
15    |
16 LL | / static FOO: () = FOO;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.unleash/recursive-zst-static.unleash.stderr
To only update this specific test, also pass `--test-args consts/recursive-zst-static.rs`


error in revision `unleash`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/recursive-zst-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unleash" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.unleash" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/recursive-zst-static.unleash/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |
   |
note: ...which requires const-evaluating + checking `FOO`...
   |
   |
LL | static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
   |                  ^^^
   = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
note: cycle used when linting `top-level module`
   |
   |
LL | / static FOO: () = FOO; //~ cycle detected when const-evaluating + checking `FOO`
LL | |
LL | | fn main() {
LL | |     FOO
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/consts/write-to-static-mut-in-static.rs stdout ----
diff of stderr:

16 LL | pub static mut C: u32 = unsafe { C = 1; 0 };
17    |                                  ^^^^^
18    = note: ...which again requires const-evaluating + checking `C`, completing the cycle
- note: cycle used when linting top-level module
+ note: cycle used when linting `top-level module`
21    |
21    |
22 LL | / pub static mut A: u32 = 0;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write-to-static-mut-in-static/write-to-static-mut-in-static.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/write-to-static-mut-in-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/write-to-static-mut-in-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write-to-static-mut-in-static" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write-to-static-mut-in-static/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL | pub static mut B: () = unsafe { A = 1; };
   |                                 ^^^^^ modifying a static's initial value from another static's initializer

error[E0391]: cycle detected when const-evaluating + checking `C`
   |
   |
LL | pub static mut C: u32 = unsafe { C = 1; 0 };
   |
   |
note: ...which requires const-evaluating + checking `C`...
   |
   |
LL | pub static mut C: u32 = unsafe { C = 1; 0 };
   |                                  ^^^^^
   = note: ...which again requires const-evaluating + checking `C`, completing the cycle
note: cycle used when linting `top-level module`
   |
   |
LL | / pub static mut A: u32 = 0;
LL | | pub static mut B: () = unsafe { A = 1; };
LL | | //~^ ERROR could not evaluate static initializer
LL | |
LL | |
LL | | fn main() {}
   | |____________^

---
diff of stderr:

5    |                       ^^^
6    |
7    = note: ...which immediately requires computing type of `Foo::X` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
10    |
10    |
11 LL | / trait Foo<X = Box<dyn Foo>> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-default-type-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::X`
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which immediately requires computing type of `Foo::X` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / trait Foo<X = Box<dyn Foo>> {
LL | |     //~^ ERROR cycle detected
LL | | }
LL | |
LL | | fn main() { }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs stdout ----
diff of stderr:

10 LL | trait Chromosome: Chromosome {
11    |                   ^^^^^^^^^^
12    = note: ...which again requires computing the super predicates of `Chromosome`, completing the cycle
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
15    |
15    |
16 LL | / trait Chromosome: Chromosome {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-supertrait-direct/cycle-trait-supertrait-direct.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-supertrait-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-supertrait-direct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-supertrait-direct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing the super predicates of `Chromosome`
   |
   |
LL | trait Chromosome: Chromosome {
   |
   |
note: ...which requires computing the super traits of `Chromosome`...
   |
   |
LL | trait Chromosome: Chromosome {
   |                   ^^^^^^^^^^
   = note: ...which again requires computing the super predicates of `Chromosome`, completing the cycle
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / trait Chromosome: Chromosome {
LL | |     //~^ ERROR cycle detected
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

9    |
10 LL | fn cycle1() -> impl Clone {
11    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle1`...
+ note: ...which requires processing MIR for `cycle1`...
14    |
14    |
15 LL | fn cycle1() -> impl Clone {
50    |
50    |
51 LL | fn cycle2() -> impl Clone {
52    | ^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `cycle2`...
+ note: ...which requires processing MIR for `cycle2`...
55    |
55    |
56 LL | fn cycle2() -> impl Clone {
82    |     ^^^^
82    |     ^^^^
83    = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
84    = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
87    |
88 LL | / use std::cell::Cell;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL |     send(cycle1().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
diff of stderr:

21 
22 query stack during panic:
23 #0 [type_of] computing type of `TransactionFuture::{opaque#0}`
- #1 [check_mod_item_types] checking item types in top-level module
+ #1 [check_mod_item_types] checking item types in `top-level module`
25 #2 [analysis] running analysis passes on this crate
27 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800/issue-86800.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-86800.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-86800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Z" "treat-err-as-bug=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/issues/issue-86800.rs:33:34
   |
   |
LL | type TransactionFuture<'__, O> = impl '__ + Future<Output = TransactionResult<O>>;
   |
   |
   = note: `TransactionFuture` must be used in combination with a concrete type within the same module

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1575:30
   0:     0x7fb3ca4c059e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4caf272822a7ee76
   1:     0x7fb3ca529b68 - core::fmt::write::h76612866d6d629a1
   2:     0x7fb3ca4b1c11 - std::io::Write::write_fmt::h15227653320da5ca
   3:     0x7fb3ca4c03a1 - std::sys_common::backtrace::print::h1c34f37e843d402d
   3:     0x7fb3ca4c03a1 - std::sys_common::backtrace::print::h1c34f37e843d402d
   4:     0x7fb3ca4c3524 - std::panicking::default_hook::{{closure}}::h356dcfad6e7cea6a
   5:     0x7fb3ca4c31e9 - std::panicking::default_hook::h42d7bf72e38a8c78
   6:     0x7fb3caea24d4 - rustc_driver[88f7deabacb91723]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb3ca4c3c74 - std::panicking::rust_panic_with_hook::heaa40f290428107b
   8:     0x7fb3ca4c3999 - std::panicking::begin_panic_handler::{{closure}}::h6f4effb8ebf70b52
   9:     0x7fb3ca4c0ad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h6b0621b1d5e7afb2
  10:     0x7fb3ca4c36a2 - rust_begin_unwind
  11:     0x7fb3ca474a93 - core::panicking::panic_fmt::h0c491680301afb82
  12:     0x7fb3cdd3f61d - <rustc_errors[dfbc4421de9b6586]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7fb3cdd3eae3 - <rustc_errors[dfbc4421de9b6586]::HandlerInner>::emit_diagnostic
  14:     0x7fb3cdd384a6 - <rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed as rustc_errors[dfbc4421de9b6586]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  15:     0x7fb3cb79a816 - <rustc_session[deb1ae4e3b1e936a]::parse::ParseSess>::emit_err::<rustc_hir_analysis[9ee0d7caf16dbe2b]::errors::UnconstrainedOpaqueType>
  16:     0x7fb3cb53fd83 - rustc_hir_analysis[9ee0d7caf16dbe2b]::collect::type_of::find_opaque_ty_constraints_for_tait
  17:     0x7fb3cb534af5 - rustc_hir_analysis[9ee0d7caf16dbe2b]::collect::type_of::type_of
  18:     0x7fb3ccb5f3e0 - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::try_execute_query::<rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt, rustc_query_system[ce7d8461fcf397ab]::query::caches::DefaultCache<rustc_span[db508b3a012ae292]::def_id::DefId, rustc_middle[28d1dc28c7d49088]::ty::Ty>>
  19:     0x7fb3ccc6baa7 - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::get_query::<rustc_query_impl[ee6959349ff9d4b9]::queries::type_of, rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt>
  20:     0x7fb3cc7adc55 - <rustc_query_impl[ee6959349ff9d4b9]::Queries as rustc_middle[28d1dc28c7d49088]::ty::query::QueryEngine>::type_of
  21:     0x7fb3cb6f160a - rustc_hir_analysis[9ee0d7caf16dbe2b]::check::check::check_opaque
  22:     0x7fb3cb6f4c1f - rustc_hir_analysis[9ee0d7caf16dbe2b]::check::check::check_item_type
  23:     0x7fb3cb70182a - rustc_hir_analysis[9ee0d7caf16dbe2b]::check::check::check_mod_item_types
  24:     0x7fb3ccb45222 - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::try_execute_query::<rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt, rustc_query_system[ce7d8461fcf397ab]::query::caches::DefaultCache<rustc_span[db508b3a012ae292]::def_id::LocalDefId, ()>>
  25:     0x7fb3ccc36a3a - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::get_query::<rustc_query_impl[ee6959349ff9d4b9]::queries::check_mod_item_types, rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt>
  26:     0x7fb3cc7d38b0 - <rustc_query_impl[ee6959349ff9d4b9]::Queries as rustc_middle[28d1dc28c7d49088]::ty::query::QueryEngine>::check_mod_item_types
  27:     0x7fb3cb51128a - <rustc_session[deb1ae4e3b1e936a]::session::Session>::time::<(), rustc_hir_analysis[9ee0d7caf16dbe2b]::check_crate::{closure#6}>
  28:     0x7fb3cb791221 - rustc_hir_analysis[9ee0d7caf16dbe2b]::check_crate
  29:     0x7fb3caff53f5 - rustc_interface[c05ac7fd3dd0ede3]::passes::analysis
  30:     0x7fb3ccb8501f - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::try_execute_query::<rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt, rustc_query_system[ce7d8461fcf397ab]::query::caches::DefaultCache<(), core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>>
  31:     0x7fb3ccc6bbbb - rustc_query_system[ce7d8461fcf397ab]::query::plumbing::get_query::<rustc_query_impl[ee6959349ff9d4b9]::queries::analysis, rustc_query_impl[ee6959349ff9d4b9]::plumbing::QueryCtxt>
  32:     0x7fb3cc7aeb3a - <rustc_query_impl[ee6959349ff9d4b9]::Queries as rustc_middle[28d1dc28c7d49088]::ty::query::QueryEngine>::analysis
  33:     0x7fb3caf10e9d - <rustc_interface[c05ac7fd3dd0ede3]::passes::QueryContext>::enter::<rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>
  34:     0x7fb3caea48da - <rustc_interface[c05ac7fd3dd0ede3]::interface::Compiler>::enter::<rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}::{closure#2}, core[2f96b64f6de70ac1]::result::Result<core[2f96b64f6de70ac1]::option::Option<rustc_interface[c05ac7fd3dd0ede3]::queries::Linker>, rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>
  35:     0x7fb3cae8f70e - rustc_span[db508b3a012ae292]::with_source_map::<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_interface[c05ac7fd3dd0ede3]::interface::create_compiler_and_run<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>::{closure#1}>
  36:     0x7fb3caeb8eb2 - rustc_interface[c05ac7fd3dd0ede3]::interface::create_compiler_and_run::<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>
  37:     0x7fb3caf151ef - <scoped_tls[365b1196530aa17]::ScopedKey<rustc_span[db508b3a012ae292]::SessionGlobals>>::set::<rustc_interface[c05ac7fd3dd0ede3]::interface::run_compiler<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>
  38:     0x7fb3caed270f - std[2a2319e5b4d061d4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c05ac7fd3dd0ede3]::util::run_in_thread_pool_with_globals<rustc_interface[c05ac7fd3dd0ede3]::interface::run_compiler<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>
  39:     0x7fb3cae91366 - std[2a2319e5b4d061d4]::panic::catch_unwind::<core[2f96b64f6de70ac1]::panic::unwind_safe::AssertUnwindSafe<<std[2a2319e5b4d061d4]::thread::Builder>::spawn_unchecked_<rustc_interface[c05ac7fd3dd0ede3]::util::run_in_thread_pool_with_globals<rustc_interface[c05ac7fd3dd0ede3]::interface::run_compiler<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>
  40:     0x7fb3caec74ea - <<std[2a2319e5b4d061d4]::thread::Builder>::spawn_unchecked_<rustc_interface[c05ac7fd3dd0ede3]::util::run_in_thread_pool_with_globals<rustc_interface[c05ac7fd3dd0ede3]::interface::run_compiler<core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>, rustc_driver[88f7deabacb91723]::run_compiler::{closure#1}>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>::{closure#0}, core[2f96b64f6de70ac1]::result::Result<(), rustc_errors[dfbc4421de9b6586]::ErrorGuaranteed>>::{closure#1} as core[2f96b64f6de70ac1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fb3ca4d04e5 - std::sys::unix::thread::Thread::new::thread_start::hc28639533aeaba4d
  42:     0x7fb3ca26ab43 - <unknown>
  43:     0x7fb3ca2fca00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (1a0af7e16 2022-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug=1
query stack during panic:
query stack during panic:
#0 [type_of] computing type of `TransactionFuture::{opaque#0}`
#1 [check_mod_item_types] checking item types in `top-level module`
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/infinite/infinite-trait-alias-recursion.rs stdout ----
---- [ui] src/test/ui/infinite/infinite-trait-alias-recursion.rs stdout ----
diff of stderr:

31    |            ^^
32    = note: ...which again requires computing the super predicates of `T1`, completing the cycle
33    = note: trait aliases cannot be recursive
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
36    |
37 LL | trait T1 = T2;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-trait-alias-recursion/infinite-trait-alias-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-trait-alias-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-trait-alias-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-trait-alias-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-trait-alias-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing the super predicates of `T1`
   |
LL | trait T1 = T2;
   | ^^^^^^^^
   |
   |
note: ...which requires computing the super traits of `T1`...
   |
LL | trait T1 = T2;
   |            ^^
   |            ^^
note: ...which requires computing the super predicates of `T2`...
   |
LL | trait T2 = T3;
   | ^^^^^^^^
   | ^^^^^^^^
note: ...which requires computing the super traits of `T2`...
   |
LL | trait T2 = T3;
   |            ^^
   |            ^^
note: ...which requires computing the super predicates of `T3`...
   |
   |
LL | trait T3 = T1 + T3;
   | ^^^^^^^^
note: ...which requires computing the super traits of `T3`...
   |
   |
LL | trait T3 = T1 + T3;
   |            ^^
   = note: ...which again requires computing the super predicates of `T1`, completing the cycle
   = note: trait aliases cannot be recursive
note: cycle used when collecting item types in `top-level module`
   |
LL | trait T1 = T2;
   | ^^^^^^^^^^^^^^

---

---- [ui] src/test/ui/infinite/infinite-vec-type-recursion.rs stdout ----
diff of stderr:

8    = note: type aliases cannot be recursive
9    = help: consider using a struct, enum, or union instead to break the cycle
10    = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
13    |
13    |
14 LL | / type X = Vec<X>;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/infinite-vec-type-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-vec-type-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-vec-type-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-vec-type-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when expanding type alias `X`
   |
   |
LL | type X = Vec<X>;
   |
   |
   = note: ...which immediately requires expanding type alias `X` again
   = note: type aliases cannot be recursive
   = help: consider using a struct, enum, or union instead to break the cycle
   = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / type X = Vec<X>;
LL | | //~^ ERROR cycle detected
LL | |
LL | | fn main() { let b: X = Vec::new(); }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/infinite/infinite-type-alias-mutual-recursion.rs stdout ----
diff of stderr:

18    = note: type aliases cannot be recursive
19    = help: consider using a struct, enum, or union instead to break the cycle
20    = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
23    |
23    |
24 LL | / type X1 = X2;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion/infinite-type-alias-mutual-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-type-alias-mutual-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-type-alias-mutual-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-type-alias-mutual-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when expanding type alias `X1`
   |
LL | type X1 = X2;
   |           ^^
   |
   |
note: ...which requires expanding type alias `X2`...
   |
LL | type X2 = X3;
   |           ^^
   |           ^^
note: ...which requires expanding type alias `X3`...
   |
LL | type X3 = X1;
   |           ^^
   |           ^^
   = note: ...which again requires expanding type alias `X1`, completing the cycle
   = note: type aliases cannot be recursive
   = help: consider using a struct, enum, or union instead to break the cycle
   = help: see <https://doc.rust-lang.org/reference/types.html#recursive-types> for more information
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / type X1 = X2;
LL | | //~^ ERROR cycle detected when expanding type alias `X1`
LL | | type X2 = X3;
LL | | type X3 = X1;
LL | |
LL | | fn main() {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-12511.rs stdout ----
diff of stderr:

20 LL | trait T2 : T1 {
21    |            ^^
22    = note: ...which again requires computing the super predicates of `T1`, completing the cycle
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
25    |
25    |
26 LL | / trait T1 : T2 {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12511/issue-12511.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12511.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12511.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12511" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12511/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing the super predicates of `T1`
   |
   |
LL | trait T1 : T2 {
   |
   |
note: ...which requires computing the super traits of `T1`...
   |
   |
LL | trait T1 : T2 {
   |            ^^
note: ...which requires computing the super predicates of `T2`...
   |
   |
LL | trait T2 : T1 {
   | ^^^^^^^^^^^^^
note: ...which requires computing the super traits of `T2`...
   |
   |
LL | trait T2 : T1 {
   |            ^^
   = note: ...which again requires computing the super predicates of `T1`, completing the cycle
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / trait T1 : T2 {
LL | | //~^ ERROR cycle detected
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
diff of stderr:

10 LL | type DefaultFoo = Foo;
11    |                   ^^^
12    = note: ...which again requires computing type of `Foo::T`, completing the cycle
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
15    |
16 LL | / #![allow(warnings)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-34373.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::T`
   |
   |
LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;  //~ ERROR cycle detected
   |
   |
note: ...which requires expanding type alias `DefaultFoo`...
   |
LL | type DefaultFoo = Foo;
   |                   ^^^
   |                   ^^^
   = note: ...which again requires computing type of `Foo::T`, completing the cycle
note: cycle used when collecting item types in `top-level module`
   |
LL | / #![allow(warnings)]
LL | |
LL | |
LL | | trait Trait<T> {
LL | |     fn foo(_: T) {}
LL | | fn main() {
LL | | }
   | |_^

---
---- [ui] src/test/ui/parser/fn-header-semantic-fail.rs stdout ----
diff of stderr:

199    |
200 LL |     const async unsafe extern "C" fn ff5() {}
201    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `main::ff5`...
+ note: ...which requires processing MIR for `main::ff5`...
203   --> $DIR/fn-header-semantic-fail.rs:12:5
204    |
205 LL |     const async unsafe extern "C" fn ff5() {}

212    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
213    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
214    = note: ...which again requires computing type of `main::ff5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
216   --> $DIR/fn-header-semantic-fail.rs:5:1
218 LL | / #![feature(const_extern_fn)]

235    |
235    |
236 LL |         const async unsafe extern "C" fn ft5() {}
237    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
+ note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
239   --> $DIR/fn-header-semantic-fail.rs:33:9
240    |
241 LL |         const async unsafe extern "C" fn ft5() {}

248    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
249    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
250    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
252   --> $DIR/fn-header-semantic-fail.rs:5:1
254 LL | / #![feature(const_extern_fn)]

271    |
271    |
272 LL |         const async unsafe extern "C" fn fi5() {}
273    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
+ note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
275   --> $DIR/fn-header-semantic-fail.rs:45:9
276    |
277 LL |         const async unsafe extern "C" fn fi5() {}

284    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
285    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
286    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
288   --> $DIR/fn-header-semantic-fail.rs:5:1
290 LL | / #![feature(const_extern_fn)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/fn-header-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:19:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:31:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
   |
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
   |
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
   |
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
   |
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
   |
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
   |
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:17:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:29:9
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
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
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
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |
   |
note: ...which requires borrow-checking `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::ff5`...
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::ff5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^


error[E0391]: cycle detected when computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |
   |
note: ...which requires borrow-checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^


error[E0391]: cycle detected when computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |
   |
note: ...which requires borrow-checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
   = note: ...which again requires computing type of `main::<impl at /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / #![feature(const_extern_fn)]
LL | |
LL | | fn main() {
LL | | fn main() {
LL | |     async fn ff1() {} // OK.
LL | |     }
LL | | }
   | |_^

---
diff of stderr:

10 LL | pub static FOO: u32 = FOO;
11    |                       ^^^
12    = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
- note: cycle used when linting top-level module
+ note: cycle used when linting `top-level module`
15    |
16 LL | / pub static FOO: u32 = FOO;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/recursive-static-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/recursive-static-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursive-static-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
LL | pub static FOO: u32 = FOO;
   | ^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires const-evaluating + checking `FOO`...
   |
LL | pub static FOO: u32 = FOO;
   |                       ^^^
   |                       ^^^
   = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
note: cycle used when linting `top-level module`
   |
LL | / pub static FOO: u32 = FOO;
LL | / pub static FOO: u32 = FOO;
LL | | //~^ ERROR cycle detected when const-evaluating + checking `FOO`
LL | |
LL | | fn main() {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
diff of stderr:

5    |                ^^^^
6    |
7    = note: ...which immediately requires computing type of `<impl at $DIR/issue-23305.rs:5:1: 5:21>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
10    |
10    |
11 LL | / pub trait ToNbt<T> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/issue-23305.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-23305.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-23305.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:21>`
   |
   |
LL | impl dyn ToNbt<Self> {}
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/issue-23305.rs:5:1: 5:21>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / pub trait ToNbt<T> {
LL | |     fn new(val: T) -> Self;
LL | | }
LL | |
LL | |
LL | | fn main() {}
   | |____________^

---
diff of stderr:

5    |             ^^^^
6    |
7    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:14:1: 14:17>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
10    |
10    |
11 LL | / #![feature(associated_type_defaults)]
24    |               ^^^^
25    |
25    |
26    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:15:1: 15:20>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
29    |
29    |
30 LL | / #![feature(associated_type_defaults)]
43    |      ^^^^
44    |
44    |
45    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:16:1: 16:10>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
48    |
48    |
49 LL | / #![feature(associated_type_defaults)]
62    |        ^^^^
63    |
63    |
64    = note: ...which immediately requires computing type of `<impl at $DIR/resolve-self-in-impl.rs:17:1: 17:13>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
67    |
67    |
68 LL | / #![feature(associated_type_defaults)]
81    | ^^^^^^^^^^^^^^^^^^^^^^
82    |
82    |
83    = note: ...which immediately requires computing trait implemented by `<impl at $DIR/resolve-self-in-impl.rs:18:1: 18:23>` again
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
86    |
86    |
87 LL | / #![feature(associated_type_defaults)]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/resolve-self-in-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/resolve-self-in-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-self-in-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:17>`
   |
   |
LL | impl Tr for Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:14:1: 14:17>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:20>`
   |
   |
LL | impl Tr for S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:20>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:10>`
   |
   |
LL | impl Self {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:16:1: 16:10>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:13>`
   |
   |
LL | impl S<Self> {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing type of `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:17:1: 17:13>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^


error[E0391]: cycle detected when computing trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:23>`
   |
   |
LL | impl Tr<Self::A> for S {} //~ ERROR cycle detected
   |
   |
   = note: ...which immediately requires computing trait implemented by `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:18:1: 18:23>` again
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | / #![feature(associated_type_defaults)]
LL | |
LL | | struct S<T = u8>(T);
LL | | trait Tr<T = u8> {
LL | |
LL | | fn main() {}
   | |____________^

---

---- [ui] src/test/ui/traits/trait-upcasting/cyclic-trait-resolution.rs stdout ----
diff of stderr:

10 LL | trait A: B + A {}
11    |              ^
12    = note: ...which again requires computing the super predicates of `A`, completing the cycle
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
15    |
15    |
16 LL | trait A: B + A {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/cyclic-trait-resolution/cyclic-trait-resolution.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/trait-upcasting/cyclic-trait-resolution.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/cyclic-trait-resolution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/cyclic-trait-resolution" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/cyclic-trait-resolution/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing the super predicates of `A`
   |
   |
LL | trait A: B + A {}
   |
   |
note: ...which requires computing the super traits of `A`...
   |
   |
LL | trait A: B + A {}
   |              ^
   = note: ...which again requires computing the super predicates of `A`, completing the cycle
note: cycle used when collecting item types in `top-level module`
   |
   |
LL | trait A: B + A {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---

7 error: internal compiler error: unexpected panic
8 
9 query stack during panic:
- #0 [trigger_delay_span_bug] trigger a delay span bug
+ #0 [trigger_delay_span_bug] triggering a delay span bug
12 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/delay_span_bug.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args treat-err-as-bug/delay_span_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1575:30

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (1a0af7e16 2022-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z treat-err-as-bug
query stack during panic:
query stack during panic:
#0 [trigger_delay_span_bug] triggering a delay span bug
------------------------------------------


---- [ui] src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs stdout ----
---- [ui] src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs stdout ----
diff of stderr:

16    |          ^^^^^^^^^^^^^^^^^^^^^
17    = note: ...which requires computing layout of `Alpha`...
18    = note: ...which again requires simplifying constant for the type system `Alpha::V3::{constant#0}`, completing the cycle
- note: cycle used when collecting item types in top-level module
+ note: cycle used when collecting item types in `top-level module`
21    |
22 LL | / #[repr(u8)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/self-in-enum-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-enum-variants/self-in-enum-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Alpha::V3::{constant#0}`
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |
   |
note: ...which requires simplifying constant for the type system `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Alpha`...
   = note: ...which again requires simplifying constant for the type system `Alpha::V3::{constant#0}`, completing the cycle
note: cycle used when collecting item types in `top-level module`
   |
LL | / #[repr(u8)]
LL | / #[repr(u8)]
LL | | enum Alpha {
LL | |     V1 = 41,
LL | |     V2 = Self::V1 as u8 + 1,    // OK; See #50072.
LL | |
LL | | fn main() {}
   | |____________^

---
---- [ui] src/test/ui/type-alias-impl-trait/auto-trait-leakage3.rs stdout ----
diff of stderr:

11    |         ^^^^^^^
12    = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
13    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in module `m`
+ note: cycle used when checking item types in `module `m``
16    |
16    |
17 LL | mod m {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auto-trait-leakage3.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/auto-trait-leakage3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo());
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `module `m``
   |
   |
LL | mod m {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/inference-cycle.rs stdout ----
diff of stderr:

11    |         ^^^^^^^
12    = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
13    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in module `m`
+ note: cycle used when checking item types in `module `m``
16    |
16    |
17 LL | mod m {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/inference-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/inference-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/inference-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo()); // Today: error
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `m::Foo: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `module `m``
   |
   |
LL | mod m {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/issue-53092-2.rs stdout ----
diff of stderr:

12    = note: ...which requires computing layout of `Bug<u8, ()>`...
13    = note: ...which requires normalizing `Bug<u8, ()>`...
14    = note: ...which again requires computing type of `Bug::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
17    |
18 LL | / #![feature(type_alias_impl_trait)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-53092-2/issue-53092-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-53092-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-53092-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-53092-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-53092-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Bug::{opaque#0}`
   |
   |
LL | type Bug<T, U> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected
   |
   |
note: ...which requires type-checking `CONST_BUG`...
   |
   |
LL | const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Bug<u8, ()>`...
   = note: ...which requires normalizing `Bug<u8, ()>`...
   = note: ...which again requires computing type of `Bug::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / #![feature(type_alias_impl_trait)]
LL | | #![allow(dead_code)]
LL | |
LL | |
LL | | type Bug<T, U> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected
LL | |     CONST_BUG(0);
LL | | }
   | |_^


error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   |
LL | const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };
   |
   |
   = note: source type: `[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-53092-2.rs:6:61: 6:68]` (0 bits)
   = note: target type: `Bug<u8, ()>` (size can vary because of [type error])

error[E0277]: the trait bound `U: From<T>` is not satisfied
   |
   |
LL |     |x| x.into() //~ ERROR the trait bound `U: From<T>` is not satisfied
   |     ^^^^^^^^^^^^ the trait `From<T>` is not implemented for `U`
note: required by a bound in `make_bug`
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-53092-2.rs:9:19
   |
   |
LL | fn make_bug<T, U: From<T>>() -> Bug<T, U> {
   |                   ^^^^^^^ required by this bound in `make_bug`
help: consider restricting type parameter `U`
   |
LL | type Bug<T, U: std::convert::From<T>> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0391, E0512.
---
---- [ui] src/test/ui/type-alias-impl-trait/reveal_local.rs stdout ----
diff of stderr:

11    |     ^^^^^^^^^^^^^^
12    = note: ...which requires evaluating trait selection obligation `Foo: core::marker::Send`...
13    = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
+ note: cycle used when checking item types in `top-level module`
15   --> $DIR/reveal_local.rs:1:1
17 LL | / #![feature(type_alias_impl_trait)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/reveal_local/reveal_local.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/reveal_local.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/reveal_local.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/reveal_local" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/reveal_local/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::{opaque#0}`
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^
   |
   |
note: ...which requires type-checking `not_good`...
   |
LL |     is_send::<Foo>();
   |     ^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `Foo: core::marker::Send`...
   = note: ...which again requires computing type of `Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in `top-level module`
   |
LL | / #![feature(type_alias_impl_trait)]
LL | |
LL | | use std::fmt::Debug;
