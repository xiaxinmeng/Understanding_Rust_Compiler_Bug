plain
[00:18:01]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:18:01] error[E0583]: file not found for module `nonstandard_style`
[00:18:01]   --> librustc_lint/lib.rs:65:5
[00:18:01]    |
[00:18:01] 65 | mod nonstandard_style;
[00:18:01]    |
[00:18:01]    |
[00:18:01]    = help: name the file either nonstandard_style.rs or nonstandard_style/mod.rs inside the directory "librustc_lint"
[00:18:01] error: aborting due to previous error
[00:18:01] 
[00:18:01] For more information about this error, try `rustc --explain E0583`.
[00:18:01] error: Could not compile `rustc_lint`.
---
travis_time:end:08132420:start=1535550364862786772,finish=1535550364873001085,duration=10214313
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10e84d20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e87c9b2
travis_time:start:0e87c9b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d3f661e
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.693508] init: failsafe main process (1096) killed by TERM signal
[   42.140982] init: plymouth-upstart-bridge main process (509) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
