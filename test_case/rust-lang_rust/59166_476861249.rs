plain
travis_time:end:13bbd0d8:start=1553631601328924305,finish=1553631678603404965,duration=77274480660
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:10:36] ...........................i........................................................................ 4700/5490
[01:10:43] .................................................................................................... 4800/5490
[01:10:46] .................................................................................................... 4900/5490
[01:10:50] .................................................................................................... 5000/5490
[01:10:54] .....................................................F.............................................. 5100/5490
[01:11:01] .................................................................................................... 5300/5490
[01:11:04] .................................................................................................... 5400/5490
[01:11:07] ............................i.............................................................
[01:11:07] failures:
[01:11:07] failures:
[01:11:07] 
[01:11:07] ---- [ui] ui/traits/trait-alias-ambiguous.rs stdout ----
[01:11:07] diff of stderr:
[01:11:07] 
[01:11:07] - error[E0034]: multiple applicable items in scope
[01:11:07] -   --> $DIR/trait-alias-ambiguous.rs:16:9
[01:11:07] + error: expected one of `(`, `+`, `::`, `;`, `<`, or `where`, found `=`
[01:11:07] +   --> $DIR/trait-alias-ambiguous.rs:14:21
[01:11:07] 3    |
[01:11:07] - LL |     t.foo();
[01:11:07] -    |       ^^^ multiple `foo` found
[01:11:07] - note: candidate #1 is defined in the trait `A`
[01:11:07] -   --> $DIR/trait-alias-ambiguous.rs:4:19
[01:11:07] -    |
[01:11:07] - LL |     pub trait A { fn foo(&self); }
[01:11:07] -    |                   ^^^^^^^^^^^^^^
[01:11:07] -    = help: to disambiguate the method call, write `A::foo(t)` instead
[01:11:07] - note: candidate #2 is defined in the trait `B`
[01:11:07] -   --> $DIR/trait-alias-ambiguous.rs:5:19
[01:11:07] -    |
[01:11:07] - LL |     pub trait B { fn foo(&self); }
[01:11:07] -    |                   ^^^^^^^^^^^^^^
[01:11:07] -    = help: to disambiguate the method call, write `B::foo(t)` instead
[01:11:07] + LL |     pub trait C = A = B;
[01:11:07] +    |                     ^ expected one of `(`, `+`, `::`, `;`, `<`, or `where` here
[01:11:07] 19 error: aborting due to previous error
[01:11:07] 20 
[01:11:07] 
[01:11:07] - For more information about this error, try `rustc --explain E0034`.
[01:11:07] - For more information about this error, try `rustc --explain E0034`.
[01:11:07] 22 
[01:11:07] 
[01:11:07] 
[01:11:07] The actual stderr differed from the expected stderr.
[01:11:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/trait-alias-ambiguous.stderr
[01:11:07] To update references, rerun the tests and pass the `--bless` flag
[01:11:07] To only update this specific test, also pass `--test-args traits/trait-alias-ambiguous.rs`
[01:11:07] error: 1 errors occurred comparing output.
[01:11:07] status: exit code: 1
[01:11:07] status: exit code: 1
[01:11:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-ambiguous/auxiliary" "-A" "unused"
[01:11:07] ------------------------------------------
[01:11:07] 
[01:11:07] ------------------------------------------
[01:11:07] stderr:
[01:11:07] stderr:
[01:11:07] ------------------------------------------
[01:11:07] {"message":"expected one of `(`, `+`, `::`, `;`, `<`, or `where`, found `=`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-ambiguous.rs","byte_start":232,"byte_end":233,"line_start":14,"line_end":14,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    pub trait C = A = B;","highlight_start":21,"highlight_end":22}],"label":"expected one of `(`, `+`, `::`, `;`, `<`, or `where` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `(`, `+`, `::`, `;`, `<`, or `where`, found `=`\n  --> /checkout/src/test/ui/traits/trait-alias-ambiguous.rs:14:21\n   |\nLL |     pub trait C = A = B;\n   |                     ^ expected one of `(`, `+`, `::`, `;`, `<`, or `where` here\n\n"}
[01:11:07] 
[01:11:07] ------------------------------------------
[01:11:07] 
[01:11:07] thread '[ui] ui/traits/trait-alias-ambiguous.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3369:9
---
[01:11:07] 
[01:11:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:11:07] 
[01:11:07] 
[01:11:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:07] 
[01:11:07] 
[01:11:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:07] Build completed unsuccessfully in 0:04:27
[01:11:07] Build completed unsuccessfully in 0:04:27
[01:11:07] make: *** [check] Error 1
[01:11:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05455f80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 21:32:35 UTC 2019
---
travis_time:end:014bfb00:start=1553635957036813691,finish=1553635957044206167,duration=7392476
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:187d2911
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b1dd638
$ dmesg | grep -i kill
