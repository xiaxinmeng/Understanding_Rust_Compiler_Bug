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

[00:04:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:13] tidy error: /checkout/src/librustc_lint/types.rs:288: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_lint/unused.rs:105: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_lint/unused.rs:108: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_lint/unused.rs:114: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:266: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:290: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_mir/hair/cx/expr.rs:241: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_privacy/lib.rs:218: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_privacy/lib.rs:374: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/hir/intravisit.rs:522: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/hir/lowering.rs:3880: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/hir/lowering.rs:4060: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/hir/lowering.rs:4079: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/middle/resolve_lifetime.rs:678: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/middle/reachable.rs:286: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/middle/liveness.rs:489: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/check/demand.rs:309: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/check/method/suggest.rs:392: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/check/writeback.rs:120: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/check/op.rs:366: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/check/op.rs:367: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/collect.rs:423: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc_typeck/collect.rs:843: line longer than 100 chars
[00:04:14] some tidy checks failed
[00:04:14] 
[00:04:14] 
[00:04:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:14] 
[00:04:14] 
[00:04:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:46
[00:04:14] Build completed unsuccessfully in 0:00:46
[00:04:14] Makefile:79: recipe for target 'tidy' failed
[00:04:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0edf34c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:25548aab:start=1531323977043156557,finish=1531323977049748557,duration=6592000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18f46d36
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0218834a
$ dmesg | grep -i kill
