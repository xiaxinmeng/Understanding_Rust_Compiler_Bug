plain
travis_time:end:02afc596:start=1552654825571455719,finish=1552654828350205826,duration=2778750107
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:24:35]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:24:35] error[E0106]: missing lifetime specifier
[00:24:35]    --> src/librustc_interface/passes.rs:557:13
[00:24:35]     |
[00:24:35] 557 | ) -> Result<hir::map::Forest> {
[00:24:35]     |
[00:24:35]     |
[00:24:35]     = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `sess`, `cstore`, one of `resolver`'s 2 lifetimes, `dep_graph`, or `krate`
[00:24:35] error[E0106]: missing lifetime specifier
[00:24:35]   --> src/librustc_interface/queries.rs:92:32
[00:24:35]    |
[00:24:35]    |
[00:24:35] 92 |     lower_to_hir: Query<(Steal<hir::map::Forest>, ExpansionResult)>,
[00:24:35] 
[00:24:35] error: aborting due to 2 previous errors
[00:24:35] 
[00:24:35] For more information about this error, try `rustc --explain E0106`.
---
travis_time:end:149a8848:start=1552656423843313926,finish=1552656424476343501,duration=633029575
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:01f25780
$ ls -lat $HOME/Library/Logs/nu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00acb1e7
$ dmesg | grep -i kill
