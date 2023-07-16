plain
Successfully built aa36b6f4096f
Successfully tagged rust-ci:latest
Built container sha256:aa36b6f4096f7b79c4b27156837116f2e7527bb2a2dfe98e3ddbad98482a9717
Uploading finished image to https://ci-caches.rust-lang.org/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b
upload failed: - to s3://rust-lang-ci-sccache2/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.096 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.315 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:27
