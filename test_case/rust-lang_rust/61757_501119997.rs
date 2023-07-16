plain
travis_time:end:125c8c6b:start=1560312421231393291,finish=1560312423032216239,duration=1800822948
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:40:57]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:40:58] error: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[00:40:58]    --> src/librustc_metadata/dynamic_lib.rs:164:38
[00:40:58]     |
[00:40:58] 164 |         use std::sync::{Mutex, Once, ONCE_INIT};
[00:40:58]     |
[00:40:58]     = note: `-D deprecated` implied by `-D warnings`
[00:40:58] 
[00:40:58] error: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[00:40:58] error: use of deprecated item 'std::sync::ONCE_INIT': the `new` function is now preferred
[00:40:58]    --> src/librustc_metadata/dynamic_lib.rs:165:29
[00:40:58]     |
[00:40:58] 165 |         static INIT: Once = ONCE_INIT;
[00:40:58]     |                             ^^^^^^^^^ help: replace the use of the deprecated item: `Once::new()`
[00:41:00] error: aborting due to 2 previous errors
[00:41:00] 
[00:41:01] error: Could not compile `rustc_metadata`.
[00:41:01] warning: build failed, waiting for other jobs to finish...
---
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1cc294ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jun 12 04:54:04 UTC 2019
trap-oxgzobynhmm1/s-fd31q2bxq4-1tzpb2u-19ca4iajxr4qm
108532 ./src/llvm-project/lldb
102836 ./.git
98204 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98060 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
