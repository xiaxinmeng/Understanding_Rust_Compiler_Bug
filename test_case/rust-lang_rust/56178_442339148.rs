plain
travis_time:end:09e4007b:start=1543385001619809772,finish=1543385056461914770,duration=54842104998
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:04]     Checking compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:46:05]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:46:10]     Finished release [optimized] target(s) in 39.17s
[00:46:10]  Documenting core v0.0.0 (/checkout/src/libcore)
_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:46:14] 
[00:46:14] 
[00:46:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[00:46:14] 
[00:46:14] 
[00:46:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:46:14] Build completed unsuccessfully in 0:05:11
[00:46:14] Build completed unsuccessfully in 0:05:11
[00:46:14] Makefile:28: recipe for target 'all' failed
[00:46:14] make: *** [all] Error 1
