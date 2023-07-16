plain
[01:09:58] 
[01:09:59] error: hidden lifetime parameters in types are deprecated
[01:09:59]   --> src/libstd/sys/wasm/os.rs:27:42
[01:09:59]    |
[01:09:59] 27 | pub fn split_paths(_unparsed: &OsStr) -> SplitPaths {
[01:09:59] 
[01:09:59] error: hidden lifetime parameters in types are deprecated
[01:09:59]   --> src/libstd/sys/wasm/path.rs:14:42
[01:09:59]    |
[01:09:59]    |
[01:09:59] 14 | pub fn parse_prefix(_: &OsStr) -> Option<Prefix> {
[01:09:59] 
[01:10:03] error: aborting due to 2 previous errors
[01:10:03] 
[01:10:03] [RUSTC-TIMING] std test:false 5.731
---
travis_time:end:07164039:start=1552698605947347058,finish=1552698605983082580,duration=35735522
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fa54092
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01851190
travis_time:start:01851190
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02170878
$ dmesg | grep -i kill
