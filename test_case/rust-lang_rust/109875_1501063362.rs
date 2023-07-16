plain
---- [ui] tests/ui/extern-mod-syntax.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/extern-mod-syntax.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-mod-syntax/a.wasm" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-mod-syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `libc::c_void`
  --> fake-test-src-base/extern-mod-syntax.rs:7:5
LL | use libc::c_void;
   |     ^^^^^^^^^^^^ no `c_void` in the root
   |
help: consider importing one of these items instead
---


---- [ui] tests/ui/issue-13560.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/auxiliary/issue-13560-3.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/auxiliary/issue-13560-3.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-13560/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-13560/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0463]: can't find crate for `issue_13560_1`
  --> fake-test-src-base/auxiliary/issue-13560-3.rs:5:25
   |
LL | #[macro_use] #[no_link] extern crate issue_13560_1 as t1;

warning: unused `#[macro_use]` import
  --> fake-test-src-base/auxiliary/issue-13560-3.rs:6:1
   |
   |
LL | #[macro_use] extern crate issue_13560_2 as t2;
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted
---
---- [ui] tests/ui/std/switch-stdout.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/std/switch-stdout.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std/switch-stdout/a.wasm" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std/switch-stdout/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `switch_stdout_to` in this scope
  --> fake-test-src-base/std/switch-stdout.rs:44:5
LL |     switch_stdout_to(f);
   |     ^^^^^^^^^^^^^^^^ not found in this scope

error: aborting due to previous error
