plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b9/03/6eea2653cb4a1f64c06eb1f9e848822cb6c9972cb5a518624eed5f74d06b/awscli-1.15.31-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 2.0MB/s eta 0:00:01
    3% |█                               | 40kB 1.7MB/s eta 0:00:01
---
[00:45:43]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:45:43]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:46:45] warning: [1] cannot be resolved, ignoring it...
[00:46:45] 
[00:46:46] warning: [::Poll] cannot be resolved, ignoring it...
[00:46:46] warning: [x] cannot be resolved, ignoring it...
[00:46:46] 
[00:46:46] warning: [] cannot be resolved, ignoring it...
[00:46:46] 
---
[00:47:00]     Checking unwind v0.0.0 (file:///checkout/src/libunwind)
[00:47:00]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:47:01]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:47:01]     Checking panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:47:05] warning: [::Poll] cannot be resolved, ignoring it...
[00:47:06]     Checking panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:47:06]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:47:06]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:47:06]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:47:06]     Checking rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:47:06]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:47:06]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:47:18] warning: [::Poll] cannot be resolved, ignoring it...
[00:47:26]     Finished release [optimized] target(s) in 1m 43.54s
[00:47:27] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:47:27]     Checking term v0.0.0 (file:///checkout/src/libterm)
[00:47:27]     Checking getopts v0.2.17
---
[00:50:55] ..........................................................................i.........................
[00:51:01] ....................................................................................................
[00:51:08] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:20] ......i.................iiiiiiiii...................................................
[00:51:20] 
[00:51:20] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:52:19] ..........................................................................i.........................
[00:52:24] ....................................................................................................
[00:52:31] ....................................................................................................
[00:52:37] ....................................................................................................
[00:52:42] ......i.................iiiiiiiii...................................................
[00:52:42] 
[00:52:42]  finished in 82.426
[00:52:42] travis_fold:end:test_ui_nll

---
[01:36:09] travis_fold:end:stage0-linkchecker

[01:36:09] travis_time:end:stage0-linkchecker:start=1527898358606045852,finish=1527898361833847070,duration=3227801218

[01:36:12] std/future/trait.Future.html:41: broken link - std/future/::Poll
[01:36:23] core/future/trait.Future.html:41: broken link - core/future/::Poll
[01:36:29] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:36:29] 
[01:36:29] 
[01:36:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:36:29] expected success, got: exit code: 101
[01:36:29] expected success, got: exit code: 101
[01:36:29] 
[01:36:29] 
[01:36:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:29] Build completed unsuccessfully in 0:48:14
[01:36:29] Makefile:58: recipe for target 'check' failed
[01:36:29] make: *** [check] Error 1
3095488 ./obj
3093184 ./obj/build
2330388 ./obj/build/x86_64-unknown-linux-gnu
728404 ./src
---
149132 ./src/llvm-emscripten/test
146624 ./.git/modules
146620 ./.git/modules/src
123180 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123176 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1lc868q43-1uytwh8-2rxy29zuufd4c
121560 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknowj/build/x86_64-unknown-linux-gnu/native/jemalloc
65428 ./src/llvm-emscripten/test/CodeGen
63988 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
63504 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
62124 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
