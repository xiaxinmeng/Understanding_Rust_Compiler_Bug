plain
[00:48:55] .................................................................................................... 2200/4601
[00:49:00] ..................i................................................................................. 2300/4601
[00:49:03] .................................................................................................... 2400/4601
[00:49:07] .................................................................................................... 2500/4601
[00:49:10] ...............................iiiiiiiii............................................................ 2600/4601
[00:49:16] .................................................................................................... 2800/4601
[00:49:20] .................................................................................................... 2900/4601
[00:49:23] ......................................................i............................................. 3000/4601
[00:49:26] .................................................................................................... 3100/4601
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:52] 
[01:01:52] running 111 tests
[01:01:55] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:01:55] ..iiii.....
[01:01:55] 
[01:01:55]  finished in 3.381
[01:01:55] travis_fold:end:test_codegen

---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:54] 
[01:28:54] running 192 tests
[01:29:24] .................................................................................................... 100/192
[01:30:20] ..........................................................F................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:31:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[01:31:18] failures:
[01:31:18] 
[01:31:18] ---- [run-make] run-make-fulldeps/simd-ffi stdout ----
[01:31:18] 
[01:31:18] 
[01:31:18] error: make failed
[01:31:18] status: exit code: 2
[01:31:18] command: "make"
[01:31:18] stdout:
[01:31:18] ------------------------------------------
[01:31:18] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/simd-ffi'
[01:31:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-linux-androideabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-linux-androideabi
[01:31:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabihf --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabihf
[01:31:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=arm-unknown-linux-gnueabi --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-arm-unknown-linux-gnueabi
[01:31:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=aarch64-unknown-linux-gnu --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-aarch64-unknown-linux-gnu
[01:31:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/simd-ffi/simd-ffi  --target=mips-unknown-linux-gnu --emit=llvm-ir,asm simd.rs -C target-feature='+neon,+sse2' -C extra-filename=-mips-unknown-linux-gnu
[01:31:18] Makefile:47: recipe for target 'mips-unknown-linux-gnu' failed
[01:31:18] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/simd-ffi'
[01:31:18] ------------------------------------------
[01:31:18] stderr:
[01:31:18] ------------------------------------------
[01:31:18] ------------------------------------------
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:21:8
[01:31:18]    |
[01:31:18] 21 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 22 | #[derive(Copy)]
[01:31:18] 23 | #[repr(simd)]
[01:31:18] 
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:36:8
[01:31:18]    |
[01:31:18]    |
[01:31:18] 36 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 37 | #[derive(Copy)]
[01:31:18] 38 | #[repr(simd)]
[01:31:18] 
[01:31:18] 
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:21:8
[01:31:18]    |
[01:31:18] 21 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 22 | #[derive(Copy)]
[01:31:18] 23 | #[repr(simd)]
[01:31:18] 
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:36:8
[01:31:18]    |
[01:31:18]    |
[01:31:18] 36 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 37 | #[derive(Copy)]
[01:31:18] 38 | #[repr(simd)]
[01:31:18] 
[01:31:18] 
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:21:8
[01:31:18]    |
[01:31:18] 21 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 22 | #[derive(Copy)]
[01:31:18] 23 | #[repr(simd)]
[01:31:18] 
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:36:8
[01:31:18]    |
[01:31:18]    |
[01:31:18] 36 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 37 | #[derive(Copy)]
[01:31:18] 38 | #[repr(simd)]
[01:31:18] 
[01:31:18] 
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:21:8
[01:31:18]    |
[01:31:18] 21 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 22 | #[derive(Copy)]
[01:31:18] 23 | #[repr(simd)]
[01:31:18] 
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:36:8
[01:31:18]    |
[01:31:18]    |
[01:31:18] 36 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 37 | #[derive(Copy)]
[01:31:18] 38 | #[repr(simd)]
[01:31:18] 
[01:31:18] 
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:21:8
[01:31:18]    |
[01:31:18] 21 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 22 | #[derive(Copy)]
[01:31:18] 23 | #[repr(simd)]
[01:31:18] 
[01:31:18] warning[E0566]: conflicting representation hints
[01:31:18]   --> simd.rs:36:8
[01:31:18]    |
[01:31:18]    |
[01:31:18] 36 | #[repr(C)]
[01:31:18]    |        ^
[01:31:18] 37 | #[derive(Copy)]
[01:31:18] 38 | #[repr(simd)]
[01:31:18] 
[01:31:18] 
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+neon' is not a recognized feature for this target (ignoring feature)
[01:31:18] '+sse2' is not a recognized feature for this target (ignoring feature)
[01:31:18] Segmentation fault (core dumped)
[01:31:18] make[1]: *** [mips-unknown-linux-gnu] Error 139
[01:31:18] ------------------------------------------
[01:31:18] 
[01:31:18] thread '[run-make] run-make-fulldeps/simd-ffi' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:31:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:31:18] test result: FAILED. 191 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:31:18] 
[01:31:18] 
[01:31:18] 
[01:31:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:31:18] 
[01:31:18] 
[01:31:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:18] Build completed unsuccessfully in 0:46:57
[01:31:18] Build completed unsuccessfully in 0:46:57
[01:31:18] Makefile:58: recipe for target 'check' failed
[01:31:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:094b5712
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1088e15d:start=1539556059893271001,finish=1539556060852755257,duration=959484256
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1243b9dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c8e621
$ dmesg | grep -i kill
