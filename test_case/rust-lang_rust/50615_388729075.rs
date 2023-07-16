plain
[00:22:25] travis_time:end:stage0-rustc:start=1526283542965234769,finish=1526284579086215499,duration=1036120980730

[00:22:25] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:22:25]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:22:25]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:22:27]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:22:27]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:22:36] error[E0412]: cannot find type `OngoingCrateTranslation` in module `back::write`
[00:22:36]    --> librustc_codegen_llvm/lib.rs:216:55
[00:22:36]     |
[00:22:36] 216 |             ongoing_codegen.downcast::<::back::write::OngoingCrateTranslation>()
[00:22:36]     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ not found in `back::write`
[00:22:36] 
ors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-3f091e6fe4ebd474.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-9fe2228b0d20fd39.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-c44730a3b9fd432b.so --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-9f809ae84e8276de.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-34de494c59e02594.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-dd375c0772cf1a0d.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-3c883c943b4407fc.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-ee923a086d887011.rlib --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-3ad962c4d7907d9a.rlib --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-2973622d1bc90969.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-79ab02d0b6c2aaf8.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-d0ff1c27bea11ea7.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-d9ef745e3327bf6c.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-e17eb5267bbed81a.so --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-74bb0abea141cba4.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-41d448831c9d08f1/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ffd9dcc5ce13143f/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-e367ee641a9e948a/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:22:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:22:40] expected success, got: exit code: 101
[00:22:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:22:40] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:22:40] travis_time:end:stage0-rustc_codegen_llvm:start=1526284579094218942,finish=1526284593848526605,duration=14754307663

[00:22:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:40] Build completed unsuccessfully in 0:17:45
[00:22:40] make: *** [all] Error 1
[00:22:40] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:033849bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
