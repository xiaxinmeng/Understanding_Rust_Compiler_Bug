plain
[00:01:59] Successfully tagged rust-ci:latest
[00:02:00] Built container sha256:d61a9c2f4ad565fed4368180decbc800228b690218ecc38dd9a9cd07e44452b3
[00:02:00] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:00] 
[00:02:00] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:06] xargs: docker: terminated by signal 13

[00:02:06] travis_time:end:0289475a:start=1532651767730041925,finish=1532651880420063215,duration=112690021290
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:06] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
##############                                                            19.9%
######################################################################## 100.0%
[00:02:24] extracting /checkout/obj/build/cache/2018-07-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:46]     Updating git repository `https://github.com/kngwyu/rls-vfs`
[00:02:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:47] Makefile:81: recipe for target 'prepare' failed
[00:02:47] make: *** [prepare] Error 1
[00:02:48] Command failed. Attempt 2/5:
[00:02:48]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:48]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:48]     Updating git repository `https://github.com/kngwyu/rls-vfs`
[00:02:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:48] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:48] Makefile:81: recipe for target 'prepare' failed
[00:02:48] make: *** [prepare] Error 1
[00:02:50] Command failed. Attempt 3/5:
[00:02:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:51]     Updating git repository `https://github.com/kngwyu/rls-vfs`
[00:02:51] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:51] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:51] make: *** [prepare] Error 1
[00:02:51] Makefile:81: recipe for target 'prepare' failed
[00:02:54] Command failed. Attempt 4/5:
[00:02:54]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:54]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:55]     Updating git repository `https://github.com/kngwyu/rls-vfs`
[00:02:55] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:55] Makefile:81: recipe for target 'prepare' failed
[00:02:55] make: *** [prepare] Error 1
[00:02:59] Command failed. Attempt 5/5:
[00:02:59]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:59]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:59]     Updating git repository `https://github.com/kngwyu/rls-vfs`
[00:03:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:03:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:00] make: *** [prepare] Error 1
[00:03:00] Makefile:81: recipe for target 'prepare' failed
[00:03:00] The command has failed after 5 attempts.
travis_time:end:006142f4:start=1532651754194734362,finish=1532651934671917794,duration=180477183432
---
travis_time:end:1e564d47:start=1532651934984865960,finish=1532651934993263573,duration=8397613
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3533871c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03e8db38
travis_time:start:03e8db38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bcbde9f
$ dmesg | grep -i kill
