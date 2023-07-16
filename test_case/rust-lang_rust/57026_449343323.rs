plain
travis_time:end:059fd5d0:start=1545386424555699954,finish=1545386425832185126,duration=1276485172
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:11]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:11]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:15]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:26]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:27] error: incorrect close delimiter: `)`
[00:06:27]    --> src/librustc/ty/query/plumbing.rs:160:26
[00:06:27]     |
[00:06:27] 149 |                     return tls::with_related_context(tcx, |icx| {
[00:06:27]     |                                                                 - un-closed delimiter
[00:06:27] 160 |                         });
[00:06:27]     |                          ^ incorrect close delimiter
[00:06:27] 
[00:06:27] 
[00:06:27] error: incorrect close delimiter: `)`
[00:06:27]    --> src/librustc/ty/query/plumbing.rs:166:22
[00:06:27]     |
[00:06:27] 140 |             let job = match lock.active.raw_entry_mut().from_key_hashed_nocheck(key_hash, key) {
[00:06:27]     |                                                                                                - un-closed delimiter
[00:06:27] 166 |                     })
[00:06:27]     |                      ^ incorrect close delimiter
[00:06:27] 
[00:06:27] error: unexpected close delimiter: `}`
[00:06:27] error: unexpected close delimiter: `}`
[00:06:27]    --> src/librustc/ty/query/plumbing.rs:185:5
[00:06:27] 185 |     }
[00:06:27]     |     ^ unexpected close delimiter
[00:06:27] 
[00:06:27] error: aborting due to 3 previous errors
