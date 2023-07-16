plain
normalized stderr:
warning: unused import: `RunCompiler`
  --> $DIR/crate-info.rs:17:44
   |
LL | use rustc_driver::{Callbacks, Compilation, RunCompiler};
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `args`
---
To only update this specific test, also pass `--test-args stable-mir/crate-info.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/stable-mir/crate-info.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/stable-mir/crate-info/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/stable-mir/crate-info/auxiliary" "--edition=2021"
stdout: none
warning: unused import: `RunCompiler`
  --> fake-test-src-base/stable-mir/crate-info.rs:17:44
   |
   |
LL | use rustc_driver::{Callbacks, Compilation, RunCompiler};
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `args`
