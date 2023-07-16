plain
travis_time:end:242e0f78:start=1542807857894820803,finish=1542807859123465963,duration=1228645160
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:34]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:35] error[E0433]: failed to resolve. Use of undeclared type or module `channel`
[00:02:35]    --> bootstrap/tool.rs:243:34
[00:02:35]     |
[00:02:35] 243 |     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);
[00:02:35]     |                                  ^^^^^^^ Use of undeclared type or module `channel`
[00:02:38] error: aborting due to previous error
[00:02:38] 
[00:02:38] For more information about this error, try `rustc --explain E0433`.
[00:02:38] error: Could not compile `bootstrap`.
---
[00:02:39]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:40] error[E0433]: failed to resolve. Use of undeclared type or module `channel`
[00:02:40]    --> bootstrap/tool.rs:243:34
[00:02:40]     |
[00:02:40] 243 |     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);
[00:02:40]     |                                  ^^^^^^^ Use of undeclared type or module `channel`
[00:02:43] error: aborting due to previous error
[00:02:43] 
[00:02:43] For more information about this error, try `rustc --explain E0433`.
[00:02:43] error: Could not compile `bootstrap`.
---
[00:02:46]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:46] error[E0433]: failed to resolve. Use of undeclared type or module `channel`
[00:02:46]    --> bootstrap/tool.rs:243:34
[00:02:46]     |
[00:02:46] 243 |     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);
[00:02:46]     |                                  ^^^^^^^ Use of undeclared type or module `channel`
[00:02:49] error: aborting due to previous error
[00:02:49] 
[00:02:49] For more information about this error, try `rustc --explain E0433`.
[00:02:49] error: Could not compile `bootstrap`.
---
[00:02:53]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:53] error[E0433]: failed to resolve. Use of undeclared type or module `channel`
[00:02:53]    --> bootstrap/tool.rs:243:34
[00:02:53]     |
[00:02:53] 243 |     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);
[00:02:53]     |                                  ^^^^^^^ Use of undeclared type or module `channel`
[00:02:56] error: aborting due to previous error
[00:02:56] 
[00:02:56] For more information about this error, try `rustc --explain E0433`.
[00:02:56] error: Could not compile `bootstrap`.
---
[00:03:01]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:01] error[E0433]: failed to resolve. Use of undeclared type or module `channel`
[00:03:01]    --> bootstrap/tool.rs:243:34
[00:03:01]     |
[00:03:01] 243 |     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);
[00:03:01]     |                                  ^^^^^^^ Use of undeclared type or module `channel`
[00:03:05] error: aborting due to previous error
[00:03:05] 
[00:03:05] For more information about this error, try `rustc --explain E0433`.
[00:03:05] error: Could not compile `bootstrap`.
