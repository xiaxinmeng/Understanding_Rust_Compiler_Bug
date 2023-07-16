plain
2019-07-26T01:08:31.3810554Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T01:08:31.4008157Z ##[command]git config gc.auto 0
2019-07-26T01:08:31.4095818Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T01:08:31.4146211Z ##[command]git config --get-all http.proxy
2019-07-26T01:08:31.4292868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61937/merge:refs/remotes/pull/61937/merge
---
2019-07-26T01:09:06.2582004Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T01:09:06.2582048Z 
2019-07-26T01:09:06.2582309Z   git checkout -b <new-branch-name>
2019-07-26T01:09:06.2582348Z 
2019-07-26T01:09:06.2582429Z HEAD is now at 080f4b6ff Merge ec5523b0028dc93eb10b914f08368779655902d9 into 890881f8f4c77e8670d4b32104c0325fcfefc90f
2019-07-26T01:09:06.2738074Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T01:09:06.2741338Z ==============================================================================
2019-07-26T01:09:06.2741403Z Task         : Bash
2019-07-26T01:09:06.2741472Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T01:15:17.0878392Z    Compiling serde_json v1.0.40
2019-07-26T01:15:21.3105105Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T01:15:29.8907431Z     Finished release [optimized] target(s) in 1m 28s
2019-07-26T01:15:29.8993340Z tidy check
2019-07-26T01:15:30.0998107Z tidy error: /checkout/src/libcore/slice/rotate.rs: missing trailing newline
2019-07-26T01:15:31.6972789Z some tidy checks failed
2019-07-26T01:15:31.6977019Z 
2019-07-26T01:15:31.6977019Z 
2019-07-26T01:15:31.6978817Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T01:15:31.6982446Z 
2019-07-26T01:15:31.6982719Z 
2019-07-26T01:15:31.6983124Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T01:15:31.6983428Z Build completed unsuccessfully in 0:01:31
2019-07-26T01:15:31.6983428Z Build completed unsuccessfully in 0:01:31
2019-07-26T01:15:33.0503816Z ##[error]Bash exited with code '1'.
2019-07-26T01:15:33.0544224Z ##[section]Starting: Checkout
2019-07-26T01:15:33.0546258Z ==============================================================================
2019-07-26T01:15:33.0546312Z Task         : Get sources
2019-07-26T01:15:33.0546379Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
