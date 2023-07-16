plain
travis_time:end:1cb57348:start=1549977252492255155,finish=1549977329767070025,duration=77274814870
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:00] ...............................i.................................................................... 4600/5382
[01:01:06] .................................................................................................... 4700/5382
[01:01:10] .................................................................................................... 4800/5382
[01:01:13] .................................................................................................... 4900/5382
[01:01:18] ..................................................F................................................. 5000/5382
[01:01:25] .................................................................................................... 5200/5382
[01:01:27] .................................................................................................... 5300/5382
[01:01:30] .....................i............................................................
[01:01:30] failures:
[01:01:30] failures:
[01:01:30] 
[01:01:30] ---- [ui] ui/traits/trait-alias-syntax.rs stdout ----
[01:01:30] diff of stderr:
[01:01:30] 
[01:01:30] 1 error: trait aliases cannot be `auto`
[01:01:30] +   --> $DIR/trait-alias-syntax.rs:4:19
[01:01:30] 3    |
[01:01:30] 3    |
[01:01:30] 4 LL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
[01:01:30] 5    |                   ^ trait aliases cannot be `auto`
[01:01:30] 6 
[01:01:30] 6 
[01:01:30] 7 error: trait aliases cannot be `unsafe`
[01:01:30] +   --> $DIR/trait-alias-syntax.rs:5:21
[01:01:30] 9    |
[01:01:30] 9    |
[01:01:30] 10 LL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
[01:01:30] 11    |                     ^ trait aliases cannot be `unsafe`
[01:01:30] 
[01:01:30] The actual stderr differed from the expected stderr.
[01:01:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/trait-alias-syntax.stderr
[01:01:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/trait-alias-syntax.stderr
[01:01:30] To update references, rerun the tests and pass the `--bless` flag
[01:01:30] To only update this specific test, also pass `--test-args traits/trait-alias-syntax.rs`
[01:01:30] error: 1 errors occurred comparing output.
[01:01:30] status: exit code: 1
[01:01:30] status: exit code: 1
[01:01:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/auxiliary" "-A" "unused"
[01:01:30] ------------------------------------------
[01:01:30] 
[01:01:30] ------------------------------------------
[01:01:30] stderr:
[01:01:30] stderr:
[01:01:30] ------------------------------------------
[01:01:30] {"message":"trait aliases cannot be `auto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":57,"byte_end":58,"line_start":4,"line_end":4,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`","highlight_start":19,"highlight_end":20}],"label":"trait aliases cannot be `auto`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `auto`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:4:19\n   |\nLL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`\n   |                   ^ trait aliases cannot be `auto`\n\n"}
[01:01:30] {"message":"trait aliases cannot be `unsafe`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":120,"byte_end":121,"line_start":5,"line_end":5,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`","highlight_start":21,"highlight_end":22}],"label":"trait aliases cannot be `unsafe`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `unsafe`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:5:21\n   |\nLL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`\n   |                     ^ trait aliases cannot be `unsafe`\n\n"}
[01:01:30] 
[01:01:30] ------------------------------------------
[01:01:30] 
[01:01:30] thread '[ui] ui/traits/trait-alias-syntax.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:01:30] 
[01:01:30] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:01:30] 
[01:01:30] 
[01:01:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:30] 
[01:01:30] 
[01:01:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:30] Build completed unsuccessfully in 0:04:23
[01:01:30] Build completed unsuccessfully in 0:04:23
[01:01:30] make: *** [check] Error 1
[01:01:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04380ebb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 14:17:09 UTC 2019
---
travis_time:end:0b0d4cb0:start=1549981031339372162,finish=1549981031345076463,duration=5704301
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22d46618
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b0dd34
travis_time:start:01b0dd34
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:017d45cd
$ dmesg | grep -i kill
