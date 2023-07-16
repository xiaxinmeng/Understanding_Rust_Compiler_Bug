plain
2020-02-22T00:02:34.9880461Z ========================== Starting Command Output ===========================
2020-02-22T00:02:34.9883967Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b4761e97-69a6-4c6d-a967-6fff8261363d.sh
2020-02-22T00:02:34.9884391Z 
2020-02-22T00:02:34.9888154Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T00:02:34.9908133Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-22T00:02:34.9911593Z Task         : Get sources
2020-02-22T00:02:34.9911922Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T00:02:34.9912238Z Version      : 1.0.0
2020-02-22T00:02:34.9912471Z Author       : Microsoft
---
2020-02-22T00:02:36.1817737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T00:02:36.1826065Z ##[command]git config gc.auto 0
2020-02-22T00:02:36.1830856Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T00:02:36.1836201Z ##[command]git config --get-all http.proxy
2020-02-22T00:02:36.1845465Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69084/merge:refs/remotes/pull/69084/merge
---
2020-02-22T00:06:01.0494631Z Successfully built 34a69d9dd0ee
2020-02-22T00:06:01.0587213Z Successfully tagged rust-ci:latest
2020-02-22T00:06:01.1578721Z Built container sha256:34a69d9dd0ee8690fc6db18ad26b8d3cd2756daa6b573f75d41e978b1f5a223a
2020-02-22T00:06:01.1597820Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/d42805294727747964a78ce70c906cffcf151e0ac7404a3765966e44959ef24b29c11dc98d21205141b0951a747cfe58c33808b212e0a5b1f239c2c509aa3b06
2020-02-22T00:06:47.3558149Z upload failed: - to s3://rust-lang-ci-sccache2/docker/d42805294727747964a78ce70c906cffcf151e0ac7404a3765966e44959ef24b29c11dc98d21205141b0951a747cfe58c33808b212e0a5b1f239c2c509aa3b06 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-02-22T00:06:47.8290511Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-02-22T00:06:47.8330180Z == clock drift check ==
2020-02-22T00:06:47.8343656Z   local time: Sat Feb 22 00:06:47 UTC 2020
2020-02-22T00:06:48.1291676Z   network time: Sat, 22 Feb 2020 00:06:48 GMT
