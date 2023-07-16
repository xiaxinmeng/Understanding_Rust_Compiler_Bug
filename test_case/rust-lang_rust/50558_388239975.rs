plain
    100% |████████████████████████████████| 61kB 8.3MB/s 
Collecting botocore==1.10.19 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/34/05/1ffe77f2b8fb03a9223f3d0743cd38cded9491b074c65f3bb664ed4b7ac6/botocore-1.10.19-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 41.6MB/s eta 0:00:01
    0% |▏                               | 20kB 42.3MB/s eta 0:00:01
    0% |▎                               | 30kB 40.2MB/s eta 0:00:01
    0% |▎                               | 40kB 29.0MB/s eta 0:00:01
---
[00:05:36]    Compiling backtrace v0.3.6
[00:05:43]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:14]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:31]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:36] error: unused imports: `ReadGuard`, `RwLock`
[00:07:36]   --> librustc/dep_graph/graph.rs:16:40
[00:07:36]    |
[00:07:36] 16 | use rustc_data_structures::sync::{Lrc, RwLock, ReadGuard, Lock};
[00:07:36]    |                                        ^^^^^^  ^^^^^^^^^
[00:07:36]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:36] 
ll] Error 1
ll] Error 1
[00:08:02] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0532c098
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 11 02:19:47 UTC 2018
185428 ./obj/build/cache/2018-04-24
149724 ./.git/modules
149720 ./.git/modules/src
149120 ./src/llvm-emscripten/test
148912 ./obj/build/bootstrap/debug/incremental
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0x8xr8x5k-aeeear-22v3gsapbxtg7
89800 ./src/llvm/test/CodeGen
83616 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71468 ./.git/modules/src/tools
70944 ./obj/build/x86_64-unknown-linux-gnu/native
