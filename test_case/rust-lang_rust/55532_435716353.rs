plain
[01:11:30] 
[01:11:30] ---- [ui] ui/lint/lint-type-overflow2.rs stdout ----
[01:11:30] diff of stderr:
[01:11:30] 
[01:11:30] 34 LL |     let x =  1.7976931348623159e+308_f64; //~ warn: literal out of range for f64
[01:11:30] 36 
[01:11:30] - warning: this expression will panic at runtime
[01:11:30] - warning: this expression will panic at runtime
[01:11:30] + warning: attempt to negate with overflow
[01:11:30] 39    |
[01:11:30] 39    |
[01:11:30] 40 LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[01:11:30] -    |                  ^^^^^ attempt to negate with overflow
[01:11:30] +    |                  ^^^^^
[01:11:30] 42    |
[01:11:30] 43 note: lint level defined here
[01:11:30] 43 note: lint level defined here
[01:11:30] 44   --> $DIR/lint-type-overflow2.rs:13:9
[01:11:30] 
[01:11:30] 45    |
[01:11:30] 46 LL | #![warn(const_err)]
[01:11:30] + 
[01:11:30] + warning: this expression will panic at runtime
[01:11:30] +   --> $DIR/lint-type-overflow2.rs:19:18
[01:11:30] +    |
[01:11:30] +    |
[01:11:30] + LL |     let x2: i8 = --128; //~ warn: literal out of range for i8
[01:11:30] +    |                  ^^^^^ attempt to negate with overflow
[01:11:30] 49 
[01:11:30] 
[01:11:30] 
[01:11:30] The actual stderr differed from the expected stderr.
[01:11:30] The actual stderr differed from the expected stderr.
[01:11:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/lint-type-overflow2.stderr
[01:11:30] To update references, rerun the tests and pass the `--bless` flag
[01:11:30] To only update this specific test, also pass `--test-args lint/lint-type-overflow2.rs`
[01:11:30] error: 1 errors occurred comparing output.
[01:11:30] status: exit code: 0
[01:11:30] status: exit code: 0
[01:11:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-type-overflow2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-type-overflow2/auxiliary" "-A" "unused"
[01:11:30] ------------------------------------------
[01:11:30] 
[01:11:30] ------------------------------------------
[01:11:30] stderr:
[01:11:30] stderr:
[01:11:30] ------------------------------------------
[01:11:30] {"message":"literal out of range for i8","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":597,"byte_end":600,"line_start":19,"line_end":19,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"    let x2: i8 = --128; //~ warn: literal out of range for i8","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":478,"byte_end":498,"line_start":12,"line_end":12,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(overflowing_literals)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: literal out of range for i8\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:19:20\n   |\nLL |     let x2: i8 = --128; //~ warn: literal out of range for i8\n   |                    ^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:12:9\n   |\nLL | #![warn(overflowing_literals)]\n   |         ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:30] {"message":"literal out of range for f32","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":654,"byte_end":672,"line_start":21,"line_end":21,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let x = -3.40282357e+38_f32; //~ warn: literal out of range for f32","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f32\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:21:14\n   |\nLL |     let x = -3.40282357e+38_f32; //~ warn: literal out of range for f32\n   |              ^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:30] {"message":"literal out of range for f32","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":726,"byte_end":744,"line_start":22,"line_end":22,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let x =  3.40282357e+38_f32; //~ warn: literal out of range for f32","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f32\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:22:14\n   |\nLL |     let x =  3.40282357e+38_f32; //~ warn: literal out of range for f32\n   |              ^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:30] {"message":"literal out of range for f64","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":798,"byte_end":825,"line_start":23,"line_end":23,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"    let x = -1.7976931348623159e+308_f64; //~ warn: literal out of range for f64","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f64\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:23:14\n   |\nLL |     let x = -1.7976931348623159e+308_f64; //~ warn: literal out of range for f64\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:30] {"message":"literal out of range for f64","code":{"code":"overflowing_literals","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":879,"byte_end":906,"line_start":24,"line_end":24,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"    let x =  1.7976931348623159e+308_f64; //~ warn: literal out of range for f64","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: literal out of range for f64\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:24:14\n   |\nLL |     let x =  1.7976931348623159e+308_f64; //~ warn: literal out of range for f64\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:11:30] {"message":"attempt to negate with overflow","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":595,"byte_end":600,"line_start":19,"line_end":19,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"    let x2: i8 = --128; //~ warn: literal out of range for i8","highlight_start":18,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":509,"byte_end":518,"line_start":13,"line_end":13,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"#![warn(const_err)]","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: attempt to negate with overflow\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:19:18\n   |\nLL |     let x2: i8 = --128; //~ warn: literal out of range for i8\n   |                  ^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:13:9\n   |\nLL | #![warn(const_err)]\n   |         ^^^^^^^^^\n\n"}
[01:11:30] {"message":"this expression will panic at runtime","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lint-type-overflow2.rs","byte_start":595,"byte_end":600,"line_start":19,"line_end":19,"column_start":18,"column_end":23,"is_primary":true,"text":[{"text":"    let x2: i8 = --128; //~ warn: literal out of range for i8","highlight_start":18,"highlight_end":23}],"label":"attempt to negate with overflow","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: this expression will panic at runtime\n  --> /checkout/src/test/ui/lint/lint-type-overflow2.rs:19:18\n   |\nLL |     let x2: i8 = --128; //~ warn: literal out of range for i8\n   |                  ^^^^^ attempt to negate with overflow\n\n"}
[01:11:30] ------------------------------------------
[01:11:30] 
[01:11:30] thread '[ui] ui/lint/lint-type-overflow2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:11:30] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:11:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:11:30] 
[01:11:30] 
[01:11:30] 
[01:11:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:30] 
[01:11:30] 
[01:11:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:30] Build completed unsuccessfully in 0:04:07
[01:11:30] Build completed unsuccessfully in 0:04:07
[01:11:30] make: *** [check] Error 1
[01:11:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:018ff906
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0a0faff7:start=1541372107584086224,finish=1541372107592674328,duration=8588104
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002969b8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cf5a3de
travis_time:start:0cf5a3de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0242e098
$ dmesg | grep -i kill
