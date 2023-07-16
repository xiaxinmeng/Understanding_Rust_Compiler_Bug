plain
........................................................................................ 11528/13486
........................................................................................ 11616/13486
.....................................................F.................................. 11704/13486
........................................................................................ 11792/13486
.........................................................F...........F.................. 11880/13486
........................................................................................ 12056/13486
........................................................................................ 12144/13486
........................................................................................ 12232/13486
........................................................................................ 12320/13486
---
diff of stderr:

2   --> $DIR/dont-try-removing-the-field.rs:12:25
3    |
4 LL |     let Foo { foo, bar, baz } = x;
-    |                         ^^^ help: if this is intentional, prefix it with an underscore: `_baz`
+    |                         ^^^ help: try ignoring the field: `baz: _`
7    = note: `#[warn(unused_variables)]` on by default
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-try-removing-the-field/dont-try-removing-the-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/dont-try-removing-the-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-try-removing-the-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-try-removing-the-field/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-try-removing-the-field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `baz`
   |
   |
LL |     let Foo { foo, bar, baz } = x; //~ WARNING unused variable: `baz`
   |                         ^^^ help: try ignoring the field: `baz: _`
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted
------------------------------------------
---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

2   --> $DIR/try-removing-the-field.rs:12:20
3    |
4 LL |     let Foo { foo, bar, .. } = x;
-    |                    ^^^ help: if this is intentional, prefix it with an underscore: `_bar`
+    |                    ^^^ help: try ignoring the field: `bar: _`
7    = note: `#[warn(unused_variables)]` on by default
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/try-removing-the-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/try-removing-the-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/try-removing-the-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `bar`
   |
   |
LL |     let Foo { foo, bar, .. } = x; //~ WARNING unused variable: `bar`
   |                    ^^^ help: try ignoring the field: `bar: _`
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/unused-closure-argument.rs stdout ----
diff of stderr:

14   --> $DIR/unused-closure-argument.rs:12:23
15    |
16 LL |         .map(|Point { x, y }| y)
-    |                       ^ help: if this is intentional, prefix it with an underscore: `_x`
+    |                       ^ help: try ignoring the field: `x: _`
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unused-closure-argument/unused-closure-argument.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/unused-closure-argument.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/unused-closure-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unused-closure-argument" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/unused-closure-argument/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused variable: `x`
   |
LL |         .map(|x| 4)
   |               ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
---

error: unused variable: `x`
  --> /checkout/src/test/ui/suggestions/unused-closure-argument.rs:12:23
   |
LL |         .map(|Point { x, y }| y)
   |                       ^ help: try ignoring the field: `x: _`
error: aborting due to 2 previous errors
------------------------------------------


