plain

---- [ui] tests/ui/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start.rs stdout ----
diff of stderr:

- error: `start` is not allowed to have `#[target_feature]`
-   --> $DIR/issue-108645-target-feature-on-start.rs:4:1
+ error: the feature named `avx2` is not valid for this target
3    |
3    |
4 LL | #[target_feature(enable = "avx2")]
- LL |
- LL |
- LL | fn start(_argc: isize, _argv: *const *const u8) -> isize { 0 }
-    | -------------------------------------------------------- `start` is not allowed to have `#[target_feature]`
+    |                  ^^^^^^^^^^^^^^^ `avx2` is not valid for this target
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start/issue-108645-target-feature-on-start.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start/auxiliary"
stdout: none
--- stderr -------------------------------
error: the feature named `avx2` is not valid for this target
  --> fake-test-src-base/rfcs/rfc-2396-target_feature-11/issue-108645-target-feature-on-start.rs:4:18
   |
LL | #[target_feature(enable = "avx2")]
   |                  ^^^^^^^^^^^^^^^ `avx2` is not valid for this target
error: aborting due to previous error
------------------------------------------



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


