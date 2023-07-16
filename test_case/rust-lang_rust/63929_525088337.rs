plain
2019-08-27T00:41:26.6101642Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T00:41:26.6335981Z ##[command]git config gc.auto 0
2019-08-27T00:41:26.6416524Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T00:41:26.6483297Z ##[command]git config --get-all http.proxy
2019-08-27T00:41:26.6633920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63929/merge:refs/remotes/pull/63929/merge
---
2019-08-27T00:42:03.1190607Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T00:42:03.1190636Z 
2019-08-27T00:42:03.1190828Z   git checkout -b <new-branch-name>
2019-08-27T00:42:03.1190857Z 
2019-08-27T00:42:03.1191112Z HEAD is now at e0c087ea9 Merge bd4b6712181b49c5110208d1a5c4378e2788fd95 into 9b91b9c10e3c87ed333a1e34c4f46ed68f1eee06
2019-08-27T00:42:03.1354258Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T00:42:03.1359004Z ==============================================================================
2019-08-27T00:42:03.1359078Z Task         : Bash
2019-08-27T00:42:03.1359144Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T00:45:33.4439117Z ##################################################                        69.5%
2019-08-27T00:45:33.4439254Z ######################################################################## 100.0%
2019-08-27T00:45:33.8680762Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-27T00:45:33.9511740Z     Updating crates.io index
2019-08-27T00:46:07.9760438Z     Updating git repository `https://github.com/djrenren/libtest.git`
2019-08-27T00:46:08.7939251Z error: failed to write /checkout/Cargo.lock
2019-08-27T00:46:08.7943324Z Caused by:
2019-08-27T00:46:08.7943374Z   failed to open: /checkout/Cargo.lock
2019-08-27T00:46:08.7943405Z 
2019-08-27T00:46:08.7944038Z Caused by:
2019-08-27T00:46:08.7944038Z Caused by:
2019-08-27T00:46:08.7944814Z   Read-only file system (os error 30)
2019-08-27T00:46:08.7984410Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-27T00:46:08.8038831Z == clock drift check ==
2019-08-27T00:46:08.8048520Z   local time: Tue Aug 27 00:46:08 UTC 2019
2019-08-27T00:46:08.9522107Z   network time: Tue, 27 Aug 2019 00:46:08 GMT
2019-08-27T00:46:08.9526209Z == end clock drift check ==
2019-08-27T00:46:08.9526209Z == end clock drift check ==
2019-08-27T00:46:10.1458443Z ##[error]Bash exited with code '1'.
2019-08-27T00:46:10.1491669Z ##[section]Starting: Checkout
2019-08-27T00:46:10.1496496Z ==============================================================================
2019-08-27T00:46:10.1496578Z Task         : Get sources
2019-08-27T00:46:10.1496621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
