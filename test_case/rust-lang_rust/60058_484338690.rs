plain
[01:44:45] test vec::test_split_at_mut ... ok
[01:44:45] test vec::test_splice_unbounded ... ok
[01:44:45] test vec::test_split_off ... ok
[01:44:45] test vec::test_swap_remove_empty ... ok
[01:44:45] died due to signal 11
[01:44:45] 
[01:44:45] 
[01:44:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:44:45] expected success, got: exit code: 3
---
travis_time:end:12916ca0:start=1555556759258130925,finish=1555556759270541288,duration=12410363
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a7b6a66
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:008de5f4
travis_time:start:008de5f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17ed89e4
$ dmesg | grep -i kill
