plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:09bf92f9
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
[00:00:23] curl: (22) The requested URL returned error: 404 Not Found
[00:00:23] The command has failed after 5 attempts.
[00:00:23] open /tmp/rustci_docker_cache: no such file or directory
[00:00:23] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:23] Sending build context to Docker daemon  498.2kB
[00:00:23] Step 1/9 : FROM ubuntu:16.04
[00:00:24] 16.04: Pulling from library/ubuntu
---
[00:03:41] Verifying status of miri...
[00:03:41] Cloning into 'rust-toolstate'...
[00:03:41] [test 332844a] (linux CI update)
[00:03:41]  1 file changed, 1 insertion(+)
[00:03:42] remote: Permission to rust-lang-nursery/rust-toolstate.git denied to rust-highfive.
[00:03:42] fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': The requested URL returned error: 403
[00:03:45]  * branch            test       -> FETCH_HEAD
[00:03:45] HEAD is now at 1e94c78 (linux CI update)
[00:03:45] [test 91c1332] (linux CI update)
[00:03:45]  1 file changed, 1 insertion(+)
[00:03:45]  1 file changed, 1 insertion(+)
[00:03:45] remote: Permission to rust-lang-nursery/rust-toolstate.git denied to rust-highfive.
[00:03:45] fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': The requested URL returned error: 403
[00:03:48]  * branch            test       -> FETCH_HEAD
[00:03:48] HEAD is now at 1e94c78 (linux CI update)
[00:03:48] [test 30fdd37] (linux CI update)
[00:03:48]  1 file changed, 1 insertion(+)
[00:03:48]  1 file changed, 1 insertion(+)
[00:03:48] remote: Permission to rust-lang-nursery/rust-toolstate.git denied to rust-highfive.
[00:03:48] fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': The requested URL returned error: 403
[00:03:49]  * branch            test       -> FETCH_HEAD
[00:03:49] HEAD is now at 1e94c78 (linux CI update)
[00:03:49] [test 0cdca8c] (linux CI update)
[00:03:49]  1 file changed, 1 insertion(+)
[00:03:49]  1 file changed, 1 insertion(+)
[00:03:49] remote: Permission to rust-lang-nursery/rust-toolstate.git denied to rust-highfive.
[00:03:49] fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': The requested URL returned error: 403
[00:03:52]  * branch            test       -> FETCH_HEAD
[00:03:52] HEAD is now at 1e94c78 (linux CI update)
[00:03:52] [test 71ec743] (linux CI update)
[00:03:52]  1 file changed, 1 insertion(+)
[00:03:52]  1 file changed, 1 insertion(+)
[00:03:52] remote: Permission to rust-lang-nursery/rust-toolstate.git denied to rust-highfive.
[00:03:52] fatal: unable to access 'https://github.com/rust-lang-nursery/rust-toolstate.git/': The requested URL returned error: 403
[00:03:56]  * branch            test       -> FETCH_HEAD
[00:03:56] HEAD is now at 1e94c78 (linux CI update)
travis_time:end:1c704f60:start=1526199933827068153,finish=1526200170332777978,duration=236505709825

