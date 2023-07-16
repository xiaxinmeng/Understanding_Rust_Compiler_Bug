plain
travis_time:end:156083f7:start=1561068109992105956,finish=1561068211102024522,duration=101109918566
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    100% |████████████████████████████████| 51kB 21.0MB/s 
Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting botocore==1.12.173 (from awscli)
  Downloading https://files.pythonhosted.org/packages/af/bb/226c21c8ff37c620412280e71dd7b0135c50d380ac212e3cd0c34d4bc6ef/botocore-1.12.173-py2.py3-none-any.whl (5.6MB)
    0% |▏                               | 20kB 27.3MB/s eta 0:00:01
    0% |▏                               | 30kB 33.8MB/s eta 0:00:01
    0% |▎                               | 40kB 34.9MB/s eta 0:00:01
    0% |▎                               | 51kB 36.2MB/s eta 0:00:01
---
[00:01:28] 
###################################                                       49.9%
######################################################################## 100.0%
[00:01:28] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:28]     Updating git repository `https://github.com/ia0/rust-itertools`
[00:01:29]     Updating git repository `https://github.com/ia0/memoffset`
[00:01:30]     Updating git repository `https://github.com/ia0/proc-macro2`
[00:01:31]     Updating git repository `https://github.com/ia0/quick-error`
[00:01:31]     Updating git repository `https://github.com/ia0/syn`
[00:01:54]  Downloading crates ...
[00:01:54]   Downloaded cc v1.0.35
[00:01:54]   Downloaded filetime v0.2.4
[00:01:54]   Downloaded num_cpus v1.8.0
---
[00:01:54]   Downloaded itoa v0.4.4
[00:01:54]   Downloaded ryu v0.2.7
[00:01:54]   Downloaded quote v0.6.12
[00:01:54]   Downloaded unicode-xid v0.1.0
[00:01:54]    Compiling proc-macro2 v0.4.30 (https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a1654272)
[00:01:54]    Compiling unicode-xid v0.1.0
[00:01:54]    Compiling syn v0.15.35 (https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d)
[00:01:55]    Compiling libc v0.2.54
[00:01:55]    Compiling serde v1.0.92
[00:01:55]    Compiling fixedbitset v0.1.9
[00:01:55]    Compiling unicode-width v0.1.5
---
[00:03:08] travis_fold:end:log-system-info
travis_fold:start:stage0-tidy
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:03:08]    Compiling proc-macro2 v0.4.30 (https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a1654272)
[00:03:08]    Compiling unicode-xid v0.1.0
[00:03:08]    Compiling syn v0.15.35 (https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d)
[00:03:08]    Compiling serde v1.0.92
[00:03:09]    Compiling ryu v0.2.7
[00:03:09]    Compiling lazy_static v1.3.0
[00:03:09]    Compiling regex v1.1.6
---

[00:04:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:15] tidy error: /checkout/src/libsyntax/ext/tt/macro_check.rs:79: TODO is deprecated; use FIXME
[00:04:15] tidy error: /checkout/src/libsyntax/ext/tt/macro_check.rs:295: TODO is deprecated; use FIXME
[00:04:15] tidy error: /checkout/src/libsyntax/ext/tt/macro_check.rs:323: TODO is deprecated; use FIXME
[00:04:15] tidy error: /checkout/src/libsyntax/ext/tt/macro_check.rs:332: TODO is deprecated; use FIXME
[00:04:20] invalid source: "git+https://github.com/ia0/rust-itertools?branch=rust_issues_61053#7551238d3957a4061b5b3f8dce3e724866d8789d"
[00:04:20] invalid source: "git+https://github.com/ia0/memoffset?branch=rust_issues_61053#de2e76aed04d0964088a40be0f3db593c168ce22"
[00:04:20] invalid source: "git+https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a165427201fa163f9a1d453fa2724f1ac516fd1d"
[00:04:20] invalid source: "git+https://github.com/ia0/quick-error?branch=rust_issues_61053#9dba52d5ceaa66d5ae342aaf220ebd45f939d8ba"
[00:04:20] invalid source: "git+https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d3374062e3d015f0bf26a6c41e448f1bb"
[00:04:20] some tidy checks failed
[00:04:20] 
[00:04:20] 
[00:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:20] 
[00:04:20] 
[00:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:20] Build completed unsuccessfully in 0:01:13
---
travis_time:end:01640358:start=1561068481433649578,finish=1561068481439194435,duration=5544857
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26b3a217
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fae6cdc
travis_time:start:0fae6cdc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04e564e4
$ dmesg | grep -i kill
