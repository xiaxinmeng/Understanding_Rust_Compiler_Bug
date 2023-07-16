plain
2019-08-01T23:34:30.6978723Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T23:34:30.7170942Z ##[command]git config gc.auto 0
2019-08-01T23:34:30.7248443Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T23:34:31.4749937Z ##[command]git config --get-all http.proxy
2019-08-01T23:34:31.4756868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63206/merge:refs/remotes/pull/63206/merge
---
2019-08-01T23:35:04.3953657Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T23:35:04.3954507Z 
2019-08-01T23:35:04.3955541Z   git checkout -b <new-branch-name>
2019-08-01T23:35:04.3956363Z 
2019-08-01T23:35:04.3956949Z HEAD is now at 8c870c33f Merge 6f16dc4d8fea444d8be6aff2b41bd36c36a7ab1b into 435236b8877cdb98c82eaebfb7887782277265c5
2019-08-01T23:35:04.4125845Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T23:35:04.4128844Z ==============================================================================
2019-08-01T23:35:04.4128904Z Task         : Bash
2019-08-01T23:35:04.4128950Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T23:38:35.6418303Z  ---> 5ee34b4b6e56
2019-08-01T23:38:35.6462453Z Successfully built 5ee34b4b6e56
2019-08-01T23:38:35.7081314Z Successfully tagged rust-ci:latest
2019-08-01T23:38:35.7670551Z Built container sha256:5ee34b4b6e56d04704cc85e34f55bed4b7db0312738b8ff65b183d4ffaabb1dd
2019-08-01T23:38:35.7688939Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3
2019-08-01T23:39:37.1503995Z upload failed: - to s3://rust-lang-ci-sccache2/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3 Unable to locate credentials
2019-08-01T23:39:38.0441118Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-01T23:39:38.0487180Z Starting sccache server...
2019-08-01T23:39:38.1392640Z configure: processing command line
2019-08-01T23:39:38.1392891Z configure: 
---
2019-08-01T23:43:01.9543566Z    Compiling serde_json v1.0.40
2019-08-01T23:43:06.3036417Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-01T23:43:14.7501125Z     Finished release [optimized] target(s) in 1m 27s
2019-08-01T23:43:14.7564149Z tidy check
2019-08-01T23:43:15.2520094Z tidy error: /checkout/src/libstd/sys/vxworks/process/process_common.rs: too many trailing newlines (2)
2019-08-01T23:43:16.5277296Z some tidy checks failed
2019-08-01T23:43:16.5277442Z 
2019-08-01T23:43:16.5277442Z 
2019-08-01T23:43:16.5278287Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-01T23:43:16.5278613Z 
2019-08-01T23:43:16.5278635Z 
2019-08-01T23:43:16.5282107Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-01T23:43:16.5282176Z Build completed unsuccessfully in 0:01:30
2019-08-01T23:43:16.5282176Z Build completed unsuccessfully in 0:01:30
2019-08-01T23:43:17.8155840Z ##[error]Bash exited with code '1'.
2019-08-01T23:43:17.8208266Z ##[section]Starting: Checkout
2019-08-01T23:43:17.8209768Z ==============================================================================
2019-08-01T23:43:17.8209815Z Task         : Get sources
2019-08-01T23:43:17.8210023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
