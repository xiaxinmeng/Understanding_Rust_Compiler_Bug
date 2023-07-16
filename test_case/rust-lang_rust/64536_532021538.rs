plain
2019-09-17T01:26:52.7740256Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T01:26:52.7999736Z ##[command]git config gc.auto 0
2019-09-17T01:26:52.8063539Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T01:26:52.8122624Z ##[command]git config --get-all http.proxy
2019-09-17T01:26:52.8295658Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64536/merge:refs/remotes/pull/64536/merge
---
2019-09-17T01:30:16.6168700Z ############################################################              83.8%
2019-09-17T01:30:16.6171867Z ######################################################################## 100.0%
2019-09-17T01:30:20.6200905Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-17T01:30:20.7126574Z     Updating crates.io index
2019-09-17T01:30:57.4054094Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-17T01:30:57.4146131Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-17T01:30:57.4160840Z Makefile:67: recipe for target 'prepare' failed
2019-09-17T01:30:57.4161135Z make: *** [prepare] Error 1
2019-09-17T01:30:58.4185019Z Command failed. Attempt 2/5:
2019-09-17T01:30:58.4185019Z Command failed. Attempt 2/5:
2019-09-17T01:30:58.6340431Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-17T01:30:58.6362318Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-17T01:30:58.6408664Z Makefile:67: recipe for target 'prepare' failed
2019-09-17T01:30:58.6408825Z make: *** [prepare] Error 1
2019-09-17T01:31:00.6423535Z Command failed. Attempt 3/5:
2019-09-17T01:31:00.6423535Z Command failed. Attempt 3/5:
2019-09-17T01:31:00.8564690Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-17T01:31:00.8578090Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-17T01:31:00.8645414Z Makefile:67: recipe for target 'prepare' failed
2019-09-17T01:31:00.8651778Z make: *** [prepare] Error 1
2019-09-17T01:31:03.8663857Z Command failed. Attempt 4/5:
2019-09-17T01:31:03.8663857Z Command failed. Attempt 4/5:
2019-09-17T01:31:04.0823622Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-17T01:31:04.0836833Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-17T01:31:04.0884061Z make: *** [prepare] Error 1
2019-09-17T01:31:04.0887293Z Makefile:67: recipe for target 'prepare' failed
2019-09-17T01:31:08.0911180Z Command failed. Attempt 5/5:
2019-09-17T01:31:08.0911180Z Command failed. Attempt 5/5:
2019-09-17T01:31:08.3085791Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-09-17T01:31:08.3103984Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-09-17T01:31:08.3149860Z make: *** [prepare] Error 1
2019-09-17T01:31:08.3156564Z Makefile:67: recipe for target 'prepare' failed
2019-09-17T01:31:08.3156676Z The command has failed after 5 attempts.
2019-09-17T01:31:08.3156719Z == clock drift check ==
2019-09-17T01:31:08.3156719Z == clock drift check ==
2019-09-17T01:31:08.3164809Z   local time: Tue Sep 17 01:31:08 UTC 2019
2019-09-17T01:31:08.4042213Z   network time: Tue, 17 Sep 2019 01:31:08 GMT
2019-09-17T01:31:08.4047502Z == end clock drift check ==
2019-09-17T01:31:09.5155085Z ##[error]Bash exited with code '1'.
2019-09-17T01:31:09.5190640Z ##[section]Starting: Checkout
2019-09-17T01:31:09.5192513Z ==============================================================================
2019-09-17T01:31:09.5192588Z Task         : Get sources
2019-09-17T01:31:09.5192636Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
