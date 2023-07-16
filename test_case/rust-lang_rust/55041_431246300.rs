plain
[00:05:16]    Compiling flate2 v1.0.3
[00:05:16]    Compiling backtrace v0.3.9
[00:05:18]    Compiling crossbeam-deque v0.2.0
[00:05:21]    Compiling rustc-rayon v0.1.1
[00:05:22] error: no rules expected the token `.`
[00:05:22]     |
[00:05:22]     |
[00:05:22] 400 |     ("thumbv8m.base-none-eabi", thumbv8m.base_none_eabi),
[00:05:22] 
travis_time:end:04d140e3:start=1539925363888306447,finish=1539925689002220123,duration=325113913676

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
---
travis_time:end:1cfd1c1f:start=1539925689446617305,finish=1539925689452220821,duration=5603516
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:081fced6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0769a37d
travis_time:start:0769a37d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynam
