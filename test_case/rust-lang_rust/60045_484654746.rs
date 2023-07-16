plain
travis_time:end:1410dfa6:start=1555614544590686327,finish=1555614545873766370,duration=1283080043
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:18:20]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:18:20]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:18:20]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:19:04]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:19:05] error: expected one of `)` or `,`, found `|`
[00:19:05]      |
[00:19:05] 3282 |                         PathResult::Module(_) |
[00:19:05] 3282 |                         PathResult::Module(_) |
[00:19:05]      |                                               ^ expected one of `)` or `,` here
[00:19:06] error[E0061]: this function takes 8 parameters but 9 parameters were supplied
[00:19:06]     --> src/librustc_resolve/lib.rs:3233:37
[00:19:06]      |
[00:19:06]      |
[00:19:06] 3233 |           let resolution = match self.resolve_qpath_anywhere(
[00:19:06] ...
[00:19:06] ...
[00:19:06] 3415 | /     fn resolve_qpath_anywhere(
[00:19:06] 3417 | |         id: NodeId,
[00:19:06] 3417 | |         id: NodeId,
[00:19:06] 3418 | |         qself: Option<&QSelf>,
[00:19:06] 3459 | |         fin_res
[00:19:06] 3460 | |     }
[00:19:06]      | |_____- defined here
[00:19:06] 
---
travis_time:end:15ef765d:start=1555615708953193561,finish=1555615708960283275,duration=7089714
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:006dddfa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f1f01c0
travis_time:start:1f1f01c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/a
