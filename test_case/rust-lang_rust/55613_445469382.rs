plain
travis_time:end:009e4a8c:start=1544282013837028283,finish=1544282016049918288,duration=2212890005
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:03] .................................................................................................... 4000/5132
[00:47:08] .................................................................................................... 4100/5132
[00:47:10] .................................................................................................... 4200/5132
[00:47:14] .................................................................................................... 4300/5132
[00:47:19] ..i................................................................................................. 4400/5132
[00:47:23] ...........F........................................................................................ 4500/5132
[00:47:29] ........................................................................................i........... 4700/5132
[00:47:32] .................................................................................................... 4800/5132
[00:47:41] .......................................................................i............................ 5100/5132
[00:47:41] .......................................................................i............................ 5100/5132
ement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"items from traits can only be used if the trait is in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait is implemented but not in scope, perhaps add a `use` for it:","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs","byte_start":692,"byte_end":692,"line_start":18,"line_end":18,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"mod foo {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use crate::foo::Bar;\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `bar` found for type `u32` in the current scope\n  --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:41:7\n   |\nLL |     x.bar(); //~ ERROR no method named `bar`\n   |       ^^^\n   |\n   = help: items from traits can only be used if the trait is in scope\nhelp: the following trait is implemented but not in scope, perhaps add a `use` for it:\n   |\nLL | use crate::foo::Bar;\n   |\n\n"}
[00:47:42] {"message":"no method named `extern_baz` found for type `u32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n