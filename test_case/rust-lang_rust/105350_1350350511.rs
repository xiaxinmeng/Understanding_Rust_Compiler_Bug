plain
test [ui] src/test/ui/transmutability/primitives/numbers.rs ... ok

failures:

---- [ui] src/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn.rs stdout ----

13 LL |       f
14    |       ^ expected reference, found `u32`
15    |
15    |
-    = note: expected reference `&dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn Fn(u32) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a))`
+    = note: expected reference `&dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a ...) + 'a)) + 'a)) + 'a))`
+            the full type name has been written to '$TEST_BUILD_DIR/higher-rank-trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.long-type-61928927756194473.txt'
17               found reference `&dyn Fn(u32)`
19 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.stderr
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/hang-on-deeply-nested-dyn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn.rs:10:5
   |
LL |   ) -> &dyn Fn(
   |  ______-
   |  ______-
LL | |     &dyn Fn(
LL | |         &dyn Fn(
LL | |             &dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(u32))))))))),
LL | |         ),
LL | |     ),
LL | | ) {
   | |_- expected `&dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn Fn(u32) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a))` because of return type
   |       ^ expected reference, found `u32`
   |
   |
   = note: expected reference `&dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a ...) + 'a)) + 'a)) + 'a))`
           the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.long-type-61928927756194473.txt'
              found reference `&dyn Fn(u32)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
