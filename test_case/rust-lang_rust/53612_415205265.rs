plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4e/cf/0c313db4b8e3b231447d3807657db4f5e7fad26d5eaeb294b3cfa1388a6c/awscli-1.15.84-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:29] 
[00:48:29] running 4155 tests
[00:48:31] .............................F......................................................................
[00:48:37] ....................................................................................................
[00:48:40] ....................................................................................................
[00:48:43] ....................................................................................................
[00:48:46] ..............i.....................................................................................
---
[00:50:22] ...................................................i................................................
[00:50:25] ....................................................................................................
[00:50:28] ....................................................................................................
[00:50:31] ..............................................................................................i.....
to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-denied-2018/anon-params-denied-2018.stderr
[00:50:32] To update references, rerun the tests and pass the `--bless` flag
[00:50:32] To only update this specific test, also pass `--test-args anon-params-denied-2018.rs`
[00:50:32] error: 1 errors occurred comparing output.
[00:50:32] status: exit code: 1
[00:50:32] status: exit code: 1
[00:50:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params-denied-2018.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-denied-2018/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params-denied-2018/auxiliary" "-A" "unused"
[00:50:32] ------------------------------------------
[00:50:32] 
[00:50:32] ------------------------------------------
[00:50:32] stderr:
[00:50:32] stderr:
[00:50:32] ------------------------------------------
[00:50:32] {"message":"expected one of `:` or `@`, found `)`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/anon-params-denied-2018.rs","byte_start":578,"byte_end":579,"line_start":16,"line_end":16,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"    fn foo(i32); //~ expected one of `:` or `@`, found `)`","highlight_start":15,"highlight_end":16}],"label":"expected one of `:` or `@` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `:` or `@`, found `)`\n  --> /checkout/src/test/ui/anon-params-denied-2018.rs:16:15\n   |\nLL |     fn foo(i32); //~ expected one of `:` or `@`, found `)`\n   |               ^ expected one of `:` or `@` here\n\n"}
[00:50:32] {"message":"expected one of `:` or `@`, found `,`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/anon-params-denied-2018.rs","byte_start":659,"byte_end":660,"line_start":18,"line_end":18,"column_start":36,"column_end":37,"is_primary":true,"text":[{"text":"    fn bar_with_default_impl(String, String) {}","highlight_start":36,"highlight_end":37}],"label":"expected one of `:` or `@` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected one of `:` or `@`, found `,`\n  --> /checkout/src/test/ui/anon-params-denied-2018.rs:18:36\n   |\nLL |     fn bar_with_default_impl(String, String) {}\n   |                                    ^ expected one of `:` or `@` here\n\n"}
[00:50:32] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:50:32] ------------------------------------------
[00:50:32] 
[00:50:32] thread '[ui] ui/anon-params-denied-2018.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:50:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:50:32] 
[00:50:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:50:32] 
[00:50:32] 
[00:50:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:32] 
[00:50:32] 
[00:50:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:32] Build completed unsuccessfully in 0:03:12
[00:50:32] Build completed unsuccessfully in 0:03:12
[00:50:32] make: *** [check] Error 1
[00:50:32] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13164b55
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
