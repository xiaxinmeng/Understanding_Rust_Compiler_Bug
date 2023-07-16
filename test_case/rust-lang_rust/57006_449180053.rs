plain
travis_time:end:01243ef1:start=1545346558041760494,finish=1545346622112984869,duration=64071224375
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:52:14] travis_fold:end:stage2-rustdoc

[00:52:14] travis_time:end:stage2-rustdoc:start=1545349764166532219,finish=1545349764474454938,duration=307922719

[00:52:14] thread '<unnamed>' panicked at 'No option 'no-doc-crate-filtering' defined', /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.17/src/lib.rs:767:21
[00:52:14] 
[00:52:14] 
[00:52:14] 
[00:52:14] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/favicon.inc" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "/checkout/src/doc/rust.md" "--markdown-css" "rust.css"
[00:52:14] 
[00:52:14] 
[00:52:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:52:14] Build completed unsuccessfully in 0:04:04
[00:52:14] Build completed unsuccessfully in 0:04:04
[00:52:14] make: *** [all] Error 1
[00:52:14] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:174daa12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 20 23:49:24 UTC 2018
---
travis_time:end:05978768:start=1545349765220290677,finish=1545349765229557051,duration=9266374
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:118ff8a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b449bda
$ dmesg | grep -i kill
