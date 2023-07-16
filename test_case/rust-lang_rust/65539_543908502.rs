plain
2019-10-18T19:19:06.7369368Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T19:19:06.7593553Z ##[command]git config gc.auto 0
2019-10-18T19:19:06.7642316Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T19:19:06.7706270Z ##[command]git config --get-all http.proxy
2019-10-18T19:19:06.7854482Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65539/merge:refs/remotes/pull/65539/merge
---
2019-10-18T19:24:50.3801883Z Successfully built 0e8cdefea362
2019-10-18T19:24:50.4785154Z Successfully tagged rust-ci:latest
2019-10-18T19:24:50.5057111Z Built container sha256:0e8cdefea362ba19ae027a75cad02b65a4bb6b30707257f0da8ea2492404256f
2019-10-18T19:24:50.5075892Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/590c864a5f2335de0a8287407f9a96030510ec86a676b9b48333a6a037ff1c9ca74007b9ad184866d613750a3d2a1a88c9ad4e657c82d0b1a088ee62c541c4ca
2019-10-18T19:25:52.4288218Z upload failed: - to s3://rust-lang-ci-sccache2/docker/590c864a5f2335de0a8287407f9a96030510ec86a676b9b48333a6a037ff1c9ca74007b9ad184866d613750a3d2a1a88c9ad4e657c82d0b1a088ee62c541c4ca An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-10-18T19:25:53.5985073Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-10-18T19:25:53.6020467Z == clock drift check ==
2019-10-18T19:25:53.6030327Z   local time: Fri Oct 18 19:25:53 UTC 2019
2019-10-18T19:25:53.8943870Z   network time: Fri, 18 Oct 2019 19:25:53 GMT
---
2019-10-18T19:29:08.2898981Z    Compiling serde_json v1.0.40
2019-10-18T19:29:10.1240476Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-18T19:29:21.9834085Z     Finished release [optimized] target(s) in 1m 31s
2019-10-18T19:29:21.9919159Z tidy check
2019-10-18T19:29:22.3954068Z tidy error: /checkout/src/test/ui/imports/reexports.rs:9: line longer than 100 chars
2019-10-18T19:29:24.2420077Z some tidy checks failed
2019-10-18T19:29:24.2420951Z Found 482 error codes
2019-10-18T19:29:24.2421280Z Found 0 error codes with no tests
2019-10-18T19:29:24.2421516Z Done!
2019-10-18T19:29:24.2421516Z Done!
2019-10-18T19:29:24.2421722Z 
2019-10-18T19:29:24.2421930Z 
2019-10-18T19:29:24.2423061Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-18T19:29:24.2423694Z 
2019-10-18T19:29:24.2423890Z 
2019-10-18T19:29:24.2449522Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-18T19:29:24.2449944Z Build completed unsuccessfully in 0:01:34
2019-10-18T19:29:24.2449944Z Build completed unsuccessfully in 0:01:34
2019-10-18T19:29:24.2478443Z == clock drift check ==
2019-10-18T19:29:24.2511295Z   local time: Fri Oct 18 19:29:24 UTC 2019
2019-10-18T19:29:24.3455777Z   network time: Fri, 18 Oct 2019 19:29:24 GMT
2019-10-18T19:29:24.3455867Z == end clock drift check ==
2019-10-18T19:29:25.6154767Z 
2019-10-18T19:29:25.6259765Z ##[error]Bash exited with code '1'.
2019-10-18T19:29:25.6302368Z ##[section]Starting: Checkout
2019-10-18T19:29:25.6304348Z ==============================================================================
2019-10-18T19:29:25.6304402Z Task         : Get sources
2019-10-18T19:29:25.6304465Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
