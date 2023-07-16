plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
-    |          not a derive macro
-    |          help: Remove from the surrounding `derive()`
+    |          ^^^^^^ not a derive macro
9    |
+ help: Remove from the surrounding `derive()`
+    |
+    |
+ LL | #[derive(inline)]
10    = help: Add as non-Derive macro
10    = help: Add as non-Derive macro
11            `#[inline]`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-4/macro-path-prelude-fail-4.stderr
To only update this specific test, also pass `--test-args macros/macro-path-prelude-fail-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/macro-path-prelude-fail-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected derive macro, found built-in attribute `inline`
  --> fake-test-src-base/macros/macro-path-prelude-fail-4.rs:1:10
   |
LL | #[derive(inline)] //~ ERROR expected derive macro, found built-in attribute `inline`
   |          ^^^^^^ not a derive macro
help: Remove from the surrounding `derive()`
  --> fake-test-src-base/macros/macro-path-prelude-fail-4.rs:1:10
   |
   |
LL | #[derive(inline)] //~ ERROR expected derive macro, found built-in attribute `inline`
   = help: Add as non-Derive macro
   = help: Add as non-Derive macro
           `#[inline]`
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/proc-macro/macro-namespace-reserved-2.rs stdout ----
diff of stderr:

56   --> $DIR/macro-namespace-reserved-2.rs:53:10
57    |
58 LL | #[derive(my_macro_attr)]
-    |          |
-    |          not a derive macro
-    |          help: Remove from the surrounding `derive()`
+    |          ^^^^^^^^^^^^^ not a derive macro
+    |          ^^^^^^^^^^^^^ not a derive macro
63    |
+ help: Remove from the surrounding `derive()`
+    |
+    |
+ LL | #[derive(my_macro_attr)]
64    = help: Add as non-Derive macro
64    = help: Add as non-Derive macro
65            `#[my_macro_attr]`

92   --> $DIR/macro-namespace-reserved-2.rs:50:10
93    |
93    |
94 LL | #[derive(crate::my_macro)]
-    |          |
-    |          not a derive macro
-    |          help: Remove from the surrounding `derive()`
+    |          ^^^^^^^^^^^^^^^ not a derive macro
+    |          ^^^^^^^^^^^^^^^ not a derive macro
99    |
+ help: Remove from the surrounding `derive()`
+    |
+    |
+ LL | #[derive(crate::my_macro)]
100    = help: Add as non-Derive macro
100    = help: Add as non-Derive macro
101            `#[crate::my_macro]`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-namespace-reserved-2/macro-namespace-reserved-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-namespace-reserved-2/macro-namespace-reserved-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/macro-namespace-reserved-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/macro-namespace-reserved-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-namespace-reserved-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/macro-namespace-reserved-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:25:5
   |
LL |     my_macro!(); //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:29:5
   |
   |
LL |     crate::my_macro_attr!(); //~ ERROR can't use a procedural macro from the same crate that defines


error: expected macro, found attribute macro `crate::my_macro_attr`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:29:5
   |
LL |     crate::my_macro_attr!(); //~ ERROR can't use a procedural macro from the same crate that defines

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:34:5
   |
   |
LL |     crate::MyTrait!(); //~ ERROR can't use a procedural macro from the same crate that defines it


error: expected macro, found derive macro `crate::MyTrait`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:34:5
   |
LL |     crate::MyTrait!(); //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:42:3
   |
   |
LL | #[my_macro_attr] //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:44:3
   |
   |
LL | #[MyTrait] //~ ERROR can't use a procedural macro from the same crate that defines it


error: expected attribute, found derive macro `MyTrait`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:44:3
   |
LL | #[MyTrait] //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:53:10
   |
   |
LL | #[derive(my_macro_attr)] //~ ERROR can't use a procedural macro from the same crate that defines it


error: expected derive macro, found attribute macro `my_macro_attr`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:53:10
   |
LL | #[derive(my_macro_attr)] //~ ERROR can't use a procedural macro from the same crate that defines it
   |          ^^^^^^^^^^^^^ not a derive macro
help: Remove from the surrounding `derive()`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:53:10
   |
   |
LL | #[derive(my_macro_attr)] //~ ERROR can't use a procedural macro from the same crate that defines it
   = help: Add as non-Derive macro
   = help: Add as non-Derive macro
           `#[my_macro_attr]`
error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:56:10
   |
   |
LL | #[derive(MyTrait)] //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:39:3
   |
   |
