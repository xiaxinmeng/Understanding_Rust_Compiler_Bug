plain
travis_time:end:1f0aa660:start=1544907861092915829,finish=1544907862150113883,duration=1057198054
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:30:44]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:30:45] error: use of deprecated item 'bitflags::core::str::<impl str>::trim_left_matches': superseded by `trim_start_matches`
[00:30:45]     --> src/librustc_resolve/lib.rs:3252:46
[00:30:45]      |
[00:30:45] 3252 |                                 enum_ty_path.trim_left_matches("std::prelude::v1::").to_owned()
[00:30:45]      |
[00:30:45]      = note: `-D deprecated` implied by `-D warnings`
[00:30:45] 
[00:30:46] error: aborting due to previous error
---
187088 ./obj/build/cache/2018-12-09
160528 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
144428 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144424 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7mgxceffg-1384krn-4zt1yysir9st
115340 ./src/llvm/test/CodeGen
114584 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
114580 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
112368 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:0f26066d:start=1544909730942015489,finish=1544909730946641482,duration=4625993
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0196cd20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:002e068c
travis_time:start:002e068c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-
