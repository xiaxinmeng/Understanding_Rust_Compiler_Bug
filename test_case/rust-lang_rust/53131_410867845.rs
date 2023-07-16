plain
[00:49:03] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:09] ....................................................................................................
[00:49:11] ....................................................................................................
[00:49:14] .................i.........................................F........................................
[00:49:19] ....................................................................................................
[00:49:21] ....................................................................................................
[00:49:24] ....................................................................................................
[00:49:24] ....................................................................................................
[00:49:27] .....iiiiiiiii......................................................................................
[00:49:33] ....................................................................................................
[00:49:37] ..........i.........................................................................................
[00:49:39] ...................i................................................................................
[00:49:43] ....................................................................................................
---
[00:49:51] 
[00:49:51] ---- [ui (nll)] ui/issue-17954.rs stdout ----
[00:49:51] diff of stderr:
[00:49:51] 
[00:49:51] - error[E0711]: thread-local variable borrowed past end of function
[00:49:51] + error[E0712]: thread-local variable borrowed past end of function
[00:49:51] 3    |
[00:49:51] 3    |
[00:49:51] 4 LL |     let a = &FOO;
[00:49:51] 9 
[00:49:51] 10 error: aborting due to previous error
[00:49:51] 11 
[00:49:51] - For more information about this error, try `rustc --explain E0711`.
[00:49:51] - For more information about this error, try `rustc --explain E0711`.
[00:49:51] + For more information about this error, try `rustc --explain E0712`.
[00:49:51] 13 
[00:49:51] 
[00:49:51] 
[00:49:51] The actual stderr differed from the expected stderr.
[00:49:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17954.nll/issue-17954.nll.stderr
[00:49:51] To update references, rerun the tests and pass the `--bless` flag
[00:49:51] To only update this specific test, also pass `--test-args issue-17954.rs`
[00:49:51] error: 1 errors occurred comparing output.
[00:49:51] status: exit code: 1
[00:49:51] command: "/checko3289660 .
2324932 ./obj
---
147700 ./obj/build/bootstrap/debug/incremental
136188 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
136184 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
133276 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha
133272 ./obj/build/bootstrap/debug/incremental/bootstrap-jdsuci5s9dha/s-f3m1mk8x4c-156dbp4-1zjq8g928qpl7
128640 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
125084 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu4138410008,finish=1533593074145137717,duration=6727709
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:1385f628
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d2311e5
travis_time:start:1d2311e5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11e9b398
$ dmesg | grep -i kill
