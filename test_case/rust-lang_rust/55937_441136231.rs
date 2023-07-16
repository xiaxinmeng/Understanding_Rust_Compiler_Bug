plain
travis_time:end:024ad94c:start=1542928391353980032,finish=1542928392503369898,duration=1149389866
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:40] .................................................................................................... 100/5048
[00:49:43] .................................................................................................... 200/5048
[00:49:46] .............................ii............................................ii...................ii.. 300/5048
[00:49:49] ..............................................................................................iii... 400/5048
[00:49:51] .....iiiiiiii.iii............................iii...........................................i........ 500/5048
[00:49:58] .................................................................................................... 700/5048
[00:50:04] ...................................................................................i...........i.... 800/5048
[00:50:07] .................................................................................................... 900/5048
[00:50:10] ..iiiii..................ii.iiii.................................................................... 1000/5048
---
[00:50:44] .................................................................................................... 2200/5048
[00:50:48] .................................................................................................... 2300/5048
[00:50:52] .................................................................................................... 2400/5048
[00:50:56] .................................................................................................... 2500/5048
[00:50:59] ...........................................................................................iiiiiiiii 2600/5048
[00:51:06] .........................................................ii......................................... 2800/5048
[00:51:09] .................................................................................................... 2900/5048
[00:51:12] .................................................................................................... 3000/5048
[00:51:15] .....................................................i.............................................. 3100/5048
---
[00:56:57] .......................i............................................................................ 2400/2886
[00:57:08] .................................................................................................... 2500/2886
[00:57:38] .................................................................................................... 2600/2886
[00:57:48] .................................................................................................... 2700/2886
[00:57:57] ........F........................................................................................... 2800/2886
ser type UserTypeAnnotation(0) vs fn(i32) -> Newt<i32> {Newt<i32>}: NoSolution","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/ufcs-polymorphic-paths.rs","byte_start":2324,"byte_end":2335,"line_start":90,"line_end":90,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    Newt::<i32>, fn(i32) -> Newt<i32>, (5);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: broken MIR in DefId(0/0:62 ~ ufcs_polymorphic_paths[317d]::main[0]::C[5]) (const Newt): bad constant user type UserTypeAnnotation(0) vs fn(i32) -> Newt<i32> {Newt<i32>}: NoSolution\n  --> /checkout/src/test/run-pass/ufcs-polymorphic-paths.rs:90:5\n   |\nLL |     Newt::<i32>, fn(i32) -> Newt<i32>, (5);\n   |     ^^^^^^^^^^^\n\n"}
[00:58:08] {"message":"broken MIR in DefId(0/0:59 ~ ufcs_polymorphic_paths[317d]::main[0]::S[3]) (const std::prelude::v1::Some): bad constant user type UserTypeAnnotation(0) vs fn(i32) -> std::option::Option<i32> {std::option::Option<i32>::Some}: NoSolution","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/ufcs-polymorphic-paths.rs","byte_start":2207,"byte_end":2218,"line_start":86,"line_end":86,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    Some::<i32>, fn(i32) -> Option<i32>, (5);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: inter
[00:58:08] 
[00:58:08] note: the compiler unexpectedly panicked. this is a bug.
[00:58:08] 
[00:58:08] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:08] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:08] 
[00:58:08] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:58:08] 
[00:58:08] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:58:08] 
[00:58:08] ------------------------------------------
[00:58:08] 
[00:58:08] thread '[run-pass] run-pass/ufcs-polymorphic-paths.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
---
[00:58:08] 
[00:58:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:58:08] 
[00:58:08] 
[00:58:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:08] 
[00:58:08] 
[00:58:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:08] Build completed unsuccessfully in 0:12:15
[00:58:08] Build completed unsuccessfully in 0:12:15
[00:58:08] make: *** [check] Error 1
[00:58:08] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ee97094
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 23 00:11:30 UTC 2018
---
travis_time:end:0059eac8:start=1542931891586816424,finish=1542931891594009028,duration=7192604
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:189d1110
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13193b09
travis_time:start:13193b09
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:067adca8
$ dmesg | grep -i kill
