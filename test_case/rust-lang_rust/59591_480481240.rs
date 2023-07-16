plain
travis_time:end:004e717f:start=1554534670243952946,finish=1554534747070305132,duration=76826352186
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:57]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:03:57]    Compiling rustc-demangle v0.1.10
[00:03:57]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:02]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:04] error[E0658]: use of unstable library feature 'slice_needle_methods' (see issue #56345)
[00:04:04]     |
[00:04:04]     |
[00:04:04] 318 |     cfg!(target_os = "redox") && s.split_match(b"/").next().unwrap_or(b"").contains(&b':')
[00:04:04]     |
[00:04:04]     = help: add #![feature(slice_needle_methods)] to the crate attributes to enable
[00:04:04] 
[00:04:04] 
[00:04:04] error[E0658]: use of unstable library feature 'slice_needle_methods' (see issue #56345)
[00:04:04]     |
[00:04:04]     |
[00:04:04] 347 |         let mut iter = os_str_as_u8_slice(file).rsplitn_match(2, b".");
[00:04:04]     |
[00:04:04]     = help: add #![feature(slice_needle_methods)] to the crate attributes to enable
[00:04:04] 
[00:04:05] error: aborting due to 2 previous errors
