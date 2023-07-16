plain
    100% |████████████████████████████████| 51kB 6.4MB/s 
Collecting botocore==1.10.51 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/ba/f6c9220d87784a85f24a8f2425edccb2f330d15c304ea2373ed8206a03ca/botocore-1.10.51-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 42.1MB/s eta 0:00:01
    0% |▏                               | 20kB 25.3MB/s eta 0:00:01
    0% |▎                               | 30kB 31.5MB/s eta 0:00:01
    0% |▎                               | 40kB 14.2MB/s eta 0:00:01
---
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:00:00] travis_fold:start:init_repo
travis_time:start:090f8175
rm 'src/llvm'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/509f29ac17874394acf4d49d6bae3cd93c652aa1.tar.gz
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/f475da63a18d50217459a601cbef69a4bcac5e71.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/d2a64395a5210a61d3512a3a5c615f5c47699443.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:44] 
[01:15:44] running 187 tests
[01:16:24] ....................i...............................................................................
[01:17:16] .........................................................................F............test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:18:18] failures:
[01:18:18] 
[01:18:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:18:18] ---- [run-make] run-make-fulldeps/target-without-atomics stdout ----
[01:18:18] ---- [run-make] run-make-fulldeps/target-without-atomics stdout ----
[01:18:18] 
[01:18:18] error: make failed
[01:18:18] status: exit code: 2
[01:18:18] command: "make"
[01:18:18] stdout:
[01:18:18] ------------------------------------------
[01:18:18] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/target-without-atomics'
[01:18:18] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics  --print cfg --target thumbv6m-none-eabi | "/checkout/src/etc/cat-and-grep.sh" -v target_has_atomic
[01:18:18] [[[ begin stdout ]]]
[01:18:18] debug_assertions
[01:18:18] target_arch="arm"
[01:18:18] target_endian="little"
[01:18:18] target_env=""
[01:18:18] target_has_atomic="16"
[01:18:18] target_has_atomic="32"
[01:18:18] target_has_atomic="8"
[01:18:18] target_has_atomic="ptr"
[01:18:18] target_os="none"
[01:18:18] target_pointer_width="32"
[01:18:18] target_vendor=""
[01:18:18] 
[01:18:18] [[[ end stdout ]]]
[01:18:18] Error: should not match: target_has_atomic
[01:18:18] Makefile:5: recipe for target 'all' failed
[01:18:18] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/target-without-atomics'
[01:18:18] ------------------------------------------
[01:18:18] stderr:
[01:18:18] ------------------------------------------
[01:18:18] ------------------------------------------
[01:18:18] make[1]: *** [all] Error 1
[01:18:18] ------------------------------------------
[01:18:18] 
[01:18:18] 
[01:18:18] thread '[run-make] run-make-fulldeps/target-without-atomics' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:18:18] 
[01:18:18] 
[01:18:18] failures:
[01:18:18]     [run-make] run-make-fulldeps/target-without-atomics
[01:18:18]     [run-make] run-make-fulldeps/target-without-atomics
[01:18:18] 
[01:18:18] test result: FAILED. 185 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:18:18] 
[01:18:18] 
[01:18:18] 
[01:18:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:18:18] 
[01:18:18] 
[01:18:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:18] Build completed unsuccessfully in 0:40:57
[01:18:18] Build completed unsuccessfully in 0:40:57
[01:18:18] make: *** [check] Error 1
[01:18:18] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29930b5a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:07307ccd:start=1530836882620641119,finish=1530836882628529675,duration=7888556
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f2c6e32
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19e86cf4
$ dmesg | grep -i kill
