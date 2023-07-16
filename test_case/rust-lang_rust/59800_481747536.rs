plain
travis_time:end:025e9e24:start=1554908079943538329,finish=1554908082123290729,duration=2179752400
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:51:18]    Compiling tempfile v3.0.5
[00:51:19]    Compiling parking_lot v0.7.1
[00:51:20]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:53:33]    Compiling rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
[00:53:34] error: cannot satisfy dependencies so `getopts` only shows up once
[00:53:34]   |
[00:53:34]   = help: having upstream crates all available in one format will likely make this go away
[00:53:34] error: aborting due to previous error
[00:53:34] 
[00:53:34] error: Could not compile `rustdoc-tool`.
[00:53:34] 
---
travis_time:end:01d54b66:start=1554911308406978153,finish=1554911308412638453,duration=5660300
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0caccdcd
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:e
