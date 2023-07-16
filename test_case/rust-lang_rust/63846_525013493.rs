plain
2019-08-26T20:06:03.9325204Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T20:06:04.5780329Z ##[command]git config gc.auto 0
2019-08-26T20:06:04.5786748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T20:06:04.5789909Z ##[command]git config --get-all http.proxy
2019-08-26T20:06:04.5795564Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63846/merge:refs/remotes/pull/63846/merge
---
2019-08-26T20:06:40.8209940Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T20:06:40.8209976Z 
2019-08-26T20:06:40.8210210Z   git checkout -b <new-branch-name>
2019-08-26T20:06:40.8210257Z 
2019-08-26T20:06:40.8210309Z HEAD is now at 50b31556c Merge 1478ef45287c4da0da0daf1715d00e8b06fa333b into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T20:06:40.8369153Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T20:06:40.8372040Z ==============================================================================
2019-08-26T20:06:40.8372123Z Task         : Bash
2019-08-26T20:06:40.8372175Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T20:12:30.1991759Z     Checking hashbrown v0.5.0
2019-08-26T20:12:31.1337194Z error: expected item after doc comment
2019-08-26T20:12:31.1337971Z   --> src/libstd/time.rs:73:1
2019-08-26T20:12:31.1338331Z    |
2019-08-26T20:12:31.1338625Z 73 | /// |  Windows  | [QueryPerformanceCounter] |
2019-08-26T20:12:31.1340415Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this doc comment doesn't document anything
2019-08-26T20:12:31.1341602Z error: aborting due to previous error
2019-08-26T20:12:31.1341910Z 
2019-08-26T20:12:31.1413422Z error: Could not compile `std`.
2019-08-26T20:12:31.1413506Z 
---
2019-08-26T20:12:31.1501932Z == clock drift check ==
2019-08-26T20:12:31.1518418Z   local time: Mon Aug 26 20:12:31 UTC 2019
2019-08-26T20:12:31.3080973Z   network time: Mon, 26 Aug 2019 20:12:31 GMT
2019-08-26T20:12:31.3081239Z == end clock drift check ==
2019-08-26T20:12:39.1653666Z ##[error]Bash exited with code '1'.
2019-08-26T20:12:39.1701544Z ##[section]Starting: Checkout
2019-08-26T20:12:39.1703542Z ==============================================================================
2019-08-26T20:12:39.1703622Z Task         : Get sources
2019-08-26T20:12:39.1703674Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
