plain
2019-07-19T19:26:21.7688197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T19:26:21.7935837Z ##[command]git config gc.auto 0
2019-07-19T19:26:21.8006279Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T19:26:21.8056776Z ##[command]git config --get-all http.proxy
2019-07-19T19:26:21.8185808Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62812/merge:refs/remotes/pull/62812/merge
---
2019-07-19T19:26:55.5084032Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T19:26:55.5084077Z 
2019-07-19T19:26:55.5084256Z   git checkout -b <new-branch-name>
2019-07-19T19:26:55.5084282Z 
2019-07-19T19:26:55.5084332Z HEAD is now at 20bf1a144 Merge b11f1d3b11ac3dce87cfe09e90c10aad364f5627 into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T19:26:55.5225089Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T19:26:55.5227512Z ==============================================================================
2019-07-19T19:26:55.5227721Z Task         : Bash
2019-07-19T19:26:55.5227762Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T19:33:06.4162022Z tidy check
2019-07-19T19:33:07.3292375Z * 577 error codes
2019-07-19T19:33:07.3292580Z * highest error code: E0732
2019-07-19T19:33:07.6383303Z * 266 features
2019-07-19T19:33:08.1914231Z Stray file with UI testing output: "/checkout/src/test/ui/feature-gates/feature-gate-dropck-ugeh.stderr"
2019-07-19T19:33:08.2206669Z some tidy checks failed
2019-07-19T19:33:08.2212229Z 
2019-07-19T19:33:08.2212229Z 
2019-07-19T19:33:08.2212930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-19T19:33:08.2213347Z 
2019-07-19T19:33:08.2213396Z 
2019-07-19T19:33:08.2227636Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-19T19:33:08.2227738Z Build completed unsuccessfully in 0:01:31
2019-07-19T19:33:08.2227738Z Build completed unsuccessfully in 0:01:31
2019-07-19T19:33:09.5356175Z ##[error]Bash exited with code '1'.
2019-07-19T19:33:09.5410564Z ##[section]Starting: Checkout
2019-07-19T19:33:09.5412121Z ==============================================================================
2019-07-19T19:33:09.5412172Z Task         : Get sources
2019-07-19T19:33:09.5412216Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
