plain
travis_time:end:051638c8:start=1545094002714405624,finish=1545094059512726135,duration=56798320511
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/63/dc/c8bfd1bd77113c033161ce31730510d1c479cf9bcc8e99edf3c906f30cce/awscli-1.16.77-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 30.1MB/s eta 0:00:01
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:47] 
[00:51:47] running 119 tests
[00:52:09] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:13] i......iii.i.....ii
[00:52:13] 
[00:52:13]  finished in 26.291
[00:52:13] travis_fold:end:test_debuginfo

---
[01:02:21] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:21]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:02:30] error[E0599]: no method named `mod_euclid` found for type `i8` in the current scope
[01:02:30]   --> src/libcore/../libcore/tests/num/int_macros.rs:35:28
[01:02:30]    |
[01:02:30] 35 |         assert!((-1 as $T).mod_euclid(MIN) == MAX);
[01:02:30]    | 
[01:02:30]    | 
[01:02:30]   ::: src/libcore/../libcore/tests/num/i8.rs:11:1
[01:02:30]    |
[01:02:30] 11 | int_module!(i8, i8);
[01:02:30]    | -------------------- in this macro invocation
[01:02:30]    |
[01:02:30]    = help: did you mean `div_euclid`?
[01:02:30] 
[01:02:30] error[E0599]: no method named `mod_euclid` found for type `i16` in the current scope
[01:02:30]   --> src/libcore/../libcore/tests/num/int_macros.rs:35:28
[01:02:30]    |
[01:02:30] 35 |         assert!((-1 as $T).mod_euclid(MIN) == MAX);
[01:02:30]    | 
[01:02:30]    | 
[01:02:30]   ::: src/libcore/../libcore/tests/num/i16.rs:11:1
[01:02:30]    |
[01:02:30] 11 | int_module!(i16, i16);
[01:02:30]    | ---------------------- in this macro invocation
[01:02:30]    |
[01:02:30]    = help: did you mean `div_euclid`?
[01:02:30] 
[01:02:30] error[E0599]: no method named `mod_euclid` found for type `i32` in the current scope
[01:02:30]   --> src/libcore/../libcore/tests/num/int_macros.rs:35:28
[01:02:30]    |
[01:02:30] 35 |         assert!((-1 as $T).mod_euclid(MIN) == MAX);
[01:02:30]    | 
[01:02:30]    | 
[01:02:30]   ::: src/libcore/../libcore/tests/num/i32.rs:11:1
[01:02:30]    |
[01:02:30] 11 | int_module!(i32, i32);
[01:02:30]    | ---------------------- in this macro invocation
[01:02:30]    |
[01:02:30]    = help: did you mean `div_euclid`?
[01:02:30] 
[01:02:30] error[E0599]: no method named `mod_euclid` found for type `i64` in the current scope
[01:02:30]   --> src/libcore/../libcore/tests/num/int_macros.rs:35:28
[01:02:30]    |
[01:02:30] 35 |         assert!((-1 as $T).mod_euclid(MIN) == MAX);
[01:02:30]    | 
[01:02:30]    | 
[01:02:30]   ::: src/libcore/../libcore/tests/num/i64.rs:11:1
[01:02:30]    |
[01:02:30] 11 | int_module!(i64, i64);
[01:02:30]    | ---------------------- in this macro invocation
[01:02:30]    |
[01:02:30]    = help: did you mean `div_euclid`?
---
travis_time:end:0c12f4e0:start=1545097825543383064,finish=1545097825627330932,duration=83947868
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:065937de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:012b27b6
$ dmesg | grep -i kill
