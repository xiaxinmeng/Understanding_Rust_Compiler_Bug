plain
[00:01:20] Successfully tagged rust-ci:latest
[00:01:21] Built container sha256:b2991e76f208a1673e46ac8de7bc8925d6ad58b34d25bc24cdfffdc993a3bcc2
[00:01:21] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:01:21] 
[00:01:21] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:01:26] xargs: docker: terminated by signal 13

[00:01:27] travis_time:end:011444fa:start=1531134454848026876,finish=1531134529231126767,duration=74383099891
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:27] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
##                                                                         3.1%
###############                                                           21.2%
######################################################################## 100.0%
[00:01:40] extracting /checkout/obj/build/cache/2018-06-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:40] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:40] Caused by:
[00:01:40] Caused by:
[00:01:40]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] make: *** [prepare] Error 1
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 2/5:
[00:01:41] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:41] Caused by:
[00:01:41] Caused by:
[00:01:41]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:41] make: *** [prepare] Error 1
[00:01:43] Command failed. Attempt 3/5:
[00:01:43] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:43] Caused by:
[00:01:43] Caused by:
[00:01:43]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:43] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] Makefile:81: recipe for target 'prepare' failed
[00:01:43] make: *** [prepare] Error 1
[00:01:46] Command failed. Attempt 4/5:
[00:01:46] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:46] Caused by:
[00:01:46] Caused by:
[00:01:46]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:46] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:46] make: *** [prepare] Error 1
[00:01:46] make: *** [prepare] Error 1
[00:01:46] Makefile:81: recipe for target 'prepare' failed
210188 ./src/llvm/test
192708 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
171516 ./.git
170316 ./obj/build/cache
