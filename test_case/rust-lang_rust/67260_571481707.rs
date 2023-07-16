plain
2020-01-07T08:09:01.0306950Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T08:09:01.0390892Z ##[command]git config gc.auto 0
2020-01-07T08:09:01.0469242Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T08:09:01.0533185Z ##[command]git config --get-all http.proxy
2020-01-07T08:09:01.0686349Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67260/merge:refs/remotes/pull/67260/merge
---
2020-01-07T08:11:32.7939426Z extracting /checkout/obj/build/cache/2019-12-18/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-01-07T08:11:32.8766071Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2020-01-07T08:11:32.8766218Z 
2020-01-07T08:11:32.8766271Z Caused by:
2020-01-07T08:11:32.8767110Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2020-01-07T08:11:32.8774758Z Build completed unsuccessfully in 0:00:10
2020-01-07T08:11:32.8825748Z == clock drift check ==
2020-01-07T08:11:32.8846580Z   local time: Tue Jan  7 08:11:32 UTC 2020
2020-01-07T08:11:32.9108995Z   network time: Tue, 07 Jan 2020 08:11:32 GMT
2020-01-07T08:11:32.9108995Z   network time: Tue, 07 Jan 2020 08:11:32 GMT
2020-01-07T08:11:32.9109355Z == end clock drift check ==
2020-01-07T08:11:40.3872266Z 
2020-01-07T08:11:40.3987435Z ##[error]Bash exited with code '1'.
2020-01-07T08:11:40.4021752Z ##[section]Starting: Checkout
2020-01-07T08:11:40.4023920Z ==============================================================================
2020-01-07T08:11:40.4023992Z Task         : Get sources
2020-01-07T08:11:40.4024039Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
