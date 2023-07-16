plain
travis_time:end:0051a0e0:start=1561215901337485858,finish=1561215904250664147,duration=2913178289
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
##########################################                                59.2%
################################################################          89.7%
######################################################################## 100.0%
[00:01:51] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:51]     Updating git repository `https://github.com/ia0/rust-itertools`
[00:01:52]     Updating git repository `https://github.com/ia0/memoffset`
[00:01:52]     Updating git repository `https://github.com/ia0/proc-macro2`
[00:01:53]     Updating git repository `https://github.com/ia0/quick-error`
[00:01:53]     Updating git repository `https://github.com/ia0/syn`
[00:02:14]  Downloading crates ...
[00:02:15]   Downloaded filetime v0.2.4
[00:02:15]   Downloaded cc v1.0.35
[00:02:15]   Downloaded num_cpus v1.8.0
---
[00:02:15]   Downloaded itoa v0.4.4
[00:02:15]   Downloaded ryu v0.2.7
[00:02:15]   Downloaded quote v0.6.12
[00:02:15]   Downloaded unicode-xid v0.1.0
[00:02:15]    Compiling proc-macro2 v0.4.30 (https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a1654272)
[00:02:15]    Compiling unicode-xid v0.1.0
[00:02:15]    Compiling syn v0.15.35 (https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d)
[00:02:15]    Compiling libc v0.2.54
[00:02:16]    Compiling serde v1.0.92
[00:02:16]    Compiling unicode-width v0.1.5
[00:02:16]    Compiling cc v1.0.35
---
[00:03:24] travis_fold:end:log-system-info
travis_fold:start:stage0-tidy
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:03:25]    Compiling proc-macro2 v0.4.30 (https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a1654272)
[00:03:25]    Compiling unicode-xid v0.1.0
[00:03:25]    Compiling syn v0.15.35 (https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d)
[00:03:25]    Compiling serde v1.0.92
[00:03:26]    Compiling ryu v0.2.7
[00:03:26]    Compiling ucd-util v0.1.3
[00:03:26]    Compiling regex v1.1.6
---

[00:04:30] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:30] tidy error: /checkout/src/libsyntax/ext/tt/macro_check.rs:90: TODO is deprecated; use FIXME
[00:04:35] invalid source: "git+https://github.com/ia0/rust-itertools?branch=rust_issues_61053#7551238d3957a4061b5b3f8dce3e724866d8789d"
[00:04:35] invalid source: "git+https://github.com/ia0/memoffset?branch=rust_issues_61053#de2e76aed04d0964088a40be0f3db593c168ce22"
[00:04:35] invalid source: "git+https://github.com/ia0/proc-macro2?branch=rust_issues_61053#a165427201fa163f9a1d453fa2724f1ac516fd1d"
[00:04:35] invalid source: "git+https://github.com/ia0/quick-error?branch=rust_issues_61053#9dba52d5ceaa66d5ae342aaf220ebd45f939d8ba"
[00:04:35] invalid source: "git+https://github.com/ia0/syn?branch=rust_issues_61053#19397d4d3374062e3d015f0bf26a6c41e448f1bb"
[00:04:35] some tidy checks failed
[00:04:35] 
[00:04:35] 
[00:04:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:35] 
[00:04:35] 
[00:04:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:35] Build completed unsuccessfully in 0:01:12
---
travis_time:end:061a3e20:start=1561216191139887896,finish=1561216191144258868,duration=4370972
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1630a775
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:301561ec
travis_time:start:301561ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03060b74
$ dmesg | grep -i kill
