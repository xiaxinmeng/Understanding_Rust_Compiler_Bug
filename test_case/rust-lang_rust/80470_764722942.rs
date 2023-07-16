plain
Successfully built ef9d50b97d38
Successfully tagged rust-ci:latest
Built container sha256:ef9d50b97d388e6ba90f8388b0997d746b3b0aae438d65cbb1bfb909e713d215
Uploading finished image to https://ci-caches.rust-lang.org/docker/1d8776fc6611839c0fe1104032e04cb412cf84a696fcac0418b82e4341b3688a883196f99e2f55a0df0b3c67d154c62dd0c4630956d70292523c69687b63cce6
upload failed: - to s3://rust-lang-ci-sccache2/docker/1d8776fc6611839c0fe1104032e04cb412cf84a696fcac0418b82e4341b3688a883196f99e2f55a0df0b3c67d154c62dd0c4630956d70292523c69687b63cce6 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.01s

 finished in 2.085 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 20 tests
iiiiiiiiiiii........

 finished in 0.517 seconds
Build completed successfully in 0:29:22
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11271 tests
.....................i.............................................................................. 100/11271
.................................iiiiiiii.iiiiiiiii................................................. 200/11271
.................................................................................................... 400/11271
.................................................................................................... 500/11271
.................................................................................................... 600/11271
....................................................i............................................... 700/11271
