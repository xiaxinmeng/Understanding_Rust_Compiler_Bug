plain
2019-09-30T23:39:14.3754788Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T23:39:14.3985719Z ##[command]git config gc.auto 0
2019-09-30T23:39:14.4092689Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T23:39:14.4169523Z ##[command]git config --get-all http.proxy
2019-09-30T23:39:14.4336371Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64935/merge:refs/remotes/pull/64935/merge
---
2019-09-30T23:47:19.3592624Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-30T23:47:19.9260791Z error[E0282]: type annotations needed
2019-09-30T23:47:19.9261153Z     --> src/librustc_errors/emitter.rs:1509:41
2019-09-30T23:47:19.9261421Z      |
2019-09-30T23:47:19.9300299Z 1509 |                     let underline_end = (span_start_pos + start + sub_len) as isize + offset;
2019-09-30T23:47:19.9300942Z      |
2019-09-30T23:47:19.9301163Z      = note: type must be known at this point
2019-09-30T23:47:19.9301194Z 
2019-09-30T23:47:20.0826755Z error: aborting due to previous error
---
2019-09-30T23:47:22.4457089Z == clock drift check ==
2019-09-30T23:47:22.4477520Z   local time: Mon Sep 30 23:47:22 UTC 2019
2019-09-30T23:47:22.6860270Z   network time: Mon, 30 Sep 2019 23:47:22 GMT
2019-09-30T23:47:22.6865142Z == end clock drift check ==
2019-09-30T23:47:23.9760595Z ##[error]Bash exited with code '1'.
2019-09-30T23:47:23.9795799Z ##[section]Starting: Checkout
2019-09-30T23:47:23.9797749Z ==============================================================================
2019-09-30T23:47:23.9797823Z Task         : Get sources
2019-09-30T23:47:23.9797870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
