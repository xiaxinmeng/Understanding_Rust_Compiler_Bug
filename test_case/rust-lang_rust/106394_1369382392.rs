plain
Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-13       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in 9b39a4aeb96d
Removing intermediate container 9b39a4aeb96d
 ---> 47983d4e4455
Step 8/8 : ENV SCRIPT ../x.py --stage 2 test --exclude src/tools/tidy &&            ../x --stage 2 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            ../x.ps1 --stage 2 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python3 ../x.py --stage 1 clippy -Awarnings &&            python2.7 ../x.py --stage 2 test src/tools/tidy
Removing intermediate container 1456a8f334ed
 ---> 95f766289a1d
Successfully built 95f766289a1d
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:95f766289a1d834a14cc07dd8e965248850c504581ab069816f7c584c2e8580d
Uploading finished image to https://ci-caches.rust-lang.org/docker/50c21f6e2f5ef4c86ad09dc6e8d3aa1311dd8436059f85515c315f744947a64d9fa6784b6a6482f78e422a00c9f99bf388e6923ca240a5dc5bf5409e818e2837
upload failed: - to s3://rust-lang-ci-sccache2/docker/50c21f6e2f5ef4c86ad09dc6e8d3aa1311dd8436059f85515c315f744947a64d9fa6784b6a6482f78e422a00c9f99bf388e6923ca240a5dc5bf5409e818e2837 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
test result: ok. 13896 passed; 0 failed; 199 ignored; 0 measured; 0 filtered out; finished in 81.47s

 finished in 82.197 seconds
Build completed successfully in 0:01:24
+ python3 ../x.py --stage 1 clippy -Awarnings
    Finished dev [unoptimized] target(s) in 0.05s
Building stage0 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.20s
Copying stage0 library from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 47.57s
Building stage0 tool cargo-clippy (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.21s
Checking stage1 library artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
thread 'main' panicked at 'could not run cargo: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/tools/clippy/src/main.rs:159:10
Build completed unsuccessfully in 0:00:48
