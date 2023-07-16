plain
[00:01:09] Successfully tagged rust-ci:latest
[00:01:09] Built container sha256:fd06f1b17d114c98e61557bd3cf89dece6cfdde159248c88b763f79aa28fc6d8
[00:01:09] Uploading finished image to s3://rust-lang-ci-sccache2/docker/cb34023c5a7cb82ffe2f0242fcd48e5a4f58c437f1020b77bf0047016c6140d2bf58efb498cb330775664c6106d735bda3d699eb5dd4e29dcbf1d174d04bfaa1
[00:01:09] 
[00:01:09] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:01:13] xargs: docker: terminated by signal 13

[00:01:13] travis_time:end:3007c844:start=1530564986771260936,finish=1530565048408807325,duration=61637546389
[CI_JOB_NAME=x86_64-gnu-tools]
[00:01:13] [CI_JOB_NAME=x86_64-gnu-tools]
---
######################################################################## 100.0%
[00:01:35] extracting /checkout/obj/build/cache/2018-06-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:57]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:00] make: *** [prepare] Error 1
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:01]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:01]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:01]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:01] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:01] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:01] make: *** [prepare] Error 1
[00:02:01] make: *** [prepare] Error 1
[00:02:01] Makefile:81: recipe for target 'prepare' failed
[00:02:04]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:04]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:04]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:04] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:04] make: *** [prepare] Error 1
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:81: recipe for target 'prepare' failed
[00:02:07]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:08]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:08]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:08] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:08] Build completed unsuccessfully in 0:00:00
[00:02:08] Makefile:81: recipe for target 'prepare' failed
[00:02:08] make: *** [prepare] Error 1
[00:02:12]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:12]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:12]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:02:13] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:13] make: *** [prepare] Error 1
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:81: recipe for target 'prepare' failed
[00:02:13] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:00074af9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:082ae0dd:start=1530565108847940288,finish=1530565108854592364,duration=6652076
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00bda58a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1170246c
$ dmesg | grep -i kill
