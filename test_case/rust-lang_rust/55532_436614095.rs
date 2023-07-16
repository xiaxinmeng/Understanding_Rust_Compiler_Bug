plain
travis_time:end:0a662696:start=1541592328806653768,finish=1541592384665233196,duration=55858579428
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:48] .................................................................................................... 2600/4997
[00:48:51] .................................................................................................... 2700/4997
[00:48:55] .................................................................................................... 2800/4997
[00:48:57] .................................................................................................... 2900/4997
[00:49:01] ....................................F............................................................... 3000/4997
[00:49:07] ......................................................................................i.i..ii....... 3200/4997
[00:49:11] .................................................................................................... 3300/4997
[00:49:14] .................................................................................................... 3400/4997
[00:49:17] ...........................................................i.ii..................................... 3500/4997
---
[00:49:59] 
[00:49:59] ---- [ui] ui/lint/lint-type-overflow2.rs stdout ----
[00:49:59] diff of stderr:
[00:49:59] 
[00:49:59] 34 LL |     let x =  1.7976931348623159e+308_f64; //~ warn: literal out of range for f64
[00:49:59] 36 
[00:49:59] - warning: attempt to negate with overflow
[00:49:59] + warning: this expression will panic at runtime
[00:49:59] 38   --> $DIR/lint-type-overflow2.rs:19:18
[00:49:59] 38   --> $DIR/lint-type-overflow2.rs:19:18
[00:49:59] 39    |
[00:49:59] 40 LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[00:49:59] -    |                  ^^^^^
[00:49:59] +    |                  ^^^^^ attempt to negate with overflow
[00:49:59] 42    |
[00:49:59] 43 note: lint level defined here
[00:49:59] 43 note: lint level defined here
[00:49:59] 44   --> $DIR/lint-type-overflow2.rs:13:9
[00:49:59] 
[00:49:59] 45    |
[00:49:59] 46 LL | #![warn(const_err)]
[00:49:59] - 
[00:49:59] - warning: this expression will panic at runtime
[00:49:59] -   --> $DIR/lint-type-overflow2.rs:19:18
[00:49:59] -    |
[00:49:59] -    |
[00:49:59] - LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[00:49:59] -    |                  ^^^^^ attempt to negate with overflow
[00:49:59] 55 
[00:49:59] 
[00:49:59] 
[00:49:59] The actual stderr differed from the expected stderr.
[00:49:59] The actual stderr differed from the expected stderr.
[00:49:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
[00:49:59] To update references, rerun the tests and pass the `--bless` flag
[00:49:59] To only update this specific test, also pass `--test-args lint/lint-type-overflow2.rs`
[00:49:59] error: 1 errors occurred comparing output.
[00:49:59] status: exit code: 0
[00:49:59] status: exit code: 0
[00:49:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/auxiliary" "-A" "unused"
[00:49:59] ------------------------------------------
[00:49:59] 
[00:49:59] ------------------------------------------
[00:49:59] stderr:
[00:49:59] stderr:
[00:49:59] ------------------------------------------
[00:49:59] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":597,"byte_end":600,"line_start":19,"line_end":19,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    let x2: i8 = --128; //~ warn: literal out of range for i8","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":478,"byte_end":498,"line_start":12,"line_end":12,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(overflowing_literals)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:19:20\n   |\nLL |     let x2: i8 = --128; //~ warn: literal out of range for i8\n   |                    ^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:12:9\n   |\nLL | #![warn(overflowing_literals)]\n   |         ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:59] {"message":"literal out of range for f32","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":706,"byte_end":724,"line_start":21,"line_end":21,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let x = -3.40282357e+38_f32; //~ warn: literal out of range for f32","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f32\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:21:14\n   |\nLL |     let x = -3.40282357e+38_f32; //~ warn: literal out of range for f32\n   |              ^^^^^^^^^^^^^^^^^^\n\n"}
[00:49:59] {"message":"literal out of range for f32","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":778,"byte_end":796,"line_start":22,"line_end":22,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let x =  3.40282357e+38_f32; //~ warn: literal out of range for f32","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f32\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:22:14[00:49:59] thread '[ui] ui/lint/lint-type-overflow2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:49:59] 
[00:49:59] 
[00:49:59] failures:
[00:49:59]     [ui] ui/lint/lint-type-overflow2.rs
[00:49:59]     [ui] ui/lint/lint-type-overflow2.rs
[00:49:59] 
[00:49:59] test result: FAILED. 4972 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:49:59] 
[00:49:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:59] 
[00:49:59] 
[00:49:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:59] 
[00:49:59] 
[00:49:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:59] Build completed unsuccessfully in 0:03:33
[00:49:59] Build completed unsuccessfully in 0:03:33
[00:49:59] make: *** [check] Error 1
[00:49:59] Makefile:58: recipe for target 'check' failed
2344960 ./obj
2344920 ./obj/build
1709152 ./obj/build/x86_64-unknown-linux-gnu
1193884 ./.git
---
143000 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
137216 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
137212 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6g5uzwb8l-53l2y9-22tmsi8iacpi9
130756 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130752 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123680 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
111080 ./src/llvm/test/CodeGen
---
56044 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build
55856 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
55852 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
55848 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
55844 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-vis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:274fc138
travis_time:start:274fc138
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:084f7dd8
$ dmesg | grep -i kill
