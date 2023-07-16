plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.110 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.49s

 finished in 2.566 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests rustc_lint

running 53 tests
...........ii...F....F...............................

---- src/builtin.rs - builtin::DEREF_NULLPTR (line 2976) stdout ----
---- src/builtin.rs - builtin::DEREF_NULLPTR (line 2976) stdout ----
error: cannot find macro `addr_of` in this scope
  |
  |
4 |     addr_of!(std::ptr::null::<i32>())
  |
  = note: consider importing one of these items:
          core::ptr::addr_of
          std::ptr::addr_of
---
---- src/builtin.rs - builtin::DEREF_NULLPTR (line 2971) stdout ----
error[E0308]: mismatched types
 --> src/builtin.rs:2973:5
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_builtin_rs_2971_0() {
  |                                                                               - help: try adding a return type: `-> &i32`
3 | unsafe {
4 |     &*core::ptr::null::<i32>()
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&i32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:24:22
