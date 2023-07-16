plain
travis_time:end:004b2493:start=1550942339849994916,finish=1550942340792632538,duration=942637622
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:45]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:57:48] error[E0432]: unresolved import `super::acle`
[00:57:48]   --> src/libcore/../stdsimd/crates/core_arch/src/aarch64/mod.rs:21:16
[00:57:48]    |
[00:57:48] 21 | pub use super::acle::*;
[00:57:48]    |                ^^^^ could not find `acle` in `super`
[00:57:48] error[E0432]: unresolved import `crate::core_arch::acle`
[00:57:48]   --> src/libcore/../stdsimd/crates/core_arch/src/arm/mod.rs:37:27
[00:57:48]    |
[00:57:48]    |
[00:57:48] 37 | pub use crate::core_arch::acle::*;
[00:57:48]    |                           ^^^^ could not find `acle` in `core_arch`
[00:57:56] error: Compilation failed, aborting rustdoc
[00:57:56] 
[00:57:57] error: Could not document `core`.
[00:57:57] 
[00:57:57] 
[00:57:57] Caused by:
[00:57:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name core src/libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:57:57] 
[00:57:57] 
[00:57:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--index-page" "/checkout/src/doc/index.md"
[00:57:57] 
[00:57:57] 
[00:57:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:57:57] Build completed unsuccessfully in 0:05:48
