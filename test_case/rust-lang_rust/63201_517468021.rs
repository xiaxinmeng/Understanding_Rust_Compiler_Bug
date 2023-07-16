plain
2019-08-01T21:20:36.6690942Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T21:20:36.6906621Z ##[command]git config gc.auto 0
2019-08-01T21:20:36.6979109Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T21:20:36.7049573Z ##[command]git config --get-all http.proxy
2019-08-01T21:20:36.7200702Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63201/merge:refs/remotes/pull/63201/merge
---
2019-08-01T21:21:11.7201498Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T21:21:11.7201559Z 
2019-08-01T21:21:11.7201788Z   git checkout -b <new-branch-name>
2019-08-01T21:21:11.7201840Z 
2019-08-01T21:21:11.7201896Z HEAD is now at 37c3cdc2c Merge e82314764554270b264ccf5bc536c2d616e82899 into 435236b8877cdb98c82eaebfb7887782277265c5
2019-08-01T21:21:11.7368334Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T21:21:11.7371495Z ==============================================================================
2019-08-01T21:21:11.7371550Z Task         : Bash
2019-08-01T21:21:11.7371596Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T21:24:42.3814277Z  ---> 6dade7a1acf6
2019-08-01T21:24:42.3857285Z Successfully built 6dade7a1acf6
2019-08-01T21:24:42.4427221Z Successfully tagged rust-ci:latest
2019-08-01T21:24:42.4938209Z Built container sha256:6dade7a1acf659422882bb85a0af8472c9bc41d0974364480fbd497d41268ea4
2019-08-01T21:24:42.4961248Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3
2019-08-01T21:25:43.6469034Z upload failed: - to s3://rust-lang-ci-sccache2/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3 Unable to locate credentials
2019-08-01T21:25:44.6023626Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-01T21:25:44.6072703Z Starting sccache server...
2019-08-01T21:25:44.6973117Z configure: processing command line
2019-08-01T21:25:44.6973225Z configure: 
---
2019-08-01T21:29:24.4784293Z     Finished release [optimized] target(s) in 1m 28s
2019-08-01T21:29:24.4850522Z tidy check
2019-08-01T21:29:25.3330068Z * 578 error codes
2019-08-01T21:29:25.3330191Z * highest error code: E0733
2019-08-01T21:29:25.7086810Z tidy error: Found 1 features without a gate test.
2019-08-01T21:29:25.7088634Z Expected a gate test for the feature 'cfg_doctest'.
2019-08-01T21:29:25.7089397Z Hint: create a failing test file named 'feature-gate-cfg_doctest.rs'
2019-08-01T21:29:25.7090039Z       in the 'ui' test suite, with its failures due to
2019-08-01T21:29:25.7090378Z       missing usage of `#![feature(cfg_doctest)]`.
2019-08-01T21:29:25.7090962Z Hint: If you already have such a test and don't want to rename it,
2019-08-01T21:29:25.7091636Z       you can also add a // gate-test-cfg_doctest line to the test file.
2019-08-01T21:29:26.3675860Z some tidy checks failed
2019-08-01T21:29:26.3686985Z 
2019-08-01T21:29:26.3686985Z 
2019-08-01T21:29:26.3688119Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-01T21:29:26.3688527Z 
2019-08-01T21:29:26.3688580Z 
2019-08-01T21:29:26.3700104Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-01T21:29:26.3700187Z Build completed unsuccessfully in 0:01:31
2019-08-01T21:29:26.3700187Z Build completed unsuccessfully in 0:01:31
2019-08-01T21:29:27.7039772Z ##[error]Bash exited with code '1'.
2019-08-01T21:29:27.7074326Z ##[section]Starting: Checkout
2019-08-01T21:29:27.7076262Z ==============================================================================
2019-08-01T21:29:27.7076346Z Task         : Get sources
2019-08-01T21:29:27.7076401Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
