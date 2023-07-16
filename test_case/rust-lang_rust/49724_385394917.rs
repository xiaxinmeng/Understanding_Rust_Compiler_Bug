plain
[00:08:08]    |     ^^^^^^^^^^^^^^^^^^^^^^^^
[00:08:08]    |
[00:08:08]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:08] 
own-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-19bf33dd406ed70a.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-bca5fb1f27226f8d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ab2cf718bbbe7ba7.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-9d60fa98a5a10c1e.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:08:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:36] expected success, got: exit code: 101
[00:08:36] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:36] travis_fold:end:stage0-rustc

[00:08:36] travis_time:end:stage0-rustc:start=1525093986068890078,finish=1525094184131824712,duration=198062934634


[00:08:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:36] Build completed unsuccessfully in 0:03:32
[00:08:36] make: *** [all] Error 1
[00:08:36] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01d5c638
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
