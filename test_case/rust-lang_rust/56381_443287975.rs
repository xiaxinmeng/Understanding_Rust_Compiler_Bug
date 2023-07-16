plain
travis_time:end:0b497ce4:start=1543600812281298193,finish=1543600876981963398,duration=64700665205
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide) registered for path 'src/doc/edition-guide'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
---
##############################################################            87.1%
######################################################################## 100.0%
[00:01:14] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:14]     Updating crates.io index
[00:01:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:20] Build completed unsuccessfully in 0:00:18
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:20] make: *** [prepare] Error 1
[00:01:21] Command failed. Attempt 2/5:
[00:01:21] Command failed. Attempt 2/5:
[00:01:22]     Updating crates.io index
[00:01:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:22] Build completed unsuccessfully in 0:00:00
[00:01:22] Makefile:81: recipe for target 'prepare' failed
[00:01:22] make: *** [prepare] Error 1
[00:01:24] Command failed. Attempt 3/5:
[00:01:24] Command failed. Attempt 3/5:
[00:01:24]     Updating crates.io index
[00:01:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:27] Command failed. Attempt 4/5:
[00:01:27] Command failed. Attempt 4/5:
[00:01:28]     Updating crates.io index
[00:01:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:32] Command failed. Attempt 5/5:
[00:01:32] Command failed. Attempt 5/5:
[00:01:32]     Updating crates.io index
[00:01:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] make: *** [prepare] Error 1
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:33] The command has failed after 5 attempts.
