plain
2019-12-19T07:23:07.5135151Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T07:23:07.5302552Z ##[command]git config gc.auto 0
2019-12-19T07:23:07.5369569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T07:23:07.5430834Z ##[command]git config --get-all http.proxy
2019-12-19T07:23:07.5569374Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67163/merge:refs/remotes/pull/67163/merge
---
2019-12-19T07:28:34.2898013Z Successfully built e559ce28d3d8
2019-12-19T07:28:34.4356546Z Successfully tagged rust-ci:latest
2019-12-19T07:28:34.4992310Z Built container sha256:e559ce28d3d8eae685794f24b7974f0d16b9141e5425e077f25e214422c0941d
2019-12-19T07:28:34.5013036Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/a4e8b7e8bb02ac2100a242c404ec9cad308c2d13ab7005efa7a2f5ca462ea44fe3b309afeb06c439a0091371f6a14487ea26f96e8916366f1b586cbeecbe4c35
2019-12-19T07:29:35.3690010Z upload failed: - to s3://rust-lang-ci-sccache2/docker/a4e8b7e8bb02ac2100a242c404ec9cad308c2d13ab7005efa7a2f5ca462ea44fe3b309afeb06c439a0091371f6a14487ea26f96e8916366f1b586cbeecbe4c35 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-12-19T07:29:36.4019490Z [CI_JOB_NAME=mingw-check]
2019-12-19T07:29:36.4047631Z == clock drift check ==
2019-12-19T07:29:36.4050908Z   local time: Thu Dec 19 07:29:36 UTC 2019
2019-12-19T07:29:36.9382972Z   network time: Thu, 19 Dec 2019 07:29:36 GMT
---
2019-12-19T07:31:00.9063798Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-19T07:31:04.5150685Z error[E0547]: missing 'issue'
2019-12-19T07:31:04.5151889Z    --> src/libcore/ptr/const_ptr.rs:284:27
2019-12-19T07:31:04.5152311Z     |
2019-12-19T07:31:04.5152988Z 284 |     #[cfg_attr(bootstrap, rustc_const_unstable(feature = "const_ptr_offset_from"))]
2019-12-19T07:31:04.5153598Z 
2019-12-19T07:31:04.5169212Z error[E0547]: missing 'issue'
2019-12-19T07:31:04.5169815Z    --> src/libcore/ptr/mut_ptr.rs:322:27
2019-12-19T07:31:04.5170449Z     |
2019-12-19T07:31:04.5170449Z     |
2019-12-19T07:31:04.5170944Z 322 |     #[cfg_attr(bootstrap, rustc_const_unstable(feature = "const_ptr_offset_from"))]
2019-12-19T07:31:04.5171557Z 
2019-12-19T07:31:08.6632331Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-19T07:31:10.2010228Z    Compiling libc v0.2.64
2019-12-19T07:31:10.9905855Z error: aborting due to 2 previous errors
---
2019-12-19T07:31:11.0859447Z   local time: Thu Dec 19 07:31:11 UTC 2019
2019-12-19T07:31:11.3616231Z   network time: Thu, 19 Dec 2019 07:31:11 GMT
2019-12-19T07:31:11.3620645Z == end clock drift check ==
2019-12-19T07:31:25.4102292Z 
2019-12-19T07:31:25.4202557Z ##[error]Bash exited with code '1'.
2019-12-19T07:31:25.4266326Z ##[section]Starting: Checkout
2019-12-19T07:31:25.4268319Z ==============================================================================
2019-12-19T07:31:25.4268375Z Task         : Get sources
2019-12-19T07:31:25.4268443Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
