plain
[00:04:41]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:04:41] error[E0412]: cannot find type `E` in this scope
[00:04:41]    --> liballoc/boxed.rs:976:23
[00:04:41]     |
[00:04:41] 976 | impl Executor for Box<E>
[00:04:41]     |                       ^ did you mean `Eq`?
[00:04:41] error[E0412]: cannot find type `E` in this scope
[00:04:41]    --> liballoc/boxed.rs:977:11
[00:04:41]     |
[00:04:41]     |
[00:04:41] 977 |     where E: Executor
[00:04:41]     |           ^ did you mean `Eq`?
[00:04:41] 
[00:04:42] error[E0307]: invalid `self` type: &mut boxed::Box<[type error]>
[00:04:42]    --> liballoc/boxed.rs:979:18
[00:04:42]     |
[00:04:42] 979 |     fn spawn_obj(&mut self, task: FutureObj<'static, ()>) -> Result<(), SpawnObjError> {
[00:04:42]     |
[00:04:42]     |
[00:04:42]     = note: type must be `boxed::Box<[type error]>` or a type that dereferences to it
[00:04:42]     = help: consider changing to `self`, `&self`, `&mut self`, or `self: Box<Self>`
[00:04:42] 
[00:04:42] error[E0307]: invalid `self` type: &boxed::Box<[type error]>
[00:04:42]    --> liballoc/boxed.rs:983:15
[00:04:42]     |
[00:04:42] 983 |     fn status(&self) -> Result<(), SpawnErrorKind> {
[00:04:42]     |
[00:04:42]     |
[00:04:42]     = note: type must be `boxed::Box<[type error]>` or a type that dereferences to it
[00:04:42]     = help: consider changing to `self`, `&self`, `&mut self`, or `self: Box<Self>`
[00:04:42]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:04:42]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:04:42] error: aborting due to 4 previous errors
[00:04:42] 
[00:04:42] 
[00:04:42] Some errors occurred: E0307, E0412.
[00:04:42] For more information about an error, try `rustc --explain E0307`.
[00:04:42] error: Could not compile `alloc`.
[00:04:42] 
[00:04:42] Caused by:
[00:04:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=0fa369be6843d38b -C extra-filename=-0fa369be6843d38b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-b64848753b7b1fae.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-13801a3823f081b4.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-70f4afff694b17e5/out` (exit code: 101)
[00:04:42] error: build failed
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:42] expected success, got: exit code: 101
[00:04:42] expected success, got: exit code: 101
[00:04:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:04:42] travis_fold:end:stage0-std

[00:04:42] travis_time:end:stage0-std:start=1532464978603267562,finish=1532465010306170390,duration=31702902828


[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:42] Build completed unsuccessfully in 0:00:32
[00:04:42] Makefile:28: recipe for target 'all' failed
[00:04:42] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01f876c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:084c7720:start=1532465011016343856,finish=1532465011030176302,duration=13832446
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13b7616a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05aa0912
travis_time:start:05aa0912
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1153d08e
$ dmesg | grep -i kill
