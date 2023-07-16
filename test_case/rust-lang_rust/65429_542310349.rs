plain
2019-10-15T16:57:35.1712707Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T16:57:35.1804152Z ##[command]git config gc.auto 0
2019-10-15T16:57:35.1884953Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T16:57:35.1938824Z ##[command]git config --get-all http.proxy
2019-10-15T16:57:35.2094824Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65429/merge:refs/remotes/pull/65429/merge
---
2019-10-15T17:00:21.4857963Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:00:21.6703609Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:00:21.6704240Z 
2019-10-15T17:00:21.6946229Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:25.1862123Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:00:26.2788583Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:00:26.2789496Z 
2019-10-15T17:00:26.3027128Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:26.3027128Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:26.6353336Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:00:28.7262421Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:00:28.7263528Z 
2019-10-15T17:00:28.7506685Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:28.7506685Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:29.0114079Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:00:32.1030909Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:00:32.1032087Z 
2019-10-15T17:00:32.1311971Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:32.1311971Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:32.4044169Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:00:36.5001369Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:00:36.5001449Z 
2019-10-15T17:00:36.5223116Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:36.5223116Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:00:36.8583126Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:00:36.8596631Z The command has failed after 5 attempts.
2019-10-15T17:00:36.8755638Z ##[error]Bash exited with code '1'.
2019-10-15T17:00:36.8793309Z ##[section]Starting: Checkout
2019-10-15T17:00:36.8795315Z ==============================================================================
2019-10-15T17:00:36.8795369Z Task         : Get sources
2019-10-15T17:00:36.8795433Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
