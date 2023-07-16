plain
[00:09:02]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:19]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:15:15]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:15]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:25] error[E0609]: no field `local_qualif` on type `&mut transform::qualify_consts::Qualifier<'a, 'tcx, 'tcx>`
[00:15:25]    --> librustc_mir/transform/qualify_consts.rs:284:37
[00:15:25]     |
[00:15:25] 284 |                     store(&mut self.local_qualif[index]);
[00:15:25] 
[00:15:29] error: aborting due to previous error
[00:15:29] 
[00:15:29] For more information about this error, try `rustc --explain E0609`.
[00:15:29] For more information about this error, try `rustc --explain E0609`.
[00:15:29] error: Could not compile `rustc_mir`.
[00:15:29] 
[00:15:29] Caused by:
[00:15:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=211ad801b6ebb337 -C extra-filename=-211ad801b6ebb337 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-2acd477744a32d6d.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-50bb024c677a86d3.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9dc12d067443a092.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-97bf376e0a007ec9.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-2bcdf7f1b66c831f.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-7108c7c6c8898c59.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-99c1eadf011fb921.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-ccde15cd50067d0a.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-faee38ea5b8ddd56.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-faee38ea5b8ddd56.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-be2b176ca32c1ea3.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-039faad2a4d9411d.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-79aaef6d504f0382.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-8426f7122ba8055e.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-eaacc24e35645dbd.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-62efdf710984afb1/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-2c40defb9d7437f7/out` (exit code: 101)
[00:17:17] error: build failed
[00:17:17] error: build failed
[00:17:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:17] expected success, got: exit code: 101
[00:17:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:17:17] travis_fold:end:stage0-rustc

[00:17:17] travis_time:end:stage0-rustc:start=1527177627151603652,finish=1527178266399226261,duration=639247622609


[00:17:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:17] Build completed unsuccessfully in 0:10:52
[00:17:17] Makefile:28: recipe for target 'all' failed
[00:17:17] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037c884a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
