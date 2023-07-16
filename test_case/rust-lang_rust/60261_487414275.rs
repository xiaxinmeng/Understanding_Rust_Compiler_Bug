plain
travis_time:end:2098fe88:start=1556482979246652902,finish=1556483063838188965,duration=84591536063
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:44]    Compiling rustc-demangle v0.1.10
[00:26:49]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:26:49]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:26:49]    Compiling hashbrown v0.3.0
[00:26:51] error: this form of character escape may only be used with characters in the range [\x00-\x7f]
[00:26:51]      |
[00:26:51]      |
[00:26:51] 1349 |         let s = CString::new(&b"abc\x01\x02\n\xE2\x80\xA6\xFF"[..]).unwrap();
[00:26:51] 
[00:26:51] error: aborting due to previous error
[00:26:51] 
[00:26:51] error: Could not compile `std`.
