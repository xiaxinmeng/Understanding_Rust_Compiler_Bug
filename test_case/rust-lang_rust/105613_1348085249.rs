plain
---- [ui] src/test/ui/consts/assert-type-intrinsics.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of constant value failed
-   --> $DIR/assert-type-intrinsics.rs:12:9
+ error[E0425]: cannot find function `assert_uninit_valid` in module `intrinsics`
3    |
3    |
- LL |         MaybeUninit::<!>::uninit().assume_init();
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to instantiate uninhabited type `!`
- error[E0080]: evaluation of constant value failed
-   --> $DIR/assert-type-intrinsics.rs:16:9
-    |
-    |
10 LL |         intrinsics::assert_uninit_valid::<&'static i32>();
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to leave type `&i32` uninitialized, which is invalid
- error[E0080]: evaluation of constant value failed
-   --> $DIR/assert-type-intrinsics.rs:20:9
+    |                     ^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `assert_ones_valid`
15    |
15    |
- LL |         intrinsics::assert_zero_valid::<&'static i32>();
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ aborted execution: attempted to zero-initialize type `&i32`, which is invalid
+   ::: $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |
+ LL |     pub fn assert_ones_valid<T>();
+    |     ----------------------------- similarly named function `assert_ones_valid` defined here
- error: aborting due to 3 previous errors
+ error: aborting due to previous error
20 
- For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/assert-type-intrinsics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/assert-type-intrinsics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assert-type-intrinsics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assert-type-intrinsics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `assert_uninit_valid` in module `intrinsics`
   |
   |
LL |         intrinsics::assert_uninit_valid::<&'static i32>();
   |                     ^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `assert_ones_valid`
  ::: /checkout/library/core/src/intrinsics.rs:969:5
   |
   |
LL |     pub fn assert_ones_valid<T>();
   |     ----------------------------- similarly named function `assert_ones_valid` defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
