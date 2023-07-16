plain
[00:00:48] sha256:5e8b97a2a0820b10338bd91674249a94679e4568fd1183ea46acff63b9883e9c
[00:00:48] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-i586-gnu-i586-i686-musl/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:48] Sending build context to Docker daemon  501.2kB
[00:00:48] Step 1/13 : FROM ubuntu:16.04
[00:00:48] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: error parsing HTTP 404 response body: invalid character ':' after top-level value: "404: Page Not Found"
[00:00:49] Sending build context to Docker daemon  501.2kB
[00:00:49] Step 1/13 : FROM ubuntu:16.04
[00:00:49] Step 1/13 : FROM ubuntu:16.04
[00:00:49] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: error parsing HTTP 404 response body: invalid character ':' after top-level value: "404: Page Not Found"
[00:00:51] Sending build context to Docker daemon  501.2kB
[00:00:52] Step 1/13 : FROM ubuntu:16.04
[00:00:52] Step 1/13 : FROM ubuntu:16.04
[00:00:52] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: error parsing HTTP 404 response body: invalid character ':' after top-level value: "404: Page Not Found"
[00:00:55] Sending build context to Docker daemon  501.2kB
[00:00:55] Step 1/13 : FROM ubuntu:16.04
[00:00:55] Step 1/13 : FROM ubuntu:16.04
[00:00:55] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:59] Sending build context to Docker daemon  501.2kB
[00:00:59] Step 1/13 : FROM ubuntu:16.04
[00:00:59] Step 1/13 : FROM ubuntu:16.04
[00:01:00] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:01:00] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:186770a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01829b63:start=1529877904406648426,finish=1529877904414793877,duration=8145451
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0084ded6
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1564c8e8
$ dmesg | grep -i kill