LL | #[crate::my_macro] //~ ERROR can't use a procedural macro from the same crate that defines it


error: expected attribute, found macro `crate::my_macro`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:39:3
   |
LL | #[crate::my_macro] //~ ERROR can't use a procedural macro from the same crate that defines it

error: can't use a procedural macro from the same crate that defines it
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:50:10
   |
   |
LL | #[derive(crate::my_macro)] //~ ERROR can't use a procedural macro from the same crate that defines


error: expected derive macro, found macro `crate::my_macro`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:50:10
   |
LL | #[derive(crate::my_macro)] //~ ERROR can't use a procedural macro from the same crate that defines
   |          ^^^^^^^^^^^^^^^ not a derive macro
help: Remove from the surrounding `derive()`
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:50:10
   |
   |
LL | #[derive(crate::my_macro)] //~ ERROR can't use a procedural macro from the same crate that defines
   = help: Add as non-Derive macro
   = help: Add as non-Derive macro
           `#[crate::my_macro]`
error: cannot find macro `my_macro_attr` in this scope
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:28:5
   |
   |
LL |     my_macro_attr!(); //~ ERROR cannot find macro `my_macro_attr` in this scope
   |
   |
   = note: `my_macro_attr` is in scope, but it is an attribute: `#[my_macro_attr]`
error: cannot find macro `MyTrait` in this scope
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:33:5
   |
   |
LL |     MyTrait!(); //~ ERROR cannot find macro `MyTrait` in this scope
   |
   |
   = note: `MyTrait` is in scope, but it is a derive macro: `#[derive(MyTrait)]`

error: cannot find attribute `my_macro` in this scope
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:38:3
   |
LL | #[my_macro] //~ ERROR cannot find attribute `my_macro` in this scope
   |
   |
   = note: `my_macro` is in scope, but it is a function-like macro

error: cannot find derive macro `my_macro` in this scope
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:48:10
   |
LL | #[derive(my_macro)] //~ ERROR cannot find derive macro `my_macro` in this scope
   |
   |
   = note: `my_macro` is in scope, but it is a function-like macro

error: cannot find derive macro `my_macro` in this scope
  --> fake-test-src-base/proc-macro/macro-namespace-reserved-2.rs:48:10
   |
LL | #[derive(my_macro)] //~ ERROR cannot find derive macro `my_macro` in this scope
   |
   |
   = note: `my_macro` is in scope, but it is a function-like macro
error: aborting due to 20 previous errors
------------------------------------------



---- [ui] tests/ui/tool-attributes/tool-attributes-misplaced-2.rs stdout ----
diff of stderr:

2   --> $DIR/tool-attributes-misplaced-2.rs:1:10
3    |
4 LL | #[derive(rustfmt::skip)]
-    |          |
-    |          not a derive macro
-    |          help: Remove from the surrounding `derive()`
+    |          ^^^^^^^^^^^^^ not a derive macro
+    |          ^^^^^^^^^^^^^ not a derive macro
9    |
+ help: Remove from the surrounding `derive()`
+    |
+    |
+ LL | #[derive(rustfmt::skip)]
10    = help: Add as non-Derive macro
10    = help: Add as non-Derive macro
11            `#[rustfmt::skip]`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-2/tool-attributes-misplaced-2.stderr
To only update this specific test, also pass `--test-args tool-attributes/tool-attributes-misplaced-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/tool-attributes/tool-attributes-misplaced-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool-attributes/tool-attributes-misplaced-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected derive macro, found tool attribute `rustfmt::skip`
  --> fake-test-src-base/tool-attributes/tool-attributes-misplaced-2.rs:1:10
   |
LL | #[derive(rustfmt::skip)] //~ ERROR expected derive macro, found tool attribute `rustfmt::skip`
   |          ^^^^^^^^^^^^^ not a derive macro
help: Remove from the surrounding `derive()`
  --> fake-test-src-base/tool-attributes/tool-attributes-misplaced-2.rs:1:10
   |
   |
LL | #[derive(rustfmt::skip)] //~ ERROR expected derive macro, found tool attribute `rustfmt::skip`
   = help: Add as non-Derive macro
   = help: Add as non-Derive macro
           `#[rustfmt::skip]`

error: expected macro, found tool attribute `rustfmt::skip`
  --> fake-test-src-base/tool-attributes/tool-attributes-misplaced-2.rs:5:5
   |
LL |     rustfmt::skip!(); //~ ERROR expected macro, found tool attribute `rustfmt::skip`

error: aborting due to 2 previous errors
------------------------------------------

