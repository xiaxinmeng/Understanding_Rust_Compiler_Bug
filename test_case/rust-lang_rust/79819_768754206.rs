plain
Successfully built a3a234a3a638
Successfully tagged rust-ci:latest
Built container sha256:a3a234a3a638d5d2d38b6fb823f832aa6490802f3ff732f57b241981cbe1fffc
Uploading finished image to https://ci-caches.rust-lang.org/docker/1c72b7d037d305d35e0735812f08df148b36d5d74ee11961d19c0c3224e3b4bbf04ff3ebce61c1c56645202b549f50aac19493c18cafdab069ba5b3de9c503bb
upload failed: - to s3://rust-lang-ci-sccache2/docker/1c72b7d037d305d35e0735812f08df148b36d5d74ee11961d19c0c3224e3b4bbf04ff3ebce61c1c56645202b549f50aac19493c18cafdab069ba5b3de9c503bb Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.068 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.451 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i....ii..i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.322 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
    --> /checkout/compiler/rustc_lint_defs/src/lib.rs:402:11
     |
402  |           $(#[$attr])*
     | 
    ::: /checkout/compiler/rustc_lint_defs/src/builtin.rs:2837:1
     |
     |
2837 | / declare_lint! {
2838 | |     /// The `semicolon_in_expressions_from_macros` lint detects trailing semicolons
2839 | |     /// in macro bodies when the macro is invoked in expression position.
2840 | |     /// This was previous accepted, but is being phased out.
2880 | |     };
2881 | | }
     | |_- in this macro invocation
     |
     |
     = note: `-D invalid-codeblock-attributes` implied by `-D warnings`
     = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:22:36
