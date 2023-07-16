plain
travis_time:end:1eebd3f4:start=1554060901286019015,finish=1554060904098983817,duration=2812964802
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:37]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:37]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:38]    Compiling rustc-demangle v0.1.10
[00:04:42]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:45] error[E0599]: no method named `next_index` found for type `[u8]` in the current scope
[00:04:45]    --> src/libstd/sys_common/os_str_bytes.rs:210:20
[00:04:45]     |
[00:04:45] 210 |         self.inner.next_index(index)
[00:04:45]     |
[00:04:45]     = help: items from traits can only be used if the trait is in scope
[00:04:45]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:04:45]             `use core::needle::Hay;`
[00:04:45]             `use core::needle::Hay;`
[00:04:45] 
[00:04:45] error[E0599]: no method named `prev_index` found for type `[u8]` in the current scope
[00:04:45]    --> src/libstd/sys_common/os_str_bytes.rs:214:20
[00:04:45]     |
[00:04:45] 214 |         self.inner.prev_index(index)
[00:04:45]     |
[00:04:45]     = help: items from traits can only be used if the trait is in scope
[00:04:45]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:04:45]             `use core::needle::Hay;`
