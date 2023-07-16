plain
.................................................................................................... 9400/11727
.................................................................................................... 9500/11727
.....................................................................i......i....................... 9600/11727
.................................................................................................... 9700/11727
.................iiiiiii.iiiiiii.................................................................... 9800/11727
.................................................................................................... 10000/11727
.................................................................................................... 10100/11727
.................................................................................................... 10200/11727
.................................................................................................... 10300/11727
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.101 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.350 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.428 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
platform support check
   Compiling tier-check v0.1.0 (/checkout/src/tools/tier-check)
    Finished release [optimized] target(s) in 0.84s
     Running `build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/tier-check /checkout/src/doc/rustc/src/platform-support.md /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`
error: target `i386-unknown-linux-gnu` is missing from platform-support.md

If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.
If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.
error: target `i486-unknown-linux-gnu` is missing from platform-support.md
If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:31:19
