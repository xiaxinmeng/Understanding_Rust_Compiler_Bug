plain
travis_time:end:0409beb0:start=1554534117326209419,finish=1554534191252103371,duration=73925893952
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:13]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:13]    Compiling rustc-demangle v0.1.10
[00:04:13]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:17]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:20] error[E0277]: expected a `core::ops::Fn<(&u8,)>` closure, found `[u8; 1]`
[00:04:20]     |
[00:04:20]     |
[00:04:20] 318 |     cfg!(target_os = "redox") && s.split(b"/").next().unwrap_or(b"").contains(&b':')
[00:04:20]     |                                    ^^^^^ expected an `Fn<(&u8,)>` closure, found `[u8; 1]`
[00:04:20]     |
[00:04:20]     = help: the trait `core::ops::Fn<(&u8,)>` is not implemented for `[u8; 1]`
[00:04:20]     = note: required because of the requirements on the impl of `for<'r> core::ops::FnMut<(&'r u8,)>` for `&[u8; 1]`
[00:04:20] 
[00:04:20] error[E0277]: expected a `core::ops::Fn<(&u8,)>` closure, found `[u8; 1]`
[00:04:20]     |
[00:04:20]     |
[00:04:20] 347 |         let mut iter = os_str_as_u8_slice(file).rsplitn(2, b".");
[00:04:20]     |                                                 ^^^^^^^ expected an `Fn<(&u8,)>` closure, found `[u8; 1]`
[00:04:20]     |
[00:04:20]     = help: the trait `core::ops::Fn<(&u8,)>` is not implemented for `[u8; 1]`
[00:04:20]     = note: required because of the requirements on the impl of `for<'r> core::ops::FnMut<(&'r u8,)>` for `&[u8; 1]`
[00:04:21] error: aborting due to 2 previous errors
[00:04:21] 
[00:04:21] For more information about this error, try `rustc --explain E0277`.
[00:04:21] error: Could not compile `std`.
