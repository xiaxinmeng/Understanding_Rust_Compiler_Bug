plain
travis_time:end:18924f20:start=1558994946674337250,finish=1558994947463391838,duration=789054588
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:50] 
[01:19:50] running 143 tests
[01:19:53] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:19:55] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:55] 
[01:19:55]  finished in 4.597
[01:19:55] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:57] 
[01:19:57] running 9 tests
[01:19:57] iiiiiiiii
[01:19:57] 
[01:19:57]  finished in 0.157
[01:19:57] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:12] 
[01:20:12] running 122 tests
[01:20:37] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:42] .i.i......iii.i.....ii
[01:20:42] 
[01:20:42]  finished in 30.255
[01:20:42] travis_fold:end:test_debuginfo

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:59] 
[01:42:59] running 199 tests
[01:43:26] ..................i...i................................................................i............ 100/199
[01:44:11] ...........................iiii.......Fi..........iiii.iii....................................i....
[01:44:11] 
[01:44:11] ---- [run-make] run-make-fulldeps/print-target-list stdout ----
[01:44:11] 
[01:44:11] error: make failed
[01:44:11] error: make failed
[01:44:11] status: exit code: 2
[01:44:11] command: "make"
[01:44:11] stdout:
[01:44:11] ------------------------------------------
[01:44:11] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/print-target-list'
[01:44:11] for target in aarch64-fuchsia aarch64-linux-android aarch64-pc-windows-msvc aarch64-unknown-cloudabi aarch64-unknown-freebsd aarch64-unknown-hermit aarch64-unknown-linux-gnu aarch64-unknown-linux-musl aarch64-unknown-netbsd aarch64-unknown-none aarch64-unknown-openbsd arm-linux-androideabi arm-unknown-linux-gnueabi arm-unknown-linux-gnueabihf arm-unknown-linux-musleabi arm-unknown-linux-musleabihf armebv7r-none-eabi armebv7r-none-eabihf armv4t-unknown-linux-gnueabi armv5te-unknown-linux-gnueabi armv5te-unknown-linux-musleabi armv6-unknown-freebsd armv6-unknown-netbsd-eabihf armv7-linux-androideabi armv7-unknown-cloudabi-eabihf armv7-unknown-freebsd armv7-unknown-linux-gnueabihf armv7-unknown-linux-musleabihf armv7-unknown-netbsd-eabihf armv7r-none-eabi armv7r-none-eabihf asmjs-unknown-emscripten i586-pc-windows-msvc i586-unknown-linux-gnu i586-unknown-linux-musl i686-apple-darwin i686-linux-android i686-pc-windows-gnu i686-pc-windows-msvc i686-unknown-cloudabi i686-unknown-dragonfly i686-unknown-freebsd i686-unknown-haiku i686-unknown-linux-gnu i686-unknown-linux-musl i686-unknown-netbsd i686-unknown-openbsd mips-unknown-linux-gnu mips-unknown-linux-musl mips-unknown-linux-uclibc mips64-unknown-linux-gnuabi64 mips64el-unknown-linux-gnuabi64 mipsel-unknown-linux-gnu mipsel-unknown-linux-musl mipsel-unknown-linux-uclibc mipsisa32r6-unknown-linux-gnu mipsisa32r6el-unknown-linux-gnu mipsisa64r6-unknown-linux-gnuabi64 mipsisa64r6el-unknown-linux-gnuabi64 msp430-none-elf nvptx64-nvidia-cuda powerpc-unknown-linux-gnu powerpc-unknown-linux-gnuspe powerpc-unknown-linux-musl powerpc-unknown-netbsd powerpc64-unknown-freebsd powerpc64-unknown-linux-gnu powerpc64-unknown-linux-musl powerpc64le-unknown-linux-gnu powerpc64le-unknown-linux-musl riscv32imac-unknown-none-elf riscv32imc-unknown-none-elf riscv64gc-unknown-none-elf riscv64imac-unknown-none-elf s390x-unknown-linux-gnu sparc-unknown-linux-gnu sparc64-unknown-linux-gnu sparc64-unknown-netbsd sparcv9-sun-solaris thumbv6m-none-eabi thumbv7a-pc-windows-msvc thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi thumbv7neon-linux-androideabi thumbv7neon-unknown-linux-gnueabihf thumbv8m.base-none-eabi thumbv8m.main-none-eabi thumbv8m.main-none-eabihf wasm32-experimental-emscripten wasm32-unknown-emscripten wasm32-unknown-unknown wasm32-wasi x86_64-apple-darwin x86_64-fortanix-unknown-sgx x86_64-fuchsia x86_64-linux-android x86_64-pc-windows-gnu x86_64-pc-windows-msvc x86_64-rumprun-netbsd x86_64-sun-solaris x86_64-unknown-cloudabi x86_64-unknown-dragonfly x86_64-unknown-freebsd x86_64-unknown-haiku x86_64-unknown-hermit x86_64-unknown-l4re-uclibc x86_64-unknown-linux-gnu x86_64-unknown-linux-gnux32 x86_64-unknown-linux-musl x86_64-unknown-netbsd x86_64-unknown-openbsd x86_64-unknown-redox x86_64-unknown-uefi; do \
[01:44:11]  LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/print-target-list/print-target-list:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --target $target --print sysroot \
[01:44:11] Makefile:6: recipe for target 'all' failed
[01:44:11] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/print-target-list'
[01:44:11] 
[01:44:11] ------------------------------------------
[01:44:11] ------------------------------------------
[01:44:11] stderr:
[01:44:11] ------------------------------------------
[01:44:11] /bin/sh: 3: Syntax error: end of file unexpected (expecting "done")
[01:44:11] make[1]: *** [all] Error 2
[01:44:11] ------------------------------------------
[01:44:11] 
[01:44:11] 
[01:44:11] 
---
[01:44:11] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:44:11] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:11] 
[01:44:11] 
[01:44:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-6.0/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:44:11] 
[01:44:11] 
[01:44:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:11] Build completed unsuccessfully in 0:36:11
[01:44:11] Build completed unsuccessfully in 0:36:11
[01:44:11] make: *** [check] Error 1
[01:44:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00119dae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 27 23:53:31 UTC 2019
---
travis_time:end:181e8960:start=1559001212967934446,finish=1559001212974144500,duration=6210054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dbfb8d9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a7ce848
travis_time:start:1a7ce848
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:292780a6
$ dmesg | grep -i kill
