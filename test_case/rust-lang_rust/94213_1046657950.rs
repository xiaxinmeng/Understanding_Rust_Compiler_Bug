plain
diff of stderr:

20   --> $DIR/gated-bad-feature.rs:5:1
21    |
22 LL | #![feature]
-    | ^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name1, ...)]`
+    | ^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
25 error: malformed `feature` attribute input
26   --> $DIR/gated-bad-feature.rs:6:1

27    |
27    |
28 LL | #![feature = "foo"]
-    | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name1, ...)]`
+    | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
31 error: aborting due to 5 previous errors
32 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature/gated-bad-feature.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/gated-bad-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/gated-bad-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0556]: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:1:25
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
   |                         ^^^^^^^^ help: expected just one word: `foo`
error[E0556]: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:1:35
   |
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
   |                                   ^^^^^^^^^^^ help: expected just one word: `foo`
error[E0557]: feature has been removed
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:8:12
   |
   |
LL | #![feature(test_removed_feature)] //~ ERROR: feature has been removed

error: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:5:1
   |
   |
LL | #![feature] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
error: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:6:1
   |
   |
LL | #![feature = "foo"] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0556, E0557.
For more information about an error, try `rustc --explain E0556`.
