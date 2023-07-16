plain

---- [ui] ui/issues/issue-24352.rs stdout ----
diff of stderr:

5    |            ^ no implementation for `f64 - {integer}`
6    |
7    = help: the trait `Sub<{integer}>` is not implemented for `f64`
+ help: consider using a floating-point literal by writing it with `.0`
+ LL |     1.0f64 - 1.0
+    |               ++
8 
9 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-24352.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24352.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24352" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24352/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: cannot subtract `{integer}` from `f64`
   |
   |
LL |     1.0f64 - 1 //~ ERROR E0277
   |            ^ no implementation for `f64 - {integer}`
   |
   = help: the trait `Sub<{integer}>` is not implemented for `f64`
help: consider using a floating-point literal by writing it with `.0`
   |
LL |     1.0f64 - 1.0 //~ ERROR E0277

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
