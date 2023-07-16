plain
travis_time:end:0a12ef63:start=1546278334149687329,finish=1546278389704758779,duration=55555071450
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:54] Successfully built 8034cc8d63c6
[00:02:54] Successfully tagged rust-ci:latest
[00:02:54] Built container sha256:8034cc8d63c6324aba380661aeed7f9bc1dd2842d144292c8119a6f336e807dc
[00:02:54] Uploading finished image to s3://rust-lang-ci-sccache2/docker/db9ff5d5ee373ab73faca5f449fef19a4b2c51bffa521b7f96dff69f63bf41d6914752d34f01e887e0176ba22b40cd598769a822432ae334a71fa8d5844e7fa6
[00:03:38] upload failed: - to s3://rust-lang-ci-sccache2/docker/db9ff5d5ee373ab73faca5f449fef19a4b2c51bffa521b7f96dff69f63bf41d6914752d34f01e887e0176ba22b40cd598769a822432ae334a71fa8d5844e7fa6 Unable to locate credentials

[00:03:38] travis_time:end:2a30c8e1:start=1546278475414904266,finish=1546278617605272790,duration=142190368524
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:03:38] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:51:11] .................................................................................................... 400/5115
[00:51:15] .................................................................................................... 500/5115
[00:51:18] ..............................i..................................................................... 600/5115
[00:51:22] .................................................................................................... 700/5115
[00:51:27] ..........................................................................................F......... 800/5115
[00:51:31] ..i...............i................................................................................. 900/5115
[00:51:35] .........................iiiii...................................................................... 1000/5115
[00:51:40] .................................................................................................... 1200/5115
[00:51:43] .................................................................................................... 1300/5115
[00:51:45] .................................................................................................... 1400/5115
[00:51:47] .................................................................................................... 1500/5115
---
[00:53:51] 
[00:53:51] ---- [ui] ui/consts/static_mut_containing_mut_ref2.rs stdout ----
[00:53:51] diff of stderr:
[00:53:51] 
[00:53:51] 4 LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[00:53:51] 5    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[00:53:51] 6 
[00:53:51] - error[E0019]: static contains unimplemented expression type
[00:53:51] -    |
[00:53:51] -    |
[00:53:51] - LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[00:53:51] + error: aborting due to previous error
[00:53:51] 12 
[00:53:51] - error: aborting due to 2 previous errors
[00:53:51] - 
[00:53:51] - 
[00:53:51] - Some errors occurred: E0017, E0019.
[00:53:51] - For more information about an error, try `rustc --explain E0017`.
[00:53:51] + For more information about this error, try `rustc --explain E0017`.
[00:53:51] 17 
[00:53:51] 
[00:53:51] 
[00:53:51] The actual stderr differed from the expected stderr.
[00:53:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/static_mut_containing_mut_ref2.stderr
[00:53:51] To update references, rerun the tests and pass the `--bless` flag
[00:53:51] To olobal mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs","byte_start":110,"byte_end":136,"line_start":5,"line_end":5,"column_start":46,"column_end":72,"is_primary":true,"text":[{"text":"pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };","highlight_start":46,"highlight_end":72}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs:5:46\n   |\nLL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };\n   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values\n\n"}
[00:53:51] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:51] {"message":"For more information about this error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0017`.\n"}
[00:53:51] ------------------------------------------
[00:53:51] 
[00:53:51] thread '[ui] ui/consts/static_mut_containing_mut_ref2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:53:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:53:51] 
[00:53:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:53:51] 
[00:53:51] 
[00:53:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:51] 
[00:53:51] 
[00:53:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:51] Build completed unsuccessfully in 0:04:02
[00:53:51] Build completed unsuccessfully in 0:04:02
[00:53:51] Makefile:58: recipe for target 'check' failed
[00:53:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00624070
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 31 18:40:31 UTC 2018
