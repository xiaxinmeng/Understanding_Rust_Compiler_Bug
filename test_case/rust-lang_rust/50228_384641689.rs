plain
[00:07:15]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:07:35]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:09]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:09]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:21] error[E0599]: no variant named `from_attr` found for type `rustc_target::abi::Integer` in the current scope
[00:14:21]    --> librustc_mir/interpret/eval_context.rs:885:36
[00:14:21]     |
[00:14:21] 885 |                     let discr_ty = layout::Integer::from_attr(self.tcx.tcx, discr_ty);
[00:14:21]     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^ variant not found in `rustc_target::abi::Integer`
[00:14:21]     |
[00:14:21]     = help: items from traits can only be used if the trait is in scope
[00:14:21]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:14:21]             candidate #1: `use rustc::ty::layout::IntegerExt;`
[00:14:23] error: aborting due to previous error
[00:14:23] 
[00:14:23] For more information about this error, try `rustc --explain E0599`.
[00:14:23] error: Could not compile `rustc_mir`.
[00:14:23] error: Could not compile `rustc_mir`.
[00:14:23] 
[00:14:23] Caused by:
[00:14:23]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8beb98c628ea2fc1 -C extra-filename=-8beb98c628ea2fc1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-21033e7d98b3b734.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-521818bd083c928c.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-0b7f7b8330cb09ce.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-de29d22526477961.so --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-70f7ed359bd54256.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-550afbc40330c44b.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-7f999938245933d1.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-959ba3a7ab7a2be3.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-0bde40de32995f14.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-cb643e904ecf9227.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-648afcf1a0c2954c.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:16:04] error: build failed
[00:16:04] error: build failed
[00:16:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:04] expected success, got: exit code: 101
[00:16:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:04] travis_fold:end:stage0-rustc

[00:16:04] travis_time:end:stage0-rustc:start=1524748512074088992,finish=1524749171467326972,duration=659393237980


[00:16:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:04] Build completed unsuccessfully in 0:11:12
[00:16:04] Makefile:28: recipe for target 'all' failed
[00:16:04] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f843603
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
