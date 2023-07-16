plain
[00:27:08]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:27:50] error: Could not compile `syntax`.
[00:27:50] 
[00:27:50] Caused by:
[00:27:50]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=6513b6f8528898e9 -C extra-filename=-6513b6f8528898e9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-575f47f158b62d9a.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-49e6d5bc53b7d428.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-31a934dcfb786c44.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-faa7967fee325699.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4db7e690e0ba82e5.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-e044971c1f0badcd.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6bb876a3fc9c03dd.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2d5dc4c4f204cfc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-2d5dc4c4f204cfc5.rlib` (signal: 11, SIGSEGV: invalid memory reference)
[00:27:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:27:50] expected success, got: exit code: 101
[00:27:50] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:27:50] travis_fold:end:stage1-rustc

[00:27:50] travis_time:end:stage1-rustc:start=1527168341428380672,finish=1527168432288240786,duration=90859860114


[00:27:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:27:50] Build completed unsuccessfully in 0:22:37
[00:27:50] make: *** [all] Error 1
[00:27:50] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05820fed
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
