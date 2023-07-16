plain
[00:05:51]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:05:54]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:54]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:58]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:06] error: no rules expected the token `accepted`
[00:06:06]    --> libsyntax/feature_gate.rs:477:6
[00:06:06]     |
[00:06:06] 477 |     (accepted, termination_trait_test, "1.28.0", Some(48854), Some(Edition::Edition2018)),
[00:06:06] 
[00:06:06] error: aborting due to previous error
[00:06:06] 
[00:06:06] error: Could not compile `syntax`.
[00:06:06] error: Could not compile `syntax`.
[00:06:06] 
[00:06:06] Caused by:
[00:06:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=4c5434c80172b18c -C extra-filename=-4c5434c80172b18c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-41b116eaee1e5535.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-0404335fb4ae3dc1.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05d59ed98ebd8949.so` (exit code: 101)
[00:06:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:06] expected success, got: exit code: 101
[00:06:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:06] travis_fold:end:stage0-rustc

[00:06:06] travis_time:end:stage0-rustc:start=1527941275944866405,finish=1527941337502711783,duration=61557845378


[00:06:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:06] Build completed unsuccessfully in 0:01:13
[00:06:06] Makefile:28: recipe for target 'all' failed
[00:06:06] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:077fbed8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
