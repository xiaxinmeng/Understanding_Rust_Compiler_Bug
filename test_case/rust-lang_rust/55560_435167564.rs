plain
travis_time:end:11d060b8:start=1541098919376356869,finish=1541098973621546274,duration=54245189405
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:22] .................................................................................................... 100/4983
[00:49:25] .................................................................................................... 200/4983
[00:49:28] ...........................................................................................ii....... 300/4983
[00:49:31] .........................................................................................iii........ 400/4983
[00:49:33] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:49:40] .................................................................................................... 700/4983
[00:49:46] ..................................................................i...........i..................... 800/4983
[00:49:49] ....................................................................................iiiii........... 900/4983
[00:49:53] .................................................................................................... 1000/4983
---
[00:50:28] .................................................................................................... 2200/4983
[00:50:32] .................................................................................................... 2300/4983
[00:50:36] .................................................................................................... 2400/4983
[00:50:40] .................................................................................................... 2500/4983
[00:50:43] ...................................................................iiiiiiiii........................ 2600/4983
[00:50:50] ..................ii................................................................................ 2800/4983
[00:50:53] .................................................................................................... 2900/4983
[00:50:57] .................................................................................................... 3000/4983
[00:50:59] .............i...................................................................................... 3100/4983
---
[00:54:25] .................................................................................................... 1500/2873
[00:54:37] ..........................................i......................................................... 1600/2873
[00:54:52] .................................................................................................... 1700/2873
[00:55:03] .................................................................................................... 1800/2873
[00:55:12] ..............................................................F.....i............................... 1900/2873
[00:55:45] .................................................................................................... 2100/2873
[00:55:52] .................................................................................................... 2200/2873
[00:56:08] ii.....................................................................i....i....................... 2300/2873
[00:56:22] ..............i..................................................................................... 2400/2873
---
[00:57:27] failures:
[00:57:27] 
[00:57:27] ---- [run-pass] run-pass/modules/mod_dir_path.rs stdout ----
[00:57:27] 
[00:57:27] error: test compilation failed although it shouldn't!
[00:57:27] status: exit code: 1
[00:57:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/modules/mod_dir_path.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/modules/mod_dir_path/auxiliary"
[00:57:27] ------------------------------------------
[00:57:27] 
[00:57:27] ------------------------------------------
[00:57:27] stderr:
[00:57:27] stderr:
[00:57:27] ------------------------------------------
[00:57:27] {"message":"couldn't read /checkout/src/test/run-pass/modules/bar/auxiliary/two_macros_2.rs: No such file or directory (os error 2)","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path.rs","byte_start":895,"byte_end":907,"line_start":31,"line_end":31,"column_start":17,"column_end":29,"is_primary":true,"text":[{"text":"            mod two_macros_2;","highlight_start":17,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path.rs","byte_start":929,"byte_end":934,"line_start":33,"line_end":33,"column_start":9,"column_end":14,"is_primary":false,"text":[{"text":"        m!();","highlight_start":9,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"m!","def_site_span":{"file_name":"/checkout/src/test/run-pass/modules/mod_dir_path.rs","byte_start":804,"byte_end":920,"line_start":29,"line_end":32,"column_start":9,"column_end":12,"is_primary":false,"text":[{"text":"        macro_rules! m { () => {","highlight_start":9,"highlight_end":33},{"text":"            #[path = \"auxiliary/two_macros_2.rs\"]","highlight_start":1,"highlight_end":50},{"text":"            mod two_macros_2;","highlight_start":1,"highlight_end":30},{"text":"        } }","highlight_start":1,"highlight_enunknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:27] 
[00:57:27] 
[00:57:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:27] Build completed unsuccessfully in 0:11:45
[00:57:27] Build completed unsuccessfully in 0:11:45
[00:57:27] make: *** [check] Error 1
[00:57:27] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f59590
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:249c3fa8:start=1541102432305113462,finish=1541102432309480727,duration=4367265
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04365852
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e6e0e9a
travis_time:start:0e6e0e9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fc7c56c
$ dmesg | grep -i kill
