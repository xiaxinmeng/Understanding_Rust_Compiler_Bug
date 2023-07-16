plain
Step 6/7 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-12       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in 7d76e4ef6d48
Removing intermediate container 7d76e4ef6d48
 ---> 9a38edd1429d
Step 7/7 : ENV SCRIPT python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator             --exclude linkchecker --stage 1 --exclude src/tools/rustdoc --exclude rustdoc-json-types             --exclude tidy --exclude ui-fulldeps &&            python2.7 ../x.py test --stage 0 error_index_generator src/tools/rustdoc rustdoc-json-types &&            python2.7 ../x.py --stage 1 test src/test/mir-opt                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/test/ui --pass=check                              --host='' --target=i686-unknown-linux-gnu &&            python2.7 ../x.py --stage 1 test src/tools/tidy
Removing intermediate container 2b6bc8c07b76
 ---> 402f22a34605
Successfully built 402f22a34605
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:402f22a3460536113137dd76e973d345747b6dbb3df426d23401f3a2eed1f6a6
Uploading finished image to https://ci-caches.rust-lang.org/docker/bc9e2d360047a9c12a614fba12e2c6f9d24df69567c15af1abb779c1eac9ede3e1fb6ced5a90e26bc3d2f216a498944b0f70598c907b23856b82e3480f135301
upload failed: - to s3://rust-lang-ci-sccache2/docker/bc9e2d360047a9c12a614fba12e2c6f9d24df69567c15af1abb779c1eac9ede3e1fb6ced5a90e26bc3d2f216a498944b0f70598c907b23856b82e3480f135301 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
[CI_JOB_NAME=x86_64-gnu-llvm-12-stage1]
---
processor : 5
vendor_id : GenuineIntel
cpu family : 6
model  : 85
+ python2.7 ../x.py test --exclude run-make-fulldeps --exclude error_index_generator --exclude linkchecker --stage 1 --exclude src/tools/rustdoc --exclude rustdoc-json-types --exclude tidy --exclude ui-fulldeps
stepping : 7
microcode : 0xffffffff
cpu MHz  : 2593.906
cache size : 36608 KB
---
test result: ok. 37 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out; finished in 2.59s

 finished in 2.646 seconds
Build completed successfully in 0:19:33
+ python2.7 ../x.py test --stage 0 error_index_generator src/tools/rustdoc rustdoc-json-types
    Finished dev [unoptimized] target(s) in 0.04s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
   Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
    Finished release [optimized] target(s) in 3.10s
Testing error-index stage0
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md
thread 'main' panicked at 'failed printing to stdout: Resource temporarily unavailable (os error 11)', library/std/src/io/stdio.rs:1015:9



command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1040 tests
