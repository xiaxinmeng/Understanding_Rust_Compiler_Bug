plain
travis_time:end:1b436310:start=1555780876421150034,finish=1555780877167297165,duration=746147131
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:47]    Compiling synstructure v0.10.1
[00:07:06]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:11]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:15]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:22] error: description for error code E0704 contains a line longer than 80 characters.
[00:07:22] if you're inserting a long URL use the footnote style to bypass this check.
[00:07:22]    --> src/libsyntax/diagnostics/macros.rs:3:37
[00:07:22] 2   | / macro_rules! register_diagnostic {
[00:07:22] 2   | / macro_rules! register_diagnostic {
[00:07:22] 3   | |     ($code:tt, $description:tt) => (__register_diagnostic! { $code, $description });
[00:07:22]     | |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:07:22] 4   | |     ($code:tt) => (__register_diagnostic! { $code })
[00:07:22]     | |_- in this expansion of `register_diagnostic!`
[00:07:22] ...
[00:07:22] 181 | / macro_rules! register_long_diagnostics {
[00:07:22] 181 | / macro_rules! register_long_diagnostics {
[00:07:22] 182 | |     ($($code:tt: $description:tt),*) => (
[00:07:22] 183 | |         $(register_diagnostic! { $code, $description })*
[00:07:22] 184 | |     );
[00:07:22] 185 | |     ($($code:tt: $description:tt),*,) => (
[00:07:22] 186 | |         $(register_diagnostic! { $code, $description })*
[00:07:22] 187 | |     )
[00:07:22] 188 | | }
[00:07:22]     | |_- in this expansion of `register_long_diagnostics!`
[00:07:22]     | 
[00:07:22]     | 
[00:07:22]    ::: src/libsyntax/error_codes.rs:6:1
[00:07:22]     |
[00:07:22] 6   | / register_long_diagnostics! {
[00:07:22] 7   | |
[00:07:22] 8   | | E0178: r##"
[00:07:22] 9   | | In types, the `+` type operator has low precedence, so it is often necessary
[00:07:22] 422 | |
[00:07:22] 423 | | }
[00:07:22]     | |_- in this macro invocation
[00:07:22] 
---
29432 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
29220 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools
28992 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps
27124 ./src/llvm-project/llgo/third_party/gofrontend
26652 ./src/llvm-emscripten/test/Transforclang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00e481aa
$ dmesg | grep -i kill
