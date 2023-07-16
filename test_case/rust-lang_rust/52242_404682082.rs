plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f7/fc/4e5fde613a2c680621c500ca1b1f4f2b53bd1d02ea4b5a91663f58e7a320/awscli-1.15.58-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:47:19] ......................................i.............................................................
[00:47:22] ............................i.......................................................................
[00:47:25] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:32] ............................................F................................i......................
  ^^^^^^^^^^^ cannot borrow as mutable
[00:47:34] +    |     ^^^^^^^^^^^ `my_ref` is a `&` reference, so the data it refers to cannot be written
[00:47:34] 9 error: aborting due to previous error
[00:47:34] 10 
[00:47:34] 
[00:47:34] 
[00:47:34] 
[00:47:34] The actual stderr differed from the expected stderr.
[00:47:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/issue-51244.nll.stderr
[00:47:34] To update references, rerun the tests and pass the `--bless` flag
[00:47:34] To only update this specific test, also pass `--test-args suggestions/issue-51244.rs`
[00:47:34] error: 1 errors occurred comparing output.
[00:47:34] status: exit code: 101
[00:47:34] status: exit code: 101
[00:47:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244.nll/auxiliary" "-A" "unused"
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] ------------------------------------------
[00:47:34] stderr:
[00:47:34] stderr:
[00:47:34] ------------------------------------------
[00:47:34] {"message":"cannot assign to `*my_ref` which is behind a `&` reference","code":{"code":"E0594""rendered":"error: aborting due to previous error\n\n"}
[00:47:34] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:47:34] ------------------------------------------
[00:47:34] 
[00:47:34] thread '[ui (nll)] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:47:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:47:34] test result: FAILED. 1557 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:47:34] 
[00:47:34] 
[00:47:34] 
[00:47:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[00:47:34] 
[00:47:34] 
[00:47:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:34] Build completed unsuccessfully in 0:02:14
[00:47:34] Build completed unsuccessfully in 0:02:14
[00:47:34] Makefile:58: recipe for target 'check' failed
[00:47:34] make: *** [check] Error 1
2350756 ./obj
2350724 ./obj/build
1757848 ./obj/build/x86_64-unknown-linux-gnu
782440 ./src
---
143628 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
133584 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
133580 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130360 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z
130356 ./obj/build/bootstrap/debug/incremental/bootstrap-2evv84e4ca5z/s-f2ujtb4zxl-1hqhos6-3b6w870is5kd0
129456 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122720 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
122452 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
107620 ./src/llvm/test/CodeGen
