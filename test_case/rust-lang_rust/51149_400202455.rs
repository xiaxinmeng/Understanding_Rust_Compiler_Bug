plain
[00:05:10] [RUSTC-TIMING] rustc_errors test:false 7.398
[00:05:11] error[E0422]: cannot find struct, variant or union type `Spanned` in this scope
[00:05:11]     --> libsyntax/feature_gate.rs:1752:34
[00:05:11]      |
[00:05:11] 1752 |             PatKind::Range(_, _, Spanned { node: RangeEnd::Excluded, .. }) => {
[00:05:11]      |                                  ^^^^^^^ not found in this scope
[00:05:11] help: possible candidate is found in another module, you can import it into scope
[00:05:11] 25   | use codemap::Spanned;
[00:05:11]      |
[00:05:11] 
[00:05:18] error: aborting due to previous error
[00:05:18] error: aborting due to previous error
[00:05:18] 
[00:05:18] For more information about this error, try `rustc --explain E0422`.
[00:05:18] [RUSTC-TIMING] syntax test:false 7.105
[00:05:18] error: Could not compile `syntax`.
[00:05:18] 
[00:05:18] Caused by:
[00:05:18]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6a2f0731783c2bd3 -C extra-filename=-6a2f0731783c2bd3 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so` (exit code: 101)
[00:05:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:18] expected success, got: exit code: 101
[00:05:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:05:18] travis_fold:end:stage0-rustc

[00:05:18] travis_time:end:stage0-rustc:start=1529996637458400446,finish=1529996705494065377,duration=68035664931

---
travis_time:end:12ba072c:start=1529996706032475977,finish=1529996706039227095,duration=6751118
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c53d6df
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d11f50c
$ dmesg | grep -i kill
