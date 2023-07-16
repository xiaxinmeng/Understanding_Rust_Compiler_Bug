plain
travis_time:end:3ca6529d:start=1555961152692420622,finish=1555961153491633310,duration=799212688
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:22]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:25] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:04:25]    --> src/libcore/iter/adapters/zip.rs:163:25
[00:04:25]     |
[00:04:25] 163 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:04:25]     |                         ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:04:25] error[E0433]: failed to resolve: use of undeclared type or module `Try`
[00:04:25]    --> src/libcore/iter/adapters/zip.rs:163:42
[00:04:25]     |
[00:04:25]     |
[00:04:25] 163 |                 None => LoopState::Break(Try::from_ok(acc)),
[00:04:25] 
[00:04:25] error[E0433]: failed to resolve: use of undeclared type or module `LoopState`
[00:04:25]    --> src/libcore/iter/adapters/zip.rs:164:33
[00:04:25]     |
[00:04:25]     |
[00:04:25] 164 |                 Some(b_item) => LoopState::from_try(fold(acc, (a_item, b_item))),
[00:04:25]     |                                 ^^^^^^^^^ use of undeclared type or module `LoopState`
[00:04:26] error[E0405]: cannot find trait `Try` in this scope
[00:04:26]   --> src/libcore/iter/adapters/zip.rs:58:18
[00:04:26]    |
[00:04:26]    |
[00:04:26] 58 |               R: Try<Ok = Acc>,
[00:04:26] help: possible candidate is found in another module, you can import it into scope
[00:04:26]    |
[00:04:26] 1  | use crate::ops::try::Try;
[00:04:26]    |
[00:04:26]    |
[00:04:26] 
[00:04:26] error[E0405]: cannot find trait `Try` in this scope
[00:04:26]   --> src/libcore/iter/adapters/zip.rs:88:18
[00:04:26]    |
[00:04:26] 88 |               R: Try<Ok = Acc>;
[00:04:26] help: possible candidate is found in another module, you can import it into scope
[00:04:26]    |
[00:04:26] 1  | use crate::ops::try::Try;
[00:04:26]    |
[00:04:26]    |
[00:04:26] 
[00:04:26] error[E0405]: cannot find trait `Try` in this scope
[00:04:26]    --> src/libcore/iter/adapters/zip.rs:158:18
[00:04:26]     |
[00:04:26] 158 |               R: Try<Ok = Acc>,
[00:04:26] help: possible candidate is found in another module, you can import it into scope
[00:04:26]     |
[00:04:26] 1   | use crate::ops::try::Try;
[00:04:26]     |
---
travis_time:end:08dc90d0:start=1555961436183486763,finish=1555961436188313581,duration=4826818
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:100b3b40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0353bf58
travis_time:start:0353bf58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0434aaea
$ dmesg | grep -i kill
