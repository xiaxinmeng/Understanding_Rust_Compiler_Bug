plain
[00:05:12]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:05:13]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:05:14]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:05:14]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:05:15] error[E0599]: no associated item named `MAX` found for type `isize` in the current scope
[00:05:15]      |
[00:05:15]      |
[00:05:15] 3883 |     debug_assert!(len * mem::size_of::<T>() <= isize::MAX as usize,
[00:05:15]      |                                                ^^^^^^^^^^ associated item not found in `isize`
[00:05:15] 
[00:05:15] error[E0599]: no associated item named `MAX` found for type `isize` in the current scope
[00:05:15]      |
[00:05:15]      |
[00:05:15] 3903 |     debug_assert!(len * mem::size_of::<T>() <= isize::MAX as usize,
[00:05:15]      |                                                ^^^^^^^^^^ associated item not found in `isize`
[00:05:16] error: aborting due to 2 previous errors
[00:05:16] 
[00:05:16] For more information about this error, try `rustc --explain E0599`.
[00:05:16] error: Could not compile `core`.
[00:05:16] error: Could not compile `core`.
[00:05:16] 
[00:05:16] Caused by:
[00:05:16]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=bd44783aadae9ca1 -C extra-filename=-bd44783aadae9ca1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 1)
[00:05:16] warning: build failed, waiting for other jobs to finish...
[00:05:25] error: build failed
[00:05:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:25] expected success, got: exit code: 101
[00:05:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:05:25] travis_fold:end:stage0-std

[00:05:25] travis_time:end:stage0-std:start=1536053410934339668,finish=1536053437064310554,duration=26129970886


[00:05:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:25] Build completed unsuccessfully in 0:00:27
[00:05:25] make: *** [all] Error 1
[00:05:25] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d0b1c05
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:075fc52c:start=1536053437777130179,finish=1536053437784670745,duration=7540566
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05a27a83
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01f196fc
travis_time:start:01f196fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b64580
$ dmesg | grep -i kill
