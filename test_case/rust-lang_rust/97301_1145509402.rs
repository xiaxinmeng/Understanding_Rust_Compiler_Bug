plain
........................................................................................ 13288/13301
.............
failures:

---- [ui] src/test/ui/stability-attribute/allow-unstable-reexport.rs stdout ----

1 error[E0658]: use of unstable library feature 'unstable_test_feature'
-   --> $DIR/allow-unstable-reexport.rs:20:9
+   --> $DIR/allow-unstable-reexport.rs:21:9
---
13    |     ^^^^^^^^

15    = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable
16 
17 error[E0658]: use of unstable library feature 'unstable_test_feature': text
-   --> $DIR/allow-unstable-reexport.rs:27:5
19    |
20 LL |     unstable_text();
21    |     ^^^^^^^^^^^^^

---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-unstable-reexport" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-unstable-reexport/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs:21:9
LL | pub use lint_stability::unstable as unstable2;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs:27:5
   |
LL |     unstable(); //~ ERROR use of unstable library feature 'unstable_test_feature'
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
  --> /checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs:28:5
   |
LL |     unstable_text(); //~ ERROR use of unstable library feature 'unstable_test_feature'
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error: aborting due to 3 previous errors
