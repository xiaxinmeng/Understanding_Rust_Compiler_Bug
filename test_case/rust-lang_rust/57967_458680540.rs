plain
travis_time:end:0ec443bb:start=1548784917189213676,finish=1548785082184606520,duration=164995392844
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#####################################################################     96.6%
######################################################################## 100.0%
[00:05:05] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:05:05]     Updating crates.io index
[00:05:15]     Updating git repository `https://github.com/eddyb/rustc-demangle`
[00:05:17]     Updating git repository `https://github.com/michaelwoerister/std-mangle-rs`
[00:05:19]   Downloaded petgraph v0.4.13
[00:05:19]   Downloaded cmake v0.1.33
[00:05:19]   Downloaded filetime v0.2.4
[00:05:19]   Downloaded getopts v0.2.17
---
[00:06:13]   Downloaded quick-error v1.2.2
[00:06:13]   Downloaded miow v0.2.1
[00:06:13]   Downloaded quote v0.3.15
[00:06:13]   Downloaded rand_core v0.3.0
[00:06:13]   Downloaded punycode v0.4.0
[00:06:14]   Downloaded signal-hook v0.1.7
[00:06:14]   Downloaded arc-swap v0.3.7
[00:06:14]   Downloaded same-file v1.0.4
[00:06:14]   Downloaded siphasher v0.2.2
---
[00:06:15]   Downloaded rls-vfs v0.7.0
[00:06:15]   Downloaded rustc-ap-graphviz v306.0.0
[00:06:15]   Downloaded winapi-x86_64-pc-windows-gnu v0.4.0
[00:06:15]   Downloaded backtrace v0.3.11
[00:06:15]   Downloaded unic-idna-punycode v0.7.0
[00:06:15]   Downloaded clap v2.32.0
[00:06:15]   Downloaded rls-blacklist v0.1.3
[00:06:15]   Downloaded tendril v0.4.0
[00:06:15]   Downloaded assert_cli v0.6.2
---
[00:07:18]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:07:18]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:07:39]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:07:40]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:07:40]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:07:45]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:00]     Finished release [optimized] target(s) in 53.29s
[00:08:00] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:08:00] travis_fold:end:stage0-std
---
[00:08:25]    Compiling rustc-demangle v0.1.10
[00:08:25]    Compiling datafrog v2.0.1
[00:08:28]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[00:08:28]    Compiling remove_dir_all v0.5.1
[00:08:28]    Compiling std-mangle-rs v0.1.0 (https://github.com/michaelwoerister/std-mangle-rs?rev=2336dcdfcc91db3cdda18eda73aca488773ac6fc#2336dcdf)
[00:08:28]    Compiling rustc_metadata v0.0.0 (/checkout/src/librustc_metadata)
[00:08:28]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:28]    Compiling rustc_fs_util v0.0.0 (/checkout/src/librustc_fs_util)
[00:08:29]    Compiling rustc-serialize v0.3.24
[00:08:29]    Compiling rustc-serialize v0.3.24
[00:08:29]    Compiling unic-idna-punycode v0.7.0
[00:08:29]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:08:30]    Compiling quick-error v1.2.2
[00:08:30]    Compiling punycode v0.4.0
[00:08:31]    Compiling crossbeam-utils v0.2.2
[00:08:31]    Compiling log v0.4.6
[00:08:32]    Compiling arrayvec v0.4.7
[00:08:32]    Compiling log_settings v0.1.2
---
[00:26:58]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:58]    Compiling libc v0.2.46
[00:26:58]    Compiling cc v1.0.28
[00:26:58]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:26:59]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:27:05]    Compiling num_cpus v1.8.0
[00:27:07]    Compiling memmap v0.6.2
[00:28:20]     Finished release [optimized] target(s) in 1m 22s
[00:28:20] travis_fold:start:stage0-rustc_codegen_llvm
---
[00:28:30]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:28:31]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:28:59]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:29:01]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:29:01]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:29:07]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:29:26]     Finished release [optimized] target(s) in 1m 06s
[00:29:26] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:29:26] travis_fold:end:stage1-std
---
[00:29:57]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[00:29:57]    Compiling remove_dir_all v0.5.1
[00:29:57]    Compiling rustc-demangle v0.1.10
[00:29:58]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:30:00]    Compiling std-mangle-rs v0.1.0 (https://github.com/michaelwoerister/std-mangle-rs?rev=2336dcdfcc91db3cdda18eda73aca488773ac6fc#2336dcdf)
[00:30:00]    Compiling rustc_fs_util v0.0.0 (/checkout/src/librustc_fs_util)
[00:30:01]    Compiling rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
[00:30:01]    Compiling rustc-serialize v0.3.24
[00:30:01]    Compiling rustc-serialize v0.3.24
[00:30:01]    Compiling unic-idna-punycode v0.7.0
[00:30:02]    Compiling quick-error v1.2.2
[00:30:02]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:30:02]    Compiling punycode v0.4.0
[00:30:04]    Compiling crossbeam-utils v0.2.2
[00:30:07]    Compiling log v0.4.6
[00:30:07]    Compiling arrayvec v0.4.7
[00:30:08]    Compiling unreachable v1.0.0
---
[00:52:54]    Compiling libc v0.2.46
[00:52:54]    Compiling cc v1.0.28
[00:52:54]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:52:54]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:52:55]    Compiling rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[00:53:03]    Compiling memmap v0.6.2
[00:53:05]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:54:33]     Finished release [optimized] target(s) in 1m 39s
[00:54:33] travis_fold:start:stage1-rustc_codegen_llvm
---
[01:02:09]     Finished release [optimized] target(s) in 20.99s
[01:02:10]     Checking libc v0.2.46
[01:02:10]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:02:10]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[01:02:10]     Checking rustc-demangle v0.1.13 (https://github.com/eddyb/rustc-demangle?rev=20d5bcc9bcea0d9413540916dd5f9fdadc7012f7#20d5bcc9)
[01:02:11]     Checking backtrace-sys v0.1.27
[01:02:11]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[01:02:15]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[01:02:15]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:58] 
[01:13:58] running 119 tests
[01:14:23] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:27] i......iii.i.....ii
[01:14:27] 
[01:14:27]  finished in 28.768
[01:14:27] travis_fold:end:test_debuginfo

