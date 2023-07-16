plain
[00:06:10]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:06:11]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:06:14]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:06:15]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:06:28] error: this feature has been stable since 1.26.0. Attribute no longer needed
[00:06:28]   --> librustc_typeck/lib.rs:87:12
[00:06:28] 87 | #![feature(underscore_lifetimes)]
[00:06:28]    |            ^^^^^^^^^^^^^^^^^^^^
[00:06:28]    |
[00:06:28]    |
[00:06:28]    = note: `-D stable-features` implied by `-D warnings`
[00:06:28] error: aborting due to previous error
[00:06:28] 
[00:06:28] error: Could not compile `rustc_typeck`.
[00:06:28] 
[00:06:28] 
[00:06:28] Caused by:
[00:06:28]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=514c40b5c54e80a9 -C extra-filename=-514c40b5c54e80a9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-1473e867ee37960b.rmeta --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-94fc6139b1ea0bf1.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-c6dc897e75251054.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2f540472f4a8fbfe.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-785ea310aee57517.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-173ef7082666a6a0.rmeta --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-961eb4e3c1c2eb94.rmeta --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-db387b7a54da029c.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-c3c60fbe86756cd5.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-3d7e7cc9006b0d41.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b70225e395c9707a.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:06:43] error: build failed
[00:06:43] error: build failed
[00:06:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:43] expected success, got: exit code: 101
[00:06:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:43] travis_fold:end:stage0-rustc

[00:06:43] travis_time:end:stage0-rustc:start=1524831529797921418,finish=1524831740722943044,duration=210925021626

