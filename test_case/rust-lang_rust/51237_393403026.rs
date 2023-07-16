plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a2/4f/d432270e1915a31e1eb4943e5080ca8e2e6a1908883d5de45ff4ac3c30a8/awscli-1.15.30-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 1.3MB 995kB/s 
Collecting botocore==1.10.30 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/da/aa/b227500e26dbbd95bd6cda78cf784f769bbad3e74b81bfc52963b55b6363/botocore-1.10.30-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 44.9MB/s eta 0:00:01
    0% |▏                               | 20kB 39.4MB/s eta 0:00:01
    0% |▎                               | 30kB 45.9MB/s eta 0:00:01
    0% |▎                               | 40kB 28.4MB/s eta 0:00:01
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:06] 
[00:47:06] running 1480 tests
[00:47:10] ...........F.............................................................................i..........
[00:47:20] ....................................................................................................
[00:47:23] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:30] ....................................................................................................
---
[00:47:56] ......................................................................i.............................
[00:48:00] ....................................................................................................
[00:48:06] ....................................................................................................
[00:48:12] ....................................................................................................
:"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":871,"byte_end":909,"line_start":32,"line_end":32,"column_start":9,"column_end":47,"is_primary":true,"text":[{"text":"        asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));","highlight_start":9,"highlight_end":47}],"label":"cannot assign twice to immutable variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/asm-out-assign-imm.rs","byte_start":831,"byte_end":836,"line_start":29,"line_end":29,"column_start":5,"column_end":10,"is_primary":false,"text":[{"text":"    x = 1;","highlight_start":5,"highlight_end":10}],"label":"first assignment to `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0384]: cannot assign twice to immutable variable `x`\n  --> /checkout/src/test/ui/asm-out-assign-imm.rs:32:9\n   |\nLL |     x = 1;\n   |     ----- first assignment to `x`\n...\nLL |         asm!(\"mov $1, $0\" : \"=r\"(x) : \"r\"(5));\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable\n\n"}
[00:48:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:16] {"message":"For more information about this error, try `rustc --explain E0384`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0384`.\n"}
[00:48:16] ------------------------------------------
[00:48:16] 
[00:48:16] thread '[ui] ui/asm-out-assign-imm.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:48:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:16] test result: FAILED. 1465 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[00:48:16] 
[00:48:16] 
[00:48:16] 
[00:48:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:16] 
