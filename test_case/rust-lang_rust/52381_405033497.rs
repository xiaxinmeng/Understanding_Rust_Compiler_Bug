plain
[00:33:30]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:33:30]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:33:30]    Compiling cc v1.0.17
[00:33:31]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:33:44] error: field is never used: `inner`
[00:33:44]    --> librustc_codegen_llvm/back/write.rs:402:5
[00:33:44]     |
[00:33:44] 402 |     inner: Box<(&'a CodegenContext, &'a Handler)>,
[00:33:44]     |
[00:33:44]     = note: `-D dead-code` implied by `-D warnings`
[00:33:44] 
[00:33:44] error: aborting due to previous error
[00:33:44] error: aborting due to previous error
[00:33:44] 
[00:33:44] error: Could not compile `rustc_codegen_llvm`.
[00:33:44] 
[00:33:44] Caused by:
[00:33:44]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=242ea4f99480000e -C extra-filename=-242ea4f99480000e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-87dc14d9ab583d99.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-f66fe06788cd566c.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-78ead663696977d7.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-c07e4b708a93a0be.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-2054d24eb8c30471.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-249f290afa33ce74.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f8edd2dffe2e499c.rlib --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-f23929c2a4c0e974.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-49056718294d24b1.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-935d26f64401991e.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-358226884af97c0c.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/ntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-928125a637a4ce53.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ea8b9df3ce19611b.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-52b599471d5882f9.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-6205756573390e4e/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-c4a763b0f59ee48a/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-3ffeaaaf0e84f757/out -L native=/usr/lib/llvm-5.0/lib` (exit code: 101)
[00:33:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:33:44] expected success, got: exit code: 101
[00:33:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:33:44] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm

---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1c4ac164
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/ti386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d8714f0
$ dmesg | grep -i kill
