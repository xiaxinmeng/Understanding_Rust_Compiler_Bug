plain
travis_time:end:1dd8995c:start=1543965120377856964,finish=1543965181862384257,duration=61484527293
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 1.4MB 928kB/s 
Collecting botocore==1.12.59 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/21/cf/71dfc14692883aaf709bc1098a56770173a760a14b0b1cb74471609181be/botocore-1.12.59-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 39.4MB/s eta 0:00:01
    0% |▏                               | 20kB 37.7MB/s eta 0:00:01
    0% |▏                               | 30kB 45.0MB/s eta 0:00:01
    0% |▎                               | 40kB 27.2MB/s eta 0:00:01
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:50] 
[00:54:50] running 120 tests
[00:54:53] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii...............i..ii 100/120
[00:54:54] ..ii.i.....iiii.....
[00:54:54] 
[00:54:54]  finished in 3.447
[00:54:54] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:08] 
[00:55:08] running 118 tests
[00:55:31] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:35] ......iii.i.....ii
[00:55:35] 
[00:55:35]  finished in 27.279
[00:55:35] travis_fold:end:test_debuginfo

---
[01:04:44] error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[01:04:44] 
[01:04:45] 
[01:04:45] running 412 tests
[01:05:01] .............................................F...................................................... 100/412
[01:05:25] .................................................................................................... 300/412
[01:05:36] .................................................................................................... 400/412
[01:05:38] ............
[01:05:38] failures:
[01:05:38] failures:
[01:05:38] 
[01:05:38] ---- collections/btree/map.rs - collections::btree::map::BTreeMap (line 69) stdout ----
[01:05:38] error[E0425]: cannot find value `book_reviews` in this scope
[01:05:38]   --> collections/btree/map.rs:101:33
[01:05:38]    |
[01:05:38] 35 | println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
[01:05:38]    |                                 ^^^^^^^^^^^^ did you mean `movie_reviews`?
[01:05:38] thread 'collections/btree/map.rs - collections::btree::map::BTreeMap (line 69)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:05:38] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:38] 
[01:05:38] 
---
[01:05:38] 
[01:05:38] error: test failed, to rerun pass '--doc'
[01:05:38] 
[01:05:38] 
[01:05:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:05:38] 
[01:05:38] 
[01:05:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:38] Build completed unsuccessfully in 0:21:24
[01:05:38] Build completed unsuccessfully in 0:21:24
[01:05:38] Makefile:58: recipe for target 'check' failed
[01:05:38] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01c95530
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec  5 00:18:49 UTC 2018
---
travis_time:end:011a8676:start=1543969130999157061,finish=1543969131066172277,duration=67015216
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b0d7f98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b06a98b
$ dmesg | grep -i kill
