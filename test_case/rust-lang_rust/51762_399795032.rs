plain
[00:00:45] sha256:8c1f0882640438d1d5e0c0acc857fedd1a16de0d9ee943660aa40de5c9b49bad
[00:00:45] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-llvm-3.9/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:46] Sending build context to Docker daemon  501.2kB
[00:00:46] Step 1/6 : FROM ubuntu:16.04
[00:00:46] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:47] Sending build context to Docker daemon  501.2kB
[00:00:47] Step 1/6 : FROM ubuntu:16.04
[00:00:47] Step 1/6 : FROM ubuntu:16.04
[00:00:47] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:49] Sending build context to Docker daemon  501.2kB
[00:00:49] Step 1/6 : FROM ubuntu:16.04
[00:00:49] Step 1/6 : FROM ubuntu:16.04
[00:00:50] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:53] Sending build context to Docker daemon  501.2kB
[00:00:53] Step 1/6 : FROM ubuntu:16.04
[00:00:53] Step 1/6 : FROM ubuntu:16.04
[00:00:53] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:57] Sending build context to Docker daemon  501.2kB
[00:00:57] Step 1/6 : FROM ubuntu:16.04
[00:00:57] Step 1/6 : FROM ubuntu:16.04
[00:00:57] Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 503 Service Unavailable
[00:00:57] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0133555e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00cc2430:start=1529880927451103207,finish=1529880927459145405,duration=8042198
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029acca7
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28d9b87e
$ dmesg | grep -i kill
