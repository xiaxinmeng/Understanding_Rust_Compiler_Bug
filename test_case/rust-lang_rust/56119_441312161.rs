plain
travis_time:end:0339316f:start=1543000156864905242,finish=1543000228264407290,duration=71399502048
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:18] .................................................................................................... 100/5050
[00:51:21] .................................................................................................... 200/5050
[00:51:24] .............................ii............................................ii...................ii.. 300/5050
[00:51:27] ..............................................................................................iii... 400/5050
[00:51:30] .....iiiiiiii.iii............................iii...........................................i........ 500/5050
[00:51:37] .................................................................................................... 700/5050
[00:51:43] ...................................................................................i...........i.... 800/5050
[00:51:46] .................................................................................................... 900/5050
[00:51:50] ..iiiii..................ii.iiii.................................................................... 1000/5050
---
[00:52:26] .................................................................................................... 2200/5050
[00:52:30] .................................................................................................... 2300/5050
[00:52:34] .................................................................................................... 2400/5050
[00:52:38] .................................................................................................... 2500/5050
[00:52:41] ...........................................................................................iiiiiiiii 2600/5050
[00:52:48] .........................................................ii......................................... 2800/5050
[00:52:51] .................................................................................................... 2900/5050
[00:52:55] .................................................................................................... 3000/5050
[00:52:58] ......................................................i............................................. 3100/5050
---
[00:54:59] .................................................................................................... 600/2886
[00:55:14] .................................................................................................... 700/2886
[00:55:25] .................................................................................................... 800/2886
[00:55:35] .................................................................................................... 900/2886
[00:55:46] .........................FF......................................................................... 1000/2886
[00:56:08] .................................................................................................... 1200/2886
[00:56:17] .................................................................................................... 1300/2886
[00:56:30] ..............................................................................i..................... 1400/2886
[00:56:41] .................................................................................................... 1500/2886
---
[00:59:08] .................................................................................................... 2500/2886
[00:59:40] .................................................................................................... 2600/2886
[00:59:49] .................................................................................................... 2700/2886
[00:59:58] .................................................................................................... 2800/2886
rpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/example-calendar.nll/auxiliary"
[01:00:11] ------------------------------------------
[01:00:11] 
[01:00:11] ------------------------------------------
[01:00:11] stderr:
[01:00:11] stderr:
[01:00:11] ------------------------------------------
[01:00:11] {"message":"expected one of `.`, `?`, `{`, or an operator, found `;`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/example-calendar.rs","byte_start":21659,"byte_end":21660,"line_start":756,"line_end":756,"column_start":42,"column_end":43,"is_primary":true,"text":[{"text":"        let first = match self.it.next()?;","highlight_start":42,"highlight_end":43}],"label":"expected one of `.`, `?`, `{`, or an operator here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/impl-trait/example-calendar.rs","byte_start":21638,"byte_end":21643,"line_start":756,"line_end":756,"column_start":21,"column_end":26,"is_primary":false,"text":[{"text":"        let first = match self.it.next()?;","highlight_start":21,"highlight_end":26}],"label":"while parsing this match expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try removing this `match`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/example-calendar.rs","byte_start":21638,"byte_end":21643,"line_start":756,"lad 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:00:11] 
[01:00:11] 
[01:00:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:11] 
[01:00:11] 
[01:00:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:11] Build completed unsuccessfully in 0:12:51
[01:00:11] Build completed unsuccessfully in 0:12:51
[01:00:11] make: *** [check] Error 1
[01:00:11] Makefile:58: recipe for target 'check' failed
