plain
travis_time:end:14d0619d:start=1561064659786871729,finish=1561064749216727195,duration=89429855466
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    100% |████████████████████████████████| 51kB 21.1MB/s 
Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting botocore==1.12.173 (from awscli)
  Downloading https://files.pythonhosted.org/packages/af/bb/226c21c8ff37c620412280e71dd7b0135c50d380ac212e3cd0c34d4bc6ef/botocore-1.12.173-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 22.9MB/s eta 0:00:01
    0% |▏                               | 30kB 26.4MB/s eta 0:00:01
    0% |▎                               | 40kB 26.2MB/s eta 0:00:01
    0% |▎                               | 51kB 27.6MB/s eta 0:00:01
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:49] 
[01:03:49] running 9 tests
[01:03:49] iiiiiiiii
[01:03:49] 
[01:03:49]  finished in 0.157
[01:03:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:05] 
[01:04:05] running 122 tests
[01:04:30] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:34] .i.i......iii.i.....ii
[01:04:34] 
[01:04:34]  finished in 29.721
[01:04:34] travis_fold:end:test_debuginfo

---
[01:05:42]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:05:47] error[E0308]: mismatched types
[01:05:47]    --> src/librustdoc/theme.rs:106:28
[01:05:47]     |
[01:05:47] 106 |     v.get(pos + 1) == Some(b'/')
[01:05:47]     |                            |
[01:05:47]     |                            expected &u8, found u8
[01:05:47]     |                            help: consider borrowing here: `&b'/'`
[01:05:47]     |
---
travis_time:end:1c30a15d:start=1561068708004126374,finish=1561068708008816556,duration=4690182
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f0a7a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a29a0da
travis_time:start:0a29a0da
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:38540820
$ dmesg | grep -i kill
