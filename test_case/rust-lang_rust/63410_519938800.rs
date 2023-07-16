plain
2019-08-09T14:01:21.3488780Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T14:01:21.3678565Z ##[command]git config gc.auto 0
2019-08-09T14:01:21.3763634Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T14:01:21.3814433Z ##[command]git config --get-all http.proxy
2019-08-09T14:01:21.3963911Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63410/merge:refs/remotes/pull/63410/merge
---
2019-08-09T14:01:56.4839570Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T14:01:56.4839601Z 
2019-08-09T14:01:56.4839818Z   git checkout -b <new-branch-name>
2019-08-09T14:01:56.4839849Z 
2019-08-09T14:01:56.4839917Z HEAD is now at dc5009d6a Merge a801a8ff4833c5e69bb3ab4881ffa1462636c9a5 into 813a3a5d4b2be4e2101faa73a067da02a704a598
2019-08-09T14:01:56.5054491Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T14:01:56.5061814Z ==============================================================================
2019-08-09T14:01:56.5061890Z Task         : Bash
2019-08-09T14:01:56.5061935Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T14:08:15.2569430Z    Compiling serde_json v1.0.40
2019-08-09T14:08:19.7642873Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-09T14:08:28.8692642Z     Finished release [optimized] target(s) in 1m 33s
2019-08-09T14:08:28.8770780Z tidy check
2019-08-09T14:08:29.4050233Z tidy error: /checkout/src/libstd/io/buffered.rs:368: trailing whitespace
2019-08-09T14:08:29.4050644Z tidy error: /checkout/src/libstd/io/buffered.rs:401: trailing whitespace
2019-08-09T14:08:31.3139368Z some tidy checks failed
2019-08-09T14:08:31.3140129Z 
2019-08-09T14:08:31.3140129Z 
2019-08-09T14:08:31.3141147Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-09T14:08:31.3141319Z 
2019-08-09T14:08:31.3141345Z 
2019-08-09T14:08:31.3141394Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-09T14:08:31.3141464Z Build completed unsuccessfully in 0:01:37
2019-08-09T14:08:31.3141464Z Build completed unsuccessfully in 0:01:37
2019-08-09T14:08:33.2944453Z ##[error]Bash exited with code '1'.
2019-08-09T14:08:33.2985989Z ##[section]Starting: Checkout
2019-08-09T14:08:33.2987641Z ==============================================================================
2019-08-09T14:08:33.2987853Z Task         : Get sources
2019-08-09T14:08:33.2987921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
