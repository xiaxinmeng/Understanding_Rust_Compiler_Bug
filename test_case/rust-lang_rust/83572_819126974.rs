plain
.................................................................................................... 9400/11755
.................................................................................................... 9500/11755
.........................................................................................i......i... 9600/11755
.................................................................................................... 9700/11755
...................................iiiiiii..iiiiii.i................................................ 9800/11755
.................................................................................................... 10000/11755
.................................................................................................... 10100/11755
.................................................................................................... 10200/11755
.................................................................................................... 10300/11755
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.172 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.982 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.366 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
................................................................................................F... 100/168
....................................................................
failures:

---- spec::tests::powerpc64le_unknown_freebsd stdout ----
thread 'spec::tests::powerpc64le_unknown_freebsd' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_target/src/spec/powerpc64le_unknown_freebsd.rs:6:52


failures:
    spec::tests::powerpc64le_unknown_freebsd
    spec::tests::powerpc64le_unknown_freebsd

test result: FAILED. 167 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_target --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:33
