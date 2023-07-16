plain
2019-08-22T20:01:19.5827217Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T20:01:19.6002486Z ##[command]git config gc.auto 0
2019-08-22T20:01:19.6087303Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T20:01:19.6182962Z ##[command]git config --get-all http.proxy
2019-08-22T20:01:19.6342033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63814/merge:refs/remotes/pull/63814/merge
---
2019-08-22T20:01:56.3229297Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T20:01:56.3231147Z 
2019-08-22T20:01:56.3232145Z   git checkout -b <new-branch-name>
2019-08-22T20:01:56.3233218Z 
2019-08-22T20:01:56.3233975Z HEAD is now at 2f4148b11 Merge 4078bb54792797934cff981daa8c933ac3ac0765 into 201e52e5fe73ccf3dd22946b1216ad8d64f8c2ba
2019-08-22T20:01:56.3377622Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T20:01:56.3380303Z ==============================================================================
2019-08-22T20:01:56.3380383Z Task         : Bash
2019-08-22T20:01:56.3380430Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T20:09:45.1555688Z    Compiling backtrace v0.3.35
2019-08-22T20:09:45.5361745Z    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-08-22T20:09:45.7372809Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-08-22T20:09:46.0483530Z    Compiling hashbrown v0.5.0
2019-08-22T20:09:47.3802982Z error: expected one of `::` or `:`, found `)`
2019-08-22T20:09:47.3803520Z   --> src/libstd/sys/wasi/mod.rs:67:37
2019-08-22T20:09:47.3803890Z    |
2019-08-22T20:09:47.3804273Z 67 | pub fn decode_error_kind(errno:: i32) -> ErrorKind {
2019-08-22T20:09:47.3804694Z    |                                     ^ expected one of `::` or `:` here
2019-08-22T20:09:50.0667877Z error: aborting due to previous error
2019-08-22T20:09:50.0668705Z 
2019-08-22T20:09:50.1078852Z error: Could not compile `std`.
2019-08-22T20:09:50.1080086Z 
---
2019-08-22T20:09:50.1214279Z == clock drift check ==
2019-08-22T20:09:50.1234821Z   local time: Thu Aug 22 20:09:50 UTC 2019
2019-08-22T20:09:50.2236095Z   network time: Thu, 22 Aug 2019 20:09:50 GMT
2019-08-22T20:09:50.2238339Z == end clock drift check ==
2019-08-22T20:09:53.8515816Z ##[error]Bash exited with code '1'.
2019-08-22T20:09:53.8563205Z ##[section]Starting: Checkout
2019-08-22T20:09:53.8565284Z ==============================================================================
2019-08-22T20:09:53.8565344Z Task         : Get sources
2019-08-22T20:09:53.8565396Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
