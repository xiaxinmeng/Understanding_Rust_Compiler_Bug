plain
[00:52:40]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:52:40] [RUSTC-TIMING] panic_unwind test:false 0.134
[00:52:40] warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`
[00:52:40] 
[00:52:43] error: functions with parameters of uninhabited types are uncallable
[00:52:43]    --> src/libstd/sys/wasm/fs.rs:256:1
[00:52:43]     |
[00:52:43] 256 |   pub fn set_perm(_p: &Path, perm: FilePermissions) -> io::Result<()> {
[00:52:43]     |   ^                          ---- this parameter has an uninhabited type
[00:52:43]     |  _|
[00:52:43] 257 | |     match perm.0 {}
[00:52:43] 258 | | }
[00:52:43]     | |_^
[00:52:43]     |
[00:52:43]     |
[00:52:43]     = note: `-D unreachable-code` implied by `-D warnings`
[00:52:43] 
[00:52:43] error: functions with parameters of uninhabited types are uncallable
[00:52:43]   --> src/libstd/sys/wasm/pipe.rs:20:1
[00:52:43]    |
[00:52:43] 20 |   pub fn read2(p1: AnonPipe,
[00:52:43]    |   ^            -- this parameter has an uninhabited type
[00:52:43]    |  _|
[00:52:43]    | |
[00:52:43] 21 | |              _v1: &mut Vec<u8>,
[00:52:43] 22 | |              _p2: AnonPipe,
[00:52:43]    | |              --- this parameter has an uninhabited type
[00:52:43] 23 | |              _v2: &mut Vec<u8>) -> io::Result<()> {
[00:52:43] 24 | |     match p1.0 {}
[00:52:43]    | |_^
[00:52:43] 
[00:52:43] error: aborting due to 2 previous errors
[00:52:43] 
---
travis_time:end:01bc8b10:start=1546455410548349964,finish=1546455410557797333,duration=9447369
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05407dc3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c73ee61
travis_time:start:0c73ee61
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00d4aa40
$ dmesg | grep -i kill
