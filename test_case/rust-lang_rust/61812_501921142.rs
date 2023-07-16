plain
travis_time:end:0082a3a2:start=1560464437639262387,finish=1560464438497883463,duration=858621076
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |███████████████████████████████▉| 5.5MB 49.3MB/s eta 0:00:01
    99% |████████████████████████████████| 5.5MB 48.0MB/s eta 0:00:01
    99% |████████████████████████████████| 5.5MB 48.1MB/s eta 0:00:01
    100% |████████████████████████████████| 5.5MB 4.2MB/s 
Requirement already satisfied: PyYAML<=5.1,>=3.10; python_version != "2.6" in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
    21% |███████                         | 10kB 23.6MB/s eta 0:00:01
    43% |██████████████                  | 20kB 30.2MB/s eta 0:00:01
    65% |█████████████████████           | 30kB 34.7MB/s eta 0:00:01
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:09] 
[01:07:09] running 9 tests
[01:07:09] iiiiiiiii
[01:07:09] 
[01:07:09]  finished in 0.148
[01:07:09] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:24] 
[01:07:24] running 122 tests
[01:07:49] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:53] .i.i......iii.i.....ii
[01:07:53] 
[01:07:53]  finished in 29.446
[01:07:53] travis_fold:end:test_debuginfo

---
[01:43:56] 
[01:43:56] failures:
[01:43:56] 
[01:43:56] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0399 (line 6170) stdout ----
[01:43:56] Test compiled successfully, but it's marked `compile_fail`.
[01:43:56] failures:
[01:43:56]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0399 (line 6170)
[01:43:56] 
[01:43:56] test result: FAILED. 679 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
---
travis_time:end:040cca9b:start=1560470687961666180,finish=1560470687966288178,duration=4621998
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ef4135
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f2d243
travis_time:start:00f2d243
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1105ee94
$ dmesg | grep -i kill
