plain
2019-10-15T17:44:07.9423685Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T17:44:07.9523894Z ##[command]git config gc.auto 0
2019-10-15T17:44:07.9606526Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T17:44:07.9660325Z ##[command]git config --get-all http.proxy
2019-10-15T17:44:07.9824733Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65444/merge:refs/remotes/pull/65444/merge
---
2019-10-15T17:46:24.0952441Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-10-15T17:46:24.1901750Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:46:24.1903934Z 
2019-10-15T17:46:24.2169356Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:24.6125983Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:46:25.7151765Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:46:25.7151963Z 
2019-10-15T17:46:25.7311909Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:25.7311909Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:26.0007819Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:46:28.1339385Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:46:28.1339627Z 
2019-10-15T17:46:28.1550343Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:28.1550343Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:28.4202955Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:46:31.5482278Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:46:31.5483324Z 
2019-10-15T17:46:31.5706629Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:31.5706629Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:31.8508018Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:46:35.9473740Z Sending build context to Docker daemon  526.3kB
2019-10-15T17:46:35.9474690Z 
2019-10-15T17:46:35.9715312Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:35.9715312Z Step 1/6 : FROM ubuntu:16.04
2019-10-15T17:46:36.2123373Z received unexpected HTTP status: 503 Service Unavailable
2019-10-15T17:46:36.2138611Z The command has failed after 5 attempts.
2019-10-15T17:46:36.2291589Z ##[error]Bash exited with code '1'.
2019-10-15T17:46:36.2330875Z ##[section]Starting: Checkout
2019-10-15T17:46:36.2332709Z ==============================================================================
2019-10-15T17:46:36.2332807Z Task         : Get sources
2019-10-15T17:46:36.2332853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
