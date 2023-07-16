plain
iiiii.i................................................................................. 11088/13547
........................................................................................ 11176/13547
........................................................................................ 11264/13547
........................................................................................ 11352/13547
...........................................................F..F......................... 11440/13547
........................................................................................ 11616/13547
........................................................................................ 11704/13547
........................................................................................ 11792/13547
........................................................................................ 11880/13547
---

---- [ui] src/test/ui/feature-gates/gated-bad-feature.rs stdout ----
diff of stderr:

28 LL | #![feature = "foo"]
29    | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
+ warning: the feature `foo` has been stable since 1.66.0-nightly and no longer requires an attribute to enable
+   --> $DIR/gated-bad-feature.rs:1:48
+    |
+    |
+ LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
+    |
+    = note: `#[warn(stable_features)]` on by default
+ 
+ 
31 error[E0635]: unknown feature `foo_bar_baz`
32   --> $DIR/gated-bad-feature.rs:1:12


34 LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
36 
- error[E0635]: unknown feature `foo`
-   --> $DIR/gated-bad-feature.rs:1:48
-    |
-    |
- LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
- 
- error: aborting due to 7 previous errors
+ error: aborting due to 6 previous errors; 1 warning emitted
44 
---
To only update this specific test, also pass `--test-args feature-gates/gated-bad-feature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/gated-bad-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/gated-bad-feature/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0556]: malformed `feature` attribute input
   |
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:6:1
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:6:1
   |
LL | #![feature] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
error: malformed `feature` attribute input
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:7:1
   |
   |
LL | #![feature = "foo"] //~ ERROR malformed `feature` attribute
   | ^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#![feature(name1, name2, ...)]`
warning: the feature `foo` has been stable since 1.66.0-nightly and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gates/gated-bad-feature.rs:1:48
   |
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]
   |
   = note: `#[warn(stable_features)]` on by default


error[E0635]: unknown feature `foo_bar_baz`
   |
   |
LL | #![feature(foo_bar_baz, foo(bar), foo = "baz", foo)]

error: aborting due to 6 previous errors; 1 warning emitted

Some errors have detailed explanations: E0556, E0557, E0635.
Some errors have detailed explanations: E0556, E0557, E0635.
For more information about an error, try `rustc --explain E0556`.
------------------------------------------


---- [ui] src/test/ui/stability-attribute/stability-attribute-implies-using-stable.rs stdout ----
diff of stderr:

- error: the feature `foo` has been partially stabilized since 1.62.0 and is succeeded by the feature `foobar`
+ error: the feature `foo` has been partially stabilized since 1.66.0-nightly and is succeeded by the feature `foobar`
3    |
4 LL | #![feature(foo)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-stable/stability-attribute-implies-using-stable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-implies-using-stable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-implies-using-stable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-stable" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-stable/auxiliary"
stdout: none
--- stderr -------------------------------
error: the feature `foo` has been partially stabilized since 1.66.0-nightly and is succeeded by the feature `foobar`
   |
LL | #![feature(foo)]
   |            ^^^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-implies-using-stable.rs:2:9
   |
LL | #![deny(stable_features)]
   |         ^^^^^^^^^^^^^^^
help: if you are using features which are still unstable, change to using `foobar`
   |
LL | #![feature(foobar)]
help: if you are using features which are now stable, remove this line
   |
   |
LL - #![feature(foo)]

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/stability-attribute/stability-attribute-implies-using-unstable.rs stdout ----
diff of stderr:

- error: the feature `foo` has been partially stabilized since 1.62.0 and is succeeded by the feature `foobar`
+ error: the feature `foo` has been partially stabilized since 1.66.0-nightly and is succeeded by the feature `foobar`
3    |
4 LL | #![feature(foo)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-unstable/stability-attribute-implies-using-unstable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-implies-using-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-implies-using-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-unstable" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-implies-using-unstable/auxiliary"
stdout: none
--- stderr -------------------------------
error: the feature `foo` has been partially stabilized since 1.66.0-nightly and is succeeded by the feature `foobar`
   |
LL | #![feature(foo)]
   |            ^^^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-implies-using-unstable.rs:2:9
   |
LL | #![deny(stable_features)]
   |         ^^^^^^^^^^^^^^^
help: if you are using features which are still unstable, change to using `foobar`
   |
LL | #![feature(foobar)]
help: if you are using features which are now stable, remove this line
   |
   |
LL - #![feature(foo)]

error: aborting due to previous error
------------------------------------------

