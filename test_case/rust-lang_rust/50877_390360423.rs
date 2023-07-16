plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/c2/1e/f70d1125f5bf6383d2ee7a76aea72bed6ba103c1bb9dd4ca051787552d2b/awscli-1.15.24-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 15.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:48:30] ..................................................i.................................................
[00:48:40] ....................................................................................i...............
[00:48:49] ......................................................................i.............................
[00:49:01] ....................................................................................................
[00:49:15] ...................................................................F................................
[00:49:42] .......................................i............................................................
[00:50:02] ......................................i.............................................................
[00:50:29] ....................................................................................................
[00:50:29] ..test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
---
[00:53:03] ---- [run-pass] run-pass/loop-break-value.rs stdout ----
[00:53:03] 
[00:53:03] error: compilation failed!
[00:53:03] status: exit code: 101
[00:53:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/loop-break-value.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/loop-break-value/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/loop-break-value/auxiliary"
[00:53:03] ------------------------------------------
[00:53:03] 
[00:53:03] ------------------------------------------
[00:53:03] stderr:
[00:53:03] stderr:
[00:53:03] ------------------------------------------
[00:53:03] error[E0426]: use of undeclared label `'inner_loop`
[00:53:03]     |
[00:53:03]     |
[00:53:03] 126 |         'inner_loop: while break 'inner_loop {
[00:53:03]     |                                  ^^^^^^^^^^^ undeclared label `'inner_loop`
[00:53:03] warning: unreachable expression
[00:53:03]   --> /checkout/src/test/run-pass/loop-break-value.rs:27:21
[00:53:03]    |
[00:53:03]    |
[00:53:03] 27 |                     break 'outer panic!();
[00:53:03]    |
[00:53:03]    = note: #[warn(unreachable_code)] on by default
[00:53:03] 
[00:53:03] warning: unreachable expression
[00:53:03] warning: unreachable expression
[00:53:03]   --> /checkout/src/test/run-pass/loop-break-value.rs:48:9
[00:53:03]    |
[00:53:03] 48 | /         break if true {
[00:53:03] 49 | |             break Default::default()
[00:53:03] 50 | |         } else {
[00:53:03] 51 | |             break [13, 14]
[00:53:03]    | |_________^
[00:53:03] 
[00:53:03] warning: unreachable expression
[00:53:03]   --> /checkout/src/test/run-pass/loop-break-value.rs:78:13
[00:53:03]   --> /checkout/src/test/run-pass/loop-break-value.rs:78:13
[00:53:03]    |
[00:53:03] 78 |             break break Default::default();
[00:53:03] 
[00:53:03] warning: unreachable statement
[00:53:03]    --> /checkout/src/test/run-pass/loop-break-value.rs:103:9
[00:53:03]     |
---
[00:53:03] 
[00:53:03] warning: unreachable statement
[00:53:03]    --> /checkout/src/test/run-pass/loop-break-value.rs:135:13
[00:53:03]     |
[00:53:03] 135 |             panic!("from_inner");
[00:53:03]     |
[00:53:03]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:53:03] 
[00:53:03] 
[00:53:03] ward/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:03] 
[00:53:03] 
[00:53:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:03] Build completed unsuccessfully in 0:11:09
[00:53:03] Build completed unsuccessfully in 0:11:09
[00:53:03] make: *** [check] Error 1
[00:53:03] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22f8add0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
