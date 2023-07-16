plain
[00:06:23]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:06:25]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:06:26]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:30]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
0m     new_parser_from_source_str(sess, name, source).parse_stmt()
[00:06:49]     | |_^
[00:06:49] 
[00:06:49] error: aborting due to 4 previous errors
[00:06:49] 
[00:06:49] 
[00:06:49] error: Could not compile `syntax`.
[00:06:49] 
[00:06:49] Caused by:
[00:06:49]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6a2f0731783c2bd3 -C extra-filename=-6a2f0731783c2bd3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib` (exit code: 101)
[00:06:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:49] expected success, got: exit code: 101
[00:06:49] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:49] travis_fold:end:stage0-rustc

[00:06:49] travis_time:end:stage0-rustc:start=1529691049296310564,finish=1529691123487131918,duration=74190821354


[00:06:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:49] Build completed unsuccessfully in 0:01:26
[00:06:49] Makefile:28: recipe for target 'all' failed
[00:06:49] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0907e5d3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
