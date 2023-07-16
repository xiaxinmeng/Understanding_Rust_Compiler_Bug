plain
[00:19:01]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:19:09] error[E0282]: type annotations needed
[00:19:09]    --> librustc_codegen_llvm/back/symbol_export.rs:324:34
[00:19:09]     |
[00:19:09] 303 |     let mut instances = DefIdMap();
[00:19:09]     |         ------------- consider giving `instances` a type
[00:19:09] ...
[00:19:09] 324 |                 match substs_map.entry(substs) {
[00:19:09]     |                                  ^^^^^ cannot infer type for `V`
[00:19:09]     = note: type must be known at this point
[00:19:09] 
[00:19:11] error: aborting due to previous error
[00:19:11] 
[00:19:11] 
[00:19:11] For more information about this error, try `rustc --explain E0282`.
[00:19:11] error: Could not compile `rustc_codegen_llvm`.
[00:19:11] 
[00:19:11] Caused by:
[00:19:11]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=07bb1c7b0a926731 -C extra-filename=-07bb1c7b0a926731 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-1ae79b79f441d17a.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-2db1efeb017cfb37.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-a5cc737c52cda9ac.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1f72f6851220bb0d.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-12916f86975023a0.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-7086fce3707083c6.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-2e1858829c05d8c4.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-5bb1bce016d641bc.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-6c69bf415bee3f40.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-a47b92d28b4be44c.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-8ccf59c5e9e172ee.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-cdef313aa912258e.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3f4afd8f1fa86b47.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c1fe7cbcb67aaf3e.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-af2c304aa183f934.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-a2149b14f8ddb9dd.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-66628a8ffaccd0ea.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-0269ec7eb012840c.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-1508bcc57003426f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5bf9074d2dea15d4.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-555c0a62ece4d8d0.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-1b0be7d8560a4fc6.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-298541be0c6807b6/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-523855930bbd979d/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-5ff4d43c6b765d83/out -L native=/usr/lib/llvm-5.0/lib` (exit code: 101)
[00:19:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:19:11] expected success, got: exit code: 101
[00:19:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:19:11] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:19:11] travis_time:end:stage0-rustc_codegen_llvm:start=1532713790964552274,finish=1532713802369289557,duration=11404737283

[00:19:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:12] Build completed unsuccessfully in 0:15:10
[00:19:12] Makefile:28: recipe for target 'all' failed
[00:19:12] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ef631a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3b5ced8b:start=1532713802903338098,finish=1532713802911235464,duration=7897366
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b5e4cd8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:022f7a90
travis_time:start:022f7a90
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05630948
$ dmesg | grep -i kill
