plain
[01:51:22] test string::test_str_truncate ... ok
[01:51:22] test str::test_unsafe_slice ... ok
[01:51:22] test string::test_str_truncate_invalid_len ... ok
[01:51:22] test string::test_str_truncate_split_codepoint ... ok
[01:51:23] died due to signal 11
[01:51:23] 
[01:51:23] 
[01:51:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:51:23] expected success, got: exit code: 3
---
travis_time:end:00f8a27c:start=1553583100392403140,finish=1553583100403478382,duration=11075242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0923f6dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0287c6e0
travis_time:start:0287c6e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:25295c00
$ dmesg | grep -i kill
