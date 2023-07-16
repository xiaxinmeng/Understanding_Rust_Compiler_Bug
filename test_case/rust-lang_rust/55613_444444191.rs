plain
travis_time:end:22d2cbf6:start=1544004371769638900,finish=1544004374102276374,duration=2332637474
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:06] .................................................................................................... 4800/5110
[00:48:09] .................................................................................................... 4900/5110
[00:48:12] .................................................................................................... 5000/5110
[00:48:14] .................................................i.................................................. 5100/5110
, perhaps add a `use` for it:\n`use crate::foo::foobar::Foobar;`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `foobar` found for type `u32` in the current scope\n  --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:35:11\n   |\nLL |         x.foobar(); //~ ERROR no method named `foobar`\n   |           ^^^^^^\n   |\n   = help: items from traits can only be used if the trait is in scope\n   = note: the following trait is implemented but not in scope, perhaps add a `use` for it:\n           `use crate::foo::foobar::Foobar;`\n\n"}
[00:48:15] {"message":"no method named `bar` found for type `u32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n