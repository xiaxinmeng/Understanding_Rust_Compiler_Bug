plain
[00:13:45]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:16:52]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:16:52]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:17:04]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:17:08] error: field is never used: `speculative`
[00:17:08]   --> librustc_resolve/lib.rs:91:5
[00:17:08] 91 |     speculative : bool,
[00:17:08]    |     ^^^^^^^^^^^^^^^^^^
[00:17:08]    |
[00:17:08]    |
[00:17:08]    = note: `-D dead-code` implied by `-D warnings`
[00:17:08] error: aborting due to previous error
[00:17:08] 
[00:17:08] error: Could not compile `rustc_resolve`.
[00:17:08] 
[00:17:08] 
[00:17:08] Caused by:
[00:17:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_resolve librustc_resolve/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8ebf33ee495224fa -C extra-filename=-8ebf33ee495224fa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f44070055fa5d2f8.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-19bf33dd406ed70a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ed50b2d1f28e8a9.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a636a569660b030.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-24d121bd290f1f21.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:17:53] error: build failed
[00:17:53] error: build failed
[00:17:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:53] expected success, got: exit code: 101
[00:17:53] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:17:53] travis_fold:end:stage0-rustc

[00:17:53] travis_time:end:stage0-rustc:start=1525022500396799757,finish=1525023264340059429,duration=763943259672


[00:17:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:53] Build completed unsuccessfully in 0:12:57
[00:17:53] make: *** [all] Error 1
[00:17:53] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c69e732
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
