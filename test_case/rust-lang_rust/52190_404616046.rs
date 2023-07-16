plain
[00:06:22]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:06:36]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:11:41]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:11:41]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:11:43] error[E0425]: cannot find value `constraints_scc` in this scope
[00:11:43]    --> librustc_mir/borrow_check/nll/region_infer/mod.rs:233:23
[00:11:43]     |
[00:11:43] 233 |             let scc = constraints_scc.scc(region);
[00:11:43]     |                       ^^^^^^^^^^^^^^^ did you mean `constraint_sccs`?
[00:11:52] error: aborting due to previous error
[00:11:52] 
[00:11:52] For more information about this error, try `rustc --explain E0425`.
[00:11:52] error: Could not compile `rustc_mir`.
[00:11:52] error: Could not compile `rustc_mir`.
[00:11:52] 
[00:11:52] Caused by:
[00:11:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=eb891085b7b80162 -C extra-filename=-eb891085b7b80162 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ebe584ed738e3de8.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-c8771f0802893cf2.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-2a1f5ee3e102fe42.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-b51deb005c05dd3b.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-6020508f01da724d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-30c8a625cec7af60.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-3ad1691935709254.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7d678b2486a5f5d9.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f3f1ff1495349b05.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-0b0f824eb96efcb5.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dfab6c84d2674220.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb4c86e007981750.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e528c05031478194.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5f603e9854c9c328.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-786292eb849f05d6.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e7051d4409bf3a37/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-49fbbb5cce716fd4/out` (exit code: 101)
[00:13:15] error: build failed
[00:13:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:15] expected success, got: exit code: 101
[00:13:15] expected success, got: exit code: 101
[00:13:15] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:13:15] travis_fold:end:stage0-rustc

[00:13:15] travis_time:end:stage0-rustc:start=1531421410806835964,finish=1531421938780663921,duration=527973827957


[00:13:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:15] Build completed unsuccessfully in 0:09:40
[00:13:15] Makefile:28: recipe for target 'all' failed
[00:13:15] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:014d4136
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
