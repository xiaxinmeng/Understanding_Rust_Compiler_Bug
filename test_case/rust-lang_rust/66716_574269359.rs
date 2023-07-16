plain
2020-01-14T16:48:23.1522474Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T16:48:23.1534556Z ##[command]git config gc.auto 0
2020-01-14T16:48:23.1538835Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T16:48:23.1540851Z ##[command]git config --get-all http.proxy
2020-01-14T16:48:23.1546576Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66716/merge:refs/remotes/pull/66716/merge
---
2020-01-14T16:51:21.2697219Z extracting /checkout/obj/build/cache/2019-12-18/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-01-14T16:51:21.3472860Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2020-01-14T16:51:21.3472967Z 
2020-01-14T16:51:21.3473133Z Caused by:
2020-01-14T16:51:21.3473682Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2020-01-14T16:51:21.3482582Z Build completed unsuccessfully in 0:00:10
2020-01-14T16:51:21.3534595Z == clock drift check ==
2020-01-14T16:51:21.3544492Z   local time: Tue Jan 14 16:51:21 UTC 2020
2020-01-14T16:51:21.4029751Z   network time: Tue, 14 Jan 2020 16:51:21 GMT
2020-01-14T16:51:21.4029751Z   network time: Tue, 14 Jan 2020 16:51:21 GMT
2020-01-14T16:51:21.4029862Z == end clock drift check ==
2020-01-14T16:51:28.8727737Z 
2020-01-14T16:51:28.8848759Z ##[error]Bash exited with code '1'.
2020-01-14T16:51:28.8878582Z ##[section]Starting: Checkout
2020-01-14T16:51:28.8880434Z ==============================================================================
2020-01-14T16:51:28.8880489Z Task         : Get sources
2020-01-14T16:51:28.8880657Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
