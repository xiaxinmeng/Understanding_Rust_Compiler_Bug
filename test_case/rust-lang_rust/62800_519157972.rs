plain
2019-08-07T15:47:54.5311116Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T15:47:54.5458746Z ##[command]git config gc.auto 0
2019-08-07T15:47:54.5517959Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T15:47:54.5567802Z ##[command]git config --get-all http.proxy
2019-08-07T15:47:54.5682387Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62800/merge:refs/remotes/pull/62800/merge
---
2019-08-07T15:48:28.8411922Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T15:48:28.8411952Z 
2019-08-07T15:48:28.8412293Z   git checkout -b <new-branch-name>
2019-08-07T15:48:28.8412320Z 
2019-08-07T15:48:28.8412362Z HEAD is now at ffbad7b7f Merge 2d78371fe87c8110ce7bcce7058eda7316c62b6a into d4abb08be6c3a06a14e285396f5e3ef367584f77
2019-08-07T15:48:28.8557097Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T15:48:28.8559394Z ==============================================================================
2019-08-07T15:48:28.8559458Z Task         : Bash
2019-08-07T15:48:28.8559493Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T15:51:38.0028462Z downloading https://static.rust-lang.org/dist/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-07T15:51:38.2328104Z 
2019-08-07T15:51:38.2329231Z ######################################################################## 100.0%
2019-08-07T15:51:39.7871926Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-07T15:51:39.8117912Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-08-07T15:51:39.8118833Z Caused by:
2019-08-07T15:51:39.8118933Z   No such file or directory (os error 2)
2019-08-07T15:51:39.8118933Z   No such file or directory (os error 2)
2019-08-07T15:51:39.8127459Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-07T15:51:39.8127763Z Build completed unsuccessfully in 0:00:15
2019-08-07T15:51:57.8875482Z ##[error]Bash exited with code '1'.
2019-08-07T15:51:57.8920591Z ##[section]Starting: Checkout
2019-08-07T15:51:57.8922418Z ==============================================================================
2019-08-07T15:51:57.8922479Z Task         : Get sources
2019-08-07T15:51:57.8922571Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
