plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/26/a5/981fcedb0cba5cd167382878c92956229d14f0ac6846c71f2cd87906d474/awscli-1.15.10-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 2.0MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
  Downloading https://files.pythonhosted.org/packages/b7/8e/ddb32ddaabd431813e180ca224e844bab8ad42fbb47ee07553f0ec44cd86/colorama-0.3.7-py2.py3-none-any.whl
Collecting botocore==1.10.10 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/77/c9/a40ebce24bbab4c7986fccdac9dade097385ad2feae73dcc47d31a1b4dc8/botocore-1.10.10-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 44.1MB/s eta 0:00:01
    0% |▏                               | 20kB 27.3MB/s eta 0:00:01
    0% |▎                               | 30kB 32.5MB/s eta 0:00:01
    0% |▎                               | 40kB 13.9MB/s eta 0:00:01
---
[00:56:50] ......i....................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:56:54] .........
[00:57:28] ....................................................................................................
[00:57:57] .......................................................................ii...........................
[00:58:50] ...................................i....................................................i.ii......test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:59:36] ................................................................................................iiii
[01:00:03] iii.................................................................................................
[01:00:33] ....................................................................................................
[01:00:59] ....................................................................................................
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:35:10] 
[01:35:10] running 183 tests
[01:35:58] .................................................F..................................................
[01:36:47] ..................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
n-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  ep-vec.rs
[01:37:56] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu  basic.rs --extern ep_lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu/libep_lib.rlib
[01:37:56] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-prelude.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown    |
[01:37:56] 
[01:37:56] 
[01:37:56] error[E0599]: no method named `internal` found for type `ep_lib::S` in the current scope
[01:37:56]   --> inner-mod.rs:18:7
[01:37:56]    |
[01:37:56] 18 |     s.internal(); // OK
[01:37:56]    |
[01:37:56]    |
[01:37:56]    = help: did you mean `external`?
[01:37:56] error: aborting due to previous error
[01:37:56] 
[01:37:56] For more information about this error, try `rustc --explain E0599`.
[01:37:56] For more information about this error, try `rustc --explain E0599`.
[01:37:56] make[1]: *** [all] Error 101
[01:37:56] ------------------------------------------
[01:37:56] 
[01:37:56] 
[01:37:56] thread '[run-make] run-make-fulldeps/extern-prelude' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[01:37:56] 
[01:37:56] 
[01:37:56] failures:
[01:37:56]     [run-make] run-make-fulldeps/extern-prelude
[01:37:56]     [run-make] run-make-fulldeps/extern-prelude
[01:37:56] 
[01:37:56] test result: FAILED. 182 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:56] 
[01:37:56] 
[01:37:56] 
[01:37:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo "$RUN_SCRIPT"" exited with 2.
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 27 01:05:14 UTC 2018
Fri, 27 Apr 2018 01:05:14 GMT
travis_time:end:002f1778:start=1524791114756601316,finish=1524791114829162918,duration=72561602
