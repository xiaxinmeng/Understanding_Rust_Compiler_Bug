plain
2019-11-19T21:35:53.1280514Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T21:35:53.1472875Z ##[command]git config gc.auto 0
2019-11-19T21:35:53.1558365Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T21:35:53.1633818Z ##[command]git config --get-all http.proxy
2019-11-19T21:35:53.1778623Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66558/merge:refs/remotes/pull/66558/merge
---
2019-11-19T21:39:10.4064694Z ##############################                                            43.0%
2019-11-19T21:39:10.4065049Z ######################################################################## 100.0%
2019-11-19T21:39:10.8155709Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-19T21:39:10.8989819Z     Updating crates.io index
2019-11-19T21:39:17.6866430Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-19T21:39:17.6902485Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-19T21:39:17.6952289Z Makefile:67: recipe for target 'prepare' failed
2019-11-19T21:39:17.6952806Z make: *** [prepare] Error 1
2019-11-19T21:39:18.6969049Z Command failed. Attempt 2/5:
2019-11-19T21:39:19.6575176Z     Updating crates.io index
2019-11-19T21:39:19.6575176Z     Updating crates.io index
2019-11-19T21:39:19.6576327Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-19T21:39:19.6576816Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-19T21:39:19.6576939Z make: *** [prepare] Error 1
2019-11-19T21:39:19.6577268Z Makefile:67: recipe for target 'prepare' failed
2019-11-19T21:39:21.1993860Z Command failed. Attempt 3/5:
2019-11-19T21:39:21.3304249Z     Updating crates.io index
2019-11-19T21:39:21.3304249Z     Updating crates.io index
2019-11-19T21:39:21.6881183Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-19T21:39:21.6908337Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-19T21:39:21.6961165Z Makefile:67: recipe for target 'prepare' failed
2019-11-19T21:39:21.6961288Z make: *** [prepare] Error 1
2019-11-19T21:39:24.6980789Z Command failed. Attempt 4/5:
2019-11-19T21:39:24.8238145Z     Updating crates.io index
2019-11-19T21:39:24.8238145Z     Updating crates.io index
2019-11-19T21:39:25.1602737Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-19T21:39:25.1627669Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-19T21:39:25.1683342Z Makefile:67: recipe for target 'prepare' failed
2019-11-19T21:39:25.1683471Z make: *** [prepare] Error 1
2019-11-19T21:39:29.1701200Z Command failed. Attempt 5/5:
2019-11-19T21:39:29.2941861Z     Updating crates.io index
2019-11-19T21:39:29.2941861Z     Updating crates.io index
2019-11-19T21:39:29.6327320Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-19T21:39:29.6349705Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-19T21:39:29.6405518Z make: *** [prepare] Error 1
2019-11-19T21:39:29.6406425Z Makefile:67: recipe for target 'prepare' failed
2019-11-19T21:39:29.6406921Z The command has failed after 5 attempts.
2019-11-19T21:39:29.6408242Z == clock drift check ==
2019-11-19T21:39:29.6408242Z == clock drift check ==
2019-11-19T21:39:29.6414567Z   local time: Tue Nov 19 21:39:29 UTC 2019
2019-11-19T21:39:29.6773159Z   network time: Tue, 19 Nov 2019 21:39:29 GMT
2019-11-19T21:39:29.6776546Z == end clock drift check ==
2019-11-19T21:39:42.9503507Z 
2019-11-19T21:39:42.9602318Z ##[error]Bash exited with code '1'.
2019-11-19T21:39:42.9635265Z ##[section]Starting: Checkout
2019-11-19T21:39:42.9637130Z ==============================================================================
2019-11-19T21:39:42.9637190Z Task         : Get sources
2019-11-19T21:39:42.9637239Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
