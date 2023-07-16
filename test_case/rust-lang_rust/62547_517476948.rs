plain
2019-08-01T21:53:23.7484079Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T21:53:23.7676665Z ##[command]git config gc.auto 0
2019-08-01T21:53:23.7755899Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T21:53:23.7796932Z ##[command]git config --get-all http.proxy
2019-08-01T21:53:23.7939104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-08-01T21:53:58.9804380Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T21:53:58.9808457Z 
2019-08-01T21:53:58.9810257Z   git checkout -b <new-branch-name>
2019-08-01T21:53:58.9811429Z 
2019-08-01T21:53:58.9812527Z HEAD is now at ffda4f707 Merge 39b5f2b4986dfcc784c75e83dfc5ab636694979d into 435236b8877cdb98c82eaebfb7887782277265c5
2019-08-01T21:53:58.9960792Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T21:53:58.9964177Z ==============================================================================
2019-08-01T21:53:58.9965261Z Task         : Bash
2019-08-01T21:53:58.9965311Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T21:57:22.2074288Z  ---> cafffb39ba8d
2019-08-01T21:57:22.2133313Z Successfully built cafffb39ba8d
2019-08-01T21:57:22.2787869Z Successfully tagged rust-ci:latest
2019-08-01T21:57:22.3226869Z Built container sha256:cafffb39ba8d8367deb800b642089e94e3ae7aabc11bd153c3d72b0060491066
2019-08-01T21:57:22.3244597Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3
2019-08-01T21:58:25.4420972Z upload failed: - to s3://rust-lang-ci-sccache2/docker/1bcedfd0fd3b0e1f17865450d97c5f933a700734ac7e085679f2dabcb79f9aa8554228d95a4689fc65827d53828df5be9ba37c197505830ce93978e771921af3 Unable to locate credentials
2019-08-01T21:58:26.3922008Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-08-01T21:58:26.3977514Z Starting sccache server...
2019-08-01T21:58:26.4879994Z configure: processing command line
2019-08-01T21:58:26.4880792Z configure: 
---
2019-08-01T22:01:56.4603721Z    Compiling serde_json v1.0.40
2019-08-01T22:02:01.0841690Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-01T22:02:10.2841488Z     Finished release [optimized] target(s) in 1m 34s
2019-08-01T22:02:10.2907159Z tidy check
2019-08-01T22:02:10.6743964Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:211: line longer than 100 chars
2019-08-01T22:02:10.6744125Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:212: line longer than 100 chars
2019-08-01T22:02:12.1996837Z some tidy checks failed
2019-08-01T22:02:12.2000364Z 
2019-08-01T22:02:12.2000364Z 
2019-08-01T22:02:12.2001663Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-01T22:02:12.2003050Z 
2019-08-01T22:02:12.2003241Z 
2019-08-01T22:02:12.2013260Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-01T22:02:12.2013703Z Build completed unsuccessfully in 0:01:37
2019-08-01T22:02:12.2013703Z Build completed unsuccessfully in 0:01:37
2019-08-01T22:02:13.5412399Z ##[error]Bash exited with code '1'.
2019-08-01T22:02:13.5445563Z ##[section]Starting: Checkout
2019-08-01T22:02:13.5447324Z ==============================================================================
2019-08-01T22:02:13.5447404Z Task         : Get sources
2019-08-01T22:02:13.5447449Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
