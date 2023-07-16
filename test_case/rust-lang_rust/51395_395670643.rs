plain
[00:21:20]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:21:20]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:21:21]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:21:21]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:21:25] error: this feature has been stable since 1.28.0. Attribute no longer needed
[00:21:25]     |
[00:21:25]     |
[00:21:25] 107 | #![feature(repr_transparent)]
[00:21:25]     |
[00:21:25]     = note: `-D stable-features` implied by `-D warnings`
[00:21:25] 
[00:21:25] error: aborting due to previous error
[00:21:25] error: aborting due to previous error
[00:21:25] 
[00:21:25] error: Could not compile `alloc`.
[00:21:25] 
[00:21:25] Caused by:
[00:21:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=629be8a8a92e38ff -C extra-filename=-629be8a8a92e38ff --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-e3f2b558276b578b.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d5d59b383a13b958.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-57f7f72e03e89590/out` (exit code: 101)
[00:21:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:25] travis_fold:end:stage1-std


[00:21:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:25] travis_time:end:stage1-std:start=1528440401168445405,finish=1528440465637308376,duration=64468862971

[00:21:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:25] Build completed unsuccessfully in 0:16:52
[00:21:25] Build completed unsuccessfully in 0:16:52
[00:21:25] make: *** [all] Error 1
[00:21:25] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ee12d25
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
