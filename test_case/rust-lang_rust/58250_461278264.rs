plain
travis_time:end:003f6956:start=1549504940076743416,finish=1549504942224633523,duration=2147890107
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/cd/29/4f5cee313320f0c284d83c0111bd5b3a26398316d09df3daf883354c9554/awscli-1.16.99-py2.py3-none-any.whl (1.4MB)
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.1MB/s eta 0:00:01
    3% |█▏                              | 51kB 2.5MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:48] 
[01:12:48] running 119 tests
[01:13:13] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:17] i......iii.i.....ii
[01:13:17] 
[01:13:17]  finished in 29.602
[01:13:17] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:39:12] 
[01:39:12] running 194 tests
[01:39:38] ......................i.............................................F............................... 100/194
[01:40:23] failures:
[01:40:23] 
[01:40:23] ---- [run-make] run-make-fulldeps/issue-19371 stdout ----
[01:40:23] 
[01:40:23] 
[01:40:23] error: make failed
[01:40:23] status: exit code: 2
[01:40:23] command: "make"
[01:40:23] stdout:
[01:40:23] ------------------------------------------
[01:40:23] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:40:23] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-19371/issue-19371  foo.rs
[01:40:23] Makefile:8: recipe for target 'all' failed
[01:40:23] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/issue-19371'
[01:40:23] ------------------------------------------
[01:40:23] stderr:
[01:40:23] ------------------------------------------
[01:40:23] error[E0425]: cannot find function `get_codegen_backend` in module `rustc_driver`
[01:40:23] error[E0425]: cannot find function `get_codegen_backend` in module `rustc_driver`
[01:40:23]   --> foo.rs:48:41
[01:40:23]    |
[01:40:23] 48 |     let codegen_backend = rustc_driver::get_codegen_backend(&sess);
[01:40:23]    |                                         ^^^^^^^^^^^^^^^^^^^ not found in `rustc_driver`
[01:40:23] error: aborting due to previous error
[01:40:23] 
[01:40:23] For more information about this error, try `rustc --explain E0425`.
[01:40:23] For more information about this error, try `rustc --explain E0425`.
[01:40:23] make[1]: *** [all] Error 1
[01:40:23] ------------------------------------------
[01:40:23] 
[01:40:23] thread '[run-make] run-make-fulldeps/issue-19371' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:40:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:40:23] 
[01:40:23] 
[01:40:23] 
[01:40:23] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:40:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-6.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:23] 
[01:40:23] 
[01:40:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:23] Build completed unsuccessfully in 0:39:22
[01:40:23] Build completed unsuccessfully in 0:39:22
[01:40:23] Makefile:48: recipe for target 'check' failed
[01:40:23] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c3d2f30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 03:42:55 UTC 2019
