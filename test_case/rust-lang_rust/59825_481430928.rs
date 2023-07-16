plain
travis_time:end:1f631d42:start=1554842818052611909,finish=1554842818976404255,duration=923792346
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:53]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:55]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:55]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:55]    Compiling rustc-demangle v0.1.10
[00:04:58] error: impl has missing stability attribute
[00:04:58]      |
[00:04:58]      |
[00:04:58] 2182 | / impl From<&String> for String {
[00:04:58] 2183 | |     #[inline]
[00:04:58] 2184 | |     fn from(s: &String) -> String {
[00:04:58] 2185 | |         s.clone()
[00:04:58] 2187 | | }
[00:04:58]      | |_^
[00:04:58] 
[00:04:58] error: aborting due to previous error
