plain
[00:04:30] Caused by:
[00:04:30]   failed to load pkg lockfile
[00:04:30] 
[00:04:30] Caused by:
[00:04:30]   no matching package named `core` found
[00:04:30] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:04:30] required by package `compiler_builtins v0.1.2`
[00:04:30]     ... which is depended on by `alloc v0.0.0 (/checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/src/liballoc)`
[00:04:30] 
[00:04:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:04:30] expected success, got: exit code: 101
[00:04:30] 
---
travis_time:end:2c156e92:start=1543310925454186252,finish=1543310925466903771,duration=12717519
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22e74c68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0920ecd5
travis_time:start:0920ecd5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bfee330
$ dmesg | grep -i kill
