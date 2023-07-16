plain
[00:05:53]    Compiling parking_lot_core v0.2.14
[00:05:54]    Compiling tempdir v0.3.7
[00:05:56]    Compiling rls-span v0.4.0
[00:05:56]    Compiling chalk-engine v0.6.0
[00:05:57] error[E0432]: unresolved import `LinkerFlavor`
[00:05:57]   --> librustc_target/spec/powerpc64le_unknown_linux_musl.rs:11:5
[00:05:57] 11 | use LinkerFlavor;
[00:05:57] 11 | use LinkerFlavor;
[00:05:57]    |     ^^^^^^^^^^^^ no `LinkerFlavor` in the root
[00:05:57] 
[00:05:57] error[E0432]: unresolved import `target`
[00:05:57]   --> librustc_target/spec/powerpc64le_unknown_linux_musl.rs:12:5
[00:05:57]    |
[00:05:57] 12 | use target::{Target, TargetResult};
[00:05:57]    |     ^^^^^^ Maybe a missing `extern crate target;`?
[00:05:57]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:05:59] error: aborting due to 2 previous errors
[00:05:59] 
[00:05:59] For more information about this error, try `rustc --explain E0432`.
[00:05:59] For more information about this error, try `rustc --explain E0432`.
[00:05:59] error: Could not compile `rustc_target`.
[00:05:59] 
[00:05:59] Caused by:
[00:05:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_target librustc_target/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" -C metadata=58741ed9de9aae4f -C extra-filename=-58741ed9de9aae4f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-056d8bc457dc18d8.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib` (exit code: 101)
[00:06:01] error: build failed
[00:06:01] error: build failed
[00:06:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:01] expected success, got: exit code: 101
[00:06:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:01] travis_fold:end:stage0-rustc

[00:06:01] travis_time:end:stage0-rustc:start=1529340107408390867,finish=1529340145361063440,duration=37952672573


[00:06:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:01] Build completed unsuccessfully in 0:00:52
[00:06:01] make: *** [all] Error 1
[00:06:01] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10a7f28c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cd65d2e:start=1529340145906491205,finish=1529340145912011310,duration=5520105
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:169a69a4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:034ad096
$ dmesg | grep -i kill
