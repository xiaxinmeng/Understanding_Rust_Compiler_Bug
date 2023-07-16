plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:16f4cbf2
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:02:48] sha256:eba3207fbb3ce77a32cf11eef774aad9cc65b9ec6ae26d793a0401552a2ad164
[00:02:48] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:02:48] Sending build context to Docker daemon  501.2kB
[00:02:48] Step 1/38 : FROM centos:5
[00:02:48] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:02:49] Sending build context to Docker daemon  501.2kB
[00:02:49] Step 1/38 : FROM centos:5
[00:02:49] Step 1/38 : FROM centos:5
[00:02:49] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:02:52] Sending build context to Docker daemon  501.2kB
[00:02:52] Step 1/38 : FROM centos:5
[00:02:52] Step 1/38 : FROM centos:5
[00:02:52] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:02:55] Sending build context to Docker daemon  501.2kB
[00:02:55] Step 1/38 : FROM centos:5
[00:02:55] Step 1/38 : FROM centos:5
[00:02:55] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:03:00] Sending build context to Docker daemon  501.2kB
[00:03:00] Step 1/38 : FROM centos:5
[00:03:00] Step 1/38 : FROM centos:5
[00:03:00] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:03:00] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1272c24c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:275775d8:start=1529881104034637768,finish=1529881104041701127,duration=7063359
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060d32dc
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04e14872
$ dmesg | grep -i kill
