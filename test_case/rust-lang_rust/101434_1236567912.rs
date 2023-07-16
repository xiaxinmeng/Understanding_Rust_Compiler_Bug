plain

---- [ui] src/test/ui-fulldeps/internal-lints/diagnostics.rs stdout ----
diff of stderr:

- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:37:14
+ error[E0053]: method `into_diagnostic` has an incompatible type for trait
3    |
3    |
- LL |         sess.struct_err("untranslatable diagnostic")
-    |              ^^^^^^^^^^
+ LL |     fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
+    |                                    |
+    |                                    |
+    |                                    expected struct `Handler`, found struct `ParseSess`
+    |                                    help: change the parameter type to match the trait: `&'a Handler`
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:6:9
-    |
-    |
- LL | #![deny(rustc::untranslatable_diagnostic)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: expected fn pointer `fn(UntranslatableInSessionDiagnostic, &'a Handler) -> DiagnosticBuilder<'_, _>`
+               found fn pointer `fn(UntranslatableInSessionDiagnostic, &'a ParseSess) -> DiagnosticBuilder<'_, _>`
12 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:54:14
+ error[E0053]: method `into_diagnostic` has an incompatible type for trait
15    |
15    |
- LL |         diag.note("untranslatable diagnostic");
- 
- 
- error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
-   --> $DIR/diagnostics.rs:68:22
+ LL |     fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
+    |                                    |
+    |                                    |
+    |                                    expected struct `Handler`, found struct `ParseSess`
+    |                                    help: change the parameter type to match the trait: `&'a Handler`
21    |
- LL |     let _diag = sess.struct_err(fluent::parser::expect_path);
-    |
- note: the lint level is defined here
-   --> $DIR/diagnostics.rs:7:9
-    |
-    |
- LL | #![deny(rustc::diagnostic_outside_of_impl)]
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: expected fn pointer `fn(TranslatableInSessionDiagnostic, &'a Handler) -> DiagnosticBuilder<'_, _>`
+               found fn pointer `fn(TranslatableInSessionDiagnostic, &'a ParseSess) -> DiagnosticBuilder<'_, _>`
30 
- error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
-   --> $DIR/diagnostics.rs:71:22
-    |
- LL |     let _diag = sess.struct_err("untranslatable diagnostic");
+ error: aborting due to 2 previous errors
36 
36 
- error: diagnostics should be created using translatable messages
-   --> $DIR/diagnostics.rs:71:22
-    |
- LL |     let _diag = sess.struct_err("untranslatable diagnostic");
- 
- error: aborting due to 5 previous errors
- 
+ For more information about this error, try `rustc --explain E0053`.
---
To only update this specific test, also pass `--test-args internal-lints/diagnostics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/diagnostics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `into_diagnostic` has an incompatible type for trait
   |
   |
LL |     fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
   |                                    |
   |                                    |
   |                                    expected struct `Handler`, found struct `ParseSess`
   |                                    help: change the parameter type to match the trait: `&'a Handler`
   |
   = note: expected fn pointer `fn(UntranslatableInSessionDiagnostic, &'a Handler) -> DiagnosticBuilder<'_, _>`
              found fn pointer `fn(UntranslatableInSessionDiagnostic, &'a ParseSess) -> DiagnosticBuilder<'_, _>`
error[E0053]: method `into_diagnostic` has an incompatible type for trait
  --> /checkout/src/test/ui-fulldeps/internal-lints/diagnostics.rs:45:36
   |
   |
LL |     fn into_diagnostic(self, sess: &'a ParseSess) -> DiagnosticBuilder<'a, ErrorGuaranteed> {
   |                                    |
   |                                    |
   |                                    expected struct `Handler`, found struct `ParseSess`
   |                                    help: change the parameter type to match the trait: `&'a Handler`
   |
   = note: expected fn pointer `fn(TranslatableInSessionDiagnostic, &'a Handler) -> DiagnosticBuilder<'_, _>`
              found fn pointer `fn(TranslatableInSessionDiagnostic, &'a ParseSess) -> DiagnosticBuilder<'_, _>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
