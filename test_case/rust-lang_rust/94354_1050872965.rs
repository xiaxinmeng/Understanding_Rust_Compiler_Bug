plain

1 error: literal out of range for `usize`
+   --> $DIR/issue-94239.rs:3:9
+    |
+ LL |         10000000000000000000..=99999999999999999999 => 0,
+    |
+    = note: `#[deny(overflowing_literals)]` on by default
+    = note: `#[deny(overflowing_literals)]` on by default
+    = note: the literal `10000000000000000000` does not fit into the type `usize` whose range is `0..=4294967295`
+ 
+ error: literal out of range for `usize`
3    |
3    |
4 LL |         10000000000000000000..=99999999999999999999 => 0,
5    |                                ^^^^^^^^^^^^^^^^^^^^
6    |
-    = note: `#[deny(overflowing_literals)]` on by default
-    = note: `#[deny(overflowing_literals)]` on by default
-    = note: the literal `99999999999999999999` does not fit into the type `usize` whose range is `0..=18446744073709551615`
+    = note: the literal `99999999999999999999` does not fit into the type `usize` whose range is `0..=4294967295`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
11 
12 
---
To only update this specific test, also pass `--test-args issues/issue-94239.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-94239.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-94239" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-94239/auxiliary"
stdout: none
--- stderr -------------------------------
error: literal out of range for `usize`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
   |
   |
LL |         10000000000000000000..=99999999999999999999 => 0, //~ ERROR literal out of range for `usize`
   |
   = note: `#[deny(overflowing_literals)]` on by default
   = note: `#[deny(overflowing_literals)]` on by default
   = note: the literal `10000000000000000000` does not fit into the type `usize` whose range is `0..=4294967295`
error: literal out of range for `usize`
  --> /checkout/src/test/ui/issues/issue-94239.rs:3:32
   |
   |
LL |         10000000000000000000..=99999999999999999999 => 0, //~ ERROR literal out of range for `usize`
   |
   |
   = note: the literal `99999999999999999999` does not fit into the type `usize` whose range is `0..=4294967295`
error: aborting due to 2 previous errors
------------------------------------------


