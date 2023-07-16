plain
travis_time:end:023c53b0:start=1554396556544060677,finish=1554396647813703033,duration=91269642356
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:24] .................................................................................................... 3100/5522
[01:08:27] .................................................................................................... 3200/5522
[01:08:32] .................................................................................................... 3300/5522
[01:08:35] ......................i............................................................................. 3400/5522
[01:08:39] ................................................................................................ii.. 3500/5522
[01:08:43] .i..ii.............................................................................................. 3600/5522
[01:08:47] ................F................................................................................... 3700/5522
[01:08:54] .......ii........................................................................................... 3900/5522
[01:08:56] .........................i.......................................................................... 4000/5522
[01:08:58] .....................................................................................i.............. 4100/5522
[01:09:01] .................................................................................................... 4200/5522
---
[01:09:58] 
[01:09:58] ---- [ui] ui/nll/issue-47388.rs stdout ----
[01:09:58] diff of stderr:
[01:09:58] 
[01:09:58] 1 error[E0594]: cannot assign to `fancy_ref.num` which is behind a `&` reference
[01:09:58] 3    |
[01:09:58] 3    |
[01:09:58] - LL |     let fancy_ref = &(&mut fancy);
[01:09:58] -    |                     ------------- help: consider changing this to be a mutable reference: `&mut (&mut fancy)`
[01:09:58] 6 LL |     fancy_ref.num = 6;
[01:09:58] 7    |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written
[01:09:58] 
[01:09:58] 
[01:09:58] The actual stderr differed from the expected stderr.
[01:09:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/issue-47388.stderr
[01:09:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/issue-47388.stderr
[01:09:58] To update references, rerun the tests and pass the `--bless` flag
[01:09:58] To only update this specific test, also pass `--test-args nll/issue-47388.rs`
[01:09:58] error: 1 errors occurred comparing output.
[01:09:58] status: exit code: 1
[01:09:58] status: exit code: 1
[01:09:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-47388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-47388/auxiliary" "-A" "unused"
[01:09:58] ------------------------------------------
[01:09:58] 
[01:09:58] ------------------------------------------
[01:09:58] stderr:
[01:09:58] stderr:
[01:09:58] ------------------------------------------
[01:09:58] {"message":"cannot assign to `fancy_ref.num` which is behind a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-47388.rs","byte_start":142,"byte_end":159,"line_start":9,"line_end":9,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    fancy_ref.num = 6; //~ ERROR E0594","highlight_start":5,"highlight_end":22}],"label":"`fancy_ref` is a `&` reference, so the data it refers to cannot be written","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to `fancy_ref.num` which is behind a `&` reference\n  --> /checkout/src/test/ui/nll/issue-47388.rs:9:5\n   |\nLL |     fancy_ref.num = 6; //~ ERROR E0594\n   |     ^^^^^^^^^^^^^^^^^ `fancy_ref` is a `&` reference, so the data it refers to cannot be written\n\n"}
[01:09:58] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[01:09:58] 
[01:09:58] ------------------------------------------
[01:09:58] 
---
[01:09:58] 
[01:09:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:09:58] 
[01:09:58] 
[01:09:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:58] 
[01:09:58] 
[01:09:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:58] Build completed unsuccessfully in 0:04:27
[01:09:58] Build completed unsuccessfully in 0:04:27
[01:09:58] Makefile:48: recipe for target 'check' failed
[01:09:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25624344
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 18:00:55 UTC 2019
---
travis_time:end:2a1c7a40:start=1554400856799884046,finish=1554400856804891109,duration=5007063
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036f04c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07e30066
travis_time:start:07e30066
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000211ab
$ dmesg | grep -i kill
