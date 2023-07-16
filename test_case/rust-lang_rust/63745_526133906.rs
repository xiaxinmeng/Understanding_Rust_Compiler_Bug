plain
2019-08-29T10:45:13.5274711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T10:45:13.5462575Z ##[command]git config gc.auto 0
2019-08-29T10:45:13.5543678Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T10:45:13.5602429Z ##[command]git config --get-all http.proxy
2019-08-29T10:45:13.5745804Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63745/merge:refs/remotes/pull/63745/merge
---
2019-08-29T10:51:16.3680616Z     Checking hashbrown v0.5.0
2019-08-29T10:51:18.0645483Z error[E0544]: multiple stability levels
2019-08-29T10:51:18.0646545Z  --> src/libstd/os/linux/syscall/mod.rs:2:1
2019-08-29T10:51:18.0646935Z   |
2019-08-29T10:51:18.0647366Z 2 | #![unstable(feature = "linux_syscall", issue = "63748")]
2019-08-29T10:51:18.0647972Z 
2019-08-29T10:51:19.8038560Z error: aborting due to previous error
2019-08-29T10:51:19.8039257Z 
2019-08-29T10:51:19.8374687Z error: Could not compile `std`.
---
2019-08-29T10:51:19.8452489Z == clock drift check ==
2019-08-29T10:51:19.8477600Z   local time: Thu Aug 29 10:51:19 UTC 2019
2019-08-29T10:51:19.9307915Z   network time: Thu, 29 Aug 2019 10:51:19 GMT
2019-08-29T10:51:19.9311940Z == end clock drift check ==
2019-08-29T10:51:24.2782997Z ##[error]Bash exited with code '1'.
2019-08-29T10:51:24.2810690Z ##[section]Starting: Checkout
2019-08-29T10:51:24.2812318Z ==============================================================================
2019-08-29T10:51:24.2812373Z Task         : Get sources
2019-08-29T10:51:24.2812438Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
