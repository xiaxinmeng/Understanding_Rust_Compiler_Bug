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
[01:10:19] travis_fold:end:stage0-linkchecker

[01:10:19] travis_time:end:stage0-linkchecker:start=1531308106477156288,finish=1531308108797621155,duration=2320464867

[01:10:19] std/macro.assert_ne.html:11: broken link - std/macro.assert.html
[01:10:21] std/macro.assert_eq.html:11: broken link - std/macro.assert.html
[01:10:21] std/io/trait.Write.html:121: broken link - std/macro.format_args.html
[01:10:23] std/macro.panic.html:26: broken link - std/macro.compile_error.html
[01:10:25] std/macro.debug_assert.html:8: broken link - std/macro.assert.html
[01:10:25] std/macro.debug_assert.html:11: broken link - std/macro.assert.html
[01:10:25] std/macro.debug_assert.html:20: broken link - std/macro.assert.html
[01:10:26] std/primitive.bool.html:7: broken link - std/macro.assert.html
[01:10:26] std/fmt/fn.format.html:3: broken link - std/macro.format_args.html
[01:10:26] std/fmt/index.html:234: broken link - std/macro.format_args.html
[01:10:26] std/fmt/struct.Arguments.html:5: broken link - std/macro.format_args.html
[01:10:26] std/fmt/struct.Arguments.html:8: broken link - std/macro.format_args.html
[01:10:26] std/fmt/struct.Formatter.html:310: broken link - std/macro.format_args.html
[01:10:28] core/macro.assert_ne.html:10: broken link - core/macro.assert.html
[01:10:29] core/macro.assert_eq.html:10: broken link - core/macro.assert.html
[01:10:29] core/macro.debug_assert.html:8: broken link - core/macro.assert.html
[01:10:29] core/macro.debug_assert.html:11: broken link - core/macro.assert.html
[01:10:29] core/macro.debug_assert.html:20: broken link - core/macro.assert.html
[01:10:30] core/fmt/struct.Arguments.html:5: broken link - std/macro.format_args.html
[01:10:30] core/fmt/struct.Arguments.html:8: broken link - std/macro.format_args.html
[01:10:30] core/fmt/struct.Formatter.html:310: broken link - std/macro.format_args.html
[01:10:34] alloc/fmt/fn.format.html:3: broken link - std/macro.format_args.html
[01:10:34] alloc/fmt/index.html:234: broken link - std/macro.format_args.html
[01:10:34] alloc/fmt/struct.Arguments.html:5: broken link - std/macro.format_args.html
[01:10:34] alloc/fmt/struct.Arguments.html:8: broken link - std/macro.format_args.html
[01:10:34] alloc/fmt/struct.Formatter.html:310: broken link - std/macro.format_args.html
[01:10:34] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:10:34] 
[01:10:34] 
[01:10:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:10:34] expected success, got: exit code: 101
[01:10:34] expected success, got: exit code: 101
[01:10:34] 
[01:10:34] 
[01:10:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:34] Build completed unsuccessfully in 0:28:45
[01:10:34] Makefile:58: recipe for target 'check' failed
[01:10:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04291a7c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
