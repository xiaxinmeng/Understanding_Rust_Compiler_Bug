plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:58] 
[00:54:58] running 97 tests
[00:55:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:01] i..ii..iii....i...i...........ii.....F.....i........ii...i.i.ii..............i...ii...i..iii.....
[00:55:01] 
[00:55:01] ---- [codegen] codegen/lifetime_start_end.rs stdout ----
[00:55:01] 
[00:55:01] 
[00:55:01] error: verification with 'FileCheck' failed
[00:55:01] status: exit code: 1
[00:55:01] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll" "/checkout/src/test/codegen/lifetime_start_end.rs"
[00:55:01] ------------------------------------------
[00:55:01] 
[00:55:01] ------------------------------------------
[00:55:01] stderr:
[00:55:01] stderr:
[00:55:01] ------------------------------------------
[00:55:01] /checkout/src/test/codegen/lifetime_start_end.rs:37:11: error: expected string not found in input
[00:55:01] // CHECK: [[E__4:%[0-9]+]] = bitcast { i32, i32 }* %_4 to i8*
[00:55:01]           ^
[00:55:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end/lifetime_start_end.ll:33:2: note: scanning from here
[00:55:01]  %9 = bitcast i32* %c to i8*
[00:55:01]  ^
[00:55:01] ------------------------------------------
[00:55:01] 
[00:55:01] thread '[codegen] codegen/lifetime_start_end.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:55:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:55:01] test result: FAILED. 72 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:55:01] 
[00:55:01] 
[00:55:01] 
[00:55:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:01] 
[00:55:01] 
[00:55:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:55:01] Build completed unsuccessfully in 0:10:20
[00:55:01] Build completed unsuccessfully in 0:10:20
[00:55:01] Makefile:58: recipe for target 'check' failed
[00:55:01] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:257f9421
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
