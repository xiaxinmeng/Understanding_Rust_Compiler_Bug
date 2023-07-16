plain
..............

failures:

---- [ui] tests/ui/lint/duplicate-trait/duplicate-trait.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/duplicate-trait/duplicate-trait.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/duplicate-trait/duplicate-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/duplicate-trait/duplicate-trait/auxiliary"
stdout: none
warning: trait `Send` was already specified
warning: trait `Send` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:10:42
   |
LL | fn duplicate_once(_a: &(dyn Any + Send + Send)) {} //~WARNING duplicate trait
   |                                          ^^^^ help: remove duplicate trait
note: the lint level is defined here
note: the lint level is defined here
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:2:9
   |
LL | #![warn(duplicate_trait)]

warning: trait `Send` was already specified
warning: trait `Send` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:12:43
   |
LL | fn duplicate_twice(_a: &(dyn Any + Send + Send + Send)) {} //~WARNING duplicate trait
   |                                           ^^^^ help: remove duplicate trait
warning: trait `Send` was already specified
warning: trait `Send` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:12:50
   |
LL | fn duplicate_twice(_a: &(dyn Any + Send + Send + Send)) {} //~WARNING duplicate trait
   |                                                  ^^^^ help: remove duplicate trait
warning: trait `Send` was already specified
warning: trait `Send` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:14:57
   |
LL | fn duplicate_out_of_order(_a: &(dyn Any + Send + Sync + Send)) {} //~WARNING duplicate trait
   |                                                         ^^^^ help: remove duplicate trait
warning: trait `Send` was already specified
warning: trait `Send` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:16:53
   |
LL | fn duplicate_multiple(_a: &(dyn Any + Send + Sync + Send + Sync)) {} //~WARNING duplicate trait
   |                                                     ^^^^ help: remove duplicate trait
warning: trait `Sync` was already specified
warning: trait `Sync` was already specified
  --> fake-test-src-base/lint/duplicate-trait/duplicate-trait.rs:16:60
   |
LL | fn duplicate_multiple(_a: &(dyn Any + Send + Sync + Send + Sync)) {} //~WARNING duplicate trait
   |                                                            ^^^^ help: remove duplicate trait
warning: 6 warnings emitted
------------------------------------------


