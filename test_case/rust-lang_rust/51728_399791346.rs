plain
[00:00:54] sha256:dc365898ae0130338f4ac4e5ddd8d3b248c287c6baa3ba22c0314117d4c95534
[00:00:54] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/armhf-gnu/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:54] Sending build context to Docker daemon  501.2kB
[00:00:54] Step 1/20 : FROM ubuntu:16.04
[00:00:54] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:55] Sending build context to Docker daemon  501.2kB
[00:00:55] Step 1/20 : FROM ubuntu:16.04
[00:00:55] Step 1/20 : FROM ubuntu:16.04
[00:00:56] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:58] Sending build context to Docker daemon  501.2kB
[00:00:58] Step 1/20 : FROM ubuntu:16.04
[00:00:58] Step 1/20 : FROM ubuntu:16.04
[00:00:58] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:01] Sending build context to Docker daemon  501.2kB
[00:01:01] Step 1/20 : FROM ubuntu:16.04
[00:01:01] Step 1/20 : FROM ubuntu:16.04
[00:01:01] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:05] Sending build context to Docker daemon  501.2kB
[00:01:05] Step 1/20 : FROM ubuntu:16.04
[00:01:05] Step 1/20 : FROM ubuntu:16.04
[00:01:06] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:06] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:052c83a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0219f7b8:start=1529878133256071644,finish=1529878133265973144,duration=9901500
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0012b7bd
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04725bb0
$ dmesg | grep -i kill
