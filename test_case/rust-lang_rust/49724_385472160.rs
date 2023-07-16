plain
[00:05:22]    Compiling jobserver v0.1.11
[00:05:23]    Compiling miniz-sys v0.1.10
[00:05:23]    Compiling backtrace-sys v0.1.16
[00:05:24]    Compiling env_logger v0.5.8
[00:05:25] error[E0658]: use of unstable library feature 'inclusive_range_methods' (see issue #49022)
[00:05:25]    --> librustc_target/abi/mod.rs:558:39
[00:05:25]     |
[00:05:25] 558 |         let start = *self.valid_range.start();
[00:05:25]     |
[00:05:25]     |
[00:05:25]     = help: add #![feature(inclusive_range_methods)] to the crate attributes to enable
[00:05:25] 
[00:05:25] error[E0658]: use of unstable library feature 'inclusive_range_methods' (see issue #49022)
[00:05:25]    --> librustc_target/abi/mod.rs:559:37
[00:05:25]     |
[00:05:25] 559 |         let end = *self.valid_range.end();
[00:05:25]     |
[00:05:25]     |
[00:05:25]     = help: add #![feature(inclusive_range_methods)] to the crate attributes to enable
[00:05:27]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:05:27] error: aborting due to 2 previous errors
[00:05:27] 
[00:05:27] For more information about this error, try `rustc --explain E0658`.
[00:05:27] For more information about this error, try `rustc --explain E0658`.
[00:05:27] error: Could not compile `rustc_target`.
[00:05:27] 
[00:05:27] Caused by:
[00:05:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_target librustc_target/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" -C metadata=40a8b14ac2445c60 -C extra-filename=-40a8b14ac2445c60 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3780f5cb4ab85083.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-51538b9509b4cdb0.so` (exit code: 101)
[00:05:29] error: build failed
[00:05:29] error: build failed
[00:05:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:29] expected success, got: exit code: 101
[00:05:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:05:29] travis_fold:end:stage0-rustc

[00:05:29] travis_time:end:stage0-rustc:start=1525109762985232624,finish=1525109791081091844,duration=28095859220


[00:05:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:29] Build completed unsuccessfully in 0:00:40
[00:05:29] Makefile:28: recipe for target 'all' failed
[00:05:29] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a6c3f1a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
