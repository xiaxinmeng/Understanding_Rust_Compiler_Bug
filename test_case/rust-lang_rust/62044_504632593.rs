plain
travis_time:end:0e0e58fa:start=1561181415628245079,finish=1561181416399812513,duration=771567434
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:24]     |
[00:05:24] 400 |             from_boxed_utf8_unchecked(buf)
[00:05:24]     |                                       ^^^ expected slice, found &[u8]
[00:05:24]     |
[00:05:24]     = note: expected type `boxed::Box<[u8]>`
[00:05:24]                found type `boxed::Box<&[u8]>`
[00:05:26] error: aborting due to previous error
[00:05:26] 
[00:05:26] For more information about this error, try `rustc --explain E0308`.
[00:05:26] error: Could not compile `alloc`.
