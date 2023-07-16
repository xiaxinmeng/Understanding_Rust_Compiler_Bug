plain
travis_time:end:09dd4498:start=1557231473839328766,finish=1557231557460512247,duration=83621183481
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:36:00]    --> src/librustc_mir/lib.rs:31:9
[00:36:00]     |
[00:36:00] 31  | #![deny(internal)]
[00:36:00]     |         ^^^^^^^^
[00:36:00]     = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[00:36:00] error: aborting due to previous error
[00:36:00] 
[00:36:01] error: Could not compile `rustc_mir`.
[00:36:01] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:03b7cd9f:start=1557233905565677800,finish=1557233905569564093,duration=3886293
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:35f2e542
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
