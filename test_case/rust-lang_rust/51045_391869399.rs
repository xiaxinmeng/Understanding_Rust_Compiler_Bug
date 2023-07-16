plain
[00:08:47]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:06]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:16:39]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:16:39]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:16:55] error[E0369]: binary operation `<` cannot be applied to type `rustc_const_math::ConstFloat`
[00:16:55]    --> librustc_mir/interpret/operator.rs:139:42
[00:16:55]     |
[00:16:55] 139 |                 Lt => PrimVal::from_bool(l < r),
[00:16:55]     |
[00:16:55]     |
[00:16:55]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `rustc_const_math::ConstFloat`
[00:16:55] 
[00:16:55] error[E0369]: binary operation `<=` cannot be applied to type `rustc_const_math::ConstFloat`
[00:16:55]    --> librustc_mir/interpret/operator.rs:140:42
[00:16:55]     |
[00:16:55] 140 |                 Le => PrimVal::from_bool(l <= r),
[00:16:55]     |
[00:16:55]     |
[00:16:55]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `rustc_const_math::ConstFloat`
[00:16:55] 
[00:16:55] error[E0369]: binary operation `>` cannot be applied to type `rustc_const_math::ConstFloat`
[00:16:55]    --> librustc_mir/interpret/operator.rs:141:42
[00:16:55]     |
[00:16:55] 141 |                 Gt => PrimVal::from_bool(l > r),
[00:16:55]     |
[00:16:55]     |
[00:16:55]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `rustc_const_math::ConstFloat`
[00:16:55] 
[00:16:55] error[E0369]: binary operation `>=` cannot be applied to type `rustc_const_math::ConstFloat`
[00:16:55]    --> librustc_mir/interpret/operator.rs:142:42
[00:16:55]     |
[00:16:55] 142 |                 Ge => PrimVal::from_bool(l >= r),
[00:16:55]     |
[00:16:55]     |
[00:16:55]     = note: an implementation of `std::cmp::PartialOrd` might be missing for `rustc_const_math::ConstFloat`
[00:16:56] error: aborting due to 4 previous errors
[00:16:56] 
[00:16:56] error: Could not compile `rustc_mir`.
[00:16:56] 
[00:16:56] 
[00:16:56] Caused by:
[00:16:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=ddc62f727eb654f5 -C extra-filename=-ddc62f727eb654f5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-1ccff5f8476ad1eb.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-5fcb0dc1f4af3379.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9d0530102f7c6a89.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-02079e25dd5b286e.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c9391cb42eb4751e.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-9b9916e05a598b7c.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-ad409c7ea45bee67.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b6e3d7fc0aaef785.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-fc67797f33795c01.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-140b05c26923af37.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6b77f29b6ee45775.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-6b77f29b6ee45775.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2bc35320c19135ba.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7819cc2ed1ba2b19.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-bda1fe2e3d9116aa.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-fbc249743facc3aa.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-5891bf0101f32727/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-2911c7b5ca977aa1/out` (exit code: 101)
[00:19:40] error: build failed
[00:19:40] error: build failed
[00:19:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:40] expected success, got: exit code: 101
[00:19:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:19:40] travis_fold:end:stage0-rustc

[00:19:40] travis_time:end:stage0-rustc:start=1527196589233628404,finish=1527197418653275030,duration=829419646626


[00:19:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:40] Build completed unsuccessfully in 0:14:06
[00:19:40] Makefile:28: recipe for target 'all' failed
[00:19:40] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aa93a90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
