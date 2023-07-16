plain
travis_time:end:01014b7c:start=1559226135180126665,finish=1559226137334986621,duration=2154859956
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:18] .................................................................................................... 3800/5598
[01:10:22] ..ii................................................................................................ 3900/5598
[01:10:24] .......................i............................................................................ 4000/5598
[01:10:26] .......................................................................................i............ 4100/5598
[01:10:28] .........................................................................................F.......... 4200/5598
[01:10:47] .................................................................................................... 4400/5598
[01:10:50] .................................................................................................... 4500/5598
[01:10:53] .................................................................................................... 4600/5598
[01:10:57] .................................................................................................... 4700/5598
---
[01:11:34] 1 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:4:11
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:6:11
[01:11:34] 3    |
[01:11:34] 4 LL |     const _: () = ();
[01:11:34] 
[01:11:34] 6 
[01:11:34] 7 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:7:11
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:7:11
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:9:11
[01:11:34] 9    |
[01:11:34] 10 LL |     const _: () = ();
[01:11:34] 
[01:11:34] 12 
[01:11:34] 13 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:10:11
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:10:11
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:12:11
[01:11:34] 15    |
[01:11:34] 16 LL |     const _: () = ();
[01:11:34] 
[01:11:34] 18 
[01:11:34] 19 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:15:8
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:15:8
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:17:8
[01:11:34] 21    |
[01:11:34] 22 LL | static _: () = ();
[01:11:34] 
[01:11:34] 24 
[01:11:34] 25 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:16:8
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:16:8
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:18:8
[01:11:34] 27    |
[01:11:34] 28 LL | struct _();
[01:11:34] 
[01:11:34] 30 
[01:11:34] 31 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:17:6
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:17:6
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:19:6
[01:11:34] 33    |
[01:11:34] 34 LL | enum _ {}
[01:11:34] 
[01:11:34] 36 
[01:11:34] 37 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:18:4
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:18:4
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:20:4
[01:11:34] 39    |
[01:11:34] 40 LL | fn _() {}
[01:11:34] 
[01:11:34] 42 
[01:11:34] 43 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:19:5
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:19:5
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:21:5
[01:11:34] 45    |
[01:11:34] 46 LL | mod _ {}
[01:11:34] 
[01:11:34] 48 
[01:11:34] 49 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:20:6
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:20:6
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:22:6
[01:11:34] 51    |
[01:11:34] 52 LL | type _ = ();
[01:11:34] 
[01:11:34] 54 
[01:11:34] 55 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:21:5
---
[01:11:34] 61 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:22:5
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:24:5
[01:11:34] 63    |
[01:11:34] 64 LL | use _ as g;
[01:11:34] 
[01:11:34] 66 
[01:11:34] 67 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:23:7
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:23:7
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:25:7
[01:11:34] 69    |
[01:11:34] 70 LL | trait _ {}
[01:11:34] 
[01:11:34] 72 
[01:11:34] 73 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:24:7
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:24:7
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:26:7
[01:11:34] 75    |
[01:11:34] 76 LL | trait _ = Copy;
[01:11:34] 
[01:11:34] 78 
[01:11:34] 79 error: expected identifier, found reserved identifier `_`
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:25:14
[01:11:34] -   --> $DIR/underscore_item_not_const.rs:25:14
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:27:14
[01:11:34] 81    |
[01:11:34] 82 LL | macro_rules! _ { () => {} }
[01:11:34] 
[01:11:34] 84 
[01:11:34] 84 
[01:11:34] 85 error: expected one of `!` or `::`, found `_`
[01:11:34] +   --> $DIR/underscore_item_not_const.rs:28:7
[01:11:34] 87    |
[01:11:34] 87    |
[01:11:34] 88 LL | union _ { f: u8 }
[01:11:34] 89    |       ^ expected one of `!` or `::` here
[01:11:34] 
[01:11:34] The actual stderr differed from the expected stderr.
[01:11:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore_item_not_const/underscore_item_not_const.stderr
[01:11:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore_item_not_const/underscore_item_not_const.stderr
[01:11:34] To update references, rerun the tests and pass the `--bless` flag
[01:11:34] To only update this specific test, also pass `--test-args parser/underscore_item_not_const.rs`
[01:11:34] error: 1 errors occurred comparing output.
[01:11:34] status: exit code: 1
[01:11:34] status: exit code: 1
[01:11:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/underscore_item_not_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore_item_not_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore_item_not_const/auxiliary" "-A" "unused"
[01:11:34] ------------------------------------------
[01:11:34] 
[01:11:34] ------------------------------------------
[01:11:34] stderr:
[01:11:34] stderr:
[01:11:34] ------------------------------------------
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:6:11
[01:11:34]    |
[01:11:34] LL |     const _: () = (); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:9:11
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL |     const _: () = (); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:12:11
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL |     const _: () = (); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:17:8
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | static _: () = (); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:18:8
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | struct _(); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:19:6
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | enum _ {} //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:20:4
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | fn _() {} //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:21:5
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | mod _ {} //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:22:6
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | type _ = (); //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:23:5
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | use _; //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:24:5
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | use _ as g; //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:25:7
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | trait _ {} //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:26:7
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | trait _ = Copy; //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] error: expected identifier, found reserved identifier `_`
[01:11:34]   --> /checkout/src/test/ui/parser/underscore_item_not_const.rs:27:14
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | macro_rules! _ { () => {} } //~ ERROR expected identifier, found reserved identifier `_`
[01:11:34] 
[01:11:34] 
[01:11:34] error: expected one of `!` or `::`, found `_`
[01:11:34]    |
[01:11:34]    |
[01:11:34] LL | union _ { f: u8 } //~ ERROR expected one of `!` or `::`, found `_`
[01:11:34]    |       ^ expected one of `!` or `::` here
[01:11:34] error: aborting due to 15 previous errors
[01:11:34] 
[01:11:34] 
[01:11:34] ------------------------------------------
---
[01:11:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:11:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:11:34] 
[01:11:34] 
[01:11:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:34] 
[01:11:34] 
[01:11:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:34] Build completed unsuccessfully in 0:04:46
[01:11:34] Build completed unsuccessfully in 0:04:46
[01:11:34] make: *** [check] Error 1
[01:11:34] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003c0f34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 15:34:02 UTC 2019
---
travis_time:end:00ccfe56:start=1559230443555786556,finish=1559230443560789183,duration=5002627
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d8dfc68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:224bc0bc
travis_time:start:224bc0bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03e06140
$ dmesg | grep -i kill
