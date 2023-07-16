plain
2019-10-15T17:48:18.1973470Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:48:18.1982882Z ##[command]git config gc.auto 0
2019-10-15T17:48:18.1984732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:48:18.1986618Z ##[command]git config --get-all http.proxy
2019-10-15T17:48:18.1989171Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65445/merge:refs/remotes/pull/65445/merge
---
2019-10-15T17:50:30.2353783Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:50:30.3241351Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:50:30.3242619Z 
2019-10-15T17:50:30.3416379Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:30.4547976Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:50:31.5411700Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:50:31.5412589Z 
2019-10-15T17:50:31.5656925Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:31.5656925Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:31.6020520Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:50:33.6945938Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:50:33.6946817Z 
2019-10-15T17:50:33.7176243Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:33.7176243Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:33.7604563Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:50:36.8524480Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:50:36.8525219Z 
2019-10-15T17:50:36.8733126Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:36.8733126Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:36.9209412Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:50:41.0123230Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:50:41.0123645Z 
2019-10-15T17:50:41.0374216Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:41.0374216Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:50:41.1414004Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:50:41.1443839Z The command has failed after 5 attempts.
2019-10-15T17:50:41.1574540Z ##[error]Bash exited with code '1'.
2019-10-15T17:50:41.1605584Z ##[section]Starting: Checkout
2019-10-15T17:50:41.1607078Z ==============================================================================
2019-10-15T17:50:41.1607121Z Task         : Get sources
2019-10-15T17:50:41.1607157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
