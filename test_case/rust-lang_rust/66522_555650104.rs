plain
2019-11-19T18:39:40.7392580Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T18:39:40.7600140Z ##[command]git config gc.auto 0
2019-11-19T18:39:40.7689238Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T18:39:40.7743188Z ##[command]git config --get-all http.proxy
2019-11-19T18:39:40.7908660Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66522/merge:refs/remotes/pull/66522/merge
---
2019-11-19T18:42:23.6554635Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-11-19T18:42:23.8303994Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:42:23.8304247Z 
2019-11-19T18:42:23.8708609Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:24.5221227Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:42:25.6338616Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:42:25.6338856Z 
2019-11-19T18:42:25.6498291Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:25.6498291Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:26.7173336Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:42:28.3400393Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:42:28.3400769Z 
2019-11-19T18:42:28.3618580Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:28.3618580Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:28.8721990Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:42:32.0770280Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:42:32.0771015Z 
2019-11-19T18:42:32.0979617Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:32.0979617Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:32.6030157Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:42:36.7416024Z Sending build context to Docker daemon  528.4kB
2019-11-19T18:42:36.7416687Z 
2019-11-19T18:42:36.7617495Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:36.7617495Z Step 1/8 : FROM ubuntu:16.04
2019-11-19T18:42:37.2824441Z Get https://registry-1.docker.io/v2/library/ubuntu/manifests/16.04: received unexpected HTTP status: 500 Internal Server Error
2019-11-19T18:42:37.2887737Z 
2019-11-19T18:42:37.2887737Z 
2019-11-19T18:42:37.2998110Z ##[error]Bash exited with code '1'.
2019-11-19T18:42:37.3029312Z ##[section]Starting: Checkout
2019-11-19T18:42:37.3031887Z ==============================================================================
2019-11-19T18:42:37.3031942Z Task         : Get sources
2019-11-19T18:42:37.3032009Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
