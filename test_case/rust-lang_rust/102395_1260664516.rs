plain

---- [ui] src/test/ui-fulldeps/internal-lints/diagnostics.rs stdout ----
diff of stderr:

- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:39:17
+ error[E0433]: failed to resolve: could not find `typeck` in `fluent`
3    |
3    |
- LL |         handler.struct_err("untranslatable diagnostic")
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:6:9
-    |
-    |
- LL | #![deny(rustc::untranslatable_diagnostic)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |         diag.note(fluent::typeck::note);
+    |                           ^^^^^^ could not find `typeck` in `fluent`
12 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:56:14
-    |
- LL |         diag.note("untranslatable diagnostic");
+ error: aborting due to previous error
18 
18 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:70:25
-    |
- LL |     let _diag = handler.struct_err(fluent::parser::expect_path);
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:7:9
-    |
-    |
- LL | #![deny(rustc::diagnostic_outside_of_impl)]
- 
- 
- error: diagnostics should only be created in `IntoDiagnostic`/`AddToDiagnostic` impls
-   --> $DIR/diagnostics.rs:73:25
-    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
- 
- 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:73:25
-    |
- LL |     let _diag = handler.struct_err("untranslatable diagnostic");
- 
- error: aborting due to 5 previous errors
- 
+ For more information about this error, try `rustc --explain E0433`.
---
To only update this specific test, also pass `--test-args internal-lints/diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: could not find `typeck` in `fluent`
   |
   |
LL |         diag.note(fluent::typeck::note);
   |                           ^^^^^^ could not find `typeck` in `fluent`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
------------------------------------------
