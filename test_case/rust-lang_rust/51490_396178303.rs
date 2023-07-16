plain
[00:05:47]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:05:49]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:50]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:53]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:01] error: description for error code E0541 contains a line longer than 80 characters.
[00:06:01] if you're inserting a long URL use the footnote style to bypass this check.
[00:06:01]    --> libsyntax/diagnostics/macros.rs:13:37
[00:06:01]     |
[00:06:01] 13  |       ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
[00:06:01]     | 
[00:06:01]     | 
[00:06:01]    ::: libsyntax/diagnostic_list.rs:16:1
[00:06:01]     |
[00:06:01] 16  | / register_long_diagnostics! {
[00:06:01] 17  | |
[00:06:01] 18  | | E0178: r##"
[00:06:01] 19  | | In types, the `+` type operator has low precedence, so it is often necessary
[00:06:01] 343 | |
[00:06:01] 344 | | }
[00:06:01]     | |_- in this macro invocation
[00:06:01] 
[00:06:01] 
[00:06:01] error: aborting due to previous error
[00:06:01] 
[00:06:01] error: Could not compile `syntax`.
[00:06:01] 
[00:06:01] Caused by:
[00:06:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6a2f0731783c2bd3 -C extra-filename=-6a2f0731783c2bd3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so` (exit code: 101)
[00:06:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:01] expected success, got: exit code: 101
[00:06:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:06:01] travis_fold:end:stage0-rustc

[00:06:01] travis_time:end:stage0-rustc:start=1528708713492163312,finish=1528708773266370054,duration=59774206742


[00:06:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:01] Build completed unsuccessfully in 0:01:11
[00:06:01] Makefile:28: recipe for target 'all' failed
[00:06:01] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00e4241e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03c4092a:start=1528708773762450307,finish=1528708773768307479,duration=5857172
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f896308
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d52f124
$ dmesg | grep -i kill
