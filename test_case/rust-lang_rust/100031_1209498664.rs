plain
.............................iii........................................................ 13288/13336
................................................
failures:

---- [ui] src/test/ui/suggestions/dont-try-removing-the-field.rs stdout ----

error: /checkout/src/test/ui/suggestions/dont-try-removing-the-field.rs:12: unexpected warning: '12:25: 12:28: unused variable: `baz` [unused_variables]'
error: /checkout/src/test/ui/suggestions/dont-try-removing-the-field.rs:12: expected warning not found: unused variable: `bar`

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-try-removing-the-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-try-removing-the-field/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-try-removing-the-field/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "12:25: 12:28: unused variable: `baz` [unused_variables]",
]

not found errors (from test file): [
    Error {
---

thread '[ui] src/test/ui/suggestions/dont-try-removing-the-field.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] src/test/ui/suggestions/try-removing-the-field.rs stdout ----

2   --> $DIR/try-removing-the-field.rs:12:20
3    |
3    |
4 LL |     let Foo { foo, bar, .. } = x;
+    |                    ^^^-
+    |                    |
+    |                    help: try removing the field
6    |
6    |
7    = note: `#[warn(unused_variables)]` on by default
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/try-removing-the-field.stderr
To only update this specific test, also pass `--test-args suggestions/try-removing-the-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/try-removing-the-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/try-removing-the-field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `bar`
  --> /checkout/src/test/ui/suggestions/try-removing-the-field.rs:12:20
   |
LL |     let Foo { foo, bar, .. } = x; //~ WARNING unused variable: `bar`
   |                    ^^^-
   |                    help: try removing the field
   |
   = note: `#[warn(unused_variables)]` on by default

