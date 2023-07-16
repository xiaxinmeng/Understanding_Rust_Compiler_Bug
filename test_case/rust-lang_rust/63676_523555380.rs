plain
2019-08-21T16:58:23.1362749Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T16:58:23.1542852Z ##[command]git config gc.auto 0
2019-08-21T16:58:23.1631166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T16:58:23.1703029Z ##[command]git config --get-all http.proxy
2019-08-21T16:58:23.1848328Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-21T16:59:00.6525399Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T16:59:00.6525494Z 
2019-08-21T16:59:00.6526137Z   git checkout -b <new-branch-name>
2019-08-21T16:59:00.6526232Z 
2019-08-21T16:59:00.6526568Z HEAD is now at f3a2b4260 Merge 926f36400f1667edec92959d8b640dea5084674c into 7b0085a613e69cb69fc9e4eb5d422fa4a39d5de1
2019-08-21T16:59:00.6703955Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T16:59:00.6706893Z ==============================================================================
2019-08-21T16:59:00.6706954Z Task         : Bash
2019-08-21T16:59:00.6707000Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T17:02:28.3162964Z ##################################################################        92.6%
2019-08-21T17:02:28.3163631Z ######################################################################## 100.0%
2019-08-21T17:02:28.7342457Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-21T17:02:28.8172743Z     Updating crates.io index
2019-08-21T17:03:03.4813514Z     Updating git repository `https://github.com/newpavlov/rust-wasi`
2019-08-21T17:03:04.1506497Z error: failed to write /checkout/Cargo.lock
2019-08-21T17:03:04.1507146Z Caused by:
2019-08-21T17:03:04.1507464Z   failed to open: /checkout/Cargo.lock
2019-08-21T17:03:04.1508464Z 
2019-08-21T17:03:04.1508532Z Caused by:
2019-08-21T17:03:04.1508532Z Caused by:
2019-08-21T17:03:04.1509330Z   Read-only file system (os error 30)
2019-08-21T17:03:04.1547038Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-21T17:03:04.1592098Z == clock drift check ==
2019-08-21T17:03:04.1602237Z   local time: Wed Aug 21 17:03:04 UTC 2019
2019-08-21T17:03:04.2329727Z   network time: Wed, 21 Aug 2019 17:03:04 GMT
2019-08-21T17:03:04.2330391Z == end clock drift check ==
2019-08-21T17:03:04.2330391Z == end clock drift check ==
2019-08-21T17:03:05.5254013Z ##[error]Bash exited with code '1'.
2019-08-21T17:03:05.5287423Z ##[section]Starting: Checkout
2019-08-21T17:03:05.5289723Z ==============================================================================
2019-08-21T17:03:05.5289782Z Task         : Get sources
2019-08-21T17:03:05.5289829Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
