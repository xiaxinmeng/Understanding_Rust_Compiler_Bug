plain
2019-07-17T16:26:39.1491408Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T16:26:39.1745909Z ##[command]git config gc.auto 0
2019-07-17T16:26:39.1811732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T16:26:39.1868328Z ##[command]git config --get-all http.proxy
2019-07-17T16:26:39.2000753Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62757/merge:refs/remotes/pull/62757/merge
---
2019-07-17T16:27:14.5691997Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T16:27:14.5692264Z 
2019-07-17T16:27:14.5692735Z   git checkout -b <new-branch-name>
2019-07-17T16:27:14.5693037Z 
2019-07-17T16:27:14.5693296Z HEAD is now at bdad9767c Merge 5138972056e1c002d0c550b16c86a3e9bb594844 into e34d4ae8692f93155cbf1135ebea4c3fd37abf4e
2019-07-17T16:27:14.5827115Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T16:27:14.5830170Z ==============================================================================
2019-07-17T16:27:14.5830249Z Task         : Bash
2019-07-17T16:27:14.5830316Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T16:30:39.2216266Z ###################                                                       27.7%
2019-07-17T16:30:39.2217197Z ######################################################################## 100.0%
2019-07-17T16:30:40.1895598Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-17T16:30:41.4481503Z     Updating crates.io index
2019-07-17T16:31:02.5924126Z error: failed to write /checkout/Cargo.lock
2019-07-17T16:31:02.5924391Z Caused by:
2019-07-17T16:31:02.5924442Z   failed to open: /checkout/Cargo.lock
2019-07-17T16:31:02.5924473Z 
2019-07-17T16:31:02.5924517Z Caused by:
2019-07-17T16:31:02.5924517Z Caused by:
2019-07-17T16:31:02.5925298Z   Read-only file system (os error 30)
2019-07-17T16:31:02.5945934Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-07-17T16:31:02.5946038Z Build completed unsuccessfully in 0:00:38
2019-07-17T16:31:09.9467371Z ##[error]Bash exited with code '1'.
2019-07-17T16:31:09.9502208Z ##[section]Starting: Checkout
2019-07-17T16:31:09.9503891Z ==============================================================================
2019-07-17T16:31:09.9503966Z Task         : Get sources
2019-07-17T16:31:09.9504036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
