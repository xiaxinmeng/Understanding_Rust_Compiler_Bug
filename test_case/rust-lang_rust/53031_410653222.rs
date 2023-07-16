plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0b2eaaf8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:24:49]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:24:49]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:24:49]    Compiling cc v1.0.18
[00:24:50]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:24:56] error[E0425]: cannot find value `CrateTypeRlib` in module `config`
[00:24:56]     --> librustc_codegen_llvm/back/write.rs:2358:70
[00:24:56]      |
[00:24:56] 2358 |         tcx.sess.crate_types.borrow().iter().any(|ct| *ct == config::CrateTypeRlib) &&
[00:24:56]      |                                                                      ^^^^^^^^^^^^^ not found in `config`
[00:25:01] error: aborting due to previous error
[00:25:01] 
[00:25:01] For more information about this error, try `rustc --explain E0425`.
[00:25:01] error: Could not compile `rustc_codegen_llvm`.
[00:25:01] error: Could not compile `rustc_codegen_llvm`.
[00:25:01] 
[00:25:01] Caused by:
[00:25:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg 'feature="jemalloc"' --cfg 'feature="rustc_target"' -C metadata=a8ce002136ec64b4 -C extra-filename=-a8ce002136ec64b4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-277ca49f42ab2e53.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-f28e2ea1414f1406.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-94b7c8f2f6f79e1b.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ba31465b048d39f4.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-00919937f377f0bc.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-18d8fee9014249a2.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-e4f798202172c031.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-dd0a9429b0aacf96.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-62a891846402192d.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-bb3fc1dfe0105c1c.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-072369dae17b1893.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-780c150b6c3acb38.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4f182245385237b5.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-5ed58e76c7dace45.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1cc3798595d9708a.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-2e835ba951928190.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1ed4d2103c0d7730.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-899ce576c6b4bcbf.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-45767bac74e332c7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-71c6a17c4b962f9c/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib -L native=/rustroot/bin/../lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../lib64` (exit code: 1)
[00:25:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:25:01] expected success, got: exit code: 101
[00:25:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[00:25:01] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

---
travis_time:end:2d2b4788:start=1533549117031028139,finish=1533549117038181310,duration=7153171
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:074602dd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21692f38
travis_time:start:21692f38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0df42b4c
$ dmesg | grep -i kill
