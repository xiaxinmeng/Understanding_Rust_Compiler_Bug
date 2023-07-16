plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 72 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....F..F...............i...............................................

---- [ui] src/test/ui-fulldeps/internal-lints/diagnostics_incorrect.rs stdout ----
diff of stderr:


4 LL |     #[rustc_lint_diagnostics(a)]
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_lint_diagnostics]`
- error: attribute should be applied to a function
+ error: attribute should be applied to a function definition
8   --> $DIR/diagnostics_incorrect.rs:5:1
9    |
---
To only update this specific test, also pass `--test-args internal-lints/diagnostics_incorrect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/diagnostics_incorrect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics_incorrect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics_incorrect/auxiliary"
stdout: none
--- stderr -------------------------------
error: malformed `rustc_lint_diagnostics` attribute input
   |
   |
LL |     #[rustc_lint_diagnostics(a)]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_lint_diagnostics]`
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics_incorrect.rs:5:1
   |
LL | #[rustc_lint_diagnostics]
LL | #[rustc_lint_diagnostics]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | struct Foo;

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui-fulldeps/internal-lints/query_stability_incorrect.rs stdout ----
diff of stderr:

4 LL |     #[rustc_lint_query_instability(a)]
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_lint_query_instability]`
- error: attribute should be applied to a function
+ error: attribute should be applied to a function definition
8   --> $DIR/query_stability_incorrect.rs:5:1
9    |
9    |
10 LL | #[rustc_lint_query_instability]
11    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
12 LL |
13 LL | struct Foo;
-    | ----------- not a function
---
To only update this specific test, also pass `--test-args internal-lints/query_stability_incorrect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/query_stability_incorrect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/query_stability_incorrect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/query_stability_incorrect/auxiliary"
stdout: none
--- stderr -------------------------------
error: malformed `rustc_lint_query_instability` attribute input
   |
   |
LL |     #[rustc_lint_query_instability(a)]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_lint_query_instability]`
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui-fulldeps/internal-lints/query_stability_incorrect.rs:5:1
   |
   |
LL | #[rustc_lint_query_instability]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a function
LL | struct Foo;

error: aborting due to 2 previous errors
------------------------------------------

