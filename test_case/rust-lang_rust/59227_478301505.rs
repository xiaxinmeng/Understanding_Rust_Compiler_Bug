plain
travis_time:end:007d25dc:start=1553992006979200145,finish=1553992009134979267,duration=2155779122
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:07:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:59] error[E0308]: mismatched types
[00:07:59]    --> src/libsyntax_ext/proc_macro_server.rs:343:44
[00:07:59]     |
[00:07:59] 343 |         if is_raw && !ast::Ident::from_str(string).can_be_raw() {
[00:07:59]     |                                            |
[00:07:59]     |                                            expected &str, found struct `syntax_pos::symbol::LocalInternedString`
[00:07:59]     |                                            help: consider borrowing here: `&string`
[00:07:59]     |
---
travis_time:end:0b328380:start=1553992869557888115,finish=1553992870100994015,duration=543105900
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1238db71
$ ls -lat $HOME/Library/Logs/Di/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16e44d80
$ dmesg | grep -i kill
