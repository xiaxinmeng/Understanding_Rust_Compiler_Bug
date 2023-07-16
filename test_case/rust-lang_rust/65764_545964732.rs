plain
2019-10-24T15:07:41.3793241Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:07:41.3954605Z ##[command]git config gc.auto 0
2019-10-24T15:07:41.4029002Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:07:41.4087325Z ##[command]git config --get-all http.proxy
2019-10-24T15:07:41.4219469Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65764/merge:refs/remotes/pull/65764/merge
---
2019-10-24T15:11:09.1107828Z 
2019-10-24T15:11:09.1108935Z ######################################################################## 100.0%
2019-10-24T15:11:09.4954677Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-10-24T15:11:09.5770176Z     Updating crates.io index
2019-10-24T15:11:13.7395896Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-24T15:11:13.7426706Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-24T15:11:13.7471291Z make: *** [prepare] Error 1
2019-10-24T15:11:13.7474779Z Makefile:67: recipe for target 'prepare' failed
2019-10-24T15:11:14.7502745Z Command failed. Attempt 2/5:
2019-10-24T15:11:14.8696361Z     Updating crates.io index
2019-10-24T15:11:14.8696361Z     Updating crates.io index
2019-10-24T15:11:16.0973991Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-24T15:11:16.0978652Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-24T15:11:16.1044757Z Makefile:67: recipe for target 'prepare' failed
2019-10-24T15:11:16.1044861Z make: *** [prepare] Error 1
2019-10-24T15:11:18.1055460Z Command failed. Attempt 3/5:
2019-10-24T15:11:18.2234748Z     Updating crates.io index
2019-10-24T15:11:18.2234748Z     Updating crates.io index
2019-10-24T15:11:19.3231561Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-24T15:11:19.3232535Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-24T15:11:19.3233206Z Makefile:67: recipe for target 'prepare' failed
2019-10-24T15:11:19.3233407Z make: *** [prepare] Error 1
2019-10-24T15:11:21.5817587Z Command failed. Attempt 4/5:
2019-10-24T15:11:21.6896348Z     Updating crates.io index
2019-10-24T15:11:21.6896348Z     Updating crates.io index
2019-10-24T15:11:22.0153768Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-24T15:11:22.0175735Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-24T15:11:22.0219256Z Makefile:67: recipe for target 'prepare' failed
2019-10-24T15:11:22.0219633Z make: *** [prepare] Error 1
2019-10-24T15:11:26.0238621Z Command failed. Attempt 5/5:
2019-10-24T15:11:26.1382372Z     Updating crates.io index
2019-10-24T15:11:26.1382372Z     Updating crates.io index
2019-10-24T15:11:26.4602011Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-10-24T15:11:26.4624118Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-10-24T15:11:26.4671309Z Makefile:67: recipe for target 'prepare' failed
2019-10-24T15:11:26.4671393Z make: *** [prepare] Error 1
2019-10-24T15:11:26.4671444Z The command has failed after 5 attempts.
2019-10-24T15:11:26.4671566Z == clock drift check ==
2019-10-24T15:11:26.4671566Z == clock drift check ==
2019-10-24T15:11:26.4691868Z   local time: Thu Oct 24 15:11:26 UTC 2019
2019-10-24T15:11:28.6300614Z   network time: == end clock drift check ==
2019-10-24T15:11:34.6344839Z 
2019-10-24T15:11:34.6450199Z ##[error]Bash exited with code '1'.
2019-10-24T15:11:34.6523114Z ##[section]Starting: Checkout
2019-10-24T15:11:34.6524541Z ==============================================================================
2019-10-24T15:11:34.6524583Z Task         : Get sources
2019-10-24T15:11:34.6524621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
