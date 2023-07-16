plain
2019-11-23T23:26:09.6705837Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T23:26:09.6921905Z ##[command]git config gc.auto 0
2019-11-23T23:26:09.6994396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T23:26:10.6605374Z ##[command]git config --get-all http.proxy
2019-11-23T23:26:10.6608286Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66681/merge:refs/remotes/pull/66681/merge
---
2019-11-23T23:30:44.5173977Z  ---> 6de740cc5443
2019-11-23T23:30:44.5213330Z Successfully built 6de740cc5443
2019-11-23T23:30:44.6441849Z Successfully tagged rust-ci:latest
2019-11-23T23:30:44.6746881Z Built container sha256:6de740cc54439184d46c95f3338940ceb4997f9599afd79cdde112c1444c6be1
2019-11-23T23:30:44.6764760Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/480929bc3bc019c75d577647f444b4de6781d09e6c536dbea2d03afb25b3d85ad948e63811757d340a630b00995141e1b0ee59f2e576e5c502c9156bf4ec6fc2
2019-11-23T23:31:13.6028864Z upload failed: - to s3://rust-lang-ci-sccache2/docker/480929bc3bc019c75d577647f444b4de6781d09e6c536dbea2d03afb25b3d85ad948e63811757d340a630b00995141e1b0ee59f2e576e5c502c9156bf4ec6fc2 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-11-23T23:31:14.6805829Z [CI_JOB_NAME=x86_64-gnu-tools]
2019-11-23T23:31:14.6848138Z == clock drift check ==
2019-11-23T23:31:14.6857200Z   local time: Sat Nov 23 23:31:14 UTC 2019
2019-11-23T23:31:14.9748892Z   network time: Sat, 23 Nov 2019 23:31:14 GMT
