plain
travis_time:end:08daefbe:start=1553076140374605868,finish=1553076142963748679,duration=2589142811
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:07:02]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:13] error[E0515]: cannot return value referencing temporary value
[00:07:13]   --> src/libsyntax/attr/mod.rs:93:33
[00:07:13]    |
[00:07:13] 93 |         self.ident().map(|name| name.as_str().get())
[00:07:13]    |                                 |
[00:07:13]    |                                 returns a value referencing data owned by the current function
[00:07:13]    |                                 temporary value created here
[00:07:13] 
[00:07:13] 
[00:07:13] error[E0515]: cannot return value referencing temporary value
[00:07:13]    --> src/libsyntax/attr/mod.rs:171:33
[00:07:13]     |
[00:07:13] 171 |         self.ident().map(|name| name.as_str().get())
[00:07:13]     |                                 |
[00:07:13]     |                                 returns a value referencing data owned by the current function
[00:07:13]     |                                 temporary value created here
[00:07:13] 
[00:07:13] 
[00:07:13] error[E0515]: cannot return value referencing temporary value
[00:07:13]    --> src/libsyntax/attr/mod.rs:209:33
[00:07:13]     |
[00:07:13] 209 |         self.ident().map(|name| name.as_str().get())
[00:07:13]     |                                 |
[00:07:13]     |                                 returns a value referencing data owned by the current function
[00:07:13]     |                                 temporary value created here
[00:07:13] 
