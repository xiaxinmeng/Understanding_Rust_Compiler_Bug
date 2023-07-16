plain
[00:03:44]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:44]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:56]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:03:56]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:03:56] error[E0425]: cannot find value `task` in this scope
[00:03:56]    --> liballoc/boxed.rs:983:28
[00:03:56]     |
[00:03:56] 983 |         (**self).spawn_obj(task)
[00:03:56] 
bstd/Cargo.toml" "--message-format" "json"
[00:03:58] expected success, got: exit code: 101
[00:03:58] expected success, got: exit code: 101
[00:03:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[00:03:58] travis_fold:end:stage0-std

[00:03:58] travis_time:end:stage0-std:start=1533544141013852465,finish=1533544167850848073,duration=26836995608


[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:58] Build completed unsuccessfully in 0:00:27
[00:03:59] make: *** [all] Error 1
[00:03:59] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2be8f658
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
