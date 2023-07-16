plain
....................................iii................................................. 13024/13079
.......................................................
failures:

---- [ui] src/test/ui/stability-attribute/allow-through-unstable-misuse.rs stdout ----
normalized stderr:
error[E0788]: `rustc_allowed_through_unstable_modules` attribute must be paired with a `stable` attribute
  --> $DIR/allow-through-unstable-misuse.rs:8:1
   |
LL | pub struct UnstableType(());


error[E0788]: `rustc_allowed_through_unstable_modules` attribute must be paired with a `stable` attribute
  --> $DIR/allow-through-unstable-misuse.rs:8:1
   |
LL | pub struct UnstableType(());

error: aborting due to 2 previous errors

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-through-unstable-misuse/allow-through-unstable-misuse.stderr
To only update this specific test, also pass `--test-args stability-attribute/allow-through-unstable-misuse.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/allow-through-unstable-misuse.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-through-unstable-misuse" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-through-unstable-misuse/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0788]: `rustc_allowed_through_unstable_modules` attribute must be paired with a `stable` attribute
  --> /checkout/src/test/ui/stability-attribute/allow-through-unstable-misuse.rs:8:1
   |
LL | pub struct UnstableType(());


error[E0788]: `rustc_allowed_through_unstable_modules` attribute must be paired with a `stable` attribute
  --> /checkout/src/test/ui/stability-attribute/allow-through-unstable-misuse.rs:8:1
   |
LL | pub struct UnstableType(());

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/stability-attribute/allowed-through-unstable.rs stdout ----

1 error[E0658]: use of unstable library feature 'unstable_test_feature'
-   --> $DIR/allowed-through-unstable.rs:10:5
+   --> $DIR/allowed-through-unstable.rs:9:5
+   --> $DIR/allowed-through-unstable.rs:9:5
3    |
4 LL | use allowed_through_unstable_core::unstable_module::NewStableTraitNotAllowedThroughUnstable;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allowed-through-unstable/allowed-through-unstable.stderr
To only update this specific test, also pass `--test-args stability-attribute/allowed-through-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/allowed-through-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allowed-through-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allowed-through-unstable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/stability-attribute/allowed-through-unstable.rs:9:5
   |
LL | use allowed_through_unstable_core::unstable_module::NewStableTraitNotAllowedThroughUnstable; //~ ERROR use of unstable library feature 'u...
   |
   = note: see issue #1 <https://github.com/rust-lang/rust/issues/1> for more information
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

