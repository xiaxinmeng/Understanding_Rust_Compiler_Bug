plain
travis_time:end:10cbfefe:start=1547432847585396569,finish=1547432848537157329,duration=951760760
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:04]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:04]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:08]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:23]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:28] error[E0412]: cannot find type `Diagnostic` in this scope
[00:07:28]    --> src/librustc/ty/query/plumbing.rs:194:43
[00:07:28]     |
[00:07:28] 194 |         diagnostics: Option<&Lock<ThinVec<Diagnostic>>>,
[00:07:28] help: possible candidates are found in other modules, you can import them into scope
[00:07:28]     |
[00:07:28] 5   | use errors::Diagnostic;
[00:07:28]     |
[00:07:28]     |
[00:07:28] 5   | use proc_macro::Diagnostic;
[00:07:28]     |
[00:07:28] 5   | use proc_macro::bridge::server::Diagnostic;
[00:07:28] 5   | use syntax::diagnostics::plugin::Diagnostic;
[00:07:28]     |
[00:07:28] 
[00:07:55] error: aborting due to previous error
---
[00:07:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:55] expected success, got: exit code: 101
[00:07:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:55] Build completed unsuccessfully in 0:03:52
[00:07:55] make: *** [all] Error 1
[00:07:55] Makefile:18: recipe for target 'all' failed
19936 ./src/tools/lldb/source
19260 ./src/tools/lldb/www/cpp_reference
19256 ./src/tools/lldb/www/cpp_reference/html
17940 ./src/tools/lldb/www/python_reference
