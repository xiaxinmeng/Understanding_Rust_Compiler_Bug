plain
travis_time:end:05083ce9:start=1545701359640151986,finish=1545701416269965243,duration=56629813257
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:04] 
[01:10:04] running 118 tests
[01:10:29] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:10:33] ......iii.i.....ii
[01:10:33] 
[01:10:33]  finished in 29.194
[01:10:33] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:05] 
[01:38:05] running 193 tests
[01:38:30] ................F...................F............................................................... 100/193
ldeps/extern-flag-fun/extern-flag-fun  bar.rs --crate-type=rlib
[01:39:17] Makefile:4: recipe for target 'all' failed
[01:39:17] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/extern-flag-fun'
[01:39:17] ------------------------------------------
[01:39:17] stderr:
[01:39:17] ------------------------------------------
[01:39:17] ------------------------------------------
[01:39:17] error: couldn't read bar.rs: No such file or directory (os error 2)
[01:39:17] error: aborting due to previous error
[01:39:17] 
[01:39:17] 
[01:39:17] make[1]: *** [all] Error 1
[01:39:17] ------------------------------------------
[01:39:17] 
[01:39:17] thread '[run-make] run-make-fulldeps/extern-flag-fun' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:39:17] 
[01:39:17] 
[01:39:17] ---- [run-make] run-make-fulldeps/mixing-formats stdout ----
[01:39:17] 
[01:39:17] error: make failed
[01:39:17] status: exit code: 2
[01:39:17] command: "make"
[01:39:17] stdout:
[01:39:17] ------------------------------------------
[01:39:17] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/mixing-formats'
[01:39:17] # Building just baz
[01:39:17] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/mixing-formats/mixing-formats  --crate-type=rlib  foo.rs
[01:39:17] Makefile:16: recipe for target 'all' failed
[01:39:17] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/mixing-formats'
[01:39:17] ------------------------------------------
[01:39:17] stderr:
[01:39:17] ------------------------------------------
[01:39:17] error: couldn't read foo.rs: No such file or directory (os error 2)
[01:39:17] error: couldn't read foo.rs: No such file or directory (os error 2)
[01:39:17] 
[01:39:17] error: aborting due to previous error
[01:39:17] 
[01:39:17] make[1]: *** [all] Error 1
[01:39:17] ------------------------------------------
[01:39:17] 
[01:39:17] thread '[run-make] run-make-fulldeps/mixing-formats' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:39:17] 
[01:39:17] 
[01:39:17] ---- [run-make] run-make-fulldeps/no-intermediate-extras stdout ----
[01:39:17] 
[01:39:17] error: make failed
[01:39:17] status: exit code: 2
[01:39:17] command: "make"
[01:39:17] stdout:
[01:39:17] ------------------------------------------
[01:39:17] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/no-intermediate-extras'
[01:39:17] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/no-intermediate-extras/no-intermediate-extras:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/sta-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-setravis_time:end:0beecfc3:start=1545701425209841722,finish=1545707382838165346,duration=5957628323624
travis_time:start:338986b6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 25 03:09:42 UTC 2018
Tue, 25 Dec 2018 03:09:42 GMT
