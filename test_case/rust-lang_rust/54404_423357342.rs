plain
[00:58:42] ....................................................................................................
[00:58:45] ....................................................i...............................................
[00:58:48] ....................................................................................................
[00:58:51] ....................................................................................................
[00:58:54] iiiiiiiii...........................................................................................
[00:59:00] ....................................................................................................
[00:59:03] .................................................................................i..................
[00:59:06] ....................................................................................................
[00:59:09] ...................................i.i..ii..........................................................
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:33:00] 
[01:33:00] running 191 tests
[01:33:31] ......................................................F.............................................
[01:34:25] ..........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:35:07] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:35:07] failures:
[01:35:07] 
[01:35:07] ---- [run-make] run-make-fulldeps/extern-prelude stdout ----
[01:35:07] 
[01:35:07] 
[01:35:07] error: make failed
[01:35:07] status: exit code: 2
[01:35:07] command: "make"
[01:35:07] stdout:
[01:35:07] ------------------------------------------
[01:35:07] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  ep-lib.rs
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  ep-vec.rs
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  basic.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  shadow-mod.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  shadow-prelude.rs --extern Vec=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_vec.rlib
[01:35:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  feature-gate.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "access to extern crates through prelude is experimental"
[01:35:07] [[[ begin stdout ]]]
[01:35:07] warning: unused variable: `s`
[01:35:07]    |
[01:35:07]    |
[01:35:07] 12 |     let s = ep_lib::S; // Feature error
[01:35:07]    |         ^ help: consider using `_s` instead
[01:35:07]    = note: #[warn(unused_variables)] on by default
[01:35:07] 
[01:35:07] 
[01:35:07] 
[01:35:07] [[[ end stdout ]]]
[01:35:07] Error: cannot match: access to extern crates through prelude is experimental
[01:35:07] Makefile:4: recipe for target 'all' failed
[01:35:07] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:35:07] ------------------------------------------
[01:35:07] stderr:
[01:35:07] ------------------------------------------
[01:35:07] warning: unused variable: `arg1`
[01:35:07] warning: unused variable: `arg1`
[01:35:07]   --> ep-vec.rs:13:12
[01:35:07]    |
[01:35:07] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:35:07]    |            ^^^^ help: consider using `_arg1` instead
[01:35:07]    = note: #[warn(unused_variables)] on by default
[01:35:07] 
[01:35:07] warning: unused variable: `arg2`
[01:35:07] warning: unused variable: `arg2`
[01:35:07]   --> ep-vec.rs:13:23
[01:35:07]    |
[01:35:07] 13 | pub fn new(arg1: f32, arg2: ()) {}
[01:35:07]    |                       ^^^^ help: consider using `_arg2` instead
[01:35:07] 
[01:35:07] warning: the feature `extern_prelude` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:35:07]   --> basic.rs:11:12
[01:35:07]    |
[01:35:07] 11 | #![feature(extern_prelude)]
[01:35:07]    |
[01:35:07]    = note: #[warn(stable_features)] on by default
[01:35:07] 
[01:35:07] warning: unused variable: `x`
[01:35:07] warning: unused variable: `x`
[01:35:07]   --> shadow-prelude.rs:16:9
[01:35:07]    |
[01:35:07] 16 |     let x = Vec::new(0f32, ()); // OK
[01:35:07]    |         ^ help: consider using `_x` instead
[01:35:07]    = note: #[warn(unused_variables)] on by default
[01:35:07] 
[01:35:07] 
[01:35:07] warning: the feature `extern_prelude` has been stable since 1.30.0 and no longer requires an attribute to enable
[01:35:07]   --> shadow-prelude.rs:13:12
[01:35:07]    |
[01:35:07] 13 | #![feature(extern_prelude)]
[01:35:07]    |
[01:35:07]    = note: #[warn(stable_features)] on by default
[01:35:07] 
[01:35:07] 
[01:35:07] make[1]: *** [all] Error 1
[01:35:07] ------------------------------------------
[01:35:07] 
[01:35:07] 
[01:35:07] thread '[run-make] run-make-fulldeps/extern-prelude' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[01:35:07] 
[01:35:07] 
[01:35:07] failures:
[01:35:07]     [run-make] run-make-fulldeps/extern-prelude
[01:35:07]     [run-make] run-make-fulldeps/extern-prelude
[01:35:07] 
[01:35:07] test result: FAILED. 190 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:35:07] 
[01:35:07] 
[01:35:07] 
[01:35:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:35:07] 
[01:35:07] 
[01:35:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:07] Build completed unsuccessfully in 0:45:29
[01:35:07] Build completed unsuccessfully in 0:45:29
[01:35:07] make: *** [check] Error 1
[01:35:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18d646c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
