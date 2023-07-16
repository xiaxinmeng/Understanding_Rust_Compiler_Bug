plain
2019-08-17T18:55:34.5788369Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T18:55:34.6011322Z ##[command]git config gc.auto 0
2019-08-17T18:55:34.6080771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T18:55:34.6146794Z ##[command]git config --get-all http.proxy
2019-08-17T18:55:34.6289631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63521/merge:refs/remotes/pull/63521/merge
---
2019-08-17T18:56:09.1715492Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T18:56:09.1715678Z 
2019-08-17T18:56:09.1716052Z   git checkout -b <new-branch-name>
2019-08-17T18:56:09.1716296Z 
2019-08-17T18:56:09.1716408Z HEAD is now at 51197a926 Merge 51dcdcfd94450851d64c6d7c4eeab41c3e61f546 into d65e272a9fe3e61aa5f229c5358e35a909435575
2019-08-17T18:56:09.1881361Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T18:56:09.1884332Z ==============================================================================
2019-08-17T18:56:09.1884731Z Task         : Bash
2019-08-17T18:56:09.1884814Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T19:00:07.4438798Z Caused by:
2019-08-17T19:00:07.4438962Z   failed to open: /checkout/Cargo.lock
2019-08-17T19:00:07.4438991Z 
2019-08-17T19:00:07.4439032Z Caused by:
2019-08-17T19:00:07.4439755Z   Read-only file system (os error 30)
2019-08-17T19:00:07.4479904Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-17T19:00:07.4543709Z == clock drift check ==
2019-08-17T19:00:07.4557007Z   local time: Sat Aug 17 19:00:07 UTC 2019
2019-08-17T19:00:07.6046113Z   network time: Sat, 17 Aug 2019 19:00:07 GMT
2019-08-17T19:00:07.6046241Z == end clock drift check ==
2019-08-17T19:00:07.6046241Z == end clock drift check ==
2019-08-17T19:00:08.9853284Z ##[error]Bash exited with code '1'.
2019-08-17T19:00:08.9891447Z ##[section]Starting: Checkout
2019-08-17T19:00:08.9893563Z ==============================================================================
2019-08-17T19:00:08.9893783Z Task         : Get sources
2019-08-17T19:00:08.9893891Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
