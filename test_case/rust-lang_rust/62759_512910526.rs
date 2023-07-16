plain
2019-07-18T17:21:07.2293493Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T17:21:07.2479688Z ##[command]git config gc.auto 0
2019-07-18T17:21:07.8169277Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T17:21:07.8173642Z ##[command]git config --get-all http.proxy
2019-07-18T17:21:07.8178426Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62759/merge:refs/remotes/pull/62759/merge
---
2019-07-18T17:21:41.3933183Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T17:21:41.3933214Z 
2019-07-18T17:21:41.3933369Z   git checkout -b <new-branch-name>
2019-07-18T17:21:41.3933409Z 
2019-07-18T17:21:41.3933445Z HEAD is now at a00c43f87 Merge 1aa10797df9ef70bd08011811d6a822a7b58e1a3 into 4ed008a420d725d9543d14acd0f7078a77b8bd16
2019-07-18T17:21:41.4064503Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T17:21:41.4067695Z ==============================================================================
2019-07-18T17:21:41.4067775Z Task         : Bash
2019-07-18T17:21:41.4067822Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T17:24:15.6468080Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-18T17:24:15.8441903Z Sending build context to Docker daemon  521.7kB
2019-07-18T17:24:15.8442698Z 
2019-07-18T17:24:15.8661616Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:16.6166730Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-07-18T17:24:17.7327262Z Sending build context to Docker daemon  521.7kB
2019-07-18T17:24:17.7327373Z 
2019-07-18T17:24:17.7558841Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:17.7558841Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:18.4177116Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-07-18T17:24:21.2229265Z Sending build context to Docker daemon  521.7kB
2019-07-18T17:24:21.2229964Z 
2019-07-18T17:24:21.2501426Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:21.2501426Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:21.8966071Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-07-18T17:24:25.0164748Z Sending build context to Docker daemon  521.7kB
2019-07-18T17:24:25.0165396Z 
2019-07-18T17:24:25.0393904Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:25.0393904Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:25.6742477Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-07-18T17:24:29.7493295Z Sending build context to Docker daemon  521.7kB
2019-07-18T17:24:29.7494211Z 
2019-07-18T17:24:29.7783268Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:29.7783268Z Step 1/8 : FROM ubuntu:16.04
2019-07-18T17:24:30.4018457Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-07-18T17:24:30.4031140Z The command has failed after 5 attempts.
2019-07-18T17:24:30.4142949Z ##[error]Bash exited with code '1'.
2019-07-18T17:24:30.4170956Z ##[section]Starting: Checkout
2019-07-18T17:24:30.4173037Z ==============================================================================
2019-07-18T17:24:30.4173148Z Task         : Get sources
2019-07-18T17:24:30.4173200Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
