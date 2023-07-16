plain
travis_time:end:13322194:start=1560018496448215263,finish=1560018585083139017,duration=88634923754
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:58:16] .................................................................................................... 3800/5657
[00:58:19] .........................................................ii......................................... 3900/5657
[00:58:22] ..............................................................................i..................... 4000/5657
[00:58:24] .................................................................................................... 4100/5657
[00:58:26] ..........................................i..........................................F.............. 4200/5657
[00:58:41] .................................................................................................... 4400/5657
[00:58:47] .................................................................................................... 4500/5657
[00:58:50] .................................................................................................... 4600/5657
[00:58:54] .................................................................................................... 4700/5657
---
[00:59:32] 
[00:59:32] ---- [ui] ui/parser/raw-byte-string-literals.rs stdout ----
[00:59:32] diff of stderr:
[00:59:32] 
[00:59:32] 1 error: bare CR not allowed in string, use \r instead
[00:59:32] +   --> $DIR/raw-byte-string-literals.rs:4:9
[00:59:32] 3    |
[00:59:32] 4 LL |     br"a
";
---
[00:59:32] 10 LL |     br"é";
[00:59:32] 11    |        ^
[00:59:32] 
[00:59:32] 12 
[00:59:32] 13 error: found invalid character; only `#` is allowed in raw string delimitation: ~
[00:59:32] +   --> $DIR/raw-byte-string-literals.rs:6:6
[00:59:32] 15    |
[00:59:32] 15    |
[00:59:32] 16 LL |     br##~"a"~##;
[00:59:32] 
[00:59:32] 
[00:59:32] The actual stderr differed from the expected stderr.
[00:59:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/raw-byte-string-literals.stderr
[00:59:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/raw-byte-string-literals.stderr
[00:59:32] To update references, rerun the tests and pass the `--bless` flag
[00:59:32] To only update this specific test, also pass `--test-args parser/raw-byte-string-literals.rs`
[00:59:32] error: 1 errors occurred comparing output.
[00:59:32] status: exit code: 1
[00:59:32] status: exit code: 1
[00:59:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/auxiliary" "-A" "unused"
[00:59:32] ------------------------------------------
[00:59:32] 
[00:59:32] ------------------------------------------
[00:59:32] stderr:
[00:59:32] stderr:
[00:59:32] ------------------------------------------
[00:59:32] error: bare CR not allowed in string, use \r instead
[00:59:32]    |
[00:59:32] LL |     br"a
[00:59:32] LL |     br"a
"; //~ ERROR bare CR not allowed in string
[00:59:32] 
[00:59:32] error: raw byte string must be ASCII
[00:59:32]   --> /checkout/src/test/ui/parser/raw-byte-string-literals.rs:5:8
[00:59:32]    |
[00:59:32]    |
[00:59:32] LL |     br"é";  //~ ERROR raw byte string must be ASCII
[00:59:32] 
[00:59:32] 
[00:59:32] error: found invalid character; only `#` is allowed in raw string delimitation: ~
[00:59:32]    |
[00:59:32]    |
[00:59:32] LL |     br##~"a"~##;  //~ ERROR only `#` is allowed in raw string delimitation
[00:59:32] 
[00:59:32] error: aborting due to 3 previous errors
[00:59:32] 
[00:59:32] 
---
[00:59:32] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:59:32] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:32] 
[00:59:32] 
[00:59:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:32] 
[00:59:32] 
[00:59:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:32] Build completed unsuccessfully in 0:55:36
---
travis_time:end:05477f8e:start=1560022168009779236,finish=1560022168014750157,duration=4970921
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23233db1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:243bd2dc
travis_time:start:243bd2dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b0162c8
$ dmesg | grep -i kill
