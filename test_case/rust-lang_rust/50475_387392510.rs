plain
[00:24:57]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:25:01]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:26:09]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:26:24]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:28:07] error: this feature has been stable since 1.27.0. Attribute no longer needed
[00:28:07]    |
[00:28:07]    |
[00:28:07] 50 | #![feature(fn_must_use)]
[00:28:07]    |
[00:28:07]    = note: `-D stable-features` implied by `-D warnings`
[00:28:07] 
[00:28:08] error: aborting due to previous error
[00:28:08] error: aborting due to previous error
[00:28:08] 
[00:28:09] error: Could not compile `rustc`.
[00:28:09] 
[00:28:09] Caused by:
[00:28:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=39284c2da38b1eb5 -C extra-filename=-39284c2da38b1eb5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c485a7f1a48de680.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-33398135bf6cc359.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ce13477f28b2521d.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-cff695d4a4682a45.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-8616faa2bf885029.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-b8460a10e41a3e85.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-cf6fe4bd350080f4.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-09bdd144093f3a08.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-2ba8b144dc11ecc3/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-da5ad65efaccd1d7/out` (exit code: 101)
[00:28:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:28:09] expected success, got: exit code: 101
[00:28:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:28:09] travis_fold:end:stage1-rustc

[00:28:09] travis_time:end:stage1-rustc:start=1525783873995183520,finish=1525784101158198088,duration=227163014568


[00:28:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:28:09] Build completed unsuccessfully in 0:23:05
[00:28:09] Makefile:28: recipe for target 'all' failed
[00:28:09] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07accbb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
