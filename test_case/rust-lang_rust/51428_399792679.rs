plain
[00:00:50] sha256:8c1f0882640438d1d5e0c0acc857fedd1a16de0d9ee943660aa40de5c9b49bad
[00:00:50] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-3.9/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:51] Sending build context to Docker daemon  501.2kB
[00:00:51] Step 1/6 : FROM ubuntu:16.04
[00:00:51] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:52] Sending build context to Docker daemon  501.2kB
[00:00:52] Step 1/6 : FROM ubuntu:16.04
[00:00:52] Step 1/6 : FROM ubuntu:16.04
[00:00:52] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:54] Sending build context to Docker daemon  501.2kB
[00:00:54] Step 1/6 : FROM ubuntu:16.04
[00:00:54] Step 1/6 : FROM ubuntu:16.04
[00:00:55] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:58] Sending build context to Docker daemon  501.2kB
[00:00:58] Step 1/6 : FROM ubuntu:16.04
[00:00:58] Step 1/6 : FROM ubuntu:16.04
[00:00:58] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:02] Sending build context to Docker daemon  501.2kB
[00:01:02] Step 1/6 : FROM ubuntu:16.04
[00:01:02] Step 1/6 : FROM ubuntu:16.04
[00:01:02] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:02] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:19d47e78
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1bb22da7:start=1529879270564342197,finish=1529879270569687648,duration=5345451
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08f33f23
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:119cb4d0
$ dmesg | grep -i kill
