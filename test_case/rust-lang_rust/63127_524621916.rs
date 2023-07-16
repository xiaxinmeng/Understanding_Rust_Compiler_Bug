plain
2019-08-25T08:44:24.7776689Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T08:44:24.7976920Z ##[command]git config gc.auto 0
2019-08-25T08:44:24.8058173Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T08:44:24.8106112Z ##[command]git config --get-all http.proxy
2019-08-25T08:44:24.8237754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-08-25T08:44:58.6931030Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T08:44:58.6931077Z 
2019-08-25T08:44:58.6931260Z   git checkout -b <new-branch-name>
2019-08-25T08:44:58.6931305Z 
2019-08-25T08:44:58.6931347Z HEAD is now at 8899dca0e Merge 3c18dce0414d63d765e23e420afe8b8d5b05eadd into 783469ca09005d135c3204a55069707d1cd705a9
2019-08-25T08:44:58.7149424Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T08:44:58.7153933Z ==============================================================================
2019-08-25T08:44:58.7154014Z Task         : Bash
2019-08-25T08:44:58.7154060Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T08:51:23.8619392Z * wasi 
2019-08-25T08:51:23.9100985Z some tidy checks failed
2019-08-25T08:51:23.9101710Z 
2019-08-25T08:51:23.9101742Z 
2019-08-25T08:51:23.9102604Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-25T08:51:23.9102770Z 
2019-08-25T08:51:23.9102795Z 
2019-08-25T08:51:23.9107970Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-25T08:51:23.9108309Z Build completed unsuccessfully in 0:01:38
2019-08-25T08:51:23.9108309Z Build completed unsuccessfully in 0:01:38
2019-08-25T08:51:23.9156220Z == clock drift check ==
2019-08-25T08:51:23.9171135Z   local time: Sun Aug 25 08:51:23 UTC 2019
2019-08-25T08:51:24.0024603Z   network time: Sun, 25 Aug 2019 08:51:23 GMT
2019-08-25T08:51:24.0028814Z == end clock drift check ==
2019-08-25T08:51:25.3692031Z ##[error]Bash exited with code '1'.
2019-08-25T08:51:25.3724471Z ##[section]Starting: Checkout
2019-08-25T08:51:25.3726515Z ==============================================================================
2019-08-25T08:51:25.3726560Z Task         : Get sources
2019-08-25T08:51:25.3726601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
