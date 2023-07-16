plain
travis_time:end:2109f981:start=1560317311741936049,finish=1560317312588596767,duration=846660718
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:22]    Compiling synstructure v0.10.2
[00:07:35]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:42]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:46]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:46] error[E0428]: the name `x86_64_sun_solaris` is defined multiple times
[00:07:46]     |
[00:07:46] 270 | / macro_rules! supported_targets {
[00:07:46] 270 | / macro_rules! supported_targets {
[00:07:46] 271 | |     ( $(($triple:expr, $module:ident),)+ ) => (
[00:07:46] 272 | |         $(mod $module;)*
[00:07:46]     | |           |
[00:07:46]     | |           |
[00:07:46]     | |           `x86_64_sun_solaris` redefined here
[00:07:46]     | |           previous definition of the module `x86_64_sun_solaris` here
[00:07:46] ...   |
[00:07:46] 327 | |     )
[00:07:46] 328 | | }
[00:07:46]     | |_- in this expansion of `supported_targets!`
[00:07:46]     | |_- in this expansion of `supported_targets!`
[00:07:46] 329 | 
[00:07:46] 330 | / supported_targets! {
[00:07:46] 331 | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
[00:07:46] 332 | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
[00:07:46] 333 | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
[00:07:46] ...   |
[00:07:46] 479 | |     ("nvptx64-nvidia-cuda", nvptx64_nvidia_cuda),
[00:07:46]     | |_- in this macro invocation
[00:07:46]     |
[00:07:46]     |
[00:07:46]     = note: `x86_64_sun_solaris` must be defined only once in the type namespace of this module
[00:07:47] error: aborting due to previous error
[00:07:47] 
[00:07:47] For more information about this error, try `rustc --explain E0428`.
[00:07:47] error: Could not compile `rustc_target`.
