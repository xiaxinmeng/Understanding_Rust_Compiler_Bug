plain
travis_time:end:27f82cf7:start=1554235015031692802,finish=1554235117419707651,duration=102388014849
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:11]  ---> db9a066852ee
[00:03:11] Successfully built db9a066852ee
[00:03:11] Successfully tagged rust-ci:latest
[00:03:11] Built container sha256:db9a066852eee64eab0a084abd5b7f9d4e3635b47821fd20b076f8e66ac131fb
[00:03:11] Uploading finished image to s3://rust-lang-ci-sccache2/docker/973de87470b38111ed63aaa9e3fc83e135764932f8cac26887a71ca620c6032c14db7f27b0283baf1c1a98f66c5250c601c891a572bcee56949feaa946b80dc4
[00:03:58] upload failed: - to s3://rust-lang-ci-sccache2/docker/973de87470b38111ed63aaa9e3fc83e135764932f8cac26887a71ca620c6032c14db7f27b0283baf1c1a98f66c5250c601c891a572bcee56949feaa946b80dc4 Unable to locate credentials

[00:03:58] travis_time:end:14e4f0e6:start=1554235239896928490,finish=1554235364464513262,duration=124567584772
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:03:58] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:05] 
[01:18:05] running 98 tests
[01:18:20] .............................F...F................................................................
[01:18:20] failures:
[01:18:20] 
[01:18:20] ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
[01:18:20] 
[01:18:20] 
[01:18:20] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:18:20] status: exit code: 1
[01:18:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
[01:18:20] ------------------------------------------
[01:18:20] 
[01:18:20] ------------------------------------------
[01:18:20] stderr:
[01:18:20] stderr:
[01:18:20] ------------------------------------------
[01:18:20] {"message":"`MirBuilt(make_extern)` should be dirty but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/function_interfaces.rs","byte_start":2817,"byte_end":2851,"line_start":99,"line_end":99,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"pub extern \"C\" fn make_extern() {}","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirBuilt(make_extern)` should be dirty but is not\n  --> /checkout/src/test/incremental/hashes/function_interfaces.rs:99:1\n   |\nLL | pub extern \"C\" fn make_extern() {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:18:20] 
[01:18:20] ------------------------------------------
[01:18:20] 
[01:18:20] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:20] thread '[incremental] incremental/hashes/function_interfaces.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:18:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:20] 
[01:18:20] ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
[01:18:20] 
[01:18:20] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:18:20] status: exit code: 1
[01:18:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
[01:18:20] ------------------------------------------
[01:18:20] 
[01:18:20] ------------------------------------------
[01:18:20] stderr:
[01:18:20] stderr:
[01:18:20] ------------------------------------------
[01:18:20] {"message":"`MirBuilt(Foo::make_method_extern)` should be dirty but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/inherent_impls.rs","byte_start":6595,"byte_end":6638,"line_start":268,"line_end":268,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    pub extern fn make_method_extern(&self) { }","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirBuilt(Foo::make_method_extern)` should be dirty but is not\n  --> /checkout/src/test/incremental/hashes/inherent_impls.rs:268:5\n   |\nLL |     pub extern fn make_method_extern(&self) { }\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:18:20] 
[01:18:20] ------------------------------------------
[01:18:20] 
[01:18:20] thread '[incremental] incremental/hashes/inherent_impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:18:20] test result: FAILED. 96 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:18:20] 
[01:18:20] 
[01:18:20] 
[01:18:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:20] 
[01:18:20] 
[01:18:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:20] Build completed unsuccessfully in 0:11:46
[01:18:20] Build completed unsuccessfully in 0:11:46
[01:18:20] Makefile:48: recipe for target 'check' failed
[01:18:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a71785d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  2 21:17:07 UTC 2019
