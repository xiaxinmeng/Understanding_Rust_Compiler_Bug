plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1691dd16
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:18:51]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:18:51]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:19:11]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:19:24]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:19:35] error[E0599]: no method named `iter` found for type `std::collections::btree_map::Keys<'_, rustc::session::config::OutputType, std::option::Option<std::path::PathBuf>>` in the current scope
[00:19:35]     --> librustc_driver/driver.rs:1358:22
[00:19:35]      |
[00:19:35] 1358 |         output_types.iter().fold(0, |acc, ot| acc +
[00:19:35] 
[00:19:35] error: aborting due to previous error
[00:19:35] 
[00:19:35] For more information about this error, try `rustc --explain E0599`.
[00:19:35] For more information about this error, try `rustc --explain E0599`.
[00:19:35] error: Could not compile `rustc_driver`.
[00:19:35] 
[00:19:35] Caused by:
[00:19:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_driver librustc_driver/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=f7646c9abc3d02c6 -C extra-filename=-f7646c9abc3d02c6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-6d643f75b30009ec.so --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-0a93173f03a79b8d.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-50b4354a63d955e7.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-37c946810bf339eb.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-a22f5323a757a1a5.so --extern rustc_borrowck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_borrowck-0b78be5e49495de3.so --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-ddfd7bb605afc401.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7b2393552f7c05ab.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a587f583bd26ea0.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-1f45a21db1899b6d.so --extern rustc_lint=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint-0c6a22e4bf4525d1.so --extern rustc_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata-e2cdb405d3015fe5.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-beaee3be21b57d8d.so --extern rustc_passes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_passes-c5404bfc39dae280.so --extern rustc_plugin=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin-981f27df1cd6e3a9.so --extern rustc_privacy=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_privacy-bfec5ac3788190b9.so --extern rustc_resolve=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_resolve-2a478c753fc3f86e.so --extern rustc_save_analysis=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_save_analysis-75820b40cd20d4fd.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5aa06a687b1a439d.so --extern rustc_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-b246e0e47fc183bc.so --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-486be882e192b694.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-2d0488f0587a9fe8.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4f33c70c5f92f8b7.so --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-5a0613786e04edc1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-4e87684380615cae.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-fcd4d8eb5dc11f1a/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-571491b5b700df39/out` (exit code: 101)

[00:19:35] travis_time:end:stage0-rustc:start=1531808415710893194,finish=1531809214704997753,duration=798994104559

[00:19:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:19:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:19:35] Build completed unsuccessfully in 0:14:18
[00:19:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:35] expected success, got: exit code: 101
[00:19:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
travis_time:end:02ebf96f:start=1531808039214594879,finish=1531809214969628033,duration=1175755033154

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:04a045ee
---
travis_time:end:061379ae:start=1531809215287034224,finish=1531809215293549675,duration=6515451
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00207a5f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02571af6
travis_time:start:02571af6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ecec07f
$ dmesg | grep -i kill
