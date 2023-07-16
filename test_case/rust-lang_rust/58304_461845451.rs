plain
travis_time:end:046b0bae:start=1549636456292843745,finish=1549636531242800166,duration=74949956421
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:08:17] .................................................................................................... 4300/5377
[01:08:20] .................................................................................................... 4400/5377
[01:08:24] .................................................................................................... 4500/5377
[01:08:29] ..........................i......................................................................... 4600/5377
[01:08:35] ........................................................................F........................... 4700/5377
[01:08:42] .................................................................................................... 4900/5377
[01:08:46] .................................................................................................... 5000/5377
[01:08:50] .................................................................................................... 5100/5377
[01:08:53] .................................................................................................... 5200/5377
[01:08:53] .................................................................................................... 5200/5377
[01:08:56] .................................................................................................... 5300/5377
[01:08:59] ................i............................................................
[01:08:59] failures:
[01:08:59] 
[01:08:59] ---- [ui] ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs stdout ----
[01:08:59] diff of stderr:
[01:08:59] 
[01:08:59] 1 error[E0511]: invalid monomorphization of `simd_saturating_add` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type
[01:08:59] -   --> $DIR/simd-intrinsic-generic-arithmetic-saturating.rs:31:9
[01:08:59] +   --> $DIR/simd-intrinsic-generic-arithmetic-saturating.rs:33:9
[01:08:59] 3    |
[01:08:59] 4 LL |         simd_saturating_add(z, z);
[01:08:59] 
[01:08:59] 6 
[01:08:59] 6 
[01:08:59] 7 error[E0511]: invalid monomorphization of `simd_saturating_sub` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type
[01:08:59] -   --> $DIR/simd-intrinsic-generic-arithmetic-saturating.rs:33:9
[01:08:59] +   --> $DIR/simd-intrinsic-generic-arithmetic-saturating.rs:35:9
[01:08:59] 9    |
[01:08:59] 10 LL |         simd_saturating_sub(z, z);
[01:08:59] 
[01:08:59] 
[01:08:59] The actual stderr differed from the expected stderr.
[01:08:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.stderr
[01:08:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/simd-intrinsic-generic-arithmetic-saturating.stderr
[01:08:59] To update references, rerun the tests and pass the `--bless` flag
[01:08:59] To only update this specific test, also pass `--test-args simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs`
[01:08:59] error: 1 errors occurred comparing output.
[01:08:59] status: exit code: 1
[01:08:59] status: exit code: 1
[01:08:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating/auxiliary" "-A" "unused"
[01:08:59] ------------------------------------------
[01:08:59] 
[01:08:59] ------------------------------------------
[01:08:59] stderr:
[01:08:59] stderr:
[01:08:59] ------------------------------------------
[01:08:59] {"message":"invalid monomorphization of `simd_saturating_add` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs","byte_start":799,"byte_end":824,"line_start":33,"line_end":33,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        simd_saturating_add(z, z);","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_saturating_add` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:33:9\n   |\nLL |         simd_saturating_add(z, z);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:08:59] {"message":"invalid monomorphization of `simd_saturating_sub` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs","byte_start":944,"byte_end":969,"line_start":35,"line_end":35,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        simd_saturating_sub(z, z);","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_saturating_sub` intrinsic: expected element type `f32` of vector type `f32x4` to be a signed or unsigned integer type\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-arithmetic-saturating.rs:35:9\n   |\nLL |         simd_saturating_sub(z, z);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:08:59] {"message":"For more information about this error, try `rustc --explain E0511`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0511`.\n"}
[01:08:59] 
[01:08:59] ------------------------------------------
[01:08:59] 
---
[01:08:59] 
[01:08:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:08:59] 
[01:08:59] 
[01:08:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:08:59] 
[01:08:59] 
[01:08:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:59] Build completed unsuccessfully in 0:04:29
[01:08:59] Build completed unsuccessfully in 0:04:29
[01:08:59] Makefile:48: recipe for target 'check' failed
[01:08:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01833ac3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 15:44:40 UTC 2019
---
travis_time:end:08849358:start=1549640682097515510,finish=1549640682103235757,duration=5720247
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05427f68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!ch
