plain
travis_time:end:03cc1cf0:start=1554311126937002861,finish=1554311129349319038,duration=2412316177
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:57:41]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:57:44] error[E0308]: mismatched types
[00:57:44]     --> src/librustdoc/clean/mod.rs:2765:51
[00:57:44]      |
[00:57:44] 2765 |                     Ok(length) => print_const(cx, length),
[00:57:44]      |                                                   |
[00:57:44]      |                                                   expected struct `rustc::ty::Const`, found reference
[00:57:44]      |                                                   expected struct `rustc::ty::Const`, found reference
[00:57:44]      |                                                   help: consider dereferencing the borrow: `*length`
[00:57:44]      = note: expected type `rustc::ty::Const<'_>`
[00:57:44]                 found type `&rustc::ty::Const<'_>`
[00:57:44] 
[00:57:44] error[E0308]: mismatched types
[00:57:44] error[E0308]: mismatched types
[00:57:44]     --> src/librustdoc/clean/mod.rs:2960:29
[00:57:44]      |
[00:57:44] 2960 |                         n = new_n;
[00:57:44]      |                             ^^^^^
[00:57:44]      |                             |
[00:57:44]      |                             expected struct `rustc::ty::Const`, found reference
[00:57:44]      |                             help: consider dereferencing the borrow: `*new_n`
[00:57:44]      = note: expected type `rustc::ty::Const<'_>`
[00:57:44]                 found type `&rustc::ty::Const<'_>`
[00:57:44] 
[00:57:44] error[E0599]: no method named `to_string` found for type `rustc::ty::Const<'_>` in the current scope
[00:57:44] error[E0599]: no method named `to_string` found for type `rustc::ty::Const<'_>` in the current scope
[00:57:44]     --> src/librustdoc/clean/mod.rs:4142:27
[00:57:44]      |
[00:57:44] 4142 |             let mut s = n.to_string();
[00:57:44]      |
[00:57:44]      = note: the method `to_string` exists but the following trait bounds were not satisfied:
[00:57:44]              `rustc::ty::Const<'_> : std::string::ToString`
[00:57:44] 
---
travis_time:end:17fb976e:start=1554314609057925204,finish=1554314609063103580,duration=5178376
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2a0f13f2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b7f4aa
$ cat ./obj/build/x86_64-unkn
