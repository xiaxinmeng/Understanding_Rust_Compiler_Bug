plain
[00:19:48]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:19:48]    Compiling cc v1.0.10
[00:19:48]    Compiling num_cpus v1.8.0
[00:19:51]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:19:58] error[E0432]: unresolved import `rustc_const_math::MAX_F32_PLUS_HALF_ULP`
[00:19:58]   --> librustc_trans/mir/rvalue.rs:18:5
[00:19:58]    |
[00:19:58] 18 | use rustc_const_math::MAX_F32_PLUS_HALF_ULP;
[00:19:58]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MAX_F32_PLUS_HALF_ULP` in the root
[00:20:02] error: aborting due to previous error
[00:20:02] 
[00:20:02] For more information about this error, try `rustc --explain E0432`.
[00:20:02] error: Could not compile `rustc_trans`.
[00:20:02] error: Could not compile `rustc_trans`.
[00:20:02] 
[00:20:02] Caused by:
[00:20:02]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_trans librustc_trans/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_back" -C metadata=44faf0b67e52f25e -C extra-filename=-44faf0b67e52f25e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-4e82a86c422a4633.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-0bebbb5d7e5d7571.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so --extern rustc_trans_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trans_utils-91360f6f51642b1c.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-9090c44b5d5f6a2c.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-9b9c9af3e08f5b49.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-3d78a256dcd72406.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-3e9ac89279d9d9fd.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5bcc8c1ccd509892.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-67fd38886086f52b.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-a9575695dd92c62e.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-809b6518a044b653.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4889218d54f10b16.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-9f3518d56a01456f.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-9a88d517684202da.rlib --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-6f259a2a9f59267f.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-1c305a7c9e7bb15b.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-d21add30ecf7f845/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:20:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:20:02] expected success, got: exit code: 101
[00:20:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:20:02] travis_fold:start:stage0-rustc_trans
travis_time:start:stage0-rustc_trans
travis_fold:end:stage0-rustc_trans


[00:20:02] travis_time:end:stage0-rustc_trans:start=1524664426843675726,finish=1524664441395413458,duration=14551737732

[00:20:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:02] Build completed unsuccessfully in 0:15:17
[00:20:02] Makefile:28: recipe for target 'all' failed
[00:20:02] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ed4051
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
