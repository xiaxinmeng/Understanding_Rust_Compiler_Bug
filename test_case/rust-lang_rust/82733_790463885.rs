plain
.................................................................................................... 9300/11524
.................................................................................................... 9400/11524
.........................................................................i......i................... 9500/11524
.................................................................................................... 9600/11524
............iiiiiii..iiiiii.i....................................................................... 9700/11524
.................................................................................................... 9900/11524
.................................................................................................... 10000/11524
.................................................................................................... 10100/11524
.................................................................................................... 10200/11524
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
test result: ok. 0 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 0.067 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.168 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.393 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
platform support check
   Compiling tier-check v0.1.0 (/checkout/src/tools/tier-check)
    Finished release [optimized] target(s) in 0.84s
     Running `build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/tier-check /checkout/src/doc/rustc/src/platform-support.md /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`
error: target `powerpc-unknown-openbsd` is missing from platform-support.md
If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
expected success, got: exit code: 1
