plain
2019-10-18T19:32:36.0474132Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T19:32:36.0691695Z ##[command]git config gc.auto 0
2019-10-18T19:32:36.0772512Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T19:32:36.0848380Z ##[command]git config --get-all http.proxy
2019-10-18T19:32:36.0998993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65168/merge:refs/remotes/pull/65168/merge
---
2019-10-18T19:38:18.1115921Z Successfully built ef0ffffa25ef
2019-10-18T19:38:18.1930419Z Successfully tagged rust-ci:latest
2019-10-18T19:38:18.2440103Z Built container sha256:ef0ffffa25ef151593a009680b55f82e70577174e8f730eeea5b38f5b10f3768
2019-10-18T19:38:18.2458464Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/590c864a5f2335de0a8287407f9a96030510ec86a676b9b48333a6a037ff1c9ca74007b9ad184866d613750a3d2a1a88c9ad4e657c82d0b1a088ee62c541c4ca
2019-10-18T19:39:21.4517441Z upload failed: - to s3://rust-lang-ci-sccache2/docker/590c864a5f2335de0a8287407f9a96030510ec86a676b9b48333a6a037ff1c9ca74007b9ad184866d613750a3d2a1a88c9ad4e657c82d0b1a088ee62c541c4ca An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-10-18T19:39:22.0843409Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-10-18T19:39:22.0870780Z == clock drift check ==
2019-10-18T19:39:22.0900324Z   local time: Fri Oct 18 19:39:22 UTC 2019
2019-10-18T19:39:22.3521868Z   network time: Fri, 18 Oct 2019 19:39:22 GMT
---
2019-10-18T19:43:30.7860507Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-18T19:43:31.3148197Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-18T19:43:37.1604810Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-10-18T19:43:39.3405853Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2019-10-18T19:43:39.8142029Z error[E0541]: unknown meta item 'issue'
2019-10-18T19:43:39.8142506Z     --> src/liballoc/boxed.rs:1097:41
2019-10-18T19:43:39.8142843Z      |
2019-10-18T19:43:39.8143247Z 1097 | #[stable(feature = "box_str_from_iter", issue = "0", since = "1.39.0")]
2019-10-18T19:43:39.8145724Z      |                                         ^^^^^^^^^^^ expected one of `since`, `note`
2019-10-18T19:43:40.0079683Z    Compiling cfg-if v0.1.8
2019-10-18T19:43:40.0504724Z    Compiling rustc-demangle v0.1.16
2019-10-18T19:43:41.0479706Z error: aborting due to previous error
2019-10-18T19:43:41.0480217Z 
---
2019-10-18T19:43:41.5699258Z   local time: Fri Oct 18 19:43:41 UTC 2019
2019-10-18T19:43:41.7076146Z   network time: Fri, 18 Oct 2019 19:43:41 GMT
2019-10-18T19:43:41.7081689Z == end clock drift check ==
2019-10-18T19:43:44.3507720Z 
2019-10-18T19:43:44.3683602Z ##[error]Bash exited with code '1'.
2019-10-18T19:43:44.3723980Z ##[section]Starting: Checkout
2019-10-18T19:43:44.3725859Z ==============================================================================
2019-10-18T19:43:44.3725952Z Task         : Get sources
2019-10-18T19:43:44.3726019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
