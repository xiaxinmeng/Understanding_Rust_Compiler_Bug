plain

Network availability confirmed.
travis_fold:start:git.checkout
travis_time:start:16a7e050
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:22] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
[00:00:22] curl: (22) The requested URL returned error: 404 Not Found
[00:00:22] The command has failed after 5 attempts.
[00:00:22] open /tmp/rustci_docker_cache: no such file or directory
[00:00:22] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:23] Sending build context to Docker daemon  501.8kB
[00:00:23] Sending build context to Docker daemon  501.8kB
[00:00:23] Error response from daemon: Dockerfile parse error line 97: unknown instruction: --SET
[00:00:24] Sending build context to Docker daemon  501.8kB
[00:00:24] Sending build context to Docker daemon  501.8kB
[00:00:24] Error response from daemon: Dockerfile parse error line 97: unknown instruction: --SET
[00:00:26] Sending build context to Docker daemon  501.8kB
[00:00:26] Sending build context to Docker daemon  501.8kB
[00:00:26] Error response from daemon: Dockerfile parse error line 97: unknown instruction: --SET
[00:00:29] Sending build context to Docker daemon  501.8kB
[00:00:29] Sending build context to Docker daemon  501.8kB
[00:00:29] Error response from daemon: Dockerfile parse error line 97: unknown instruction: --SET
[00:00:33] Sending build context to Docker daemon  501.8kB
[00:00:33] Sending build context to Docker daemon  501.8kB
[00:00:33] Error response from daemon: Dockerfile parse error line 97: unknown instruction: --SET
[00:00:33] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:15916058
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
