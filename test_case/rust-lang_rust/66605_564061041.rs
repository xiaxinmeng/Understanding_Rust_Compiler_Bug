plain
2019-12-10T12:43:49.2029591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T12:43:49.2230757Z ##[command]git config gc.auto 0
2019-12-10T12:43:49.2300573Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T12:43:49.2351787Z ##[command]git config --get-all http.proxy
2019-12-10T12:43:49.2506984Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66605/merge:refs/remotes/pull/66605/merge
---
2019-12-10T12:48:29.6624781Z Successfully built 99b2394420c7
2019-12-10T12:48:30.0715762Z Successfully tagged rust-ci:latest
2019-12-10T12:48:30.0994646Z Built container sha256:99b2394420c7f446d7a8a32adbb6f0ba373765f5ce3ccd1aca0e7fb2e0a57935
2019-12-10T12:48:30.1020986Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/2781c8e6f0d7ff5702addf75da3252d9fe2c3b87f17f4cf2f6886d0417f523eab2b6e8b3a56c5580ab060276d097ffa9209fe0fe08f69277e52760e31cdece60
2019-12-10T12:49:35.3161931Z upload failed: - to s3://rust-lang-ci-sccache2/docker/2781c8e6f0d7ff5702addf75da3252d9fe2c3b87f17f4cf2f6886d0417f523eab2b6e8b3a56c5580ab060276d097ffa9209fe0fe08f69277e52760e31cdece60 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-12-10T12:49:37.3346415Z [CI_JOB_NAME=mingw-check]
2019-12-10T12:49:37.3348911Z == clock drift check ==
2019-12-10T12:49:37.3349477Z   local time: Tue Dec 10 12:49:36 UTC 2019
2019-12-10T12:49:37.3349750Z   network time: Tue, 10 Dec 2019 12:49:36 GMT
---
2019-12-10T12:49:37.3352197Z   local time: Tue Dec 10 12:49:37 UTC 2019
2019-12-10T12:49:37.3352245Z   network time: Tue, 10 Dec 2019 12:49:37 GMT
2019-12-10T12:49:37.3352289Z == end clock drift check ==
2019-12-10T12:49:37.5835846Z 
2019-12-10T12:49:37.5956437Z ##[error]Bash exited with code '127'.
2019-12-10T12:49:37.5984455Z ##[section]Starting: Checkout
2019-12-10T12:49:37.5985920Z ==============================================================================
2019-12-10T12:49:37.5985968Z Task         : Get sources
2019-12-10T12:49:37.5986023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
