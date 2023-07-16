plain
travis_time:end:3ced7299:start=1558062380246887013,finish=1558062467926219920,duration=87679332907
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:15:17]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:23] error[E0057]: this function takes 2 parameters but 1 parameter was supplied
[00:15:23]    --> src/librustc_typeck/check/op.rs:534:40
[00:15:23]     |
[00:15:23] 534 |                 if (l_ty.sty == Str || is_std_string(l_ty)) && (
[00:15:23] 
[00:15:23] error[E0057]: this function takes 2 parameters but 1 parameter was supplied
[00:15:23]    --> src/librustc_typeck/check/op.rs:535:44
[00:15:23]     |
[00:15:23]     |
[00:15:23] 535 |                         r_ty.sty == Str || is_std_string(r_ty) ||
[00:15:23] 
[00:15:24] error[E0057]: this function takes 2 parameters but 1 parameter was supplied
[00:15:24]    --> src/librustc_typeck/check/op.rs:569:40
[00:15:24]     |
[00:15:24]     |
[00:15:24] 569 |                 if (l_ty.sty == Str || is_std_string(l_ty)) && is_std_string(rhs_ty) =>
[00:15:24] 
[00:15:24] error[E0057]: this function takes 2 parameters but 1 parameter was supplied
[00:15:24]    --> src/librustc_typeck/check/op.rs:569:64
[00:15:24]     |
[00:15:24]     |
[00:15:24] 569 |                 if (l_ty.sty == Str || is_std_string(l_ty)) && is_std_string(rhs_ty) =>
[00:15:24] 
[00:15:24] error: aborting due to 4 previous errors
[00:15:24] 
[00:15:24] For more information about this error, try `rustc --explain E0057`.
