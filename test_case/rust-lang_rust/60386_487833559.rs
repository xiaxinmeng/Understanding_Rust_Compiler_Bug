plain
travis_time:end:1b608217:start=1556601178754700657,finish=1556601262808219352,duration=84053518695
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:11:25] .................................................................................................... 4300/5471
[01:11:29] .................................................................................................... 4400/5471
[01:11:32] .................................................................................................... 4500/5471
[01:11:36] .................................................................................................... 4600/5471
[01:11:41] .......................................................................F............................ 4700/5471
[01:11:49] .................................................................................................... 4900/5471
[01:11:53] .................................................................................................... 5000/5471
[01:11:56] .................................................................................................... 5100/5471
[01:12:00] .................................................................................................... 5200/5471
---
[01:12:08] ------------------------------------------
[01:12:08] stderr:
[01:12:08] ------------------------------------------
[01:12:08] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:12:08]   left: `[":22] Unit = Unit", ":23] a = Unit", ":29] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":30] b = Point {", "    x: 42,", "    y: 24,", "}", ":38]", ":42] &a = NoCopy(", "    1337,", ")", ":42] dbg!(& a) = NoCopy(", "    1337,", ")", ":47] f(&42) = 42", "before", ":52] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":60] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":63] 1 = 1", ":63] 2 = 2", ":67] 1u8 = 1", ":67] 2u32 = 2", ":67] \"Yeah\" = \"Yeah\""]`,
[01:12:08]  right: `[":22] Unit = Unit", ":23] a = Unit", ":29] Point{x: 42, y: 24,} = Point {", "    x: 42,", "    y: 24,", "}", ":30] b = Point {", "    x: 42,", "    y: 24,", "}", ":38]", ":42] &a = NoCopy(", "    1337,", ")", ":42] dbg!(& a) = NoCopy(", "    1337,", ")", ":47] f(&42) = 42", "before", ":52] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":52] { foo += 1; eprintln!(\"before\"); 7331 } = 7331", ":60] (\"Yeah\",) = (", "    \"Yeah\",", ")", ":63] 1 = 1", ":63] 2 = 2", ":67] 1u8 = 1", ":67] 2u32 = 2", ":67] \"Yeah\" = \"Yeah\""]`', /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:72:5
[01:12:08] 
[01:12:08] ------------------------------------------
[01:12:08] 
[01:12:08] 
---
[01:12:08] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:12:08] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:08] 
[01:12:08] 
[01:12:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:08] 
[01:12:08] 
[01:12:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:08] Build completed unsuccessfully in 0:04:22
[01:12:08] Build completed unsuccessfully in 0:04:22
[01:12:08] make: *** [check] Error 1
[01:12:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d26106c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 06:26:41 UTC 2019
---
travis_time:end:00a9bf32:start=1556605602339685939,finish=1556605602344297309,duration=4611370
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3784f28c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08ab0280
travis_time:start:08ab0280
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:029e522f
$ dmesg | grep -i kill
