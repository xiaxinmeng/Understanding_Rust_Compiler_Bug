plain
 ---> 3e9b7da7489f
Successfully built 3e9b7da7489f
Successfully tagged rust-ci:latest
Built container sha256:3e9b7da7489fc390607588da3bd1e5c03652dafd3540e27fcfb1c89b65c6dff7
Uploading finished image to https://ci-caches.rust-lang.org/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174
upload failed: - to s3://rust-lang-ci-sccache2/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
.................................................................................................... 9400/11763
.................................................................................................... 9500/11763
............................................................................................i......i 9600/11763
.................................................................................................... 9700/11763
......................................iiiiiii..iiiiii.i............................................. 9800/11763
.................................................................................................... 10000/11763
.................................................................................................... 10100/11763
.................................................................................................... 10200/11763
.................................................................................................... 10300/11763
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.166 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.356 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 200/254
.........................................F............
failures:

---- tests::test_show stdout ----
thread 'tests::test_show' panicked at 'assertion failed: `(left == right)`
  left: `"Any { .. }"`,
 right: `"Any"`', library/alloc/src/tests.rs:52:5

failures:
    tests::test_show


test result: FAILED. 253 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

error: test failed, to rerun pass '-p alloc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:41
