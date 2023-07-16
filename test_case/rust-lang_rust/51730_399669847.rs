plain
[00:03:05]    Compiling cmake v0.1.30
[00:03:05]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:09]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:11]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:11] error[E0277]: the trait bound `T: marker::Unpin` is not satisfied in `option::Option<T>`
[00:03:11]     |
[00:03:11]     |
[00:03:11] 278 |             PinMut::get_mut(self).as_mut().map(|x| PinMut::new_unchecked(x))
[00:03:11]     |             ^^^^^^^^^^^^^^^ within `option::Option<T>`, the trait `marker::Unpin` is not implemented for `T`
[00:03:11]     |
[00:03:11]     = help: consider adding a `where T: marker::Unpin` bound
[00:03:11]     = note: required because it appears within the type `option::Option<T>`
[00:03:11] note: required by `<mem::PinMut<'a, T>>::get_mut`
[00:03:11]    --> libcore/mem.rs:1125:5
[00:03:11]     |
[00:03:11] 1125|     pub fn get_mut(this: PinMut<'a, T>) -> &'a mut T {
[00:03:11] 
[00:03:12]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:13] error: aborting due to previous error
[00:03:13] 
[00:03:13] 
[00:03:13] For more information about this error, try `rustc --explain E0277`.
[00:03:13] error: Could not compile `core`.
[00:03:13] 
[00:03:13] Caused by:
[00:03:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=e9cdce497aae9e81 -C extra-filename=-e9cdce497aae9e81 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:13] warning: build failed, waiting for other jobs to finish...
[00:03:22] error: build failed
[00:03:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:22] expected success, got: exit code: 101
[00:03:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:22] travis_fold:end:stage0-std

[00:03:22] travis_time:end:stage0-std:start=1529754011872707736,finish=1529754036075484498,duration=24202776762


[00:03:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:22] Build completed unsuccessfully in 0:00:25
[00:03:22] make: *** [tidy] Error 1
[00:03:22] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037f73c1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0746202c:start=1529754036612463694,finish=1529754036620034512,duration=7570818
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2d1defa0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24bf14ee
$ dmesg | grep -i kill
