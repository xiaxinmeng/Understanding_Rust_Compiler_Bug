plain
travis_time:end:08c79090:start=1556570367146233269,finish=1556570452733570815,duration=85587337546
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
DEPRECATION: Python 2.7 will reach the end of its life on January 1st, 2020. Please upgrade your Python as Python 2.7 won't be maintained after that date. A future version of pip will drop support for Python 2.7.
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/ec/78/01f03354bf02d4c3d1a0de01cbe77529b79df4ea078284f1172690834586/awscli-1.16.148-py2.py3-none-any.whl (1.5MB)
    1% |▍                               | 20kB 2.1MB/s eta 0:00:01
    2% |▋                               | 30kB 3.0MB/s eta 0:00:01
    2% |▉                               | 40kB 2.0MB/s eta 0:00:01
    3% |█                               | 51kB 2.5MB/s eta 0:00:01
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:36] 
[01:19:36] running 9 tests
[01:19:36] iiiiiiiii
[01:19:36] 
[01:19:36]  finished in 0.149
[01:19:36] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:52] 
[01:19:52] running 121 tests
[01:20:16] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:20] i.i......iii.i.....ii
[01:20:20] 
[01:20:20]  finished in 29.059
[01:20:20] travis_fold:end:test_debuginfo

---
[01:42:48] 
[01:42:48] failures:
[01:42:48] 
[01:42:48] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11390) stdout ----
[01:42:48] error: expected `[`, found keyword `fn`
[01:42:48]   |
[01:42:48] 8 | #fn main() {}
[01:42:48]   |  ^^ expected `[`
[01:42:48] 
[01:42:48] 
[01:42:48] error[E0578]: cannot find module `foo` in module `crate`
[01:42:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11392:19
[01:42:48]   |
[01:42:48] 4 |     pub(in crate::foo) struct Bar {
[01:42:48] 
[01:42:48] error: aborting due to 2 previous errors
[01:42:48] 
[01:42:48] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11390)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:42:48] 
[01:42:48] 
[01:42:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:48] Build completed unsuccessfully in 0:34:38
[01:42:48] make: *** [check] Error 1
[01:42:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0032b23d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 22:23:49 UTC 2019
