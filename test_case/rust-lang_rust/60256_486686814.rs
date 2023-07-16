plain
travis_time:end:09cbb92c:start=1556199268327592591,finish=1556199270489011385,duration=2161418794
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:30:00]    Compiling chalk-macros v0.1.0
[00:30:04]    Compiling humantime v1.2.0
[00:30:04]    Compiling backtrace-sys v0.1.27
[00:30:04]    Compiling miniz-sys v0.1.11
[00:30:04] error[E0283]: type annotations required: cannot resolve `std::option::Option<_>: std::convert::Into<std::option::Option<&str>>`
[00:30:04]    |
[00:30:04]    |
[00:30:04] 12 |         .define("MINIZ_NO_STDIO", None)
[00:30:04] 
[00:30:04] error: aborting due to previous error
[00:30:04] 
[00:30:04] For more information about this error, try `rustc --explain E0283`.
---
travis_time:end:024dc3af:start=1556201087937006377,finish=1556201087942299209,duration=5292832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c329b53
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d75f2e1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native
