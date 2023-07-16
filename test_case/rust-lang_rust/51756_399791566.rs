plain
[00:00:52] sha256:dc365898ae0130338f4ac4e5ddd8d3b248c287c6baa3ba22c0314117d4c95534
[00:00:52] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/armhf-gnu/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:52] Sending build context to Docker daemon  501.2kB
[00:00:52] Step 1/20 : FROM ubuntu:16.04
[00:00:52] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:53] Sending build context to Docker daemon  501.2kB
[00:00:53] Step 1/20 : FROM ubuntu:16.04
[00:00:53] Step 1/20 : FROM ubuntu:16.04
[00:00:53] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:56] Sending build context to Docker daemon  501.2kB
[00:00:56] Step 1/20 : FROM ubuntu:16.04
[00:00:56] Step 1/20 : FROM ubuntu:16.04
[00:00:56] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:59] Sending build context to Docker daemon  501.2kB
[00:00:59] Step 1/20 : FROM ubuntu:16.04
[00:00:59] Step 1/20 : FROM ubuntu:16.04
[00:00:59] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:03] Sending build context to Docker daemon  501.2kB
[00:01:03] Step 1/20 : FROM ubuntu:16.04
[00:01:03] Step 1/20 : FROM ubuntu:16.04
[00:01:03] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:03] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:19a5c478
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1aa6364b:start=1529878389398375000,finish=1529878389405505544,duration=7130544
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ca418d
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aa7fcf9
$ dmesg | grep -i kill
