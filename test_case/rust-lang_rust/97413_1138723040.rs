plain
..............................................................iii....................... 13200/13281
.................................................................................
failures:

---- [ui] src/test/ui/return/return-impl-maybe-sized.rs stdout ----

1 error: return type should be sized
-  --> src/test/ui/return/return-impl-maybe-sized.rs:1:17
-   |
-   |
- 1 | fn test_fn() -> impl ?Sized {}
-   |                 ^^^^^^^^^^^
+   --> $DIR/return-impl-maybe-sized.rs:3:17
+    |
+ LL | fn test_fn() -> impl ?Sized {}
6 
7 error: aborting due to previous error
+ 
8 
---
To only update this specific test, also pass `--test-args return/return-impl-maybe-sized.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-impl-maybe-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-maybe-sized" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-maybe-sized/auxiliary"
stdout: none
--- stderr -------------------------------
error: return type should be sized
  --> /checkout/src/test/ui/return/return-impl-maybe-sized.rs:3:17
   |
LL | fn test_fn() -> impl ?Sized {}

error: aborting due to previous error
------------------------------------------

