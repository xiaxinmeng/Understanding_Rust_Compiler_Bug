plain
........................................................................................ 11440/13064
........................................................................................ 11528/13064
.......................i........i........i.....i............................i........... 11616/13064
........................................................................................ 11704/13064
...F...F................................................................................ 11792/13064
........................................................................................ 11968/13064
........................................................................................ 12056/13064
........................................................................................ 12144/13064
.......................................................................................i 12232/13064
---

---- [ui] src/test/ui/trait-bounds/issue-93008.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `&'static mut (): Clone` is not satisfied
-   --> $DIR/issue-93008.rs:5:5
-    |
- LL |     &'static mut (): Clone,
-    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&'static mut ()`
-    = help: see issue #48214
-    = help: see issue #48214
-    = help: add `#![feature(trivial_bounds)]` to the crate attributes to enable
- error: aborting due to previous error
- 
- For more information about this error, try `rustc --explain E0277`.
- 
---
To only update this specific test, also pass `--test-args trait-bounds/issue-93008.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/issue-93008.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-93008" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=3" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-93008/auxiliary"
stdout: none
stderr: none


---- [ui] src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs stdout ----
error[E0601]: `main` function not found in crate `select_param_env_instead_of_blanket`
  --> $DIR/select-param-env-instead-of-blanket.rs:26:2
   |
LL | }
LL | }
   |  ^ consider adding a `main` function to `$DIR/select-param-env-instead-of-blanket.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

---
To only update this specific test, also pass `--test-args trait-bounds/select-param-env-instead-of-blanket.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/select-param-env-instead-of-blanket" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/select-param-env-instead-of-blanket/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `select_param_env_instead_of_blanket`
  --> /checkout/src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs:26:2
LL | }
LL | }
   |  ^ consider adding a `main` function to `/checkout/src/test/ui/trait-bounds/select-param-env-instead-of-blanket.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
