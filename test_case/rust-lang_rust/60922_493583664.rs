plain
travis_time:end:06baa528:start=1558119585810180565,finish=1558119680379516684,duration=94569336119
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:01:35] From https://github.com/rust-lang/rust
[00:01:35]  * branch                    beta       -> FETCH_HEAD
[00:01:35]  * branch                    master     -> FETCH_HEAD
[00:01:35]    b982867a734..823a75d9ba3  master     -> origin/master
[00:01:36] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/llvm.url'. Skipping second one!
[00:01:36] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/llvm.branch'. Skipping second one!
[00:01:36] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.path'. Skipping second one!
[00:01:36] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.url'. Skipping second one!
---
[01:05:04] .................................................................................................... 400/5536
[01:05:07] .................................................................................................... 500/5536
[01:05:11] ...............................................i.................................................... 600/5536
[01:05:15] .................................................................................................... 700/5536
[01:05:19] ........................F..F........................................................................ 800/5536
[01:05:29] ...........i...............i........................................................................ 1000/5536
[01:05:32] ............................................iiiii................................................... 1100/5536
[01:05:36] .................................................................................................... 1200/5536
[01:05:38] .................................................................................................... 1300/5536
---
[01:06:27] .................................................................................................... 2700/5536
[01:06:32] .................................................................................................... 2800/5536
[01:06:36] .................................................................................................... 2900/5536
[01:06:40] .................................................................................................... 3000/5536
[01:06:44] ................................F................................................................... 3100/5536
[01:06:51] .................................................................................................... 3300/5536
[01:06:54] ..................................i................................................................. 3400/5536
[01:06:58] .................................................................................................... 3500/5536
[01:07:01] ........ii...i..ii.................................................................................. 3600/5536
---
[01:08:16] 26 
[01:08:16] 
[01:08:16] 
[01:08:16] The actual stderr differed from the expected stderr.
[01:08:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/invalid-const-arg-for-type-param.stderr
[01:08:16] To update references, rerun the tests and pass the `--bless` flag
[01:08:16] To only update this specific test, also pass `--test-args const-generics/invalid-const-arg-for-type-param.rs`
[01:08:16] error: 1 errors occurred comparing output.
[01:08:16] status: exit code: 1
[01:08:16] status: exit code: 1
[01:08:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/auxiliary" "-A" "unused"
[01:08:16] ------------------------------------------
[01:08:16] 
[01:08:16] ------------------------------------------
[01:08:16] stderr:
[01:08:16] stderr:
[01:08:16] ------------------------------------------
[01:08:16] {"message":"wrong number of const arguments: expected 0, found 1","code":{"code":"E0107","explanation":"\nThis error means that an incorrect number of generic arguments were provided:\n\n