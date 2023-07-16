plain
travis_time:end:08b9b73e:start=1556477280331010720,finish=1556477363078024064,duration=82747013344
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:24:48]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:24:54]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:24:54]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:24:54]    Compiling hashbrown v0.3.0
[00:24:55] error: FIXME: OutOfRangeHexEscape
[00:24:55]      |
[00:24:55]      |
[00:24:55] 1349 |         let s = CString::new(&b"abc\x01\x02\n\xE2\x80\xA6\xFF"[..]).unwrap();
[00:24:55] 
[00:24:55] error: aborting due to previous error
[00:24:55] 
[00:24:55] error: Could not compile `std`.
