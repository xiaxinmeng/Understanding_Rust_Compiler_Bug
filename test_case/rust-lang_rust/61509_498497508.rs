plain
[00:05:04]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:05:06] error[E0093]: unrecognized intrinsic function: `unchecked_add`
[00:05:06]     --> src/libcore/intrinsics.rs:1246:5
[00:05:06]      |
[00:05:06] 1246 |     pub fn unchecked_add<T>(x: T, y: T) -> T;
[00:05:06]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:05:06] 
[00:05:06] error[E0093]: unrecognized intrinsic function: `unchecked_sub`
[00:05:06]      |
[00:05:06]      |
[00:05:06] 1251 |     pub fn unchecked_sub<T>(x: T, y: T) -> T;
[00:05:06]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:05:06] 
[00:05:06] error[E0093]: unrecognized intrinsic function: `unchecked_mul`
[00:05:06]      |
[00:05:06]      |
[00:05:06] 1256 |     pub fn unchecked_mul<T>(x: T, y: T) -> T;
[00:05:06]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
[00:05:06]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:05:06]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:05:07]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:05:07]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
travis_time:end:0645e16c:start=1559616100525901506,finish=1559616100532084297,duration=6182791
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25ed23c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11a6eaf8
travis_time:start:11a6eaf8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:058b8288
$ dmesg | grep -i kill
