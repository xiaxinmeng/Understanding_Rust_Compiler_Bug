plain
[00:23:33]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:23:37]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:24:58]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:25:07]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:26:49] error: static item is never used: `ASSERT_TY_KIND`
[00:26:49]    --> librustc/ty/context.rs:833:9
[00:26:49]     |
[00:26:49] 833 |         static ASSERT_TY_KIND: () = [()][!(::std::mem::size_of::<ty::TyKind>() <= 24) as usize];
[00:26:49]     |
[00:26:49]     = note: `-D dead-code` implied by `-D warnings`
[00:26:49] 
[00:26:49] 
[00:26:49] error: static item is never used: `ASSERT_TYS`
[00:26:49]    --> librustc/ty/context.rs:835:9
[00:26:49]     |
[00:26:49] 835 |         static ASSERT_TYS: () = [()][!(::std::mem::size_of::<ty::TyS>() <= 32) as usize];
[00:26:49] 
[00:26:50] error: aborting due to 2 previous errors
[00:26:50] 
[00:26:50] error: Could not compile `rustc`.
[00:26:50] error: Could not compile `rustc`.
[00:26:50] 
[00:26:50] Caused by:
[00:26:50]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=bc944c9562c6c006 -C extra-filename=-bc944c9562c6c006 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-9fb19ec27e881758.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-63c33d6c489b468b.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-fb1b999a7bcef473.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-da03a033b1a119c5.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-dedcf92488701337.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/j/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-59b94b3f8088ae78.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e750d1bdc476112a/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3ba1d52dd8ca0552/out` (exit code: 1)
[00:26:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"

[00:26:50] expected success, got: exit code: 101
[00:26:50] travis_time:end:stage1-rustc:start=1536669552523595571,finish=1536669800116544602,duration=247592949031


[00:26:50] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:26:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:50] Build completed unsuccessfully in 0:21:14
[00:26:50] Makefile:28: recipe for target 'all' failed
[00:26:50] Makefile:28: recipe for target 'all' failed
[00:26:50] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b95a7a1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
149020 ./obj/build/bootstrap/debug/incremental
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4pbq51198-1k9kiq3-1jtu7ee7y4raz
104700 ./src/tools/lldb
99052 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
