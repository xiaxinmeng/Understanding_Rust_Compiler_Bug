plain
2019-11-25T03:03:34.4052497Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T03:03:35.2774893Z ##[command]git config gc.auto 0
2019-11-25T03:03:35.2779034Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T03:03:35.2783113Z ##[command]git config --get-all http.proxy
2019-11-25T03:03:35.2786522Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66613/merge:refs/remotes/pull/66613/merge
---
2019-11-25T03:07:15.8511973Z ####################                                                      29.0%
2019-11-25T03:07:15.8512163Z ######################################################################## 100.0%
2019-11-25T03:07:16.2442519Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-25T03:07:16.3222883Z     Updating crates.io index
2019-11-25T03:07:23.4454198Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T03:07:23.4483244Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T03:07:23.4526771Z make: *** [prepare] Error 1
2019-11-25T03:07:23.4528295Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T03:07:24.4550129Z Command failed. Attempt 2/5:
2019-11-25T03:07:24.4550129Z Command failed. Attempt 2/5:
2019-11-25T03:07:24.6704016Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T03:07:24.6721691Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T03:07:24.6769967Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T03:07:24.6770052Z make: *** [prepare] Error 1
2019-11-25T03:07:26.6785098Z Command failed. Attempt 3/5:
2019-11-25T03:07:26.6785098Z Command failed. Attempt 3/5:
2019-11-25T03:07:26.8864416Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T03:07:26.8877522Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T03:07:26.8922587Z make: *** [prepare] Error 1
2019-11-25T03:07:26.8923222Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T03:07:29.8941739Z Command failed. Attempt 4/5:
2019-11-25T03:07:29.8941739Z Command failed. Attempt 4/5:
2019-11-25T03:07:30.1002283Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T03:07:30.1019351Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T03:07:30.1061239Z make: *** [prepare] Error 1
2019-11-25T03:07:30.1066075Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T03:07:34.1077759Z Command failed. Attempt 5/5:
2019-11-25T03:07:34.1077759Z Command failed. Attempt 5/5:
2019-11-25T03:07:34.3076848Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-11-25T03:07:34.3093969Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-25T03:07:34.3144349Z make: *** [prepare] Error 1
2019-11-25T03:07:34.3144742Z Makefile:67: recipe for target 'prepare' failed
2019-11-25T03:07:34.3144816Z The command has failed after 5 attempts.
2019-11-25T03:07:34.3147875Z == clock drift check ==
2019-11-25T03:07:34.3147875Z == clock drift check ==
2019-11-25T03:07:34.3176714Z   local time: Mon Nov 25 03:07:34 UTC 2019
2019-11-25T03:07:34.3675154Z   network time: Mon, 25 Nov 2019 03:07:34 GMT
2019-11-25T03:07:34.3675214Z == end clock drift check ==
2019-11-25T03:07:47.6034710Z 
2019-11-25T03:07:47.6131680Z ##[error]Bash exited with code '1'.
2019-11-25T03:07:47.6167064Z ##[section]Starting: Checkout
2019-11-25T03:07:47.6168512Z ==============================================================================
2019-11-25T03:07:47.6168556Z Task         : Get sources
2019-11-25T03:07:47.6168614Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
