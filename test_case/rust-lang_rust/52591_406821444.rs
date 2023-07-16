plain
[00:12:17]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:12:20] error[E0282]: type annotations needed
[00:12:20]    --> librustc_metadata/locator.rs:505:54
[00:12:20]     |
[00:12:20] 454 |         let mut candidates = FxHashMap::default();
[00:12:20]     |             -------------- consider giving `candidates` a type
[00:12:20] ...
[00:12:20] 505 |                         CrateFlavor::Rlib => { rlibs.insert(p, kind); }
[00:12:20]     |                                                      ^^^^^^ cannot infer type for `_`
[00:12:20]     |
[00:12:20]     = note: type must be known at this point
[00:12:20] error: aborting due to previous error
[00:12:20] 
[00:12:20] For more information about this error, try `rustc --explain E0282`.
[00:12:20] error: Could not compile `rustc_metadata`.
[00:12:20] error: Could not compile `rustc_metadata`.
[00:12:20] 
[00:12:20] Caused by:
[00:12:20]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_metadata librustc_metadata/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=5e2ff68b68a48934 -C extra-filename=-5e2ff68b68a48934 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-e808ab63ca762e81.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3531a50c702021e6.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6755b57849a2d1c7.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2792a41a3401596f.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5aa06a687b1a439d.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-62c7dbd871cbb630.so --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-b845264c18cc378a.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-04bbe800f5e81b33.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out` (exit code: 101)
[00:15:08] error: build failed
[00:15:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:08] expected success, got: exit code: 101
[00:15:08] expected success, got: exit code: 101
[00:15:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:15:08] travis_fold:end:stage0-rustc

[00:15:08] travis_time:end:stage0-rustc:start=1532204058887071121,finish=1532204679901494457,duration=621014423336


[00:15:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:08] Build completed unsuccessfully in 0:11:16
[00:15:08] Makefile:28: recipe for target 'all' failed
[00:15:08] make: *** [all] Error 1
22656 ./src/llvm-emscripten/test/MC
21292 ./.git/modules/src/liblibc
21136 ./src/llvm-emscripten/test/Transforms
21132 ./.git/modules/src/liblibc/objects