---
[01:40:18] status: exit code: 2
[01:40:18] command: "make"
[01:40:18] stdout:
[01:40:18] ------------------------------------------
[01:40:18] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:40:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  a.rs --cfg x -C prefer-dynamic
[01:40:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  b.rs -C prefer-dynamic
[01:40:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard/b
[01:40:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard  a.rs --cfg y -C prefer-dynamic
[01:40:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard/a-b-a-linker-guard/b && exit 1 || exit 0
[01:40:18] Makefile:8: recipe for target 'all' failed
[01:40:18] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:40:18] ------------------------------------------
[01:40:18] stderr:
[01:40:18] ------------------------------------------
[01:40:18] warning: unused variable: `x`
[01:40:18] warning: unused variable: `x`
[01:40:18]  --> a.rs:5:12
[01:40:18]   |
[01:40:18] 5 | pub fn foo(x: u32) { }
[01:40:18]   |            ^ help: consider prefixing with an underscore: `_x`
[01:40:18]   = note: #[warn(unused_variables)] on by default
[01:40:18] 
[01:40:18] warning: unused variable: `x`
[01:40:18]  --> a.rs:8:12
[01:40:18]  --> a.rs:8:12
[01:40:18]   |
[01:40:18] 8 | pub fn foo(x: i32) { }
[01:40:18]   |            ^ help: consider prefixing with an underscore: `_x`
[01:40:18]   = note: #[warn(unused_variables)] on by default
[01:40:18] 
[01:40:18] 
[01:40:18] make[1]: *** [all] Error 1
[01:40:18] ------------------------------------------
[01:40:18] 
[01:40:18] thread '[run-make] run-make-fulldeps/a-b-a-linker-guard' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3291:9
[01:40:18] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:40:18] test result: FAILED. 192 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:40:18] 
[01:40:18] 
[01:40:18] 
[01:40:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:18] 
[01:40:18] 
[01:40:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:18] Build completed unsuccessfully in 0:37:32
[01:40:18] Build completed unsuccessfully in 0:37:32
[01:40:18] make: *** [check] Error 1
[01:40:18] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aff0cf8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 29 19:45:08 UTC 2019
---
travis_time:end:020acb1c:start=1548791113546933386,finish=1548791113553068666,duration=6135280
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d71bfd0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00eb0ae8
travis_time:start:00eb0ae8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:210843c0
$ dmesg | grep -i kill
