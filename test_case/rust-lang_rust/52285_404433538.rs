plain
[00:06:45]     Checking rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:06:45]     Checking rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:06:45]     Checking rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:06:45]     Checking rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:06:46] error: trait objects without an explicit `dyn` are deprecated
[00:06:46]    --> librustc_driver/driver.rs:103:51
[00:06:46]     |
[00:06:46] 103 |             let main_handler = move |worker: &mut FnMut()| {
[00:06:46]     |                                                   ^^^^^^^ help: use `dyn`: `dyn FnMut()`
[00:06:46] note: lint level defined here
[00:06:46]    --> librustc_driver/lib.rs:17:9
[00:06:46]     |
[00:06:46]     |
[00:06:46] 17  | #![deny(bare_trait_objects)]
[00:06:46] 
[00:06:47] error: aborting due to previous error
[00:06:47] 
[00:06:47] error: Could not compile `rustc_driver`.
[00:06:47] error: Could not compile `rustc_driver`.
[00:06:47] 
[00:06:47] Caused by:
[00:06:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_driver librustc_driver/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=6944e9a6624022ce -C extra-filename=-6944e9a6624022ce --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-3cc39b778ad6eff3.rmeta --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-30e4f8963f056c5e.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-89c43c67e66c2c5e.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f65a6b6472ec671d.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-c613fc077385773c.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-b64425f1f54b957b.rmeta --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-53e4bbf2576caee3.rmeta --extern rustc_borrowck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_borrowck-161cbe35ee3b01ee.rmeta --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-0feed5260ec3ff3d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-461b7b50453d68b9.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2447ee5be2489ecc.rmeta --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-e53b758743ed661d.rmeta --extern rustc_lint=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-ce28d92807c7e9b4.rmeta --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-3ddd54cbbbd41071.rmeta --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-b6cabaafc6999e5a.rmeta --extern rustc_passes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-fc3b1d8a1c14802d.rmeta --extern rustc_plugin=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin-821c6482d0f79575.rmeta --extern rustc_privacy=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-7e53c89a5f68e632.rmeta --extern rustc_resolve=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-f4dd4dbd0bd0d316.rmeta --extern rustc_save_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-b48f2d39ea24f9cf.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4067f6d4cf3f919f.rmeta --extern rustc_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-49e1023b7b10dbc6.rmeta --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-d4c974e9b7ae3500.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-941b736db1a7e933.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-92c83a4033c4c4c6.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-f1b1985ded62dc67.rmeta --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-c428771c8b2d0dd1.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-0fb58bdfaac5981c.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e7051d4409bf3a37/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-49fbbb5cce716fd4/out` (exit code: 101)
[00:06:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:47] expected success, got: exit code: 101
[00:06:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:06:47] travis_fold:end:stage0-rustc

[00:06:47] travis_time:end:stage0-rustc:start=1531380116296861112,finish=1531380263013893833,duration=146717032721

---
travis_time:end:0db311e9:start=1531380263551228018,finish=1531380263557107342,duration=5879324
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d7c9253
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:171fffb5
$ dmesg | grep -i kill
