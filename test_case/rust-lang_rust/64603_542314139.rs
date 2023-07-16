plain
2019-10-15T17:07:55.1423032Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:07:55.1433169Z ##[command]git config gc.auto 0
2019-10-15T17:07:55.1435540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:07:55.1437546Z ##[command]git config --get-all http.proxy
2019-10-15T17:07:55.1440228Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64603/merge:refs/remotes/pull/64603/merge
---
2019-10-15T17:10:19.3614163Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:10:19.4503337Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:10:19.4504392Z 
2019-10-15T17:10:19.4717628Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:19.5921216Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:10:20.6838602Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:10:20.6839101Z 
2019-10-15T17:10:20.7115877Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:20.7115877Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:20.7530550Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:10:22.8384846Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:10:22.8386287Z 
2019-10-15T17:10:22.8628899Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:22.8628899Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:22.9102021Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:10:26.0030173Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:10:26.0031167Z 
2019-10-15T17:10:26.0270647Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:26.0270647Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:26.0680858Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:10:30.1545296Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:10:31.1718918Z 
2019-10-15T17:10:31.1721113Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:31.1721113Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:10:31.1721357Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:10:31.1721621Z The command has failed after 5 attempts.
2019-10-15T17:10:31.1786476Z ##[error]Bash exited with code '1'.
2019-10-15T17:10:31.1893650Z ##[section]Starting: Checkout
2019-10-15T17:10:31.1895396Z ==============================================================================
2019-10-15T17:10:31.1895474Z Task         : Get sources
2019-10-15T17:10:31.1895522Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
