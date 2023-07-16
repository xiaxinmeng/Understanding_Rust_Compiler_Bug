plain
[00:54:09] ....................................................................................................
[00:54:12] ....................................................i...............................................
[00:54:14] ....................................................................................................
[00:54:17] ....................................................................................................
[00:54:20] iiiiiiiii...........................................................................................
[00:54:26] ....................................................................................................
[00:54:30] ...................................................................................i................
[00:54:32] ....................................................................................................
[00:54:35] ......................................i.i..ii.......................................................
---
[00:59:35] .......................................................................................i............
[00:59:38] ....................................................................................................
[00:59:41] ....................................................................................................
[00:59:43] ....................................................................................................
[00:59:46] ..i.ii.ii.ii.............................i..........................................................
[00:59:46] test result: ok. 6714 passed; 0 failed; 88 ignored; 0 measured; 0 filtered out
[00:59:46] 
[00:59:46]  finished in 397.397
[00:59:46] travis_fold:end:test_ui_nll
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:10] 
[01:28:10] running 191 tests
[01:28:40] ............................................................F.......................................
[01:29:35] ..........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:30:25] failures:
[01:30:25] 
[01:30:25] ---- [run-make] run-make-fulldeps/extern-prelude stdout ----
[01:30:25] 
[01:30:25] 
[01:30:25] error: make failed
[01:30:25] status: exit code: 2
[01:30:25] command: "make"
[01:30:25] stdout:
[01:30:25] ------------------------------------------
[01:30:25] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/extern-prelude'
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  ep-lib.rs
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  ep-vec.rs
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  basic.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  shadow-mod.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  shadow-prelude.rs --extern Vec=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_vec.rlib
[01:30:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude  feature-gate.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude/extern-prelude/libep_lib.rlib 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "access to extern crates through prelude is experimental"
[01:30:25] [[[ begin stdout ]]]
[01:30:25] warning: unused variable: `s`
[01:30:25]    |
[01:30:25]    |
bler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:30:25] 
[01:30:25] 
[01:3
travis_time:end:161f5588:start=1537477488374307001,finish=1537482914393587792,duration=5426019280791
