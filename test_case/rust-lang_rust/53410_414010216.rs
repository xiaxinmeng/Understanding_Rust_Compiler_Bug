plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/6b/fa/89c248eaacccd816fdea88206060a7cd221f227855782ff7b0ffb80d725a/awscli-1.15.81-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 8.0MB/s 
Collecting botocore==1.10.80 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/5e/cf/b97f44993766af17bf64aeddadf66f63b6ebf3d700565cc7ee7b13cd0067/botocore-1.10.80-py2.py3-none-any.whl (4.5MB)
    0% |                                | 10kB 36.6MB/s eta 0:00:01
    0% |▏                               | 20kB 29.6MB/s eta 0:00:01
    0% |▏                               | 30kB 35.3MB/s eta 0:00:01
    0% |▎                               | 40kB 23.4MB/s eta 0:00:01
---
[00:51:41] ....................................................................................................
[00:51:45] ....................................................................................................
[00:51:47] .i..................................................................................................
[00:51:51] ....................................................................................................
[00:51:53] ..................................................iiiiiiiii.........................................
[00:51:59] ....................................................................................................
[00:52:03] ....................................................................................................
[00:52:06] ...............................i....................................................................
[00:52:08] .................................i...............................................i.i..ii............
---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:42] 
[01:21:42] running 189 tests
[01:22:13] .......................................................................................F............
[01:23:07] ........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:23:47] failures:
[01:23:47] 
[01:23:47] ---- [run-make] run-make-fulldeps/libtest-json stdout ----
[01:23:47] 
[01:23:47] 
[01:23:47] error: make failed
[01:23:47] status: exit code: 2
[01:23:47] command: "make"
[01:23:47] stdout:
[01:23:47] ------------------------------------------
[01:23:47] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:23:47] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
[01:23:47] RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json || true
[01:23:47] cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json | "/usr/bin/python2.7" validate_json.py
[01:23:47] # Compare to output file
[01:23:47] diff output.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json
[01:23:47] 2,9c2,9
[01:23:47] < { "type": "test", "event": "started", "name": "a" }
[01:23:47] < { "type": "test", "name": "a", "event": "ok" }
[01:23:47] < { "type": "test", "event": "started", "name": "b" }
[01:23:47] < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'b' panicked at 'assertion failed: false', f.rs:18:5\nnote: Run with `RUST_BACKTRACE=1` for a backtrace.\n" }
[01:23:47] < { "type": "test", "event": "started", "name": "c" }
[01:23:47] < { "type": "test", "name": "c", "event": "ok" }
[01:23:47] < { "type": "test", "event": "started", "name": "d" }
[01:23:47] < { "type": "test", "name": "d", "event": "ignored" }
[01:23:47] ---
[01:23:47] > { "type": "test", "event": "started", "name": "f::a" }
[01:23:47] > { "type": "test", "name": "f::a", "event": "ok" }
[01:23:47] > { "type": "test", "event": "started", "name": "f::b" }
[01:23:47] > { "type": "test", "name": "f::b", "event": "failed", "stdout": "thread 'f::b' panicked at 'assertion failed: false', f.rs:18:5\nnote: Run with `RUST_BACKTRACE=1` for a backtrace.\n" }
[01:23:47] > { "type": "test", "event": "started", "name": "f::c" }
[01:23:47] > { "type": "test", "name": "f::c", "event": "ok" }
[01:23:47] > { "type": "test", "event": "started", "name": "f::d" }
[01:23:47] > { "type": "test", "name": "f::d", "event": "ignored" }
[01:23:47] Makefile:8: recipe for target 'all' failed
[01:23:47] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:23:47] ------------------------------------------
[01:23:47] stderr:
[01:23:47] ------------------------------------------
[01:23:47] ------------------------------------------
[01:23:47] make[1]: *** [all] Error 1
[01:23:47] ------------------------------------------
[01:23:47] 
[01:23:47] 
[01:23:47] thread '[run-make] run-make-fulldeps/libtest-json' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[01:23:47] 
[01:23:47] 
[01:23:47] failures:
[01:23:47]     [run-make] run-make-fulldeps/libtest-json
[01:23:47]     [run-make] run-make-fulldeps/libtest-json
[01:23:47] 
[01:23:47] test result: FAILED. 188 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:23:47] 
[01:23:47] 
[01:23:47] 
[01:23:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:062c3400
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
