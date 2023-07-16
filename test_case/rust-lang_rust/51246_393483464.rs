plain
[00:08:00]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:20]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:15:05]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:05]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:09] error[E0599]: no variant named `from_str` found for type `polonius_engine::Algorithm` in the current scope
[00:15:09]    --> librustc_mir/borrow_check/nll/mod.rs:165:17
[00:15:09]     |
[00:15:09] 165 |                 Algorithm::from_str(&algorithm).unwrap(),
[00:15:09]     |                 ^^^^^^^^^^^^^^^^^^^ variant not found in `polonius_engine::Algorithm`
[00:15:19] error: aborting due to previous error
[00:15:19] 
[00:15:19] For more information about this error, try `rustc --explain E0599`.
[00:15:20] error: Could not compile `rustc_mir`.
[00:15:20] error: Could not compile `rustc_mir`.
[00:15:20] 
[00:15:20] Caused by:
[00:15:20]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=37665b7817f51641 -C extra-filename=-37665b7817f51641 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-be994de8b3050feb.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4074300b8e2af006.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5d0a8a65bb9fe29f.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ac6d20f88773f905.so --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-fea5947f694181d1.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-656208abd197f17f.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-78ebc049d4f53f46.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-642a7efe79e24835.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-f16e9eee09cda644.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-5ebb1e8ea909a7e0.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-e46cee1a85353cbb.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-37a39e1dc238018f.so --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-11fcd35bf55f48b3.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-88673787176f9d86/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out` (exit code: 101)
[00:17:21] error: build failed
[00:17:21] error: build failed
[00:17:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:21] expected success, got: exit code: 101
[00:17:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:17:21] travis_fold:end:stage0-rustc

[00:17:21] travis_time:end:stage0-rustc:start=1527760740441013970,finish=1527761464485628886,duration=724044614916


[00:17:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:21] Build completed unsuccessfully in 0:12:17
[00:17:21] Makefile:28: recipe for target 'all' failed
[00:17:21] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cdd4615
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
