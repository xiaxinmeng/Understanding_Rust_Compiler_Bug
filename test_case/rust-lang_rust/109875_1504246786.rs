plain
failures:

---- [ui] tests/ui/issue-13560.rs stdout ----

error: auxiliary build of "/checkout/tests/ui/auxiliary/issue-13560-3.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/auxiliary/issue-13560-3.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-13560/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-13560/auxiliary"
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

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std/switch-stdout" && RUST_TEST_THREADS="16" "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std/switch-stdout/a.js"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', fake-test-src-base/std/switch-stdout.rs:39:62
------------------------------------------



