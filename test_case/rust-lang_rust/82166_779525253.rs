plain
.................................................................................................... 9200/11451
.................................................................................................... 9300/11451
.................................................................................................... 9400/11451
..........i......i.................................................................................. 9500/11451
................................................iiiiiii..iiiiii.i................................... 9600/11451
.................................................................................................... 9800/11451
.................................................................................................... 9900/11451
.................................................................................................... 10000/11451
.................................................................................................... 10100/11451
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.080 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.463 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
platform support check
   Compiling tier-check v0.1.0 (/checkout/src/tools/tier-check)
    Finished release [optimized] target(s) in 0.93s
     Running `build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/tier-check /checkout/src/doc/rustc/src/platform-support.md /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`
error: target `s390x-unknown-linux-musl` is missing from platform-support.md


If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:32:30
