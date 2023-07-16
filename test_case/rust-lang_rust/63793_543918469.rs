plain
2019-10-18T19:57:30.3352317Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T19:57:30.3529390Z ##[command]git config gc.auto 0
2019-10-18T19:57:30.3615741Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T19:57:30.3674900Z ##[command]git config --get-all http.proxy
2019-10-18T19:57:30.3843785Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63793/merge:refs/remotes/pull/63793/merge
---
2019-10-18T20:04:07.4984777Z Successfully built 8bca76fe03a6
2019-10-18T20:04:07.5942258Z Successfully tagged rust-ci:latest
2019-10-18T20:04:07.6413549Z Built container sha256:8bca76fe03a6763488c15dda7f4829e10c6fe329ca3a4508873f9ffb4f611f69
2019-10-18T20:04:07.6443703Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/05b7aa214422df826ddee7c673e265d99cbffc5c6131af0644d932cc63250480b0454de0bfc7971401fdd78d073112a34629aa615ffacd86a349840aa057a6af
2019-10-18T20:05:34.7883967Z upload failed: - to s3://rust-lang-ci-sccache2/docker/05b7aa214422df826ddee7c673e265d99cbffc5c6131af0644d932cc63250480b0454de0bfc7971401fdd78d073112a34629aa615ffacd86a349840aa057a6af An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-10-18T20:05:35.6970961Z [CI_JOB_NAME=mingw-check]
2019-10-18T20:05:35.7006636Z == clock drift check ==
2019-10-18T20:05:35.7017867Z   local time: Fri Oct 18 20:05:35 UTC 2019
2019-10-18T20:05:36.2348543Z   network time: Fri, 18 Oct 2019 20:05:36 GMT
---
2019-10-18T20:07:31.9073657Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-10-18T20:07:32.0317534Z error: attributes are not yet allowed on `if` expressions
2019-10-18T20:07:32.0318384Z    --> src/libcore/iter/adapters/mod.rs:520:13
2019-10-18T20:07:32.0318631Z     |
2019-10-18T20:07:32.0318928Z 520 |             #[cfg(boostrap_stdarch_ignore_this)]
2019-10-18T20:07:32.0319304Z 
2019-10-18T20:07:32.0429579Z error: aborting due to previous error
2019-10-18T20:07:32.0429671Z 
2019-10-18T20:07:32.0442471Z error: could not compile `core`.
---
2019-10-18T20:07:36.8800126Z   local time: Fri Oct 18 20:07:36 UTC 2019
2019-10-18T20:07:36.9678332Z   network time: Fri, 18 Oct 2019 20:07:36 GMT
2019-10-18T20:07:36.9678731Z == end clock drift check ==
2019-10-18T20:07:51.1388839Z 
2019-10-18T20:07:51.1506831Z ##[error]Bash exited with code '1'.
2019-10-18T20:07:51.1553206Z ##[section]Starting: Checkout
2019-10-18T20:07:51.1555260Z ==============================================================================
2019-10-18T20:07:51.1555325Z Task         : Get sources
2019-10-18T20:07:51.1555395Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
