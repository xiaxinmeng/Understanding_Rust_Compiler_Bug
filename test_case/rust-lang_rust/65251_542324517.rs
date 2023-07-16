plain
2019-10-15T17:34:15.1171487Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:34:15.1181939Z ##[command]git config gc.auto 0
2019-10-15T17:34:15.1184130Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:34:15.1186886Z ##[command]git config --get-all http.proxy
2019-10-15T17:34:15.1189969Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65251/merge:refs/remotes/pull/65251/merge
---
2019-10-15T17:36:53.1275865Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:36:53.3524704Z Sending build context to Docker daemon  521.2kB
2019-10-15T17:36:53.3524883Z 
2019-10-15T17:36:53.3759264Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:36:54.0643436Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:36:55.1558183Z Sending build context to Docker daemon  521.2kB
2019-10-15T17:36:55.1559600Z 
2019-10-15T17:36:55.1798755Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:36:55.1798755Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:36:55.4422993Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:36:57.5343440Z Sending build context to Docker daemon  521.2kB
2019-10-15T17:36:57.5344143Z 
2019-10-15T17:36:57.5639544Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:36:57.5639544Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:36:57.8028680Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:37:00.9030245Z Sending build context to Docker daemon  521.2kB
2019-10-15T17:37:00.9031222Z 
2019-10-15T17:37:00.9237248Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:37:00.9237248Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:37:01.1664036Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:37:05.2481733Z Sending build context to Docker daemon  521.2kB
2019-10-15T17:37:05.2484586Z 
2019-10-15T17:37:05.2677978Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:37:05.2677978Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:37:05.5526494Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:37:05.5539834Z The command has failed after 5 attempts.
2019-10-15T17:37:05.5676271Z ##[error]Bash exited with code '1'.
2019-10-15T17:37:05.5735726Z ##[section]Starting: Checkout
2019-10-15T17:37:05.5737679Z ==============================================================================
2019-10-15T17:37:05.5737734Z Task         : Get sources
2019-10-15T17:37:05.5737807Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
