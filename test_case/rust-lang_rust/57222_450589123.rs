plain
travis_time:end:0b0bf1e2:start=1546206512003102183,finish=1546206513056322263,duration=1053220080
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:59]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:04] error[E0369]: binary operation `==` cannot be applied to type `char::ToLowercase`
[00:03:04]    --> src/libcore/char/methods.rs:908:9
[00:03:04]     |
[00:03:04] 908 |         self.to_lowercase() == other.to_lowercase()
[00:03:04]     |
[00:03:04]     = note: an implementation of `std::cmp::PartialEq` might be missing for `char::ToLowercase`
[00:03:04] 
[00:03:04] 
   (Some(t), Some(o)) if t.eq_ignore_case(o) => (),
[00:03:04]     | |                                                          -- match arm with an incompatible type
[00:03:04] 650 | |             (None, None) => (),
[00:03:04] 652 | |         }
[00:03:04]     | |_________^ expected bool, found ()
[00:03:04]     |
[00:03:04]     = note: expected type `bool`
