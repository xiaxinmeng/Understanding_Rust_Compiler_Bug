plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4a/7d/9c0f35dc594b78137929523f9632ec649b69af2d30ca6a7a8c60d414b23a/awscli-1.15.9-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 8.5MB/s 
Collecting botocore==1.10.9 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/fe/13/45a8eeb5d78e2578b8e55df58e3921369efe51eaa57a9d2a36e7bef45bcc/botocore-1.10.9-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.2MB/s eta 0:00:01
    0% |▏                               | 20kB 40.1MB/s eta 0:00:01
    0% |▎                               | 30kB 45.2MB/s eta 0:00:01
    0% |▎                               | 40kB 27.2MB/s eta 0:00:01
---
[00:54:20] ......i........................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:23] .....
[00:54:55] ....................................................................................................
[00:55:24] .......................................................................ii...........................
[00:56:17] ..................................i....................................................i.ii........test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:59] ...............................................................................................iiiii
[00:57:25] ii..................................................................................................
[00:57:55] ....................................................................................................
[00:57:55] ....................................................................................................
[00:58:20] ..............................................................................................F.....
[00:58:38] failures:
[00:58:38] 
[00:58:38] ---- [run-pass] run-pass/vec-const-new.rs stdout ----
[00:58:38]  
[00:58:38]  
[00:58:38] error: compilation failed!
[00:58:38] status: exit code: 101
[00:58:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/vec-const-new.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vec-const-new.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/vec-const-new.stage2-x86_64-unknown-linux-gnu.aux"
[00:58:38] ------------------------------------------
[00:58:38] 
[00:58:38] ------------------------------------------
[00:58:38] stderr:
[00:58:38] stderr:
[00:58:38] ------------------------------------------
[00:58:38] error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
[00:58:38]    |
[00:58:38]    |
[00:58:38] 13 | const MY_VEC: Vec<usize> = Vec::new();
[00:58:38] 
[00:58:38] error: aborting due to previous error
[00:58:38] 
[00:58:38] For more information about this error, try `rustc --explain E0015`.
---
[00:58:38] 
[00:58:38] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:58:38] 
[00:58:38] 
[00:58:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnuWed, 25 Apr 2018 22:43:06 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
