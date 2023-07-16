plain
travis_time:end:01b338aa:start=1555814615557146478,finish=1555814616473687350,duration=916540872
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:49:38]    Compiling parking_lot_core v0.4.0
[00:49:43]    Compiling tempfile v3.0.5
[00:49:44]    Compiling parking_lot v0.7.1
[00:49:45]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:49:46] error: couldn't read src/librustdoc/html/static/SourceSerifPro-LICENSE.txt: No such file or directory (os error 2)
[00:49:46]   --> src/librustdoc/html/static_files.rs:94:41
[00:49:46]    |
[00:49:46] 94 |     pub static LICENSE: &'static [u8] = include_bytes!("static/SourceSerifPro-LICENSE.txt");
[00:49:46] 
[00:49:46] error: couldn't read src/librustdoc/html/static/SourceCodePro-LICENSE.txt: No such file or directory (os error 2)
[00:49:46]    --> src/librustdoc/html/static_files.rs:106:41
[00:49:46]     |
[00:49:46]     |
[00:49:46] 106 |     pub static LICENSE: &'static [u8] = include_bytes!("static/SourceCodePro-LICENSE.txt");
[00:49:46] 
[00:49:51] error: aborting due to 2 previous errors
[00:49:51] 
[00:49:51] error: Could not compile `rustdoc`.
---
travis_time:end:0392e740:start=1555817619372226400,finish=1555817619376985690,duration=4759290
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21781a11
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:115b2bc8
$ cat ./obj/build/x86_64-unkno
