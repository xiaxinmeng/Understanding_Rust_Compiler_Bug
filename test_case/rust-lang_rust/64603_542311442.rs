plain
2019-10-15T17:01:06.3406029Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:01:06.3495907Z ##[command]git config gc.auto 0
2019-10-15T17:01:06.3584104Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:01:06.3663761Z ##[command]git config --get-all http.proxy
2019-10-15T17:01:06.3808332Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64603/merge:refs/remotes/pull/64603/merge
---
2019-10-15T17:03:29.8657148Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:03:29.9721278Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:03:29.9722175Z 
2019-10-15T17:03:30.0013737Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:30.3423474Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:03:31.4847443Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:03:31.4847630Z 
2019-10-15T17:03:31.5044872Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:31.5044872Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:31.7599915Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:03:33.8595124Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:03:33.8595896Z 
2019-10-15T17:03:33.8809916Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:33.8809916Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:34.2156335Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:03:37.3300437Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:03:37.3301069Z 
2019-10-15T17:03:37.3485189Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:37.3485189Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:37.5891587Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:03:41.9928676Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:03:41.9928736Z 
2019-10-15T17:03:41.9928812Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:41.9928812Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:03:41.9928942Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:03:41.9928992Z The command has failed after 5 attempts.
2019-10-15T17:03:41.9990035Z ##[error]Bash exited with code '1'.
2019-10-15T17:03:42.0093694Z ##[section]Starting: Checkout
2019-10-15T17:03:42.0095493Z ==============================================================================
2019-10-15T17:03:42.0095563Z Task         : Get sources
2019-10-15T17:03:42.0095622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
