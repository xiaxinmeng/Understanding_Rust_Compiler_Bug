plain
[00:09:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:46]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:18:03]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:18:03]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:18:10] error[E0609]: no field `block` on type `(borrow_check::borrow_set::TwoPhaseUse, rustc::mir::Location)`
[00:18:10]    --> librustc_mir/borrow_check/nll/invalidation.rs:595:32
[00:18:10]     |
[00:18:10] 595 |         if activation_location.block == location.block {
[00:18:10] 
[00:18:10] 
[00:18:10] error[E0609]: no field `statement_index` on type `(borrow_check::borrow_set::TwoPhaseUse, rustc::mir::Location)`
[00:18:10]    --> librustc_mir/borrow_check/nll/invalidation.rs:596:33
[00:18:10]     |
[00:18:10] 596 |             activation_location.statement_index >= location.statement_index
[00:18:10] 
[00:18:10] 
[00:18:10] error[E0609]: no field `block` on type `(borrow_check::borrow_set::TwoPhaseUse, rustc::mir::Location)`
[00:18:10]    --> librustc_mir/borrow_check/nll/invalidation.rs:598:87
[00:18:10]     |
[00:18:10] 598 |             self.mir.dominators().is_dominated_by(location.block, activation_location.block)
[00:18:10] 
[00:18:20] error: aborting due to 3 previous errors
[00:18:20] 
[00:18:20] For more information about this error, try `rustc --explain E0609`.
[00:18:20] For more information about this error, try `rustc --explain E0609`.
[00:18:20] error: Could not compile `rustc_mir`.
[00:18:20] 
[00:18:20] Caused by:
[00:18:20]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=78225296bcd14d2a -C extra-filename=-78225296bcd14d2a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9a1651e7b43f567b.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-550225fbe3657ec3.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-67c14357c369f5d3.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-71e4694314b7f91c.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a476bbed33563995.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-56d64b1c20d3e94a.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-3024734921a8d3c2.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-7575351e73c880a2.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-36d42e1c314b3a21.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ae3ca4254eee93.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f223cc35df6c29ec.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b295ea05411d660b.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
[00:20:58] error: build failed
[00:20:58] travis_fold:end:stage0-rustc


[00:20:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"

[00:20:58] expected success, got: exit code: 101
[00:20:58] expected success, got: exit code: 101
[00:20:58] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:20:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:58] Build completed unsuccessfully in 0:15:19
[00:20:58] Build completed unsuccessfully in 0:15:19
[00:20:58] Makefile:28: recipe for target 'all' failed
[00:20:58] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:016c19aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
