plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.073 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.387 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
platform support check
   Compiling tier-check v0.1.0 (/checkout/src/tools/tier-check)
    Finished release [optimized] target(s) in 0.87s
     Running `build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/tier-check /checkout/src/doc/rustc/src/platform-support.md /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`
error: target `aarch64-apple-ios-sim` is missing from platform-support.md

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:31:30
