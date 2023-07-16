plain

---- [ui] src/test/ui/const-generics/issues/issue-100313.rs stdout ----
diff of stderr:

4 LL |             *(B as *const bool as *mut bool) = false;
6    |             |
-    |             writing to alloc7 which is read-only
+    |             writing to alloc8 which is read-only
+    |             writing to alloc8 which is read-only
8    |             inside `T::<&true>::set_false` at $DIR/issue-100313.rs:10:13
9 ...
10 LL |     x.set_false();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-100313/issue-100313.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-100313.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-100313.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-100313" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-100313/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issues/issue-100313.rs:10:13
   |
   |
LL |             *(B as *const bool as *mut bool) = false;
   |             |
   |             writing to alloc8 which is read-only
   |             writing to alloc8 which is read-only
   |             inside `T::<&true>::set_false` at /checkout/src/test/ui/const-generics/issues/issue-100313.rs:10:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...
LL |     x.set_false();
   |     ------------- inside `_` at /checkout/src/test/ui/const-generics/issues/issue-100313.rs:18:5
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/issue-49296.rs stdout ----
diff of stderr:

2   --> $DIR/issue-49296.rs:9:16
3    |
4 LL | const X: u64 = *wat(42);
-    |                ^^^^^^^^ pointer to alloc3 was dereferenced after this allocation got freed
+    |                ^^^^^^^^ pointer to alloc4 was dereferenced after this allocation got freed
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296/issue-49296.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/issue-49296.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-49296.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/issue-49296.rs:9:16
   |
   |
LL | const X: u64 = *wat(42);
   |                ^^^^^^^^ pointer to alloc4 was dereferenced after this allocation got freed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
