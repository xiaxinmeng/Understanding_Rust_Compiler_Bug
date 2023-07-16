plain
---- [ui] tests/ui/stability-attribute/default-body-stability-ok-impls.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/stability-attribute/default-body-stability-ok-impls.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/default-body-stability-ok-impls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/default-body-stability-ok-impls/auxiliary"
stdout: none
error[E0046]: not all trait items implemented, missing: `fun2`
  --> fake-test-src-base/stability-attribute/default-body-stability-ok-impls.rs:11:1
   |
   |
LL | / impl JustTrait for Type {
LL | |     const CONSTANT: usize = 1;
LL | |
LL | |     fn fun() {}
LL | | }
   |
   |
   = note: default implementation of `fun2` is unstable
   = note: use of unstable library feature 'fun_default_body': reason
   = help: add `#![feature(fun_default_body)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0046`.
------------------------------------------
