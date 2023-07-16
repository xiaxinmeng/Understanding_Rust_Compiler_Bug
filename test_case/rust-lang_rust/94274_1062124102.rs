plain

102 error[E0004]: non-exhaustive patterns: `C` not covered
103   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:20:11
104    |
- LL | /     enum Foo {
- LL | |         A, B, C,
-    | |               - not covered
- LL | |     }
-    | |_____- `Foo` defined here
- ...
- LL |       match Foo::A {
-    |             ^^^^^^ pattern `C` not covered
+ LL |     match Foo::A {
+    |           ^^^^^^ pattern `C` not covered
-    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
+ note: `Foo` defined here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:12:15
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:12:15
+    |
+ LL |     enum Foo {
+    |          ---
+ LL |         A, B, C,
+    |               ^ not covered
115    = note: the matched value is of type `Foo`
+ help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
+    |
+ LL ~         Foo::B => {}
+ LL +         C => todo!()
116 
117 error: aborting due to previous error; 10 warnings emitted
118 

---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unknown lint: `non_exhaustive_omitted_patterns`
   |
LL | #![deny(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
LL |     enum Foo {
   |          ---
LL |         A, B, C,
   |               ^ not covered
   = note: the matched value is of type `Foo`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         Foo::B => {}
LL +         C => todo!()

error: aborting due to previous error; 10 warnings emitted

For more information about this error, try `rustc --explain E0004`.
