plain
travis_time:end:003734ba:start=1540487457112272094,finish=1540487537073431280,duration=79961159186
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.12.31->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2f/e9/b02e8a1a8c53a55a4f37df1e8e111539d0a3e76828bcd252947a5200b797/python_dateutil-2.7.4-py2.py3-none-any.whl (211kB)
    4% |█▌                              | 10kB 44.6MB/s eta 0:00:01
    9% |███                             | 20kB 20.6MB/s eta 0:00:01
    14% |████▋                           | 30kB 26.5MB/s eta 0:00:01
    19% |██████▏                         | 40kB 15.0MB/s eta 0:00:01
---
[00:50:30] .................................................................................................... 2200/4951
[00:50:34] .................................................................................................... 2300/4951
[00:50:38] .................................................................................................... 2400/4951
[00:50:42] .................................................................................................... 2500/4951
[00:50:45] ......................................................iiiiiiiii..................................... 2600/4951
[00:50:52] ....ii.............................................................................................. 2800/4951
[00:50:55] .................................................................................................... 2900/4951
[00:50:59] ..............................................................................................i..... 3000/4951
[00:51:02] .................................................................................................... 3100/4951
---
[00:56:03] ..................................................................................................ii 2200/2870
[00:56:19] .....................................................................i....i......................... 2300/2870
[00:56:34] ............i....................................................................................... 2400/2870
[00:56:49] .................................................................................................... 2500/2870
[00:57:13] ................................................................F.F................................. 2600/2870
[00:57:31] .................................................................................................... 2800/2870
[00:57:41] ......................................................................
[00:57:41] failures:
[00:57:41] 
[00:57:41] 
[00:57:41] ---- [run-pass] run-pass/traits/trait-alias-object-type.rs stdout ----
[00:57:41] 
[00:57:41] error: test compilation failed although it shouldn't!
[00:57:41] status: exit code: 1
[00:57:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/traits/trait-alias-object-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-object-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-object-type/auxiliary"
[00:57:41] ------------------------------------------
[00:57:41] 
[00:57:41] ------------------------------------------
[00:57:41] stderr:
[00:57:41] stderr:
[00:57:41] ------------------------------------------
[00:57:41] {"message":"the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified","code":{"code":"E0191","explanation":"\nTrait objects need to have all associated types specified. Erroneous code\nexample:\n\n