plain
travis_time:end:358d39b2:start=1549941271478237868,finish=1549941272462697366,duration=984459498
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:56:31] ..............................i..................................................................... 4600/5381
[00:56:36] .................................................................................................... 4700/5381
[00:56:39] .................................................................................................... 4800/5381
[00:56:43] .................................................................................................... 4900/5381
[00:56:46] ................................................F................................................... 5000/5381
[00:56:53] .................................................................................................... 5200/5381
[00:56:55] .................................................................................................... 5300/5381
[00:56:57] ....................i............................................................
[00:56:57] failures:
[00:56:57] failures:
[00:56:57] 
[00:56:57] ---- [ui] ui/traits/trait-alias-syntax.rs stdout ----
[00:56:57] diff of stderr:
[00:56:57] 
[00:56:57] 1 error: trait aliases cannot be `auto`
[00:56:57] +   --> $DIR/trait-alias-syntax.rs:4:19
[00:56:57] 3    |
[00:56:57] 3    |
[00:56:57] 4 LL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
[00:56:57] 5    |                   ^ trait aliases cannot be `auto`
[00:56:57] 6 
[00:56:57] 6 
[00:56:57] 7 error: trait aliases cannot be `unsafe`
[00:56:57] +   --> $DIR/trait-alias-syntax.rs:5:21
[00:56:57] 9    |
[00:56:57] 9    |
[00:56:57] 10 LL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
[00:56:57] 11    |                     ^ trait aliases cannot be `unsafe`
[00:56:57] 
[00:56:57] The actual stderr differed from the expected stderr.
[00:56:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/trait-alias-syntax.stderr
[00:56:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/trait-alias-syntax.stderr
[00:56:57] To update references, rerun the tests and pass the `--bless` flag
[00:56:57] To only update this specific test, also pass `--test-args traits/trait-alias-syntax.rs`
[00:56:57] error: 1 errors occurred comparing output.
[00:56:57] status: exit code: 1
[00:56:57] status: exit code: 1
[00:56:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/auxiliary" "-A" "unused"
[00:56:57] ------------------------------------------
[00:56:57] 
[00:56:57] ------------------------------------------
[00:56:57] stderr:
[00:56:57] stderr:
[00:56:57] ------------------------------------------
[00:56:57] {"message":"trait aliases cannot be `auto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":57,"byte_end":58,"line_start":4,"line_end":4,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`","highlight_start":19,"highlight_end":20}],"label":"trait aliases cannot be `auto`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `auto`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:4:19\n   |\nLL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`\n   |                   ^ trait aliases cannot be `auto`\n\n"}
[00:56:57] {"message":"trait aliases cannot be `unsafe`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":120,"byte_end":121,"line_start":5,"line_end":5,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`","highlight_start":21,"highlight_end":22}],"label":"trait aliases cannot be `unsafe`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `unsafe`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:5:21\n   |\nLL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`\n   |                     ^ trait aliases cannot be `unsafe`\n\n"}
[00:56:57] 
[00:56:57] ------------------------------------------
[00:56:57] 
[00:56:57] thread '[ui] ui/traits/trait-alias-syntax.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[00:56:57] 
[00:56:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[00:56:57] 
[00:56:57] 
[00:56:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:57] 
[00:56:57] 
[00:56:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:57] Build completed unsuccessfully in 0:03:55
[00:56:57] Build completed unsuccessfully in 0:03:55
[00:56:57] make: *** [check] Error 1
[00:56:57] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fe150ed
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 04:11:40 UTC 2019
---
travis_time:end:1bdd093a:start=1549944701266284833,finish=1549944701270593134,duration=4308301
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06dd99d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d5ef110
travis_time:start:1d5ef110
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:085bdb2d
$ dmesg | grep -i kill
