plain
[00:23:10]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:23:26]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:28:32]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:28:32]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:28:34] error[E0658]: The `!` type is experimental (see issue #35121)
[00:28:34]     --> librustc_typeck/check/coercion.rs:1274:25
[00:28:34]      |
[00:28:34] 1274 | impl AsCoercionSite for ! {
[00:28:34]      |
[00:28:34]      |
[00:28:34]      = help: add #![feature(never_type)] to the crate attributes to enable
[00:28:34] error: aborting due to previous error
[00:28:34] 
[00:28:34] For more information about this error, try `rustc --explain E0658`.
[00:28:34] error: Could not compile `rustc_typeck`.
[00:28:34] error: Could not compile `rustc_typeck`.
[00:28:34] 
[00:28:34] Caused by:
[00:28:34]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=74af54dfc2929890 -C extra-filename=-74af54dfc2929890 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-c4c7b11d5784a197.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-caf863901ba00d4d.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3d30704c9bf94823.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ae4e6f9a0a04fcfd.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e7f38b02feb371ba.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-4751eaacfa9b82b1.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-7c8f07fcfb43dd0e.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-94d776423bcbc65e.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-7c1e997399553280.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4acf4151e5e9c3f9.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-306b41e2ad232afa/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-21ebc33766ad71e1/out` (exit code: 101)
[00:28:34] warning: build failed, waiting for other jobs to finish...
[00:28:34] error[E0658]: The `!` type is experimental (see issue #35121)
[00:28:34]    --> librustc_mir/borrow_check/nll/region_infer/dfs.rs:159:18
[00:28:34]     |
[00:28:34] 159 |     type Early = !;
[00:28:34]     |
[00:28:34]     |
[00:28:34]     = help: add #![feature(never_type)] to the crate attributes to enable
[00:28:34] 
[00:28:34] error[E0658]: The `!` type is experimental (see issue #35121)
[00:28:34]    --> librustc_mir/borrow_check/nll/region_infer/dfs.rs:170:89
[00:28:34]     |
[00:28:34] 170 |     fn add_to_target_region(&mut self, point_index: RegionElementIndex) -> Result<bool, !> {
[00:28:34]     |
[00:28:34]     |
[00:28:34]     = help: add #![feature(never_type)] to the crate attributes to enable
[00:28:34] 
[00:28:34] error[E0658]: The `!` type is experimental (see issue #35121)
[00:28:34]    --> librustc_mir/borrow_check/nll/region_infer/dfs.rs:180:86
[00:28:34]     |
[00:28:34] 180 |     fn add_universal_regions_outlived_by_source_to_target(&mut self) -> Result<bool, !> {
[00:28:34]     |
[00:28:34]     |
[00:28:34]     = help: add #![feature(never_type)] to the crate attributes to enable
[00:28:34] 
[00:28:34] error[E0658]: The `!` type is experimental (see issue #35121)
[00:28:34]    --> librustc_mir/interpret/const_eval.rs:198:24
[00:28:34]     |
[00:28:34] 198 |     type MemoryKinds = !;
[00:28:34]     |
[00:28:34]     |
[00:28:34]     = help: add #![feature(never_type)] to the crate attributes to enable
[00:28:34] error: aborting due to 4 previous errors
[00:28:34] 
[00:28:34] For more information about this error, try `rustc --explain E0658`.
[00:28:34] error: Could not compile `rustc_mir`.
[00:28:34] error: Could not compile `rustc_mir`.
[00:28:34] 
[00:28:34] Caused by:
[00:28:34]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=68f9749be38056d4 -C extra-filename=-68f9749be38056d4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ae4e6f9a0a04fcfd.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-c4c7b11d5784a197.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e7f38b02feb371ba.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-30add66c119d2ec6.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-d401bc3652c24ca6.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-94d776423bcbc65e.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-4751eaacfa9b82b1.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4acf4151e5e9c3f9.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-1cd6988448867a7d.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-77b7f1eec921fd34.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9f4c6335ef95c2b7.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-7c1e997399553280.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-caf863901ba00d4d.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-24fcd3d511791c40.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-07d9bc58d0cd25b1.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-07d9bc58d0cd25b1.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-306b41e2ad232afa/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-21ebc33766ad71e1/out` (exit code: 101)
[00:29:08] error: build failed
[00:29:08] error: build failed
[00:29:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:29:08] expected success, got: exit code: 101
[00:29:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:29:08] travis_fold:end:stage1-rustc

[00:29:08] travis_time:end:stage1-rustc:start=1524242267141012193,finish=1524242711671590783,duration=444530578590


[00:29:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:29:08] Build completed unsuccessfully in 0:24:27
[00:29:08] make: *** [all] Error 1
[00:29:08] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1acddd2d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
