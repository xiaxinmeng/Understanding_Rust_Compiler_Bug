plain
2019-10-15T17:37:18.3630623Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:37:18.3641998Z ##[command]git config gc.auto 0
2019-10-15T17:37:18.3644631Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:37:18.3647204Z ##[command]git config --get-all http.proxy
2019-10-15T17:37:18.3649592Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65068/merge:refs/remotes/pull/65068/merge
---
2019-10-15T17:39:41.8127766Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:39:42.1485130Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:39:42.1487527Z 
2019-10-15T17:39:42.1495843Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:42.9816067Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:39:44.0766020Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:39:44.0766992Z 
2019-10-15T17:39:44.0957310Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:44.0957310Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:44.6460506Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:39:46.7446765Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:39:46.7447636Z 
2019-10-15T17:39:46.7635619Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:46.7635619Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:47.3169493Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:39:50.4104493Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:39:50.4104821Z 
2019-10-15T17:39:50.4319267Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:50.4319267Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:50.9751883Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:39:55.0844681Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:39:55.0844829Z 
2019-10-15T17:39:55.1078388Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:55.1078388Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:39:55.7365414Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:39:55.7378172Z The command has failed after 5 attempts.
2019-10-15T17:39:55.7526376Z ##[error]Bash exited with code '1'.
2019-10-15T17:39:55.7561160Z ##[section]Starting: Checkout
2019-10-15T17:39:55.7562879Z ==============================================================================
2019-10-15T17:39:55.7562931Z Task         : Get sources
2019-10-15T17:39:55.7563606Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
