plain
travis_time:end:2891c1e8:start=1548373286162712927,finish=1548373287110645168,duration=947932241
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:11:54] .................................................................................................... 3500/5327
[01:11:58] .................................................................................................... 3600/5327
[01:12:01] ......................................................ii............................................ 3700/5327
[01:12:04] ........................................................................i........................... 3800/5327
[01:12:06] ................................................F................................................... 3900/5327
[01:12:11] .................................................................................................... 4100/5327
[01:12:23] .................................................................................................... 4200/5327
[01:12:26] .................................................................................................... 4300/5327
[01:12:29] .................................................................................................... 4400/5327
---
[01:13:05] ---- [ui] ui/parser/issue-32214.rs stdout ----
[01:13:05] diff of stderr:
[01:13:05] 
[01:13:05] 3    |
[01:13:05] 4 LL | pub fn test<W, I: Trait<Item=(), W> >() {}
[01:13:05] 5    |                                  ^ must be declared prior to associated type bindings
[01:13:05] + help: move the type parameter prior to the first associated type binding
[01:13:05] +    |
[01:13:05] + LL | pub fn test<W, I: Trait<W, Item=()> >() {}
[01:13:05] +    |                         ^^       --
[01:13:05] 7 error: aborting due to previous error
[01:13:05] 8 
[01:13:05] 
[01:13:05] 
[01:13:05] 
[01:13:05] The actual stderr differed from the expected stderr.
[01:13:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-32214/issue-32214.stderr
[01:13:05] To update references, rerun the tests and pass the `--bless` flag
[01:13:05] To only update this specific test, also pass `--test-args parser/issue-32214.rs`
[01:13:05] error: 1 errors occurred comparing output.
[01:13:05] status: exit code: 1
[01:13:05] status: exit code: 1
[01:13:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-32214.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-32214/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-32214/auxiliary" "-A" "unused"
[01:13:05] ------------------------------------------
[01:13:05] 
[01:13:05] ------------------------------------------
[01:13:05] stderr:
[01:13:05] stderr:
[01:13:05] ------------------------------------------
[01:13:05] {"message":"type parameters must be declared prior to associated type bindings","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/parser/issue-32214.rs","byte_start":113,"byte_end":114,"line_start":5,"line_end":5,"column_start":34,"column_end":35,"is_primary":true,"text":[{"text":"pub fn test<W, I: Trait<Item=(), W> >() {}","highlight_start":34,"highlight_end":35}],"label":"must be declared prior to associated type bindings","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move the type parameter prior to the first associated type binding","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/parser/issue-32214.rs","byte_start":111,"byte_end":114,"line_start":5,"line_end":5,"column_start":32,"column_end":35,"is_primary":true,"text":[{"text":"pub fn test<W, I: Trait<Item=(), W> >() {}","highlight_start":32,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"/checkout/src/test/ui/parser/issue-32214.rs","byte_start":104,"byte_end":104,"line_start":5,"line_end":5,"column_start":25,"column_end":25,"is_primary":true,"text":[{"text":"pub fn test<W, I: Trait<Item=(), W> >() {}","highlight_start":25,"highlight_end":25}],"label":null,"suggested_replacement":"W, ","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: type parameters must be declared prior to associated type bindings\n  --> /checkout/src/test/ui/parser/issue-32214.rs:5:34\n   |\nLL | pub fn test<W, I: Trait<Item=(), W> >() {}\n   |                                  ^ must be declared prior to associated type bindings\nhelp: move the type parameter prior to the first associated type binding\n   |\nLL | pub fn test<W, I: Trait<W, Item=()> >() {}\n   |                         ^^       --\n\n"}
[01:13:05] 
[01:13:05] ------------------------------------------
[01:13:05] 
[01:13:05] thread '[ui] ui/parser/issue-32214.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
---
[01:13:05] 
[01:13:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:13:05] 
[01:13:05] 
[01:13:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:05] 
[01:13:05] 
[01:13:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:05] Build completed unsuccessfully in 0:04:26
[01:13:05] Build completed unsuccessfully in 0:04:26
[01:13:05] make: *** [check] Error 1
[01:13:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:026bf7af
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 25 00:54:44 UTC 2019
---
travis_time:end:062ed0d0:start=1548377685363866670,finish=1548377685368402563,duration=4535893
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06e1c510
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'
