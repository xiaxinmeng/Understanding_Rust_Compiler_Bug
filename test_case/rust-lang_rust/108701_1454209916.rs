plain
---- [ui] tests/ui/target-feature/wasm-safe.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/target-feature/wasm-safe.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/wasm-safe" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/wasm-safe/auxiliary"
stdout: none
--- stderr -------------------------------
error: `main` function is not allowed to have `#[target_feature]`
  --> fake-test-src-base/target-feature/wasm-safe.rs:44:1
LL | fn main() {}
LL | fn main() {}
   | ^^^^^^^^^ `main` function is not allowed to have `#[target_feature]`
error: aborting due to previous error
------------------------------------------


