plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f3/43/c345035af7ca97094721b98573ea23f74d899b86bd02c70928cd5ded1666/awscli-1.16.23-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 12.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:58:03] ....................................................................................................
[00:58:06] ....................................................................................................
[00:58:09] ....................................................................................................
[00:58:12] ....................................................................................................
[00:58:16] ...........................F....................................................................i...
[00:58:22] ........................................................i.i..ii.....................................
[00:58:22] ........................................................i.i..ii.....................................
[00:58:25] ....................F...............................................................................
[00:58:29] .....................................................F....................................i.........
[00:58:35] ....................................................................................................
[00:58:38] ....................................................................................................
[00:58:41] ....................................................................................................
[00:58:44] ...............................i....................................................................
---
[01:03:28] ....................................................................................................
[01:03:32] ...............................i....................................................................
[01:03:36] ....................................................................................................
[01:03:39] ....................................................................................................
ut b = 3; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:30:9
[01:03:43] -    |
[01:03:43] - LL |     let mut a = vec![3]; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:32:10
[01:03:43] -    |
[01:03:43] - LL |     let (mut a, b) = (1, 2); //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |          |
[01:03:43] -    |          help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:34:9
[01:03:43] -    |
[01:03:43] - LL |     let mut a; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:38:9
[01:03:43] -    |
[01:03:43] - LL |     let mut b; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43 need to be mutable
[01:03:43 need to be mutable
[01:03:43] -    |         ----^
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:84:9
[01:03:43] -    |
[01:03:43] - LL |     let mut v : &mut Vec<()> = &mut vec![]; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] 120   --> $DIR/lint-unused-mut-variables.rs:61:13
[01:03:43] 121    |
[01:03:43] 122 LL |     fn what(mut foo: isize) {} //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] 124    |             |
[01:03:43] 125    |             help: remove this `mut`
[01:03:43] 126 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:79:20
[01:03:43] -    |
[01:03:43] - LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
[01:03:43] -    |                    |
[01:03:43] -    |                    help: remove this `mut`
[01:03:43] - 
[01:03:43] - error: variable does not need to be mutable
[01:03:43] - error: variable does not need to be mutable
[01:03:43] -   --> $DIR/lint-unused-mut-variables.rs:143:9
[01:03:43] -    |
[01:03:43] - LL |     let mut b = vec![2]; //[lexical]~ ERROR: variable does not need to be mutable
[01:03:43] -    |         |
[01:03:43] -    |         help: remove this `mut`
[01:03:43] -    |
[01:03:43] -    |
[01:src/test/ui/lint/lint-unused-mut-variables.rs:61:13\n   |\nLL |     fn what(mut foo: isize) {} //[lexical]~ ERROR: variable does not need to be mutable\n   |             ----^^^\n   |             |\n   |             help: remove this `mut`\n\n"}
[01:03:43] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:03:43] ------------------------------------------
[01:03:43] 
[01:03:43] thread '[ui] ui/lint/lint-unused-mut-variables.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[01:03:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:43] 
[01:03:43] ---- [ui] ui/nll/capture-mut-ref.rs stdout ----
[01:03:43] 
[01:03:43] error: ui test compiled successfully!
[01:03:43] status: exit code: 0
[01:03:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/capture-mut-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/auxiliary" "-A" "unused"
[01:03:43] ------------------------------------------
[01:03:43] 
[01:03:43] ------------------------------------------
[01:03:43] stderr:
[01:03:43] stderr:
[01:03:43] ------------------------------------------
[01:03:43] 
[01:03:43] -----------
[01:03:43] 
[01:03:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:03:43] 
[01:03:43] 
[01:03:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:43] 
[01:03:43] 
[01:03:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:43] Build completed unsuccessfully in 0:07:52
[01:03:43] Build completed unsuccessfully in 0:07:52
[01:03:43] make: *** [check] Error 1
[01:03:43] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b6a1ca2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
