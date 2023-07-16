plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ce/50/6789babdceaae0e7d4c43b66a76052c5f8b8ef2416075f5604c8961adb94/awscli-1.15.39-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.7MB/s 
Collecting botocore==1.10.39 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ac/35/bf811140cc44b45ebe1601b83151e86b57bfed31b06c451a0f87383f9eed/botocore-1.10.39-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 41.1MB/s eta 0:00:01
    0% |▏                               | 20kB 35.5MB/s eta 0:00:01
    0% |▎                               | 30kB 39.0MB/s eta 0:00:01
    0% |▎                               | 40kB 15.3MB/s eta 0:00:01
---
[01:16:18]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:16:18] error[E0432]: unresolved import `super::super::vec`
[01:16:18]     --> liballoc/collections/vec_deque.rs:2894:27
[01:16:18]      |
[01:16:18] 2894 |         use super::super::vec::Vec;
[01:16:18]      |                           ^^^ Could not find `vec` in `super`
[01:16:18] error[E0432]: unresolved import `super::super::vec`
[01:16:18]     --> liballoc/collections/vec_deque.rs:2910:27
[01:16:18]      |
[01:16:18] 2910 |         use super::super::vec::Vec;
[01:16:18] 2910 |         use super::super::vec::Vec;
[01:16:18]      |                           ^^^ Could not find `vec` in `super`
[01:16:22] error: aborting due to 2 previous errors
[01:16:22] 
[01:16:22] For more information about this error, try `rustc --explain E0432`.
[01:16:22] error: Could not compile `alloc`.
[01:16:22] error: Could not compile `alloc`.
[01:16:22] warning: build failed, waiting for other jobs to finish...
[01:16:54] error: build failed
[01:16:54] 
[01:16:54] 
[01:16:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:16:54] 
[01:16:54] 
[01:16:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:54] Build completed unsuccessfully in 0:28:13
[01:16:54] Build completed unsuccessfully in 0:28:13
[01:16:54] make: *** [check] Error 1
[01:16:54] Makefile:58: recipe for target 'check' failed
