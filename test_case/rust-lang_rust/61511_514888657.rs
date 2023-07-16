plain
2019-07-25T03:57:51.6755665Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T03:57:51.6990806Z ##[command]git config gc.auto 0
2019-07-25T03:57:51.7072070Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T03:57:51.7139106Z ##[command]git config --get-all http.proxy
2019-07-25T03:57:51.7281746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61511/merge:refs/remotes/pull/61511/merge
---
2019-07-25T03:58:27.1725100Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T03:58:27.1725144Z 
2019-07-25T03:58:27.1725369Z   git checkout -b <new-branch-name>
2019-07-25T03:58:27.1725402Z 
2019-07-25T03:58:27.1725875Z HEAD is now at e3e270d95 Merge 4687c64f1e65f7561ab2658d9b42a2658967c8f7 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-25T03:58:27.1861661Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T03:58:27.1864821Z ==============================================================================
2019-07-25T03:58:27.1864908Z Task         : Bash
2019-07-25T03:58:27.1864958Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T04:04:57.0978163Z tidy check
2019-07-25T04:04:58.0058835Z * 577 error codes
2019-07-25T04:04:58.0059008Z * highest error code: E0732
2019-07-25T04:04:58.2982373Z * 267 features
2019-07-25T04:04:58.5476035Z tidy error: `/checkout/src/libcore/macros/mod.rs` contains `#[test]`; libcore tests must be placed inside `src/libcore/tests/`
2019-07-25T04:04:58.8562118Z some tidy checks failed
2019-07-25T04:04:58.8562250Z 
2019-07-25T04:04:58.8562250Z 
2019-07-25T04:04:58.8563495Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-25T04:04:58.8563621Z 
2019-07-25T04:04:58.8563651Z 
2019-07-25T04:04:58.8572714Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T04:04:58.8572805Z Build completed unsuccessfully in 0:01:32
2019-07-25T04:04:58.8572805Z Build completed unsuccessfully in 0:01:32
2019-07-25T04:05:00.1769564Z ##[error]Bash exited with code '1'.
2019-07-25T04:05:00.1801055Z ##[section]Starting: Checkout
2019-07-25T04:05:00.1802772Z ==============================================================================
2019-07-25T04:05:00.1802828Z Task         : Get sources
2019-07-25T04:05:00.1803027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
