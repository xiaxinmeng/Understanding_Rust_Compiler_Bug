plain
[00:04:24] sha256:d53c5e2d2845469b98e0a20fb01039c47f8791676fbac22778f4e4cafb2b5bce
[00:04:24] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:04:24] Sending build context to Docker daemon  520.7kB
[00:04:24] Step 1/41 : FROM centos:5
[00:04:25] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:04:26] Sending build context to Docker daemon  520.7kB
[00:04:26] Step 1/41 : FROM centos:5
[00:04:26] Step 1/41 : FROM centos:5
[00:04:26] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:04:28] Sending build context to Docker daemon  520.7kB
[00:04:28] Step 1/41 : FROM centos:5
[00:04:28] Step 1/41 : FROM centos:5
[00:04:28] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:04:32] Sending build context to Docker daemon  520.7kB
[00:04:32] Step 1/41 : FROM centos:5
[00:04:32] Step 1/41 : FROM centos:5
[00:04:32] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
[00:04:36] Sending build context to Docker daemon  520.7kB
[00:04:36] Step 1/41 : FROM centos:5
[00:04:36] Step 1/41 : FROM centos:5
[00:04:36] Get https://registry-1.docker.io/v2/library/centos/manifests/5: received unexpected HTTP status: 503 Service Unavailable
travis_time:end:116bbe00:start=1556212813284715939,finish=1556213090119882229,duration=276835166290
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:09596a39
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00dc330e:start=1556213090932415737,finish=1556213090939344740,duration=6929003
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f2f29ee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:344028f0
travis_time:start:344028f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c52d59
$ dmesg | grep -i kill
