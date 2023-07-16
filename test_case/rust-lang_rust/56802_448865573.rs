plain
travis_time:end:0eeb761c:start=1545275892308755942,finish=1545275947775039463,duration=55466283521
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    100% |████████████████████████████████| 51kB 10.7MB/s 
Collecting botocore==1.12.69 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/72/ba/a188505f67a78a686aa24d8511a18cb5a8bb27705c9d1b1bb81bee97a138/botocore-1.12.69-py2.py3-none-any.whl (5.2MB)
    0% |                                | 10kB 35.6MB/s eta 0:00:01
    0% |▏                               | 20kB 36.8MB/s eta 0:00:01
    0% |▏                               | 30kB 42.4MB/s eta 0:00:01
    0% |▎                               | 40kB 46.4MB/s eta 0:00:01
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:33] 
[01:05:33] running 119 tests
[01:05:56] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[01:05:59] i......iii.i.....ii
[01:05:59] 
[01:05:59]  finished in 26.055
[01:05:59] travis_fold:end:test_debuginfo

---
[01:17:18] .................................................................................................... 900/916
[01:17:19] ................
[01:17:19] failures:
[01:17:19] 
[01:17:19] ---- iter::test_iterator_rev_nth_back stdout ----
[01:17:19] thread 'iter::test_iterator_rev_nth_back' panicked at 'assertion failed: `(left == right)`
[01:17:19]  right: `0`', src/libcore/../libcore/tests/iter.rs:1032:9
[01:17:19] 
[01:17:19] 
[01:17:19] failures:
[01:17:19] failures:
[01:17:19]     iter::test_iterator_rev_nth_back
[01:17:19] 
[01:17:19] test result: FAILED. 913 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:17:19] 
[01:17:19] error: test failed, to rerun pass '--test coretests'
[01:17:19] 
[01:17:19] 
[01:17:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:17:19] 
[01:17:19] 
[01:17:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:19] Build completed unsuccessfully in 0:22:17
[01:17:19] Build completed unsuccessfully in 0:22:17
[01:17:19] Makefile:58: recipe for target 'check' failed
[01:17:19] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25c5d42c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 20 04:36:35 UTC 2018
