plain

   Doc-tests rustc_lint_defs

running 92 tests
ii..i.....i......i.ii.i.............iii.......................i...F.........i...............

---- src/builtin.rs - builtin::RESERVED_PREFIX (line 3250) stdout ----
---- src/builtin.rs - builtin::RESERVED_PREFIX (line 3250) stdout ----
Test compiled successfully, but it's marked `compile_fail`.
failures:
    src/builtin.rs - builtin::RESERVED_PREFIX (line 3250)

test result: FAILED. 78 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.70s
test result: FAILED. 78 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.70s

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:14
