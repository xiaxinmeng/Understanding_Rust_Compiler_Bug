plain
[00:19:04]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:19:04]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:19:04]    Compiling cc v1.0.15
[00:19:05]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:19:18] error[E0531]: cannot find tuple struct/variant `AddrSpaceIdx` in this scope
[00:19:18]      |
[00:19:18]      |
[00:19:18] 1569 |         let lifetime_intrinsic = if let AddrSpaceIdx(0) = addr_space {
[00:19:18] help: possible candidate is found in another module, you can import it into scope
[00:19:18]      |
[00:19:18]      |
[00:19:18] 13   | use rustc_target::spec::AddrSpaceIdx;
[00:19:18] 
[00:19:18] 
[00:19:19] error[E0599]: no method named `as_str` found for type `&str` in the current scope
[00:19:19]      |
[00:19:19]      |
[00:19:19] 1570 |             self.cx.get_intrinsic(intrinsic.as_str())
[00:19:19]      |
[00:19:19]      = help: did you mean `as_ptr`?
[00:19:19] 
[00:19:19] 
[00:19:19] error[E0599]: no method named `as_str` found for type `&str` in the current scope
[00:19:19]      |
[00:19:19]      |
[00:19:19] 1573 |             self.cx.get_intrinsic(intrinsic.as_str())
[00:19:19]      |
[00:19:19]      = help: did you mean `as_ptr`?
[00:19:19] 
[00:19:21] error: aborting due to 3 previous errors
[00:19:21] error: aborting due to 3 previous errors
[00:19:21] 
[00:19:21] Some errors occurred: E0531, E0599.
[00:19:21] For more information about an error, try `rustc --explain E0531`.
[00:19:21] error: Could not compile `rustc_codegen_llvm`.
[00:19:21] 
[00:19:21] Caused by:
[00:19:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=2d21adb963ecf69d -C extra-filename=-2d21adb963ecf69d --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-d8b3f1986e621085.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-9983dd82aacb13ea.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-811af56cd876b828.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-7e498553834e51b1.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-e4852f04133fa94c.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-fdf9979cc7b59588.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-023d781fbd65d983.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-9f23e69aa8cce25b.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-1dddb0fa9d8a512f.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-d2d71ef52600c83f.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-c0fe93376ee8cff2.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-be9737b074a8dae0.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-4c6f9bc809b4278a.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-25582ce13d3618ea.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e036f8f5b9204e52.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4c6197a9c6b06d9c.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-42e74c2f020e6a01.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-8fc603afb8ea9b13.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-06dc5f95410eaad9.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-a2c79048744e2ad8.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-c4b130070c8b640c/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:19:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:19:21] expected success, got: exit code: 101
[00:19:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:19:21] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:19:21] travis_time:end:stage0-rustc_codegen_llvm:start=1530649616666100953,finish=1530649634465767886,duration=17799666933

[00:19:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:21] Build completed unsuccessfully in 0:14:39
[00:19:21] Makefile:28: recipe for target 'all' failed
[00:19:21] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f858d62
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15fa7084:start=1530649635016326467,finish=1530649635024854202,duration=8527735
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19fc9d44
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1aa839f4
$ dmesg | grep -i kill
