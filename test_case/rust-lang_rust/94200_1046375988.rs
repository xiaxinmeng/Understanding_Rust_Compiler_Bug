plain

---- [ui] ui/feature-gates/gated-bad-feature.rs stdout ----
diff of stderr:

28 LL | #![feature = "foo"]
29    | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name1, ...)]`
30 
- error[E0635]: unknown feature `foo_bar_baz`
-   --> $DIR/gated-bad-feature.rs:1:12
-    |
- LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
- 
37 error[E0635]: unknown feature `foo`
38   --> $DIR/gated-bad-feature.rs:1:48
39    |
39    |

40 LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
+ 
+ 
+ error[E0635]: unknown feature `foo_bar_baz`
+   --> $DIR/gated-bad-feature.rs:1:12
+    |
+ LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
42 
43 error: aborting due to 7 previous errors
44 

---
To only update this specific test, also pass `--test-args feature-gates/gated-bad-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/gated-bad-feature.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature/auxiliary"
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
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:6:1
   |
   |
LL | #![feature] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name1, ...)]`
error: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:7:1
   |
   |
LL | #![feature = "foo"] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name1, ...)]`
error[E0635]: unknown feature `foo`
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:1:48
   |
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]


error[E0635]: unknown feature `foo_bar_baz`
   |
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0556, E0557, E0635.
