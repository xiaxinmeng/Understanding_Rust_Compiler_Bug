plain
[00:13:20]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:28] error[E0308]: mismatched types
[00:13:28]    --> librustc_mir/hair/pattern/_match.rs:935:20
[00:13:28]     |
[00:13:28] 935 |             if let Some(ptr) = val.to_ptr() {
[00:13:28]     |                    ^^^^^^^^^ expected enum `std::result::Result`, found enum `std::option::Option`
[00:13:28]     = note: expected type `std::result::Result<rustc::mir::interpret::Pointer, rustc::mir::interpret::EvalError<'_>>`
[00:13:28]                found type `std::option::Option<_>`
[00:13:28] 
[00:13:32] error: aborting due to previous error
[00:13:32] error: aborting due to previous error
[00:13:32] 
[00:13:32] For more information about this error, try `rustc --explain E0308`.
[00:13:32] error: Could not compile `rustc_mir`.
[00:13:32] 
[00:13:32] Caused by:
[00:13:32]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=6ef51208e3ed61cd -C extra-filename=-6ef51208e3ed61cd --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-6e0be5cf77966185.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ccde2368d50449de.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-6a2f0731783c2bd3.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f81250751e0ebed7.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a7df2fc298b4fb06.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-670f6bac60d99b34.so --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9d6678b1c0f0b46.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:15:18] error: build failed
[00:15:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:18] expected success, got: exit code: 101
[00:15:18] expected success, got: exit code: 101
[00:15:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:15:18] travis_fold:end:stage0-rustc

[00:15:18] travis_time:end:stage0-rustc:start=1530009321992557290,finish=1530009951890527563,duration=629897970273


[00:15:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:18] Build completed unsuccessfully in 0:10:42
[00:15:18] Makefile:28: recipe for target 'all' failed
[00:15:18] make: *** [all] Error 1
