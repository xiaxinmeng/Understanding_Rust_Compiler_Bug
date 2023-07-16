plain
Successfully built 3e0fa85229d6
Successfully tagged rust-ci:latest
Built container sha256:3e0fa85229d602c7b0d64b13828342b151bae28ba64881bea3dfca62108fcb2f
Uploading finished image to https://ci-caches.rust-lang.org/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b
upload failed: - to s3://rust-lang-ci-sccache2/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
.................................................................................................... 9400/11715
.................................................................................................... 9500/11715
........................................................i......i.................................... 9600/11715
.................................................................................................... 9700/11715
..iiiiiii...iiiiiii................................................................................. 9800/11715
.................................................................................................... 10000/11715
.................................................................................................... 10100/11715
.................................................................................................... 10200/11715
.................................................................................................... 10300/11715
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.112 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.416 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

   Doc-tests core

running 2843 tests
iiiiii.................F............................................................................ 100/2843
...ii............................................................................................... 300/2843
....................................................i............................................... 400/2843
.................................................................................................... 500/2843
.........................i..i...................iiii................................................ 600/2843
---
---- src/bool.rs - bool::bool::take (line 39) stdout ----
error[E0658]: use of unstable library feature 'bool_take'
 --> src/bool.rs:41:14
  |
5 | assert_eq!(b.take(), true);
  |
  |
  = help: add `#![feature(bool_take)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'bool_take'
 --> src/bool.rs:42:14
  |
  |
6 | assert_eq!(b.take(), false);
  |
  |
  = help: add `#![feature(bool_take)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:51
