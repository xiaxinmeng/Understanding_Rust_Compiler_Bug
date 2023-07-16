plain
travis_time:end:16098de2:start=1546507615723167172,finish=1546507674067662956,duration=58344495784
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:31] .................................................................................................... 4200/5225
[01:01:34] .................................................................................................... 4300/5225
[01:01:38] .................................................................................i.................. 4400/5225
[01:01:45] .................................................................................................... 4500/5225
[01:01:49] ....................................F............................................................... 4600/5225
[01:01:57] .................................................................................................... 4800/5225
[01:02:00] .................................................................................................... 4900/5225
[01:02:03] .................................................................................................... 5000/5225
[01:02:06] .................................................................................................... 5100/5225
[01:02:06] .................................................................................................... 5100/5225
[01:02:09] ................................................................i................................... 5200/5225
[01:02:10] .........................
[01:02:10] failures:
[01:02:10] 
[01:02:10] ---- [ui] ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs stdout ----
[01:02:10] diff of stderr:
[01:02:10] 
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u2`
[01:02:10] +   --> $DIR/simd-intrinsic-generic-bitmask.rs:67:21
[01:02:10] +    |
[01:02:10] + LL |         let _: u8 = simd_bitmask(m2);
[01:02:10] + 
[01:02:10] + 
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u4`
[01:02:10] +   --> $DIR/simd-intrinsic-generic-bitmask.rs:68:21
[01:02:10] +    |
[01:02:10] + LL |         let _: u8 = simd_bitmask(m4);
[01:02:10] + 
[01:02:10] + 
[01:02:10] 1 error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u8`
[01:02:10] +   --> $DIR/simd-intrinsic-generic-bitmask.rs:70:22
[01:02:10] +    |
[01:02:10] + LL |         let _: u16 = simd_bitmask(m16);
[01:02:10] + 
[01:02:10] + 
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`
[01:02:10] +   --> $DIR/simd-intrinsic-generic-bitmask.rs:71:22
[01:02:10] +    |
[01:02:10] + LL |         let _: u32 = simd_bitmask(m32);
[01:02:10] + 
[01:02:10] + 
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`
[01:02:10] +   --> $DIR/simd-intrinsic-generic-bitmask.rs:72:22
[01:02:10] +    |
[01:02:10] + LL |         let _: u64 = simd_bitmask(m64);
[01:02:10] + 
[01:02:10] + 
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u2`
[01:02:10] 2   --> $DIR/simd-intrinsic-generic-bitmask.rs:74:22
[01:02:10] 3    |
[01:02:10] 4 LL |         let _: u16 = simd_bitmask(m2);
[01:02:10] 
[01:02:10] 10 LL |         let _: u16 = simd_bitmask(m8);
[01:02:10] 12 
[01:02:10] 12 
[01:02:10] - error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u16`
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`
[01:02:10] 14   --> $DIR/simd-intrinsic-generic-bitmask.rs:80:22
[01:02:10] 15    |
[01:02:10] 16 LL |         let _: u32 = simd_bitmask(m16);
[01:02:10] 17    |                      ^^^^^^^^^^^^^^^^^
[01:02:10] 18 
[01:02:10] 18 
[01:02:10] - error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u32`
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`
[01:02:10] 20   --> $DIR/simd-intrinsic-generic-bitmask.rs:83:22
[01:02:10] 21    |
[01:02:10] 22 LL |         let _: u64 = simd_bitmask(m32);
[01:02:10] 23    |                      ^^^^^^^^^^^^^^^^^
[01:02:10] 24 
[01:02:10] 24 
[01:02:10] - error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u128`, expected `u64`
[01:02:10] + error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u128`, expected `u8`
[01:02:10] 26   --> $DIR/simd-intrinsic-generic-bitmask.rs:86:23
[01:02:10] 27    |
[01:02:10] 28 LL |         let _: u128 = simd_bitmask(m64);
[01:02:10] 29    |                       ^^^^^^^^^^^^^^^^^
[01:02:10] 30 
[01:02:10] - error: aborting due to 5 previous errors
[01:02:10] + error: aborting due to 10 previous errors
[01:02:10] + error: aborting due to 10 previous errors
[01:02:10] 32 
[01:02:10] 33 For more information about this error, try `rustc --explain E0511`.
[01:02:10] 34 
[01:02:10] 
[01:02:10] 
[01:02:10] The actual stderr differed from the expected stderr.
[01:02:10] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.stderr
[01:02:10] To update references, rerun the tests and pass the `--bless` flag
[01:02:10] To only update this specific test, also pass `--test-args simd-intrinsic/simd-intrinsic-generic-bitmask.rs`
[01:02:10] error: 1 errors occurred comparing output.
[01:02:10] status: exit code: 1
[01:02:10] status: exit code: 1
[01:02:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask/auxiliary" "-A" "unused"
[01:02:10] ------------------------------------------
[01:02:10] 
[01:02:10] ------------------------------------------
[01:02:10] stderr:
[01:02:10] stderr:
[01:02:10] ------------------------------------------
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u2`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2272,"byte_end":2288,"line_start":67,"line_end":67,"column_start":21,"column_end":37,"is_primary":true,"text":[{"text":"        let _: u8 = simd_bitmask(m2);","highlight_start":21,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u2`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:67:21\n   |\nLL |         let _: u8 = simd_bitmask(m2);\n   |                     ^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u4`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2310,"byte_end":2326,"line_start":68,"line_end":68,"column_start":21,"column_end":37,"is_primary":true,"text":[{"text":"        let _: u8 = simd_bitmask(m4);","highlight_start":21,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u8`, expected `u4`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:68:21\n   |\nLL |         let _: u8 = simd_bitmask(m4);\n   |                     ^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2387,"byte_end":2404,"line_start":70,"line_end":70,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"        let _: u16 = simd_bitmask(m16);","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:70:22\n   |\nLL |         let _: u16 = simd_bitmask(m16);\n   |                      ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2427,"byte_end":2444,"line_start":71,"line_end":71,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"        let _: u32 = simd_bitmask(m32);","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:71:22\n   |\nLL |         let _: u32 = simd_bitmask(m32);\n   |                      ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2467,"byte_end":2484,"line_start":72,"line_end":72,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"        let _: u64 = simd_bitmask(m64);","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:72:22\n   |\nLL |         let _: u64 = simd_bitmask(m64);\n   |                      ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u2`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2508,"byte_end":2524,"line_start":74,"line_end":74,"column_start":22,"column_end":38,"is_primary":true,"text":[{"text":"        let _: u16 = simd_bitmask(m2);","highlight_start":22,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u2`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:74:22\n   |\nLL |         let _: u16 = simd_bitmask(m2);\n   |                      ^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2596,"byte_end":2612,"line_start":77,"line_end":77,"column_start":22,"column_end":38,"is_primary":true,"text":[{"text":"        let _: u16 = simd_bitmask(m8);","highlight_start":22,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u16`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:77:22\n   |\nLL |         let _: u16 = simd_bitmask(m8);\n   |                      ^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2684,"byte_end":2701,"line_start":80,"line_end":80,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"        let _: u32 = simd_bitmask(m16);","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u32`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:80:22\n   |\nLL |         let _: u32 = simd_bitmask(m16);\n   |                      ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2774,"byte_end":2791,"line_start":83,"line_end":83,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"        let _: u64 = simd_bitmask(m32);","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u64`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:83:22\n   |\nLL |         let _: u64 = simd_bitmask(m32);\n   |                      ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u128`, expected `u8`","code":{"code":"E0511","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs","byte_start":2865,"byte_end":2882,"line_start":86,"line_end":86,"column_start":23,"column_end":40,"is_primary":true,"text":[{"text":"        let _: u128 = simd_bitmask(m64);","highlight_start":23,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0511]: invalid monomorphization of `simd_bitmask` intrinsic: bitmask `u128`, expected `u8`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:86:23\n   |\nLL |         let _: u128 = simd_bitmask(m64);\n   |                       ^^^^^^^^^^^^^^^^^\n\n"}
[01:02:10] {"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}
[01:02:10] {"message":"For more information about this error, try `rustc --explain E0511`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0511`.\n"}
[01:02:10] ------------------------------------------
[01:02:10] 
[01:02:10] thread '[ui] ui/simd-intrinsic/simd-intrinsic-generic-bitmask.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:02:10] 
---
[01:02:10] 
[01:02:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:02:10] 
[01:02:10] 
[01:02:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:10] 
[01:02:10] 
[01:02:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:10] Build completed unsuccessfully in 0:04:06
[01:02:10] Build completed unsuccessfully in 0:04:06
[01:02:10] Makefile:48: recipe for target 'check' failed
[01:02:10] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25a3320e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan  3 10:30:13 UTC 2019
---
travis_time:end:0eb961d3:start=1546511414220776297,finish=1546511414231680324,duration=10904027
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:287eda60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:071cbf86
$ dmesg | grep -i kill
