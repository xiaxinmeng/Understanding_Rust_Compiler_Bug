plain
travis_time:end:09fa544d:start=1550596811992231198,finish=1550596813592030955,duration=1599799757
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:00]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:07:03] error[E0432]: unresolved import `super::acle`
[01:07:03]   --> src/libcore/../stdsimd/crates/core_arch/src/aarch64/mod.rs:21:16
[01:07:03]    |
[01:07:03] 21 | pub use super::acle::*;
[01:07:03]    |                ^^^^ could not find `acle` in `super`
[01:07:03] error[E0432]: unresolved import `super::acle`
[01:07:03]   --> src/libcore/../stdsimd/crates/core_arch/src/arm/mod.rs:37:16
[01:07:03]    |
[01:07:03]    |
[01:07:03] 37 | pub use super::acle::*;
[01:07:03]    |                ^^^^ could not find `acle` in `super`
[01:07:11] error: Compilation failed, aborting rustdoc
[01:07:11] 
[01:07:12] error: Could not document `core`.
[01:07:12] 
[01:07:12] 
[01:07:12] Caused by:
[01:07:12]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name core src/libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[01:07:12] 
[01:07:12] 
[01:07:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--index-page" "/checkout/src/doc/index.md"
[01:07:12] 
[01:07:12] 
[01:07:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:07:12] Build completed unsuccessfully in 0:05:45
[01:07:12] Build completed unsuccessfully in 0:05:45
[01:07:12] Makefile:18: recipe for target 'all' failed
[01:07:12] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13c4c7e7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 19 18:27:36 UTC 2019
