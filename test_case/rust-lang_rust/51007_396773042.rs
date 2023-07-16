plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d3/6b/89af9577fc71e5b06b4b6cb196f0bafad885947c8d31518fe673ff90a638/awscli-1.15.37-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
[01:37:59] failures:
[01:37:59] 
[01:37:59] ---- [run-pass] run-pass/issue-38226.rs stdout ----
[01:37:59] 
[01:37:59] error: compilation failed!
[01:37:59] status: signal: 6
[01:37:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-38226.rs" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-38226/a.js" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-Cno-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-38226/auxiliary"
[01:37:59] ------------------------------------------
[01:37:59] 
[01:37:59] ------------------------------------------
[01:37:59] stderr:
[01:37:59] stderr:
[01:37:59] ------------------------------------------
[01:37:59] rustc: /checkout/src/llvm-emscripten/include/llvm/ADT/StringRef.h:236: char llvm::StringRef::operator[](size_t) const: Assertion `Index < Length && "Invalid index!"' failed.
[01:37:59] ------------------------------------------
[01:37:59] 
[01:37:59] thread '[run-pass] run-pass/issue-38226.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:37:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:37:59] 
[01:37:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:37:59] 
[01:37:59] 
[01:37:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/asmjs-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-asmjs-unknown-emscripten" "--mode" "run-pass" "--target" "asmjs-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/4.1.1_64bit/bin/node" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:37:59] 
[01:37:59] 
[01:37:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target asmjs-unknown-emscripten src/test/run-pass src/test/run-fail src/libstd src/liballoc src/libcore
[01:37:59] Build completed unsuccessfully in 1:33:22
---
travis_time:end:016842c0:start=1528848556450140659,finish=1528848556455291891,duration=5151232
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1091c910
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e975fa
$ dmesg | grep -i kill
